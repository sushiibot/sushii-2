use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use std::error::Error;
use std::sync::Arc;
use twilight_model::gateway::event::DispatchEvent;

use crate::model::{Action, Condition, RuleContext, Trigger};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Rule {
    pub id: Uuid,
    /// Name of this rule
    pub name: String,
    /// If this rule is enabled or not
    pub enabled: bool,
    /// Event that triggers this rule
    pub trigger: Trigger,
    /// Conditions that need to pass before running actions
    pub conditions: Condition,
    /// Actions are executed sequentially if condition passes
    pub actions: Vec<Action>,
}

impl Rule {
    pub async fn check_event(
        &self,
        event: Arc<DispatchEvent>,
        ctx: &RuleContext<'_>,
    ) -> Result<bool, Box<dyn Error>> {
        // if event.kind() != self.trigger {
        //     return None;
        // }

        if !self.enabled {
            return Ok(false);
        }

        let passes_conditions = self.conditions.check_event(event.clone(), ctx).await?;

        if !passes_conditions {
            return Ok(false);
        }

        metrics::increment_counter!("rule_triggered", "event_name" => event.kind().name().unwrap_or("UNKNOWN"));

        tracing::debug!("Rule triggered");

        // Run all actions in order if passes conditions
        for action in &self.actions {
            action.execute(event.clone(), &ctx).await?;
        }

        Ok(true)
    }
}
