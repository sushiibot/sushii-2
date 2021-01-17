use serde::{Deserialize, Serialize};

use crate::error::Result;

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
}
