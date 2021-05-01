use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::error::Result;

#[derive(
    Deserialize, Serialize, sqlx::Type, Clone, Copy, Eq, PartialEq, Debug, schemars::JsonSchema,
)]
#[sqlx(type_name = "rule_scope", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RuleScope {
    Guild,
    Channel,
    User,
}

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct RuleGauge {
    pub time: DateTime<Utc>,
    pub guild_id: i64,
    pub scope: RuleScope,
    pub scope_id: i64,
    pub name: String,
    pub value: i64,
}

impl RuleGauge {
    pub async fn get(
        pool: &sqlx::PgPool,
        guild_id: u64,
        scope: RuleScope,
        scope_id: u64,
        name: &str,
    ) -> Result<Option<Self>> {
        sqlx::query_as!(
            RuleGauge,
            r#"
                SELECT time, guild_id, scope as "scope: RuleScope",
                       scope_id, name, value
                  FROM app_public.rule_gauges
                 WHERE guild_id = $1
                   AND scope = $2
                   AND scope_id = $3
                   AND name = $4
                 ORDER BY time DESC
                 LIMIT 1
            "#,
            guild_id as i64,
            scope as _,
            scope_id as i64,
            name,
        )
        .fetch_optional(pool)
        .await
        .map_err(Into::into)
    }

    /// Gets the current value of a gauge or 0 if it doesn't exist
    pub async fn get_count(
        pool: &sqlx::PgPool,
        guild_id: u64,
        scope: RuleScope,
        scope_id: u64,
        name: &str,
    ) -> Result<i64> {
        let counter = RuleGauge::get(pool, guild_id, scope, scope_id, name)
            .await?
            .map(|c| c.value);

        Ok(counter.unwrap_or(0))
    }

    pub async fn inc(
        pool: &sqlx::PgPool,
        guild_id: u64,
        scope: RuleScope,
        scope_id: u64,
        name: &str,
    ) -> Result<Self> {
        sqlx::query_as!(
            RuleGauge,
            r#"
            INSERT INTO app_public.rule_gauges (time, guild_id, scope, scope_id, name, value)
                VALUES (NOW(), $1, $2, $3, $4,
                    -- select the most recent record and increment
                    coalesce((SELECT value + 1
                      FROM app_public.rule_gauges
                     WHERE guild_id = $1
                       AND scope = $2
                       AND scope_id = $3
                       AND name = $4
                     ORDER BY time DESC
                     LIMIT 1),
                     1
                    )
                )
                RETURNING time, guild_id, scope as "scope: RuleScope",
                          scope_id, name, value
            "#,
            guild_id as i64,
            scope as _,
            scope_id as i64,
            name,
        )
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    pub async fn dec(
        pool: &sqlx::PgPool,
        guild_id: u64,
        scope: RuleScope,
        scope_id: u64,
        name: &str,
    ) -> Result<Self> {
        sqlx::query_as!(
            RuleGauge,
            r#"
            INSERT INTO app_public.rule_gauges (time, guild_id, scope, scope_id, name, value)
                VALUES (NOW(), $1, $2, $3, $4,
                    -- select the most recent record and increment
                    coalesce((SELECT value - 1
                      FROM app_public.rule_gauges
                     WHERE guild_id = $1
                       AND scope = $2
                       AND scope_id = $3
                       AND name = $4
                     ORDER BY time DESC
                     LIMIT 1),
                     0
                    )
                )
                RETURNING time, guild_id, scope as "scope: RuleScope",
                          scope_id, name, value
            "#,
            guild_id as i64,
            scope as _,
            scope_id as i64,
            name,
        )
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    pub async fn reset(
        pool: &sqlx::PgPool,
        guild_id: u64,
        scope: RuleScope,
        scope_id: u64,
        name: &str,
    ) -> Result<Self> {
        sqlx::query_as!(
            RuleGauge,
            r#"
            INSERT INTO app_public.rule_gauges (time, guild_id, scope, scope_id, name, value)
                VALUES (NOW(), $1, $2, $3, $4, 0)
                RETURNING time, guild_id, scope as "scope: RuleScope",
                          scope_id, name, value
            "#,
            guild_id as i64,
            scope as _,
            scope_id as i64,
            name,
        )
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }
}
