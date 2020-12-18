use serenity::{model::prelude::*, prelude::*};
use std::fmt::Write;
use std::time::Duration;

use crate::error::{Error, Result};
use crate::model::{sql::*, sushii_config::*};

/// Struct to help send mod action entries to the mod log
#[derive(Debug)]
pub struct ModLogReporter<'a> {
    guild_id: &'a GuildId,
    user: &'a User,
    action: &'a str,
    duration: Option<Duration>,

    /// Mute entry if this is an unmute
    initial_entry: Option<ModLogEntry>,
    placeholder_reason: Option<String>,
}

impl<'a> ModLogReporter<'a> {
    pub fn new(guild_id: &'a GuildId, user: &'a User, action: &'a str) -> Self {
        ModLogReporter {
            guild_id,
            user,
            action,
            // duration and initial_entry default None
            duration: None,
            initial_entry: None,
            placeholder_reason: None,
        }
    }

    pub fn mute_duration(mut self, duration: Option<Duration>) -> Self {
        self.duration = duration;
        self
    }

    pub fn initial_entry(mut self, initial_entry: Option<ModLogEntry>) -> Self {
        self.initial_entry = initial_entry;
        self
    }

    pub async fn execute(&self, ctx: &Context) -> Result<ModLogEntry> {
        // check if a ban/unban command was used instead of discord right click ban
        // add the action to the database if not pendings
        let mut entry = match ModLogEntry::get_pending_entry(
            &ctx,
            self.action,
            self.guild_id.0,
            self.user.id.0,
        )
        .await?
        {
            Some(entry) => entry,
            None => {
                ModLogEntry::new(self.action, false, self.guild_id.0, &self.user)
                    .save(&ctx)
                    .await?
            }
        };

        // If this is an AUTOMATED unmute **only**, use the initial entry executor
        // though im not sure if it should even be also the muter?
        /*
        let executor_id = if let Some(reason) = entry.reason {
            if reason.starts_with("Automated Unmute: Mute expired") {
                self.initil
            }
        }
        */

        let executor_user = get_user_or_bot(&ctx, entry.executor_id).await;
        let guild_conf = match GuildConfig::from_id(&ctx, self.guild_id).await? {
            Some(c) => c,
            None => {
                tracing::error!(
                    ?self.guild_id,
                    ?self.user,
                    "No guild config found while handling mod_log"
                );

                return Err(Error::Sushii("Missing guild".into()));
            }
        };

        let placeholder_reason = if entry.reason.is_none() {
            let prefix = match guild_conf.prefix {
                Some(p) => p,
                None => SushiiConfig::get(&ctx).await.default_prefix.clone(),
            };

            format!("Responsible moderator: Please use `{}reason {} [reason]` to set a reason for this case.",
                prefix, entry.case_id)
        } else {
            "N/A".into()
        };

        if let Some(channel_id) = guild_conf.log_mod {
            match self
                .send_message(
                    &ctx,
                    &entry,
                    channel_id as u64,
                    executor_user,
                    placeholder_reason,
                )
                .await
            {
                Ok(msg) => {
                    entry.msg_id.replace(msg.id.0 as i64);
                }
                Err(e) => tracing::error!("Failed to send mod log entry message: {}", e),
            }
        }

        entry.pending = false;
        entry.save(&ctx).await?;

        Ok(entry)
    }

    async fn send_message(
        &self,
        ctx: &Context,
        entry: &ModLogEntry,
        channel_id: u64,
        executor_user: User,
        placeholder_reason: String,
    ) -> Result<Message> {
        let mut s = String::new();

        let _ = writeln!(
            s,
            "**User:** {} `{}` | `{}`",
            self.user.mention(),
            self.user.tag(),
            self.user.id.0
        );
        let _ = writeln!(s, "**Action:** {}", entry.action);

        if entry.action == "mute" {
            let _ = writeln!(
                s,
                "**Duration:** {}",
                self.duration.map_or_else(
                    || "Indefinite".to_string(),
                    |d| humantime::format_duration(d).to_string(),
                )
            );
        }

        let _ = writeln!(
            s,
            "**Reason:** {}",
            entry.reason.clone().unwrap_or(placeholder_reason)
        );

        ChannelId(channel_id)
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.author(|a| {
                        a.icon_url(executor_user.face());
                        a.name(executor_user.tag());

                        a
                    });

                    e.description(s);

                    e.footer(|f| {
                        f.text(format!("Case #{}", &entry.case_id));

                        f
                    });

                    e.timestamp(entry.action_time.format("%Y-%m-%dT%H:%M:%S").to_string());

                    e.color(entry.color());

                    e
                })
            })
            .await
            .map_err(Into::into)
    }
}

async fn get_user_or_bot(ctx: &Context, id: Option<i64>) -> User {
    // No user provided, use bot
    if id.is_none() {
        return ctx.cache.current_user().await.into();
    }

    // Fetch from cache or http
    if let Some(user) = crate::utils::user::get_user(&ctx, id.unwrap() as u64).await {
        return user;
    }

    // Still failed, use bot
    ctx.cache.current_user().await.into()
}
