use chrono::{naive::NaiveDateTime, offset::Utc, Datelike};
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

fn get_rank(rank: Option<i64>, total: Option<i64>) -> (i64, i64) {
    match (rank, total) {
        // I don't think rank can be 0 since it's row_number
        (Some(rank), Some(total)) => (rank, total),
        _ => (0, 0),
    }
}

fn fmt_rank(rank: Option<i64>, total: Option<i64>) -> String {
    match (rank, total) {
        (Some(rank), Some(total)) => {
            format!("{}/{}", rank, total)
        }
        _ => "N/A".to_string(),
    }
}

impl UserLevelRanked {
    // Ranks would be for user's last message timeframes, so if user sent a
    // message on 12/02/2020, daily rank would ONLY be for those users who have
    // last_msg in that day So if it is the next day, 12/03 it would be stale
    fn reset_stale_ranks(mut self) -> Self {
        let now = Utc::now().naive_utc();

        if now.ordinal() != self.last_msg.ordinal() {
            self.msg_day_rank = None;
        }

        if now.iso_week() != self.last_msg.iso_week() {
            self.msg_week_rank = None;
        }

        if now.month() != self.last_msg.month() {
            self.msg_month_rank = None;
        }

        self
    }

    pub fn get_rank_all_time(&self) -> (i64, i64) {
        get_rank(self.msg_all_time_rank, self.msg_all_time_total)
    }

    pub fn get_rank_month(&self) -> (i64, i64) {
        get_rank(self.msg_month_rank, self.msg_month_total)
    }

    pub fn get_rank_week(&self) -> (i64, i64) {
        get_rank(self.msg_week_rank, self.msg_week_total)
    }

    pub fn get_rank_day(&self) -> (i64, i64) {
        get_rank(self.msg_day_rank, self.msg_day_total)
    }

    // Format ranks to string
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

        ranked_from_id_query(&pool, user_id, guild_id)
            .await
            .map(|o| o.map(UserLevelRanked::reset_stale_ranks))
    }
}

async fn ranked_from_id_query(
    pool: &sqlx::PgPool,
    user_id: UserId,
    guild_id: GuildId,
) -> Result<Option<UserLevelRanked>> {
    sqlx::query_as!(
        UserLevelRanked,
        // If target user has not sent a message today, it would have incorrect ranks
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
