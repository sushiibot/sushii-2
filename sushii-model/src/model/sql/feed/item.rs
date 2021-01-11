use serde::{Deserialize, Serialize};

use crate::error::Result;

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct FeedItem {
    feed_id: String,
    item_id: String,
}
