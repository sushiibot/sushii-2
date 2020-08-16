use chrono::naive::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct ModLogEntry {
    pub guild_id: i64,
    pub case_id: i64,

    /// Metadata
    pub action: String,
    pub action_time: NaiveDateTime,
    pub pending: bool,

    /// Target user info
    pub user_id: i64,
    pub user_tag: String,

    /// Moderator id
    pub executor_id: Option<i64>,
    pub reason: Option<String>,
    pub msg_id: Option<i64>,
}
