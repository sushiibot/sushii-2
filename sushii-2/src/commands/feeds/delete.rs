use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::parse_channel;

use crate::model::sql::*;

#[command]
#[required_permissions("MANAGE_GUILD")]
async fn delete(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            msg.channel_id.say(ctx, "Error: Not in guild").await?;
            return Ok(());
        }
    };

    let feed_id = match args.single::<String>() {
        Ok(s) => s,
        Err(_) => {
            msg.channel_id
                .say(
                    ctx,
                    "Error: Give a feed ID to delete. Use `feeds list` to find the feed ID.",
                )
                .await?;

            return Ok(());
        }
    };

    let channel_id_str = match args.single::<String>() {
        Ok(s) => s,
        Err(_) => {
            msg.channel_id
                .say(
                    ctx,
                    "Error: Give the Discord channel of the feed you want to delete",
                )
                .await?;

            return Ok(());
        }
    };

    let channel_id = match channel_id_str
        .parse::<u64>()
        .ok()
        .or_else(|| parse_channel(&channel_id_str))
    {
        Some(id) => id,
        None => {
            msg.channel_id.say(ctx, "Error: Invalid channel").await?;

            return Ok(());
        }
    };

    let guild_channels = match msg.guild_field(ctx, |g| g.channels.clone()).await {
        Some(channels) => channels,
        None => {
            tracing::warn!(?msg, "Failed to get guild_channels");

            msg.channel_id
                .say(ctx, "Error: Couldn't find server channels, try again?")
                .await?;

            return Ok(());
        }
    };

    match guild_channels.get(&ChannelId(channel_id)) {
        Some(c) if c.kind != ChannelType::Text => {
            msg.channel_id
                .say(
                    ctx,
                    "Error: Channel is not a text channel. Try a different one.",
                )
                .await?;

            return Ok(());
        }
        None => {
            msg.channel_id
                .say(ctx, "Error: Channel is not found in this server")
                .await?;
            return Ok(());
        }
        _ => {}
    }

    let sub = match FeedSubscription::from_id(ctx, guild_id, &feed_id).await? {
        Some(s) => s,
        None => {
            msg.channel_id
                .say(
                    ctx,
                    format!(
                        "There is no feed with ID `{}`, you check feed IDs with `feed list`",
                        feed_id
                    ),
                )
                .await?;

            return Ok(());
        }
    };

    sub.delete(ctx).await?;

    msg.channel_id
        .say(ctx, format!("Deleted feed `{}`", feed_id))
        .await?;

    Ok(())
}
