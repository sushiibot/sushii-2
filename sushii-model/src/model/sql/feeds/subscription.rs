use serde::{Deserialize, Serialize};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::error::Result;
use crate::keys::DbPool;

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct FeedSubscription {
    pub feed_id: String,
    pub guild_id: i64,
    pub channel_id: i64,
    pub mention_role: Option<i64>,
}

impl FeedSubscription {
    pub fn new(feed_id: impl Into<String>, guild_id: i64, channel_id: i64) -> Self {
        Self {
            feed_id: feed_id.into(),
            guild_id,
            channel_id,
            mention_role: None,
        }
    }

    pub fn mention_role(mut self, mention_role: Option<i64>) -> Self {
        self.mention_role = mention_role;
        self
    }

    pub async fn from_id(ctx: &Context, guild_id: GuildId, feed_id: &str) -> Result<Option<Self>> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        sqlx::query_as!(
            FeedSubscription,
            r#"
            SELECT *
              FROM app_public.feed_subscriptions
             WHERE guild_id = $1
               AND feed_id = $2
            "#,
            guild_id.0 as i64,
            feed_id
        )
        .fetch_optional(&pool)
        .await
        .map_err(Into::into)
    }

    pub async fn from_guild_id(ctx: &Context, guild_id: GuildId) -> Result<Vec<Self>> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        sqlx::query_as!(
            FeedSubscription,
            r#"
            SELECT *
              FROM app_public.feed_subscriptions
             WHERE guild_id = $1
            "#,
            i64::from(guild_id),
        )
        .fetch_all(&pool)
        .await
        .map_err(Into::into)
    }

    pub async fn from_feed_id(ctx: &Context, feed_id: &str) -> Result<Vec<Self>> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        sqlx::query_as!(
            FeedSubscription,
            r#"
            SELECT *
              FROM app_public.feed_subscriptions
             WHERE feed_id = $1
            "#,
            feed_id,
        )
        .fetch_all(&pool)
        .await
        .map_err(Into::into)
    }

    pub async fn get_matching_vlive(pool: &sqlx::PgPool, feed_ids: &[String]) -> Result<Vec<Self>> {
        get_matching_vlive(pool, feed_ids).await
    }

    pub async fn save(self, ctx: &Context) -> Result<Self> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        sqlx::query_as!(
            FeedSubscription,
            r#"
            INSERT INTO app_public.feed_subscriptions
                 VALUES ($1, $2, $3, $4)
            "#,
            self.feed_id,
            self.guild_id,
            self.channel_id,
            self.mention_role,
        )
        .execute(&pool)
        .await?;

        Ok(self)
    }

    pub async fn delete(self, ctx: &Context) -> Result<()> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        delete_query(&pool, &self).await
    }

    pub async fn delete_pool(&self, pool: &sqlx::PgPool) -> Result<()> {
        delete_query(pool, self).await
    }
}

async fn get_matching_vlive(pool: &sqlx::PgPool, feed_ids: &[String]) -> Result<Vec<FeedSubscription>> {
    sqlx::query_as!(
        FeedSubscription,
        r#"
            SELECT *
              FROM app_public.feed_subscriptions
             WHERE feed_id = ANY($1)
            "#,
        feed_ids as _,
    )
    .fetch_all(pool)
    .await
    .map_err(Into::into)
}

async fn delete_query(pool: &sqlx::PgPool, sub: &FeedSubscription) -> Result<()> {
    sqlx::query_as!(
        FeedSubscription,
        r#"
        DELETE FROM app_public.feed_subscriptions
                WHERE feed_id = $1
                AND channel_id = $2
        "#,
        // composite primary key
        sub.feed_id,
        sub.channel_id,
    )
    .execute(pool)
    .await?;

    Ok(())
}
