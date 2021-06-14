use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use std::error::Error;
use std::sync::Arc;

use crate::model::{Action, Condition, Event, RuleContext, Trigger};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Rule {
    #[schemars(skip)]
    pub id: Uuid,
    #[schemars(skip)]
    pub guild_id: u64,
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
        event: Arc<Event>,
        mut ctx: &mut RuleContext<'_>,
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

        tracing::debug!("Rule triggered on {:?}", event.kind().name());

        // Run all actions in order if passes conditions
        for action in &self.actions {
            action.execute(event.clone(), &mut ctx).await?;
        }

        Ok(true)
    }
}
