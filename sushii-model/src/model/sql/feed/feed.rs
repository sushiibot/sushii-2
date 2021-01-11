use chrono::naive::NaiveDateTime;
use chrono::offset::Utc;
use serde::{Deserialize, Serialize};
use serenity::model::prelude::*;
use serenity::prelude::*;
use sqlx::types::Json;

use crate::error::Result;
use crate::keys::DbPool;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RssFeedMetadata {
    /// Feed title, e.g. Twitter @username
    pub title: String,
    /// rss-bridge feed url to actually request
    pub feed_url: String,
    /// Original data url, e.g. https://twitter.com/username
    pub source_url: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VliveChannelMeta {
    pub channel_seq: i64,
    pub channel_code: String,
    pub channel_name: String,
    pub channel_icon_url: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VliveVideosMetadata {
    pub channel: VliveChannelMeta,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VliveBoardMetadata {
    pub channel: VliveChannelMeta,
    pub board_id: i64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum FeedMetadata {
    Rss(RssFeedMetadata),
    VliveBoard(VliveBoardMetadata),
    VliveVideos(VliveVideosMetadata),
}

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct Feed {
    /// Feed unique identifier, can be feed URL or a vlive channel
    pub feed_id: String,
    pub metadata: Json<FeedMetadata>,
}

impl Feed {
    pub fn new(feed_id: impl Into<String>, metadata: FeedMetadata) -> Self {
        Self {
            feed_id: feed_id.into(),
            metadata: Json(metadata),
        }
    }

    #[cfg(not(feature = "feed_process"))]
    pub async fn save(self, ctx: &Context) -> Result<Self> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        sqlx::query_as!(
            Feed,
            r#"
            INSERT INTO feeds
                 VALUES ($1, $2)
            ON CONFLICT (feed_id)
              DO UPDATE
                    SET metadata = $2
            "#,
            self.feed_id,
            self.metadata as _, // Converts to serde_json::Value I think
        )
        .execute(&pool)
        .await?;

        // Instead of RETURNING * just return self
        Ok(self)
    }
}
