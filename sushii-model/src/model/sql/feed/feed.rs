use chrono::naive::NaiveDateTime;
use chrono::offset::Utc;
use serde::{Deserialize, Serialize};
use serenity::model::prelude::*;
use serenity::prelude::*;
use sqlx::types::Json;

use crate::error::Result;
use crate::keys::DbPool;

pub trait Id {
    fn id(&self) -> String;
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RssFeedMetadata {
    /// Feed title, e.g. Twitter @username
    pub title: String,
    /// rss-bridge feed url to actually request
    pub feed_url: String,
    /// Original data url, e.g. https://twitter.com/username
    pub source_url: String,
}

impl Id for RssFeedMetadata {
    fn id(&self) -> String {
        format!("rss:{}", self.feed_url)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VliveChannelMeta {
    pub channel_seq: Option<i64>,
    pub channel_code: String,
    pub channel_name: String,
    pub channel_icon_url: String,
}

impl VliveChannelMeta {
    pub fn new(
        channel_seq: Option<i64>,
        channel_code: String,
        channel_name: String,
        channel_icon_url: String,
    ) -> Self {
        Self {
            channel_seq,
            channel_code,
            channel_name,
            channel_icon_url,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VliveVideosMetadata {
    pub channel: VliveChannelMeta,
}

impl Id for VliveVideosMetadata {
    fn id(&self) -> String {
        format!("vlive:videos:{}", self.channel.channel_code)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct VliveBoardMetadata {
    pub channel: VliveChannelMeta,
    pub board_id: i64,
}

impl Id for VliveBoardMetadata {
    fn id(&self) -> String {
        format!("vlive:board:{}:{}", self.channel.channel_code, self.board_id)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum FeedMetadata {
    Rss(RssFeedMetadata),
    VliveBoard(VliveBoardMetadata),
    VliveVideos(VliveVideosMetadata),
}

impl FeedMetadata {
    pub fn rss(title: String, feed_url: String, source_url: String) -> Self {
        Self::Rss(RssFeedMetadata {
            title,
            feed_url,
            source_url,
        })
    }

    pub fn vlive_videos(
        channel_seq: Option<i64>,
        channel_code: String,
        channel_name: String,
        channel_icon_url: String,
    ) -> Self {
        Self::VliveVideos(VliveVideosMetadata {
            channel: VliveChannelMeta::new(
                channel_seq,
                channel_code,
                channel_name,
                channel_icon_url,
            ),
        })
    }

    pub fn vlive_board(
        channel_seq: Option<i64>,
        channel_code: String,
        channel_name: String,
        channel_icon_url: String,
        board_id: i64,
    ) -> Self {
        Self::VliveBoard(VliveBoardMetadata {
            channel: VliveChannelMeta::new(
                channel_seq,
                channel_code,
                channel_name,
                channel_icon_url,
            ),
            board_id,
        })
    }
}

impl Id for FeedMetadata {
    fn id(&self) -> String {
        match self {
            Self::Rss(m) => m.id(),
            Self::VliveBoard(m) => m.id(),
            Self::VliveVideos(m) => m.id(),
        }
    }
}

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct Feed {
    /// Feed unique identifier, can be feed URL or a vlive channel
    pub feed_id: String,
    pub metadata: Json<FeedMetadata>,
}

impl Feed {
    pub fn new(metadata: FeedMetadata) -> Self {
        Self {
            feed_id: metadata.id(),
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
