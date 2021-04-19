use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum Action {
    Ban {
        user_id: u64,
        /// Days of messages to delete, max 8
        delete_days: u8,
        /// None for permanent, otherwise duration in seconds
        duration: Option<u64>,
    },
}
