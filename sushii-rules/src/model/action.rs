use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use twilight_model::gateway::event::DispatchEvent;
use twilight_model::id::ChannelId;

use super::RuleContext;
use crate::error::Error;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum Action {
    Ban {
        user_id: u64,
        /// Days of messages to delete, max 8
        delete_days: u8,
        /// None for permanent, otherwise duration in seconds
        duration: Option<u64>,
    },
    Reply {
        content: String,
    },
    SendMessage {
        channel_id: u64,
        content: String,
    },
}

impl Action {
    pub async fn execute(&self, event: Arc<DispatchEvent>, ctx: &RuleContext) -> Result<()> {
        match *self {
            Self::Reply { ref content } => {
                let channel_id = event.channel_id().ok_or(Error::MissingChannelId)?;

                ctx.http
                    .create_message(channel_id)
                    .content(content)?
                    .await?;

                Ok(())
            }
            _ => Ok(()),
        }
    }
}

pub trait HasChannelId {
    fn channel_id(&self) -> Option<ChannelId>;
}

impl HasChannelId for DispatchEvent {
    fn channel_id(&self) -> Option<ChannelId> {
        match *self {
            Self::MessageCreate(ref msg) => Some(msg.channel_id),
            _ => None,
        }
    }
}
