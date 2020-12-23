use chrono::{naive::NaiveDateTime, offset::Utc, Datelike};
use serde::{Deserialize, Serialize};

#[cfg(not(feature = "graphql"))]
use serenity::{model::prelude::*, prelude::*};
#[cfg(not(feature = "graphql"))]
use crate::keys::DbPool;

#[cfg(feature = "graphql")]
use juniper::GraphQLObject;

use crate::error::Result;
use crate::model::BigInt;

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
#[cfg_attr(
    feature = "graphql",
    graphql(description = "A user's level and ranks in a single guild"),
    derive(GraphQLObject),
)]
pub struct UserLevelRanked {
    pub user_id: BigInt,
    pub guild_id: BigInt,
    pub msg_all_time: BigInt,
    pub msg_month: BigInt,
    pub msg_week: BigInt,
    pub msg_day: BigInt,
    pub last_msg: NaiveDateTime,

    // Rank (row #) / Total in category
    pub msg_all_time_rank: Option<BigInt>,
    pub msg_all_time_total: Option<BigInt>,

    pub msg_month_rank: Option<BigInt>,
    pub msg_month_total: Option<BigInt>,

    pub msg_week_rank: Option<BigInt>,
    pub msg_week_total: Option<BigInt>,

    pub msg_day_rank: Option<BigInt>,
    pub msg_day_total: Option<BigInt>,
}

fn get_rank(rank: Option<BigInt>, total: Option<BigInt>) -> (i64, i64) {
    match (rank, total) {
        // I don't think rank can be 0 since it's row_number
        (Some(rank), Some(total)) => (rank.0, total.0),
        _ => (0, 0),
    }
}

fn fmt_rank(rank: Option<BigInt>, total: Option<BigInt>) -> String {
    match (rank, total) {
        (Some(rank), Some(total)) => {
            format!("{}/{}", rank.0, total.0)
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

    /// Get a single user's rank
    #[cfg(feature = "graphql")]
    pub async fn from_id(
        pool: &sqlx::PgPool,
        user_id: BigInt,
        guild_id: BigInt,
    ) -> Result<Option<UserLevelRanked>> {
        ranked_from_id_query(pool, user_id.0, guild_id.0)
            .await
            .map(|o| o.map(UserLevelRanked::reset_stale_ranks))
    }

    /// Get a single user's rank
    #[cfg(not(feature = "graphql"))]
    pub async fn from_id(
        ctx: &Context,
        user_id: UserId,
        guild_id: GuildId,
    ) -> Result<Option<UserLevelRanked>> {
        let data = ctx.data.read().await;
        let pool = data.get::<DbPool>().unwrap();

        ranked_from_id_query(&pool, i64::from(user_id), i64::from(guild_id))
            .await
            .map(|o| o.map(UserLevelRanked::reset_stale_ranks))
    }
}

async fn ranked_from_id_query(
    pool: &sqlx::PgPool,
    user_id: i64,
    guild_id: i64,
) -> Result<Option<UserLevelRanked>> {
    sqlx::query_as!(
        UserLevelRanked,
        // If target user has not sent a message today, it would have incorrect ranks
        r#"
            SELECT user_id as "user_id: BigInt",
                   guild_id as "guild_id: BigInt",
                   msg_all_time as "msg_all_time: BigInt",
                   msg_month as "msg_month: BigInt",
                   msg_week as "msg_week: BigInt",
                   msg_day as "msg_day: BigInt",
                   last_msg,
                   msg_all_time_rank as "msg_all_time_rank: BigInt",
                   msg_all_time_total as "msg_all_time_total: BigInt",
                   msg_month_rank as "msg_month_rank: BigInt",
                   msg_month_total as "msg_month_total: BigInt",
                   msg_week_rank as "msg_week_rank: BigInt",
                   msg_week_total as "msg_week_total: BigInt",
                   msg_day_rank as "msg_day_rank: BigInt",
                   msg_day_total as "msg_day_total: BigInt"
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
        guild_id,
        user_id,
    )
    .fetch_optional(pool)
    .await
    .map_err(Into::into)
}
