use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use twilight_http::request::AuditLogReason;
use twilight_model::id::RoleId;

use sushii_model::model::sql::{RuleGauge, RuleScope};

use crate::model::has_id::*;
use crate::model::{Event, RuleContext};

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
    pub async fn execute(&self, event: Arc<Event>, ctx: &mut RuleContext<'_>) -> Result<()> {
        match *self {
            Self::Reply { ref content } => {
                let channel_id = event.channel_id()?;
                let message_id = event.message_id()?;

                let rendered_content = ctx.render_string(event, content).await?;

                ctx.http
                    .create_message(channel_id)
                    .content(rendered_content)?
                    .reply(message_id)
                    // Add required mentions in order to ping the user
                    .allowed_mentions()
                    .replied_user(true)
                    .build()
                    .await?;
            }
            // Moderation
            Self::Ban {
                delete_days,
                duration: _,
                ref reason,
            } => {
                let guild_id = event.guild_id()?;
                let user_id = event.user_id()?;

                let mut fut = ctx
                    .http
                    .create_ban(guild_id, user_id)
                    .delete_message_days(delete_days)?;

                // TODO: Add default reason
                if let Some(reason) = reason {
                    fut = fut.reason(reason)?;
                }

                fut.await?;
            }
            Self::Mute {
                duration: _,
                ref reason,
            } => {
                let guild_id = event.guild_id()?;
                let user_id = event.user_id()?;

                let mut fut = ctx
                    .http
                    .add_guild_member_role(guild_id, user_id, RoleId(123));

                // TODO: Add default reason
                if let Some(reason) = reason {
                    fut = fut.reason(reason)?;
                }

                fut.await?;
            }
            // Counters
            Self::AddCounter { ref name, scope } => {
                let guild_id = event.guild_id()?;
                let scope_id = event.scope_id(scope)?;

                let counter =
                    RuleGauge::inc(&ctx.pg_pool, guild_id.0, scope, scope_id, name).await?;

                ctx.data.actions.push(serde_json::to_value(&counter)?);

                // Only trigger if incrementing from a twilight event Don't
                // trigger another if this is currently a counter otherwise that
                // would cause infinite loops
                if let Event::Twilight(original_event) = (*event).clone() {
                    tracing::debug!(?counter, "Triggering new Counter event");
                    ctx.channel_tx
                        .send(Event::Counter {
                            counter,
                            original_event,
                        })
                        .await?;
                }
            }
            Self::SubtractCounter { ref name, scope } => {
                let guild_id = event.guild_id()?;
                let scope_id = event.scope_id(scope)?;

                let counter =
                    RuleGauge::dec(&ctx.pg_pool, guild_id.0, scope, scope_id, name).await?;

                ctx.data.actions.push(serde_json::to_value(&counter)?);

                if let Event::Twilight(original_event) = (*event).clone() {
                    tracing::debug!(?counter, "Triggering new Counter event");
                    ctx.channel_tx
                        .send(Event::Counter {
                            counter,
                            original_event,
                        })
                        .await?;
                }
            }
            Self::ResetCounter { ref name, scope } => {
                let guild_id = event.guild_id()?;
                let scope_id = event.scope_id(scope)?;

                let counter =
                    RuleGauge::reset(&ctx.pg_pool, guild_id.0, scope, scope_id, name).await?;

                ctx.data.actions.push(serde_json::to_value(&counter)?);

                if let Event::Twilight(original_event) = (*event).clone() {
                    tracing::debug!(?counter, "Triggering new Counter event");
                    ctx.channel_tx
                        .send(Event::Counter {
                            counter,
                            original_event,
                        })
                        .await?;
                }
            }
            _ => {}
        }

        Ok(())
    }
}
