use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::fmt::Write;

use crate::model::sql::*;

#[command]
#[required_permissions("MANAGE_GUILD")]
async fn list(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            msg.channel_id.say(ctx, "Error: Not in guild").await?;
            return Ok(());
        }
    };

    let subs = FeedSubscription::from_guild_id(ctx, guild_id).await?;

    if subs.is_empty() {
        msg.channel_id
            .say(
                ctx,
                "There are no active feed subscriptions in this guild, you can add \
            one with `feed add`",
            )
            .await?;

        return Ok(());
    }

    let mut s = String::new();

    writeln!(s, "Feed ID - Channel - Mention Role - Feed Name")?;

    for sub in subs {
        // TODO: Prevent N + 1 here, though shouldn't be too many queries
        let feed = match Feed::from_id(ctx, &sub.feed_id).await? {
            Some(f) => f,
            None => {
                tracing::warn!(?sub, "Feed subscription missing feed");
                continue;
            }
        };

        write!(s, "`{}` - <#{}> ", feed.feed_id, sub.channel_id as u64)?;
        if let Some(id) = sub.mention_role {
            write!(s, "<@&{}> ", id)?;
        }
        writeln!(s, "{}", feed.name().unwrap_or("Unknown Feed"))?;
    }

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("Server Feeds");
                e.description(s);

                e
            });

            m
        })
        .await?;

    Ok(())
}
