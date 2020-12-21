use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::parse_mention;
use std::collections::HashMap;
use std::fmt::Write;

use crate::model::sql::ModLogEntry;

#[command]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
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
        let _ = write!(
            s,
            "`[{} | #{}]` **{}**",
            entry.action_time.format("%y-%m-%d %H:%M"),
            entry.case_id,
            entry.action
        );

        if let Some(id) = entry.executor_id {
            let _ = write!(s, " by <@{}>", id);
        }

        if let Some(ref reason) = entry.reason {
            let _ = write!(s, " for `{}`", reason);
        }

        let _ = writeln!(s);
    }

    let action_counts = entries.iter().fold(HashMap::new(), |mut acc, case| {
        let entry = acc.entry(&case.action).or_insert(0u64);
        *entry += 1;

        acc
    });

    let mut action_counts_vec: Vec<(&String, u64)> = action_counts.into_iter().collect();
    action_counts_vec.sort_by_cached_key(|x| x.0.chars().rev().collect::<String>());

    let action_counts_string = action_counts_vec
        .iter()
        .map(|case| format!("{} - {}", case.0, case.1))
        .collect::<Vec<String>>()
        .join("\n");

    let target_user = UserId(user_id).to_user(&ctx).await;

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
                e.field("Summary", action_counts_string, false);
                e.color(0xe67e22);

                e.footer(|f| f.text("Date format: YY-MM-DD • Times in UTC"));

                e
            })
        })
        .await?;

    Ok(())
}
