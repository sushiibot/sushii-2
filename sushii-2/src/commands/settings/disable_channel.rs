use lazy_static::lazy_static;
use regex::Regex;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::HashSet;

use crate::model::sql::*;

#[command]
#[required_permissions("MANAGE_GUILD")]
#[aliases("listdisabledchannels", "channels")]
async fn disabledchannels(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_conf = GuildConfig::from_msg_or_respond(&ctx, msg).await?;

    let channels = match guild_conf.disabled_channels {
        Some(c) => c,
        None => {
            msg.channel_id
                .say(&ctx.http, "There are no disabled channels")
                .await?;

            return Ok(());
        }
    };

    let channel_ids_str = channels
        .into_iter()
        .map(|id| format!("<#{}>", id as u64))
        .collect::<Vec<_>>()
        .join("\n");

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Disabled Channels");
                e.color(0xe67e22);

                e.description(channel_ids_str);

                e
            })
        })
        .await?;

    Ok(())
}

#[command]
#[required_permissions("MANAGE_GUILD")]
#[aliases("disablechannels")]
async fn disablechannel(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut guild_conf = GuildConfig::from_msg_or_respond(&ctx, msg).await?;
    let guild_channels: HashSet<u64> = match msg
        .guild_field(ctx, |g| {
            g.channels
                .keys()
                .map(|id| id.0)
                .collect::<HashSet<u64>>()
                .clone()
        })
        .await
    {
        Some(c) => c,
        None => {
            msg.channel_id
                .say(&ctx.http, "Error: Failed to get guild channels")
                .await?;

            return Ok(());
        }
    };

    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?:<#|[^#&\d]|^)(\d{17,19})>?").unwrap();
    }

    let channels_str = args.rest();
    let channel_ids: Vec<u64> = RE
        .captures_iter(channels_str)
        .filter_map(|caps| caps.get(1).and_then(|m| m.as_str().parse::<u64>().ok()))
        .filter(|id| guild_channels.contains(id))
        .collect();

    if channel_ids.is_empty() {
        msg.channel_id
            .say(&ctx.http, "Error: Please give channels to disable")
            .await?;

        return Ok(());
    }

    let channel_ids_str = channel_ids
        .iter()
        .map(|id| format!("<#{}>", id))
        .collect::<Vec<_>>()
        .join("\n");

    let mut new_disabled_channels: Vec<i64> =
        if let Some(ref disabled_channels) = guild_conf.disabled_channels {
            channel_ids
                .into_iter()
                .map(|id| id as i64)
                .chain(disabled_channels.iter().map(|id| *id))
                .collect()
        } else {
            channel_ids.into_iter().map(|id| id as i64).collect()
        };

    new_disabled_channels.sort();
    new_disabled_channels.dedup();

    guild_conf.disabled_channels.replace(new_disabled_channels);
    guild_conf.save(ctx).await?;

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Disabled Channels");
                e.color(0xe67e22);

                e.description(channel_ids_str);

                e
            })
        })
        .await?;

    Ok(())
}

#[command]
#[required_permissions("MANAGE_GUILD")]
#[aliases("enablechannels")]
async fn enablechannel(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut guild_conf = GuildConfig::from_msg_or_respond(&ctx, msg).await?;
    let guild_channels: HashSet<u64> = match msg
        .guild_field(ctx, |g| {
            g.channels
                .keys()
                .map(|id| id.0)
                .collect::<HashSet<u64>>()
                .clone()
        })
        .await
    {
        Some(c) => c,
        None => {
            msg.channel_id
                .say(&ctx.http, "Error: Failed to get guild channels")
                .await?;

            return Ok(());
        }
    };

    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?:<#|[^#&\d]|^)(\d{17,19})>?").unwrap();
    }

    let channels_str = args.rest();
    let channel_ids: Vec<u64> = RE
        .captures_iter(channels_str)
        .filter_map(|caps| caps.get(1).and_then(|m| m.as_str().parse::<u64>().ok()))
        .filter(|id| guild_channels.contains(id))
        .collect();

    if channel_ids.is_empty() {
        msg.channel_id
            .say(&ctx.http, "Error: Please give channels to disable")
            .await?;

        return Ok(());
    }

    let channel_ids_str = channel_ids
        .iter()
        .map(|id| format!("<#{}>", id))
        .collect::<Vec<_>>()
        .join("\n");

    if let Some(ref disabled_channels) = guild_conf.disabled_channels {
        let mut set: HashSet<i64> = disabled_channels.iter().cloned().collect();

        for id in channel_ids {
            set.remove(&(id as i64));
        }

        guild_conf
            .disabled_channels
            .replace(set.into_iter().collect::<Vec<i64>>());
    }

    guild_conf.save(ctx).await?;

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Enabled Channels");
                e.color(0x2ecc71);

                e.description(channel_ids_str);

                e
            })
        })
        .await?;

    Ok(())
}
