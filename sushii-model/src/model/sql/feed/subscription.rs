use serde::{Deserialize, Serialize};
use serenity::prelude::*;

use crate::error::Result;
use crate::keys::DbPool;

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct FeedSubscription {
    feed_id: String,
    guild_id: i64,
    channel_id: i64,
    mention_role: Option<i64>,
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

    #[cfg(feature = "feed_process")]
    pub async fn from_feed_id(pool: &sqlx::PgPool, feed_id: &str) -> Result<Vec<Self>> {
        sqlx::query_as!(
            FeedSubscription,
            r#"
            SELECT *
              FROM feed_subscriptions
             WHERE feed_id = $1
            "#,
            feed_id,
        )
        .fetch_all(pool)
        .await
        .map_err(Into::into)
    }

    pub async fn save(self, ctx: &Context) -> Result<Self> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        sqlx::query_as!(
            FeedSubscription,
            r#"
            INSERT INTO feed_subscriptions
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
}
