use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use twilight_model::gateway::event::EventType;

use crate::error::Error;

// Main 3 triggers should just be:
// message, member update, member join / leave
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Trigger {
    #[serde(rename = "GUILD_MEMBER_ADD")]
    MemberAdd,
    #[serde(rename = "GUILD_MEMBER_REMOVE")]
    MemberRemove,
    #[serde(rename = "GUILD_MEMBER_UPDATE")]
    MemberUpdate,
    #[serde(rename = "GUILD_MEMBERS_CHUNK")]
    MemberChunk,
    MessageCreate,
    MessageDelete,
    MessageDeleteBulk,
    MessageUpdate,
    /// # Sushii Counter
    /// When a counter is modified
    Counter,
    /// # Sushii level up
    /// When a member levels up
    LevelUp,
}

impl TryFrom<EventType> for Trigger {
    type Error = Error;

    fn try_from(event_type: EventType) -> Result<Self, Error> {
        match event_type {
            EventType::MemberAdd => Ok(Self::MemberAdd),
            EventType::MemberRemove => Ok(Self::MemberRemove),
            EventType::MemberUpdate => Ok(Self::MemberUpdate),
            EventType::MemberChunk => Ok(Self::MemberChunk),
            EventType::MessageCreate => Ok(Self::MessageCreate),
            EventType::MessageDelete => Ok(Self::MessageDelete),
            EventType::MessageDeleteBulk => Ok(Self::MessageDeleteBulk),
            EventType::MessageUpdate => Ok(Self::MessageUpdate),
            _ => Err(Error::UnsupportedEvent),
        }
    }
}

impl Trigger {
    pub fn name(&self) -> Option<&'static str> {
        match self {
            Self::MemberAdd => Some("GUILD_MEMBER_ADD"),
            Self::MemberRemove => Some("GUILD_MEMBER_REMOVE"),
            Self::MemberUpdate => Some("GUILD_MEMBER_UPDATE"),
            Self::MemberChunk => Some("GUILD_MEMBERS_CHUNK"),
            Self::MessageCreate => Some("MESSAGE_CREATE"),
            Self::MessageDelete => Some("MESSAGE_DELETE"),
            Self::MessageDeleteBulk => Some("MESSAGE_DELETE_BULK"),
            Self::MessageUpdate => Some(""),
            Self::Counter => Some("COUNTER"),
            Self::LevelUp => Some("LEVEL_UP"),
        }
    }
}
