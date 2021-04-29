use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::error::Result;

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct RuleGauge {
    pub time: DateTime<Utc>,
    pub guild_id: i64,
    pub name: String,
    pub value: i64,
}

impl RuleGauge {
    /// Gets the current value of a gauge or 0 if it doesn't exist
    pub async fn get_count(pool: &sqlx::PgPool, guild_id: u64, name: &str) -> Result<i64> {
        sqlx::query!(
            r#"
            SELECT coalesce(
                (SELECT value
                  FROM app_public.rule_gauges
                 WHERE guild_id = $1
                   AND name = $2
                 ORDER BY time DESC
                 LIMIT 1),
                0
            ) as "value!"
            "#,
            guild_id as i64,
            name,
        )
        .fetch_one(pool)
        .await
        .map(|r| r.value)
        .map_err(Into::into)
    }

    pub async fn inc(pool: &sqlx::PgPool, guild_id: u64, name: &str) -> Result<Self> {
        sqlx::query_as!(
            RuleGauge,
            r#"
            INSERT INTO app_public.rule_gauges (time, guild_id, name, value)
                VALUES (NOW(), $1, $2,
                    -- select the most recent record and increment
                    coalesce((SELECT value + 1
                      FROM app_public.rule_gauges
                     WHERE guild_id = $1
                       AND name = $2
                     ORDER BY time DESC
                     LIMIT 1),
                     1
                    )
                )
                RETURNING *
            "#,
            guild_id as i64,
            name,
        )
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    pub async fn dec(pool: &sqlx::PgPool, guild_id: u64, name: &str) -> Result<Self> {
        sqlx::query_as!(
            RuleGauge,
            r#"
            INSERT INTO app_public.rule_gauges (time, guild_id, name, value)
                VALUES (NOW(), $1, $2,
                    -- select the most recent record and increment
                    coalesce((SELECT value - 1
                      FROM app_public.rule_gauges
                     WHERE guild_id = $1
                       AND name = $2
                     ORDER BY time DESC
                     LIMIT 1),
                     0
                    )
                )
                RETURNING *
            "#,
            guild_id as i64,
            name,
        )
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    pub async fn reset(pool: &sqlx::PgPool, guild_id: u64, name: &str) -> Result<Self> {
        sqlx::query_as!(
            RuleGauge,
            r#"
            INSERT INTO app_public.rule_gauges (time, guild_id, name, value)
                VALUES (NOW(), $1, $2, 0)
                RETURNING *
            "#,
            guild_id as i64,
            name,
        )
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }
}
