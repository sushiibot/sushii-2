use lazy_static::lazy_static;
use regex::Regex;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::error::Error as ModelError;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::Error;
use std::collections::HashSet;
use std::fmt::Write;

use crate::keys::CacheAndHttpContainer;
use crate::model::sql::{ModLogEntry, ModLogEntryDb};

#[command]
#[only_in("guild")]
async fn ban(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            msg.channel_id.say(&ctx.http, "No guild found").await?;

            return Ok(());
        }
    };

    lazy_static! {
        // Can overflow, so need to handle later
        static ref RE: Regex = Regex::new(r"\d{18,19}").unwrap();
    }

    let ids_and_reason = args.rest();

    let (ids, end) = RE
        .find_iter(ids_and_reason)
        .fold((Vec::new(), 0), |mut acc, id_match| {
            if let Ok(id) = id_match.as_str().parse::<u64>() {
                acc.0.push(id);
                acc.1 = id_match.end();
            }

            acc
        });

    let reason = &ids_and_reason[end..].trim();

    let mut bans = match guild_id.bans(&ctx.http).await {
        Ok(val) => val.iter().map(|x| x.user.id.0).collect::<HashSet<u64>>(),
        Err(e) => {
            tracing::warn!("Failed to get guild bans: {}", e);

            HashSet::new()
        }
    };

    let data = &ctx.data.read().await;
    let cache_http = data.get::<CacheAndHttpContainer>().unwrap();

    let mut s = String::new();

    for id in ids {
        let user = match UserId(id).to_user(cache_http).await {
            Ok(u) => u,
            Err(e) => {
                let _ = write!(s, ":x: {} - Error: Failed to fetch user: {}\n", id, &e);

                continue;
            }
        };

        let user_tag_id = format!("`{} ({})`", user.tag(), user.id.0);

        if bans.contains(&id) {
            let _ = write!(s, ":x: {} - Error: User is already banned\n", user_tag_id);
            continue;
        }

        let entry = match ModLogEntry::new("ban", true, guild_id.0, &user)
            .save(&ctx)
            .await
        {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("Failed to save mod log entry: {}", e);

                let _ = write!(
                    s,
                    ":x: {} - Error: Something went wrong saving this case :(\n",
                    &user_tag_id
                );
                continue;
            }
        };

        let ban_res = if reason.is_empty() {
            guild_id.ban(&ctx.http, user, 7u8).await
        } else {
            guild_id
                .ban_with_reason(&ctx.http, user, 7u8, &reason)
                .await
        };

        match ban_res {
            Err(Error::Model(ModelError::InvalidPermissions(permissions))) => {
                let e = format!(
                    "I don't have permission to ban this user, requires: `{:?}`.",
                    permissions
                );
                let _ = write!(s, ":question: {} - Error: {}\n", &user_tag_id, &e);
                if let Err(e) = entry.delete(&ctx).await {
                    tracing::error!("Failed to delete entry: {}", e);
                }
            }
            Err(Error::Model(ModelError::DeleteMessageDaysAmount(num))) => {
                let e = format!(
                    "The number of days worth of messages to delete is over the maximum: ({}).",
                    num
                );
                let _ = write!(s, ":x: {} - Error: {}\n", &user_tag_id, &e);
                if let Err(e) = entry.delete(&ctx).await {
                    tracing::error!("Failed to delete entry: {}", e);
                }
            }
            Err(_) => {
                let e = "There was an unknown error trying to ban this user.";
                let _ = write!(s, ":question: {} - Error: {}\n", &user_tag_id, &e);
                if let Err(e) = entry.delete(&ctx).await {
                    tracing::error!("Failed to delete entry: {}", e);
                }
            }
            Ok(_) => {
                let _ = write!(s, ":hammer: {} banned.\n", &user_tag_id);
                // add the ban to hashset to prevent dupe bans
                bans.insert(id);
            }
        }
    }

    msg.channel_id.say(&ctx.http, "Banned").await?;

    Ok(())
}
