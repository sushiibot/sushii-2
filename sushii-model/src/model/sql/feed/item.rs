use serenity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::keys::DbPool;

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct FeedItem {
    feed_id: String,
    item_id: String,
}

impl FeedItem {
    pub fn new(feed_id: impl Into<String>, item_id: impl Into<String>) -> Self {
        Self {
            feed_id: feed_id.into(),
            item_id: item_id.into(),
        }
    }

    pub async fn from_id(ctx: &Context, feed_id: &str, item_id: &str) -> Result<Option<Self>> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        sqlx::query_as!(
            FeedItem,
            r#"
            SELECT *
              FROM feed_items
             WHERE feed_id = $1
               AND item_id = $2
            "#,
            feed_id,
            item_id,
        )
        .fetch_optional(&pool)
        .await
        .map_err(Into::into)
    }

    pub async fn save(self, ctx: &Context) -> Result<Self> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        sqlx::query_as!(
            FeedItem,
            r#"
            INSERT INTO feed_items
                 VALUES ($1, $2)
            "#,
            self.feed_id,
            self.item_id,
        )
        .execute(&pool)
        .await?;

        Ok(self)
    }
}
