use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::model::sql::*;

#[command]
#[sub_commands(set, on, off, toggle)]
async fn joinmsg(ctx: &Context, msg: &Message) -> CommandResult {
    let _ = msg
        .channel_id
        .say(
            &ctx.http,
            "Available sub-commands for `joinmsg` are `set`, `on`, `off`, and `toggle`",
        )
        .await?;

    Ok(())
}

/// Set the moderation log channel
#[command]
async fn set(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut conf = GuildConfig::from_msg_or_respond(&ctx, msg).await?;

    let join_msg = args.rest();

    if join_msg.is_empty() {
        msg.channel_id
            .say(
                &ctx.http,
                "Error: Give a join message. You can use the placeholders \
                    `<mention>`, `<username>`, `<server>` to get the corresponding values.",
            )
            .await?;

        return Ok(());
    }

    conf.join_msg.replace(join_msg.to_string());
    conf.save(&ctx).await?;

    msg.channel_id
        .say(&ctx.http, format!("Join message to: {}", join_msg))
        .await?;

    Ok(())
}

/// Turns off moderation log
#[command]
async fn off(ctx: &Context, msg: &Message) -> CommandResult {
    let mut conf = GuildConfig::from_msg_or_respond(&ctx, msg).await?;

    if !conf.join_msg_enabled {
        let _ = msg
            .channel_id
            .say(&ctx.http, "Error: Join messages are already off")
            .await?;

        return Ok(());
    }

    conf.join_msg_enabled = false;
    conf.save(&ctx).await?;

    let _ = msg
        .channel_id
        .say(
            &ctx.http,
            "<:offline:316354467031416832> Turned off join messages",
        )
        .await?;

    Ok(())
}

/// Turns on join messages
#[command]
async fn on(ctx: &Context, msg: &Message) -> CommandResult {
    let mut conf = GuildConfig::from_msg_or_respond(&ctx, msg).await?;

    if conf.join_msg_enabled {
        let _ = msg
            .channel_id
            .say(&ctx.http, "Error: Join messages are already on")
            .await?;

        return Ok(());
    }

    conf.join_msg_enabled = true;
    conf.save(&ctx).await?;

    let _ = msg
        .channel_id
        .say(
            &ctx.http,
            "<:online:316354435745972244> Turned on join messages",
        )
        .await?;

    Ok(())
}

/// Toggles join messages
#[command]
async fn toggle(ctx: &Context, msg: &Message) -> CommandResult {
    let mut conf = GuildConfig::from_msg_or_respond(&ctx, msg).await?;

    conf.join_msg_enabled = !conf.join_msg_enabled;
    conf.save(&ctx).await?;

    let on_or_off = if conf.join_msg_enabled {
        ("<:online:316354435745972244>", "on")
    } else {
        ("<:offline:316354467031416832>", "off")
    };

    let _ = msg
        .channel_id
        .say(
            &ctx.http,
            format!("{} Toggled join messages `{}`", on_or_off.0, on_or_off.1),
        )
        .await?;

    Ok(())
}
