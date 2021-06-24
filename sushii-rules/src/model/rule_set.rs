use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::types::Uuid;
use std::collections::HashMap;
use sqlx::types::Json;
use redis::AsyncCommands;

use crate::error::Result;
use crate::model::Rule;

const RULE_SET_TIMEOUT_SECS: usize = 30;

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

impl RuleSet {
    pub fn key(guild_id: u64) -> String {
        format!("guild_rule_sets:{}", guild_id)
    }

    pub async fn sets_from_guild_id(redis_pool: deadpool_redis::Pool, pool: &sqlx::PgPool, guild_id: u64) -> Result<Vec<RuleSet>> {
        let mut conn = redis_pool.get().await?;
        let redis_key = Self::key(guild_id);
        let cached_sets_str: Option<String> = conn.get(&redis_key).await?;

        if let Some(cached_sets_str) = cached_sets_str {
            let cached_sets = serde_json::from_str(&cached_sets_str)?;

            return Ok(cached_sets);
        }

        let db_sets = RuleSetDb::sets_from_guild_id(pool, guild_id).await?;
        let db_sets_str = serde_json::to_string(&db_sets)?;
        // Cache in redis
        conn.set_ex(&redis_key, db_sets_str, RULE_SET_TIMEOUT_SECS).await?;

        Self::from_rule_sets_db(pool, db_sets).await
    }

    async fn from_rule_sets_db(pool: &sqlx::PgPool, rule_sets_db: Vec<RuleSetDb>) -> Result<Vec<Self>> {
        let mut rule_sets = Vec::new();

        for set in rule_sets_db {
            let set = Self {
                id: set.id,
                guild_id: set.guild_id,
                name: set.name,
                description: set.description,
                enabled: set.enabled,
                editable: set.editable,
                author: set.author,
                category: set.category,
                config: set.config.0,
                rules: Rule::from_set_id(pool, set.id).await?,
            };

            rule_sets.push(set);
        }

        Ok(rule_sets)
    }
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
    pub async fn sets_from_guild_id(pool: &sqlx::PgPool, guild_id: u64) -> Result<Vec<RuleSetDb>> {
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
