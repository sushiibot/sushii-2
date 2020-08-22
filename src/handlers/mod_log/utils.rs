use serenity::{model::prelude::*, prelude::*};

use crate::error::Result;
use crate::model::{sql::*, sushii_config::*};

pub async fn guild_ban_handler(
    ctx: &Context,
    guild_id: &GuildId,
    user: &User,
    action: &str,
) -> Result<()> {
    // check if a ban/unban command was used instead of discord right click ban
    // add the action to the database if not pendings
    let mut entry =
        match ModLogEntry::get_pending_entry(&ctx, action, guild_id.0, user.id.0).await? {
            Some(entry) => entry,
            None => {
                ModLogEntry::new(action, false, guild_id.0, &user)
                    .save(&ctx)
                    .await?
            }
        };

    let executor_user = get_user_or_bot(&ctx, entry.executor_id).await;
    let guild_conf = match GuildConfig::from_id(&ctx, guild_id).await? {
        Some(c) => c,
        None => {
            tracing::error!(?guild_id, ?user, "No guild config found while handling ban");
            return Ok(());
        }
    };

    if entry.reason.is_none() {
        let prefix = match guild_conf.prefix {
            Some(p) => p,
            None => SushiiConfig::get(&ctx).await.default_prefix,
        };

        entry.reason.replace(
            format!("Responsible moderator: Please use `{}reason {} [reason]` to set a reason for this case.",
                prefix, entry.case_id)
        );
    }

    if let Some(channel_id) = guild_conf.log_mod {
        match send_mod_log_entry(&ctx, channel_id, &entry, &executor_user, &user).await {
            Ok(msg) => {
                entry.msg_id.replace(msg.id.0 as i64);
            }
            Err(e) => tracing::error!("Failed to send mod log entry message: {}", e),
        }
    }

    entry.pending = false;
    entry.save(&ctx).await?;

    Ok(())
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

async fn send_mod_log_entry(
    ctx: &Context,
    channel_id: i64,
    mod_log_entry: &ModLogEntry,
    executor_user: &User,
    user: &User,
) -> Result<Message> {
    ChannelId(channel_id as u64)
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.author(|a| {
                    a.icon_url(executor_user.face());
                    a.name(executor_user.tag());

                    a
                });

                e.field(
                    "User",
                    format!("{}\n{}\n{}", user.mention(), user.tag(), user.id.0),
                    false,
                );
                e.field("Action", &mod_log_entry.action, false);
                // Reason shouldn't be a None but just to make sure
                e.field(
                    "Reason",
                    mod_log_entry
                        .reason
                        .clone()
                        .unwrap_or_else(|| "N/A".to_string()),
                    false,
                );

                e.footer(|f| {
                    f.text(format!("Case #{}", &mod_log_entry.case_id));

                    f
                });

                e.timestamp(
                    mod_log_entry
                        .action_time
                        .format("%Y-%m-%dT%H:%M:%S%")
                        .to_string(),
                );

                e
            })
        })
        .await
        .map_err(Into::into)
}
