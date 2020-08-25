use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::error::Error as ModelError;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::Error;
use std::fmt::Write;

use super::utils::parse_id_reason;
use crate::keys::CacheAndHttpContainer;
use crate::model::sql::{ModLogEntry, ModLogEntryDb};

// TODO: Merge repeat logic with ban command?
#[command]
#[only_in("guild")]
async fn kick(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            msg.channel_id.say(&ctx.http, "No guild found").await?;

            return Ok(());
        }
    };

    let (mut ids, reason) = parse_id_reason(args);

    // Remove duplicates instead of just keeping track of already kicked
    ids.sort_unstable();
    ids.dedup();

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

        let entry = match ModLogEntry::new("kick", true, guild_id.0, &user)
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

        let kick_res = if reason.is_empty() {
            guild_id.kick(&ctx.http, user).await
        } else {
            guild_id.kick_with_reason(&ctx.http, user, &reason).await
        };

        match kick_res {
            Err(Error::Model(ModelError::InvalidPermissions(permissions))) => {
                let e = format!(
                    "I don't have permission to kick this user, requires: `{:?}`.",
                    permissions
                );
                let _ = write!(s, ":question: {} - Error: {}\n", &user_tag_id, &e);
                if let Err(e) = entry.delete(&ctx).await {
                    tracing::error!("Failed to delete entry: {}", e);
                }
            }
            Err(_) => {
                let e = "There was an unknown error trying to kick this user.";
                let _ = write!(s, ":question: {} - Error: {}\n", &user_tag_id, &e);
                if let Err(e) = entry.delete(&ctx).await {
                    tracing::error!("Failed to delete entry: {}", e);
                }
            }
            Ok(_) => {
                let _ = write!(s, ":boot: {} kick.\n", &user_tag_id);
            }
        }
    }

    msg.channel_id.say(&ctx.http, "Kicked").await?;

    Ok(())
}
