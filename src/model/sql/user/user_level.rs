use chrono::{naive::NaiveDateTime, offset::Utc, Datelike, Duration};
use serde::{Deserialize, Serialize};
use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::error::Result;
use crate::keys::DbPool;

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct UserLevel {
    pub user_id: i64,
    pub guild_id: i64,
    pub msg_all_time: i64,
    pub msg_month: i64,
    pub msg_week: i64,
    pub msg_day: i64,
    pub last_msg: NaiveDateTime,
}

impl UserLevel {
    pub fn new(user_id: UserId, guild_id: GuildId) -> Self {
        Self {
            user_id: user_id.into(),
            guild_id: guild_id.into(),
            // msg_counts all default 5 since it would be created on first message
            msg_all_time: 5,
            msg_month: 5,
            msg_week: 5,
            msg_day: 5,
            last_msg: Utc::now().naive_local(),
        }
    }

    /// Checks if user is eligible for increment, limited to once per minute
    pub fn eligible(&self) -> bool {
        let now = Utc::now().naive_local();

        // Now is past (last message + 1 minute)
        now > (self.last_msg + Duration::minutes(1))
    }

    /// Increments values with the time intervals reset accordingly
    pub fn inc(self) -> Self {
        self.reset_intervals().inc_fields()
    }

    /// Resets intervals that have expired
    fn reset_intervals(mut self) -> Self {
        let now = Utc::now().naive_local();

        if now.ordinal() != self.last_msg.ordinal() {
            self.msg_day = 0;
        }

        if now.iso_week() != self.last_msg.iso_week() {
            self.msg_week = 0;
        }

        if now.month() != self.last_msg.month() {
            self.msg_month = 0;
        }

        self
    }

    /// Increment all fields by 5
    fn inc_fields(mut self) -> Self {
        // Add enough to be rounded to 5
        let to_add = 5 - self.msg_all_time % 5;

        self.msg_all_time += to_add;
        self.msg_month += to_add;
        self.msg_week += to_add;
        self.msg_day += to_add;

        self
    }
}

#[async_trait]
pub trait UserLevelDb {
    async fn from_id(
        ctx: &Context,
        user_id: UserId,
        guild_id: GuildId,
    ) -> Result<Option<UserLevel>>;

    async fn save(&self, ctx: &Context) -> Result<UserLevel>;
}

#[async_trait]
impl UserLevelDb for UserLevel {
    async fn from_id(
        ctx: &Context,
        user_id: UserId,
        guild_id: GuildId,
    ) -> Result<Option<UserLevel>> {
        let data = ctx.data.read().await;
        let pool = data.get::<DbPool>().unwrap();

        from_id_query(&pool, user_id, guild_id).await
    }

    async fn save(&self, ctx: &Context) -> Result<UserLevel> {
        let data = ctx.data.read().await;
        let pool = data.get::<DbPool>().unwrap();

        upsert_query(&pool, &self).await
    }
}

async fn from_id_query(
    pool: &sqlx::PgPool,
    user_id: UserId,
    guild_id: GuildId,
) -> Result<Option<UserLevel>> {
    sqlx::query_as!(
        UserLevel,
        r#"
            SELECT *
              FROM user_levels
             WHERE user_id = $1
               AND guild_id = $2
        "#,
        i64::from(user_id),
        i64::from(guild_id),
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
          RETURNING *
        "#,
        user_level.user_id,
        user_level.guild_id,
        user_level.msg_all_time,
        user_level.msg_month,
        user_level.msg_week,
        user_level.msg_day,
        user_level.last_msg,
    )
    .fetch_one(pool)
    .await
    .map_err(Into::into)
}
