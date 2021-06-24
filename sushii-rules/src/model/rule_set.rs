use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::types::Uuid;
use std::collections::HashMap;
use sqlx::types::Json;

use crate::error::Result;
use crate::model::Rule;

/// Rule set used in engine and front end schema
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RuleSet {
    pub id: Uuid,
    /// Guild ID this rule set belongs to
    pub guild_id: i64,
    /// Name of this rule set, should be the feature name
    pub name: String,
    /// Description of rule set
    pub description: Option<String>,
    /// If this rule set is enabled or not, should pass down to all containing
    /// rules
    pub enabled: bool,
    /// If the guild can edit this rule set
    pub editable: bool,
    /// Author ID of this rule set, for display in web UI
    pub author: Option<i64>,
    /// Rule set category, e.g. moderation, fun, etc.
    pub category: Option<String>,
    /// Rule set configuration, map of json values
    pub config: HashMap<String, Value>,
    /// List of rules in this rule set
    pub rules: Vec<Rule>,
}

/// Rule set from database
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
struct RuleSetDb {
    pub id: Uuid,
    /// Guild ID this rule set belongs to
    pub guild_id: i64,
    /// Name of this rule set, should be the feature name
    pub name: String,
    /// Description of rule set
    pub description: Option<String>,
    /// If this rule set is enabled or not, should pass down to all containing
    /// rules
    pub enabled: bool,
    /// If the guild can edit this rule set
    pub editable: bool,
    /// Author ID of this rule set, for display in web UI
    pub author: Option<i64>,
    /// Rule set category, e.g. moderation, fun, etc.
    pub category: Option<String>,
    /// Rule set configuration, map of json values
    pub config: Json<HashMap<String, Value>>,
}

impl RuleSetDb {
    pub async fn get_guild_rule_sets(pool: &sqlx::PgPool, guild_id: u64) -> Result<Vec<RuleSetDb>> {
        sqlx::query_as!(
            RuleSetDb,
            r#"select id,
                    guild_id,
                    name,
                    description,
                    enabled,
                    editable,
                    author,
                    category,
                    config as "config!: Json<HashMap<String, Value>>"
               from app_public.guild_rule_sets
              where guild_id = $1
            "#,
            guild_id as i64,
        )
        .fetch_all(pool)
        .await
        .map_err(Into::into)
    }
}
