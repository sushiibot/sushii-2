use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::error::Result;

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct BotStat {
    name: String,
    category: String,
    count: i64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl BotStat {
    pub async fn inc<'a, E: sqlx::Executor<'a, Database = sqlx::Postgres>>(
        exec: E,
        category: &str,
        name: &str,
        amt: i64,
    ) -> Result<Self> {
        sqlx::query_as!(
            BotStat,
            r#"
                INSERT INTO app_public.bot_stats (name, category, count)
                     VALUES ($1, $2, $3)
                ON CONFLICT (name, category)
                  DO UPDATE SET count = app_public.bot_stats.count + $3
                RETURNING *
            "#,
            name,
            category,
            amt,
        )
        .fetch_one(exec)
        .await
        .map_err(Into::into)
    }

    pub async fn set<'a, E: sqlx::Executor<'a, Database = sqlx::Postgres>>(
        exec: E,
        category: &str,
        name: &str,
        amt: i64,
    ) -> Result<Self> {
        sqlx::query_as!(
            BotStat,
            r#"
                INSERT INTO app_public.bot_stats (name, category, count)
                     VALUES ($1, $2, $3)
                ON CONFLICT (name, category)
                  DO UPDATE SET count = $3
                RETURNING *
            "#,
            name,
            category,
            amt,
        )
        .fetch_one(exec)
        .await
        .map_err(Into::into)
    }
}
