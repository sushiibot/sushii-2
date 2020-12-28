use chrono::{naive::NaiveDateTime, offset::Utc, Datelike, Duration};
use serde::{Deserialize, Serialize};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[cfg(feature = "graphql")]
use juniper::GraphQLObject;

use crate::error::Result;
use crate::keys::DbPool;
use crate::model::BigInt;

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
#[cfg_attr(
    feature = "graphql",
    graphql(description = "A user's level in a single guild"),
    derive(GraphQLObject)
)]
pub struct UserLevel {
    pub user_id: BigInt,
    pub guild_id: BigInt,
    pub msg_all_time: BigInt,
    pub msg_month: BigInt,
    pub msg_week: BigInt,
    pub msg_day: BigInt,
    pub last_msg: NaiveDateTime,
}

impl UserLevel {
    pub fn new(user_id: UserId, guild_id: GuildId) -> Self {
        Self {
            user_id: user_id.into(),
            guild_id: guild_id.into(),
            // msg_counts all default 5 since it would be created on first message
            msg_all_time: 5u64.into(),
            msg_month: 5u64.into(),
            msg_week: 5u64.into(),
            msg_day: 5u64.into(),
            last_msg: Utc::now().naive_utc(),
        }
    }

    /// Checks if user is eligible for increment, limited to once per minute
    pub fn eligible(&self) -> bool {
        let now = Utc::now().naive_utc();

        // Now is past (last message + 1 minute)
        now > (self.last_msg + Duration::minutes(1))
    }

    /// Increments values with the time intervals reset accordingly
    pub fn inc(mut self) -> Self {
        self.reset_intervals().inc_fields();

        // Set last_message to now, so that next XP inc is minimum 1 minute later
        // Must be set AFTER intervals are reset, otherwise they will never be reset
        self.last_msg = Utc::now().naive_utc();

        self
    }

    /// Resets intervals that have expired
    fn reset_intervals(&mut self) -> &mut Self {
        let now = Utc::now().naive_utc();

        if now.ordinal() != self.last_msg.ordinal() {
            self.msg_day = 0u64.into();
        }

        if now.iso_week() != self.last_msg.iso_week() {
            self.msg_week = 0u64.into();
        }

        if now.month() != self.last_msg.month() {
            self.msg_month = 0u64.into();
        }

        self
    }

    /// Increment all fields by 5
    fn inc_fields(&mut self) -> &mut Self {
        // Add enough to be rounded to 5
        let to_add = 5 - self.msg_all_time.0 % 5;

        self.msg_all_time.0 += to_add;
        self.msg_month.0 += to_add;
        self.msg_week.0 += to_add;
        self.msg_day.0 += to_add;

        self
    }

    #[cfg(feature = "graphql")]
    pub async fn from_id(
        pool: &sqlx::PgPool,
        user_id: BigInt,
        guild_id: BigInt,
    ) -> Result<Option<UserLevel>> {
        from_id_query(pool, user_id.0, guild_id.0).await
    }

    #[cfg(not(feature = "graphql"))]
    pub async fn from_id(
        ctx: &Context,
        user_id: UserId,
        guild_id: GuildId,
    ) -> Result<Option<UserLevel>> {
        let data = ctx.data.read().await;
        let pool = data.get::<DbPool>().unwrap();

        from_id_query(&pool, i64::from(user_id), i64::from(guild_id)).await
    }

    pub async fn save(&self, ctx: &Context) -> Result<UserLevel> {
        let data = ctx.data.read().await;
        let pool = data.get::<DbPool>().unwrap();

        upsert_query(&pool, &self).await
    }
}

async fn from_id_query(
    pool: &sqlx::PgPool,
    user_id: i64,
    guild_id: i64,
) -> Result<Option<UserLevel>> {
    sqlx::query_as!(
        UserLevel,
        r#"
            SELECT user_id as "user_id: BigInt",
                   guild_id as "guild_id: BigInt",
                   msg_all_time as "msg_all_time: BigInt",
                   msg_month as "msg_month: BigInt",
                   msg_week as "msg_week: BigInt",
                   msg_day as "msg_day: BigInt",
                   last_msg
              FROM user_levels
             WHERE user_id = $1
               AND guild_id = $2
        "#,
        user_id,
        guild_id,
    )
    .fetch_optional(pool)
    .await
    .map_err(Into::into)
}

async fn upsert_query(pool: &sqlx::PgPool, user_level: &UserLevel) -> Result<UserLevel> {
    sqlx::query_as!(
        UserLevel,
        r#"
        INSERT INTO user_levels (user_id, guild_id, msg_all_time, msg_month, msg_week, msg_day, last_msg)
             VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (user_id, guild_id)
          DO UPDATE
                SET msg_all_time = $3,
                    msg_month = $4,
                    msg_week = $5,
                    msg_day = $6,
                    last_msg = $7
          RETURNING user_id as "user_id: BigInt",
                    guild_id as "guild_id: BigInt",
                    msg_all_time as "msg_all_time: BigInt",
                    msg_month as "msg_month: BigInt",
                    msg_week as "msg_week: BigInt",
                    msg_day as "msg_day: BigInt",
                    last_msg
        "#,
        user_level.user_id.0,
        user_level.guild_id.0,
        user_level.msg_all_time.0,
        user_level.msg_month.0,
        user_level.msg_week.0,
        user_level.msg_day.0,
        user_level.last_msg,
    )
    .fetch_one(pool)
    .await
    .map_err(Into::into)
}
