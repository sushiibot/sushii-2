use chrono::naive::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::error::Result;
use crate::keys::DbPool;

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct UserLevelRanked {
    pub user_id: i64,
    pub guild_id: i64,
    pub msg_all_time: i64,
    pub msg_month: i64,
    pub msg_week: i64,
    pub msg_day: i64,
    pub last_msg: NaiveDateTime,

    // Rank (row #) / Total in category
    pub msg_all_time_rank: Option<i64>,
    pub msg_all_time_total: Option<i64>,

    pub msg_month_rank: Option<i64>,
    pub msg_month_total: Option<i64>,

    pub msg_week_rank: Option<i64>,
    pub msg_week_total: Option<i64>,

    pub msg_day_rank: Option<i64>,
    pub msg_day_total: Option<i64>,
}

fn fmt_rank(rank: Option<i64>, total: Option<i64>) -> String {
    match (rank, total) {
        (Some(rank), _) if rank == 0 => "N/A".to_string(),
        (Some(rank), Some(total)) => {
            format!("{}/{}", rank, total)
        }
        _ => "N/A".to_string(),
    }
}

impl UserLevelRanked {
    pub fn fmt_rank_all_time(&self) -> String {
        fmt_rank(self.msg_all_time_rank, self.msg_all_time_total)
    }

    pub fn fmt_rank_month(&self) -> String {
        fmt_rank(self.msg_month_rank, self.msg_month_total)
    }

    pub fn fmt_rank_week(&self) -> String {
        fmt_rank(self.msg_week_rank, self.msg_week_total)
    }

    pub fn fmt_rank_day(&self) -> String {
        fmt_rank(self.msg_day_rank, self.msg_day_total)
    }
}

#[async_trait]
pub trait UserLevelRankedDb {
    async fn from_id(
        ctx: &Context,
        user_id: UserId,
        guild_id: GuildId,
    ) -> Result<Option<UserLevelRanked>>;
}

#[async_trait]
impl UserLevelRankedDb for UserLevelRanked {
    async fn from_id(
        ctx: &Context,
        user_id: UserId,
        guild_id: GuildId,
    ) -> Result<Option<UserLevelRanked>> {
        let data = ctx.data.read().await;
        let pool = data.get::<DbPool>().unwrap();

        ranked_from_id_query(&pool, user_id, guild_id).await
    }
}

async fn ranked_from_id_query(
    pool: &sqlx::PgPool,
    user_id: UserId,
    guild_id: GuildId,
) -> Result<Option<UserLevelRanked>> {
    sqlx::query_as!(
        UserLevelRanked,
        r#"
            SELECT * 
                FROM (
                    SELECT *,
                        ROW_NUMBER() OVER(PARTITION BY EXTRACT(DOY FROM last_msg) ORDER BY msg_day DESC) AS msg_day_rank,
                        COUNT(*) OVER(PARTITION BY EXTRACT(DOY FROM last_msg)) AS msg_day_total,

                        ROW_NUMBER() OVER(PARTITION BY EXTRACT(WEEK FROM last_msg) ORDER BY msg_week DESC) AS msg_week_rank,
                        COUNT(*) OVER(PARTITION BY EXTRACT(WEEK FROM last_msg)) AS msg_week_total,

                        ROW_NUMBER() OVER(PARTITION BY EXTRACT(MONTH FROM last_msg) ORDER BY msg_month DESC) AS msg_month_rank,
                        COUNT(*) OVER(PARTITION BY EXTRACT(MONTH FROM last_msg)) AS msg_month_total,

                        ROW_NUMBER() OVER(ORDER BY msg_all_time DESC) AS msg_all_time_rank,
                        COUNT(*) OVER() AS msg_all_time_total
                    FROM user_levels WHERE guild_id = $1
                ) t
            WHERE t.user_id = $2
        "#,
        i64::from(guild_id),
        i64::from(user_id),
    )
    .fetch_optional(pool)
    .await
    .map_err(Into::into)
}
