use chrono::{naive::NaiveDateTime, offset::Utc, Duration};
use serde::{Deserialize, Serialize};
use serenity::async_trait;
use serenity::prelude::*;

use crate::error::Result;
use crate::keys::DbPool;

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct Mute {
    pub guild_id: i64,
    pub user_id: i64,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
}

#[async_trait]
pub trait MuteDb {
    async fn from_id(ctx: &Context, guild_id: u64, user_id: u64) -> Result<Option<Mute>>;

    async fn save(&self, ctx: &Context) -> Result<Mute>;
    async fn delete(&self, ctx: &Context) -> Result<()>;
}

impl Mute {
    pub fn new(guild_id: u64, user_id: u64, duration: Option<Duration>) -> Self {
        let now = Utc::now().naive_local();

        Mute {
            guild_id: guild_id as i64,
            user_id: user_id as i64,
            start_time: now,
            end_time: duration.map(|d| now + d),
        }
    }
}

#[async_trait]
impl MuteDb for Mute {
    async fn from_id(ctx: &Context, guild_id: u64, user_id: u64) -> Result<Option<Mute>> {
        let data = ctx.data.read().await;
        let pool = data.get::<DbPool>().unwrap();

        get_from_id_query(&pool, guild_id, user_id).await
    }

    async fn save(&self, ctx: &Context) -> Result<Self> {
        let data = ctx.data.read().await;
        let pool = data.get::<DbPool>().unwrap();

        upsert_query(&pool, &self).await
    }

    async fn delete(&self, ctx: &Context) -> Result<()> {
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
        "#,
        guild_id as i64,
        user_id as i64,
    )
    .fetch_optional(pool)
    .await
    .map_err(Into::into)
}

async fn upsert_query(pool: &sqlx::PgPool, mute: &Mute) -> Result<Mute> {
    sqlx::query_as!(
        Mute,
        r#"
        INSERT INTO mutes
             VALUES ($1, $2, $3, $4)
        ON CONFLICT (guild_id, user_id)
          DO UPDATE
                SET start_time = $3,
                    end_time = $4
            RETURNING *
        "#,
        mute.guild_id,
        mute.user_id,
        mute.start_time,
        mute.end_time,
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
