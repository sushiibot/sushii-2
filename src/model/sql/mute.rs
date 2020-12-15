use chrono::{naive::NaiveDateTime, offset::Utc, Duration};
use serde::{Deserialize, Serialize};
use serenity::prelude::*;

use crate::error::Result;
use crate::keys::DbPool;

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct Mute {
    pub guild_id: i64,
    pub user_id: i64,

    /// (guild_id, case_id) foreign key to originating mute mod action
    pub case_id: Option<i64>,

    /// Allow Mutes to be added from other places other than on_member_update in
    /// order to change duration on a per mute basis
    pub pending: bool,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
}

impl Mute {
    pub fn new(guild_id: u64, user_id: u64, duration: Option<Duration>) -> Self {
        let now = Utc::now().naive_utc();

        Mute {
            guild_id: guild_id as i64,
            user_id: user_id as i64,
            case_id: None,
            pending: false,
            start_time: now,
            end_time: duration.map(|d| now + d),
        }
    }

    pub fn case_id(mut self, case_id: i64) -> Self {
        self.case_id.replace(case_id);
        self
    }

    pub fn pending(mut self, pending: bool) -> Self {
        self.pending = pending;
        self
    }

    /// Gets total mute duration
    pub fn get_duration(&self) -> Option<Duration> {
        self.end_time
            .map(|t| t.signed_duration_since(self.start_time))
            .map(|d| Duration::seconds(d.num_seconds()))
    }

    /// Gets remaining mute duration
    pub fn get_duration_remaining(&self) -> Option<Duration> {
        self.end_time
            .map(|t| t.signed_duration_since(Utc::now().naive_utc()))
            .map(|d| Duration::seconds(d.num_seconds()))
    }

    /// Gets total mute duration with Std Duration
    pub fn get_std_duration(&self) -> Option<std::time::Duration> {
        self.get_duration().and_then(|d| d.to_std().ok())
    }

    /// Gets remaining mute duration with Std Duration
    pub fn get_std_duration_remaining(&self) -> Option<std::time::Duration> {
        self.get_duration_remaining().and_then(|d| d.to_std().ok())
    }

    /// Gets human readable formatted duration string of total mute duration
    pub fn get_human_duration(&self) -> Option<String> {
        self.get_std_duration()
            .map(|d| humantime::format_duration(d).to_string())
    }

    /// Gets human readable formatted duration string of remaining time
    pub fn get_human_duration_remaining(&self) -> Option<String> {
        self.get_std_duration_remaining()
            .map(|d| humantime::format_duration(d).to_string())
    }

    /// Gets a NON-pending mute from guild and user ID
    pub async fn from_id(ctx: &Context, guild_id: u64, user_id: u64) -> Result<Option<Mute>> {
        let data = ctx.data.read().await;
        let pool = data.get::<DbPool>().unwrap();

        get_from_id_query(&pool, guild_id, user_id).await
    }

    /// Gets a mute from guild and user ID that can be EITHER pending or non-pending
    pub async fn from_id_any_pending(
        ctx: &Context,
        guild_id: u64,
        user_id: u64,
    ) -> Result<Option<Mute>> {
        let data = ctx.data.read().await;
        let pool = data.get::<DbPool>().unwrap();

        get_from_id_any_pending_query(&pool, guild_id, user_id).await
    }

    /// Gets all currently expired mutes
    pub async fn get_expired(ctx: &Context) -> Result<Vec<Mute>> {
        let data = ctx.data.read().await;
        let pool = data.get::<DbPool>().unwrap();

        get_expired_query(&pool).await
    }

    /// Gets all ongoing mutes in a guild
    pub async fn get_ongoing(ctx: &Context, guild_id: u64) -> Result<Vec<Mute>> {
        let data = ctx.data.read().await;
        let pool = data.get::<DbPool>().unwrap();

        get_ongoing_query(&pool, guild_id).await
    }

    /// Saves a mute to the database
    pub async fn save(&self, ctx: &Context) -> Result<Self> {
        let data = ctx.data.read().await;
        let pool = data.get::<DbPool>().unwrap();

        upsert_query(&pool, &self).await
    }

    /// Deletes a mute from the database
    pub async fn delete(&self, ctx: &Context) -> Result<()> {
        let data = ctx.data.read().await;
        let pool = data.get::<DbPool>().unwrap();

        delete_mute_query(&pool, self.guild_id, self.user_id).await
    }
}

async fn get_from_id_query(
    pool: &sqlx::PgPool,
    guild_id: u64,
    user_id: u64,
) -> Result<Option<Mute>> {
    sqlx::query_as!(
        Mute,
        r#"
            SELECT *
              FROM mutes
             WHERE guild_id = $1
               AND user_id = $2
               AND pending = false
        "#,
        guild_id as i64,
        user_id as i64,
    )
    .fetch_optional(pool)
    .await
    .map_err(Into::into)
}

async fn get_from_id_any_pending_query(
    pool: &sqlx::PgPool,
    guild_id: u64,
    user_id: u64,
) -> Result<Option<Mute>> {
    sqlx::query_as!(
        Mute,
        r#"
            SELECT *
              FROM mutes
             WHERE guild_id = $1
               AND user_id = $2
        "#,
        guild_id as i64,
        user_id as i64,
    )
    .fetch_optional(pool)
    .await
    .map_err(Into::into)
}

async fn get_expired_query(pool: &sqlx::PgPool) -> Result<Vec<Mute>> {
    sqlx::query_as!(
        Mute,
        r#"
            SELECT *
              FROM mutes
             WHERE end_time < timezone('UTC', now())
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(Into::into)
}

async fn get_ongoing_query(pool: &sqlx::PgPool, guild_id: u64) -> Result<Vec<Mute>> {
    sqlx::query_as!(
        Mute,
        r#"
            SELECT *
              FROM mutes
             WHERE guild_id = $1
        "#,
        guild_id as i64
    )
    .fetch_all(pool)
    .await
    .map_err(Into::into)
}

async fn upsert_query(pool: &sqlx::PgPool, mute: &Mute) -> Result<Mute> {
    sqlx::query_as!(
        Mute,
        r#"
        INSERT INTO mutes (guild_id, user_id, start_time, end_time, pending, case_id)
             VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (guild_id, user_id)
          DO UPDATE
                SET start_time = $3,
                    end_time = $4,
                    pending = $5
            RETURNING *
        "#,
        // Not in the order of the struct fields, but in the order of columns or it make error :(
        // Pending and case_id were added afterwards so they at the end
        mute.guild_id,
        mute.user_id,
        mute.start_time,
        mute.end_time,
        mute.pending,
        mute.case_id,
    )
    .fetch_one(pool)
    .await
    .map_err(Into::into)
}

// This is exported as to not have to make a new Mute instance or fetch from db to delete a mute
pub async fn delete_mute(ctx: &Context, guild_id: u64, user_id: u64) -> Result<()> {
    let data = ctx.data.read().await;
    let pool = data.get::<DbPool>().unwrap();

    delete_mute_query(&pool, guild_id as i64, user_id as i64).await
}

async fn delete_mute_query(pool: &sqlx::PgPool, guild_id: i64, user_id: i64) -> Result<()> {
    sqlx::query!(
        r#"
            DELETE FROM mutes
                  WHERE guild_id = $1
                    AND user_id = $2
        "#,
        guild_id,
        user_id,
    )
    .execute(pool)
    .await?;

    Ok(())
}
