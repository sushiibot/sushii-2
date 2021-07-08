use redis::AsyncCommands;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::types::Json;
use sqlx::types::Uuid;
use std::collections::HashMap;

use crate::error::Result;
use crate::model::Rule;

const RULE_SET_TIMEOUT_SECS: usize = 30;

/// Rule set used in engine and front end schema
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RuleSet {
    #[schemars(skip)]
    pub id: i64,
    /// Guild ID this rule set belongs to
    #[schemars(skip)]
    pub guild_id: Option<i64>,
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
    #[schemars(skip)]
    pub config: HashMap<String, Value>,
    /// List of rules in this rule set
    #[schemars(skip)]
    pub rules: Vec<Rule>,
}

impl RuleSet {
    pub fn key(guild_id: u64) -> String {
        format!("guild_rule_sets:{}", guild_id)
    }

    /// Returns enabled rule sets from guild_id, **including** global rule sets
    pub async fn sets_from_guild_id(
        redis_pool: deadpool_redis::Pool,
        pool: &sqlx::PgPool,
        guild_id: u64,
    ) -> Result<Vec<RuleSet>> {
        let mut conn = redis_pool.get().await?;
        let redis_key = Self::key(guild_id);
        let cached_sets_str: Option<String> = conn.get(&redis_key).await?;

        if let Some(cached_sets_str) = cached_sets_str {
            tracing::debug!("Found cached rule set: {}", redis_key);
            let cached_sets = serde_json::from_str(&cached_sets_str)?;

            return Ok(cached_sets);
        }

        tracing::debug!("Rule set not cached: {}", redis_key);
        let db_sets = RuleSetDb::sets_from_guild_id(pool, guild_id).await?;
        let sets = Self::from_rule_sets_db(pool, db_sets).await?;
        tracing::debug!("Fetched {} rule sets from guild {}", sets.len(), guild_id);

        let sets_str = serde_json::to_string(&sets)?;
        // Cache in redis
        conn.set_ex(&redis_key, sets_str, RULE_SET_TIMEOUT_SECS)
            .await?;
        tracing::debug!("Cached rule set: {}", redis_key);

        Ok(sets)
    }

    async fn from_rule_sets_db(
        pool: &sqlx::PgPool,
        rule_sets_db: Vec<RuleSetDb>,
    ) -> Result<Vec<Self>> {
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
                config: set.config.map_or_else(|| HashMap::new(), |c| c.0),
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
    pub id: i64,
    /// Guild ID this rule set belongs to
    pub guild_id: Option<i64>,
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
    pub config: Option<Json<HashMap<String, Value>>>,
}

impl RuleSetDb {
    /// Returns enabled rule sets from guild_id, **including** global rule sets
    pub async fn sets_from_guild_id(pool: &sqlx::PgPool, guild_id: u64) -> Result<Vec<RuleSetDb>> {
        // only query sets where a config is added *and* enabled
        sqlx::query_as!(
            RuleSetDb,
            r#"select id as "id!: i64",
                      s.guild_id,
                      name as "name!: String",
                      description,
                      s.enabled as "enabled!: bool",
                      editable as "editable!: bool",
                      author,
                      category,
                      config as "config: Json<HashMap<String, Value>>"
               from app_public.guild_rule_sets s
                    left join app_public.guild_rule_set_configs c
                           on s.id = c.set_id
              where (s.enabled = true and (c.enabled is null or c.enabled = true))
                and (s.guild_id = $1 or s.guild_id is null)
            "#,
            guild_id as i64,
        )
        .fetch_all(pool)
        .await
        .map_err(Into::into)
    }
}
