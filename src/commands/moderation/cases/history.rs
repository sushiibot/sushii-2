use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::parse_mention;
use std::fmt::Write;

use crate::keys::CacheAndHttpContainer;
use crate::model::sql::{ModLogEntry, ModLogEntryDb};

#[command]
#[only_in("guild")]
async fn history(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id.0,
        None => {
            msg.channel_id.say(&ctx.http, "No guild found").await?;

            return Ok(());
        }
    };

    let user_id_str = match args.single::<String>() {
        Ok(s) => s,
        Err(_) => {
            msg.channel_id
                .say(&ctx.http, "Please give a user ID")
                .await?;

            return Ok(());
        }
    };

    let user_id = match user_id_str
        .parse::<u64>()
        .ok()
        .or_else(|| parse_mention(user_id_str))
    {
        Some(id) => id,
        None => {
            msg.channel_id
                .say(&ctx.http, "Invalid user ID given")
                .await?;

            return Ok(());
        }
    };

    let entries = match ModLogEntry::get_user_entries(&ctx, guild_id, user_id).await {
        Ok(entries) => entries,
        Err(e) => {
            msg.channel_id
                .say(
                    &ctx.http,
                    "Something went wrong getting user case history :(",
                )
                .await?;

            tracing::error!(?msg, user_id, "Failed to get user mod log entries: {}", e);

            return Ok(());
        }
    };

    if entries.is_empty() {
        msg.channel_id
            .say(&ctx.http, "No cases found for user")
            .await?;

        return Ok(());
    }

    let mut s = String::new();

    for entry in &entries {
        let _ = write!(s, "`[Case #{}] {}`", entry.case_id, entry.action);

        if let Some(id) = entry.executor_id {
            let _ = write!(s, " by <@{}>", id);
        }

        if let Some(ref reason) = entry.reason {
            let _ = write!(s, " for `{}`", reason);
        }

        let _ = write!(s, "\n");
    }

    let data = &ctx.data.read().await;
    let cache_http = data.get::<CacheAndHttpContainer>().unwrap();

    let target_user = UserId(user_id).to_user(&cache_http).await;

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.author(|a| {
                    if let Ok(ref user) = target_user {
                        a.icon_url(user.face());
                    }

                    a.name(format!(
                        "Case history for {} (ID: {})",
                        target_user
                            .map(|u| u.tag())
                            .unwrap_or_else(|_| "user".to_string()),
                        user_id
                    ));

                    a
                });

                e.description(&s);
                e.color(0xe67e22);

                e
            })
        })
        .await?;

    Ok(())
}
