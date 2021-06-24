use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use std::error::Error;
use std::sync::Arc;
use sqlx::types::Json;
use std::result::Result as StdResult;

use crate::error::Result;
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
    ) -> StdResult<bool, Box<dyn Error>> {
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

        metrics::increment_counter!("rule_triggered", "event_name" => event
            .kind()
            .ok()
            .and_then(|k| k.name())
            .unwrap_or("UNKNOWN")
        );

        tracing::debug!("Rule triggered on {:?}", event.kind().map(|k| k.name()));

        // Run all actions in order if passes conditions
        for action in &self.actions {
            action.execute(event.clone(), &mut ctx).await?;
        }

        Ok(true)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct RuleDb {
    pub id: Uuid,
    pub guild_id: i64,
    /// Name of this rule
    pub name: String,
    /// If this rule is enabled or not
    pub enabled: bool,
    /// Event that triggers this rule
    pub trigger: Json<Trigger>,
    /// Conditions that need to pass before running actions
    pub conditions: Json<Condition>,
    /// Actions are executed sequentially if condition passes
    pub actions: Json<Vec<Action>>,
}

impl RuleDb {
    pub async fn from_set_id(&self, pool: &sqlx::PgPool, set_id: Uuid) -> Result<Vec<RuleDb>> {
         sqlx::query_as!(
            RuleDb,
            r#"select id,
                      guild_id,
                      name,
                      enabled,
                      trigger as "trigger!: Json<Trigger>",
                      conditions as "conditions!: Json<Condition>",
                      actions as "actions!: Json<Vec<Action>>"
                 from app_public.guild_rules
                where set_id = $1
            "#,
            set_id,
        )
        .fetch_all(pool)
        .await
        .map_err(Into::into)
    }
}