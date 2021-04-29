use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use twilight_model::id::RoleId;

use sushii_model::model::sql::{RuleGauge, RuleScope};

use crate::model::{RuleContext, Event};
use crate::model::has_id::*;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum Action {
    /// # Reply
    /// Sends a reply to a message trigger
    Reply { content: String },
    /// # Send message
    /// Sends a message to a channel
    SendMessage { channel_id: u64, content: String },
    // Counters
    /// # Add to a counter
    AddCounter {
        /// Name of counter
        name: String,
        /// Scope this counter applies to
        scope: RuleScope,
    },
    /// # Subtract from a counter
    SubtractCounter {
        /// Name of counter
        name: String,
        /// Scope this counter applies to
        scope: RuleScope,
    },
    /// # Reset a counter
    ResetCounter {
        /// Name of counter
        name: String,
        /// Scope this counter applies to
        scope: RuleScope,
    },
    // Moderation stuff
    /// # Ban
    /// Bans a user
    Ban {
        /// Days of messages to delete, max 8
        delete_days: u64,
        /// None for permanent, otherwise duration in seconds
        duration: Option<u64>,
        /// Reason for ban
        reason: Option<String>,
    },
    /// # Mute
    /// Mutes a user
    Mute {
        /// None for permanent, otherwise duration in seconds
        duration: Option<u64>,
        /// Reason for mute
        reason: Option<String>,
    },
}

impl Action {
    pub async fn execute(&self, event: Arc<Event>, ctx: &RuleContext<'_>) -> Result<()> {
        match *self {
            Self::Reply { ref content } => {
                let channel_id = event.channel_id()?;

                ctx.http
                    .create_message(channel_id)
                    .content(content)?
                    .await?;
            }
            // Moderation
            Self::Ban {
                delete_days,
                duration,
                ref reason,
            } => {
                let guild_id = event.guild_id()?;
                let user_id = event.user_id()?;

                ctx.http
                    .create_ban(guild_id, user_id)
                    .delete_message_days(delete_days)?
                    .await?;
            }
            Self::Mute {
                duration,
                ref reason,
            } => {
                let guild_id = event.guild_id()?;
                let user_id = event.user_id()?;

                ctx.http
                    .add_guild_member_role(guild_id, user_id, RoleId(123))
                    .await?;
            }
            // Counters
            Self::AddCounter { ref name, scope } => {
                let guild_id = event.guild_id()?;
                let scope_id = event.scope_id(scope)?;

                RuleGauge::inc(&ctx.pg_pool, guild_id.0, scope, scope_id, name).await?;
            }
            Self::SubtractCounter { ref name, scope } => {
                let guild_id = event.guild_id()?;
                let scope_id = event.scope_id(scope)?;

                RuleGauge::dec(&ctx.pg_pool, guild_id.0, scope, scope_id, name).await?;
            }
            Self::ResetCounter { ref name, scope } => {
                let guild_id = event.guild_id()?;
                let scope_id = event.scope_id(scope)?;

                RuleGauge::reset(&ctx.pg_pool, guild_id.0, scope, scope_id, name).await?;
            }
            _ => {}
        }

        Ok(())
    }
}
