use anyhow::Result;
use async_recursion::async_recursion;
use chrono::Duration;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::sync::Arc;
use twilight_http::request::AuditLogReason;
use twilight_model::id::RoleId;

use sushii_model::model::sql::{ModLogEntry, Mute, RuleGauge, RuleScope};

use crate::error::Error;
use crate::model::has_id::*;
use crate::model::{Condition, Event, RuleContext};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
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
    /// # Conditional Actions
    /// Run actions based on additional conditions
    SubCondition {
        /// Condition to run the actions
        condition: Condition,
        /// Actions to run if conditions pass
        actions: Vec<Action>,
        /// Actions to run if conditions do **not** pass
        actions_else: Vec<Action>,
    },
}

impl Action {
    #[async_recursion]
    pub async fn execute(&self, event: Arc<Event>, mut ctx: &mut RuleContext<'_>) -> Result<()> {
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
                duration,
                ref reason,
            } => {
                let guild_id = event.guild_id()?;
                let user = event.user()?;

                let mute_role = ctx
                    .guild_config
                    .mute_role
                    .ok_or(Error::GuildConfigMissingField("mute_role".into()))?;

                // Start transaction since adding role could fail and we don't
                // want pending entry sitting around if it does
                let mut txn = ctx.pg_pool.begin().await?;

                // Create pending case
                let entry = ModLogEntry::new(
                    "mute",
                    true,
                    guild_id.0,
                    user.id.0,
                    &format!("{}#{:0>4}", user.name, user.discriminator),
                )
                .reason(reason)
                .save_exec(&mut txn)
                .await?;

                // Add new mute entry
                let mute_entry = Mute::new(
                    guild_id.0,
                    user.id.0,
                    duration
                        .and_then(|s| s.try_into().ok())
                        .map(Duration::seconds),
                )
                .pending(true)
                .save_exec(&mut txn)
                .await?;

                // After everything else successful, commit
                // Can't do this after the role add since we need it in the db
                // before role is added
                txn.commit().await?;

                let mut add_role_fut =
                    ctx.http
                        .add_guild_member_role(guild_id, user.id, RoleId(mute_role as u64));

                // TODO: Add default reason
                if let Some(reason) = reason {
                    add_role_fut = add_role_fut.reason(reason)?;
                }

                // Add mute role to user
                if let Err(e) = add_role_fut.await {
                    entry.delete_exec(&ctx.pg_pool).await?;
                    mute_entry.delete_exec(&ctx.pg_pool).await?;

                    return Err(e.into());
                }

                // Add mute entry to handlebars ctx data
                ctx.data.actions.push(serde_json::to_value(&entry)?);
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
            Self::SubCondition {
                ref condition,
                ref actions,
                ref actions_else,
            } => {
                if condition.check_event(event.clone(), ctx).await? {
                    for action in actions {
                        action.execute(event.clone(), &mut ctx).await?;
                    }
                } else {
                    for action in actions_else {
                        action.execute(event.clone(), &mut ctx).await?;
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }
}
