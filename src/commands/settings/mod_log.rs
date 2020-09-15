use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::parse_channel;

use crate::model::sql::*;

/// Moderation log settings
#[command]
#[sub_commands(set, on, off, toggle)]
async fn modlog(ctx: &Context, msg: &Message) -> CommandResult {
    let _ = msg
        .channel_id
        .say(
            &ctx.http,
            "Available sub-commands for `modlog` are `set`, `on`, `off`, and `toggle`",
        )
        .await?;

    Ok(())
}

/// Set the moderation log channel
#[command]
#[num_args(1)]
async fn set(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut conf = GuildConfig::from_msg_or_respond(&ctx, msg).await?;

    let channel_id = match args.single::<String>().ok().and_then(parse_channel) {
        Some(id) => id,
        None => {
            msg.channel_id
                .say(
                    &ctx.http,
                    "Invalid channel, please provide a guild #channel",
                )
                .await?;

            return Ok(());
        }
    };

    conf.log_mod.replace(channel_id as i64);
    conf.save(&ctx).await?;

    msg.channel_id
        .say(
            &ctx.http,
            format!("Updated mod log channel to <#{}>", channel_id),
        )
        .await?;

    Ok(())
}

/// Turns off moderation log
#[command]
async fn off(ctx: &Context, msg: &Message) -> CommandResult {
    let mut conf = GuildConfig::from_msg_or_respond(&ctx, msg).await?;

    if !conf.log_mod_enabled {
        let _ = msg
            .channel_id
            .say(&ctx.http, "Moderation logs are already off")
            .await?;

        return Ok(());
    }

    conf.log_mod_enabled = false;
    conf.save(&ctx).await?;

    let _ = msg
        .channel_id
        .say(&ctx.http, "Turned off moderation logs")
        .await?;

    Ok(())
}

/// Turns on moderation log
#[command]
async fn on(ctx: &Context, msg: &Message) -> CommandResult {
    let mut conf = GuildConfig::from_msg_or_respond(&ctx, msg).await?;

    if conf.log_mod_enabled {
        let _ = msg
            .channel_id
            .say(&ctx.http, "Moderation logs are already on")
            .await?;

        return Ok(());
    }

    conf.log_mod_enabled = true;
    conf.save(&ctx).await?;

    let _ = msg
        .channel_id
        .say(&ctx.http, "Turned on moderation logs")
        .await?;

    Ok(())
}

/// Toggles moderation log
#[command]
async fn toggle(ctx: &Context, msg: &Message) -> CommandResult {
    let mut conf = GuildConfig::from_msg_or_respond(&ctx, msg).await?;

    conf.log_mod_enabled = !conf.log_mod_enabled;
    conf.save(&ctx).await?;

    let on_or_off = if conf.log_mod_enabled {
        "on"
    } else {
        "off"
    };

    let _ = msg
        .channel_id
        .say(&ctx.http, format!("Toggled moderation logs `{}`", on_or_off))
        .await?;

    Ok(())
}
