use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use sqlx::types::Uuid;
use std::error::Error;
use std::result::Result as StdResult;
use std::sync::Arc;

use crate::error::Result;
use crate::model::{Action, Condition, Event, RuleContext, Trigger};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Rule {
    #[schemars(skip)]
    pub id: i64,
    /// Name of this rule
    pub name: String,
    /// If this rule is enabled or not
    pub enabled: bool,
    /// Event that triggers this rule
    pub trigger: Trigger,
    /// # Conditions
    /// Conditions that need to pass before running actions
    pub conditions: Condition,
    /// # Actions
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

    pub async fn from_set_id(pool: &sqlx::PgPool, set_id: i64) -> Result<Vec<Rule>> {
        let db_rules = RuleDb::from_set_id(pool, set_id).await?;

        let mut rules = Vec::new();

        for rule in db_rules {
            rules.push(Rule {
                id: rule.id,
                name: rule.name,
                enabled: rule.enabled,
                trigger: rule.trigger.0,
                conditions: rule.conditions.0,
                actions: rule.actions.0,
            });
        }

        Ok(rules)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
struct RuleDb {
    pub id: i64,
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
    pub async fn from_set_id(pool: &sqlx::PgPool, set_id: i64) -> Result<Vec<RuleDb>> {
        sqlx::query_as!(
            RuleDb,
            r#"select id,
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
