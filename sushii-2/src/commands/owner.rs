use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::parse_channel;
use std::fmt::Write;

use crate::keys::ShardManagerContainer;

#[command]
#[owners_only]
async fn quit(ctx: &Context, msg: &Message) -> CommandResult {
    let manager = ctx
        .data
        .read()
        .await
        .get::<ShardManagerContainer>()
        .cloned()
        .unwrap();
    msg.channel_id.say(ctx, "bye").await?;

    manager.lock().await.shutdown_all().await;

    Ok(())
}

#[command]
#[owners_only]
async fn say(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let channel_str = args.single::<String>()?;

    let channel_id = match channel_str
        .parse::<u64>()
        .ok()
        .or_else(|| parse_channel(channel_str))
    {
        Some(id) => id,
        None => {
            msg.reply(ctx, "Invalid channel").await?;

            return Ok(());
        }
    };

    let s = args.rest().trim();

    if s.is_empty() {
        msg.reply(ctx, "Empty message").await?;

        return Ok(());
    }

    ChannelId(channel_id).say(ctx, s).await?;

    Ok(())
}

#[command]
#[owners_only]
async fn listservers(ctx: &Context, msg: &Message) -> CommandResult {
    let mut s = format!("{} total guilds cached\n", ctx.cache.guild_count());

    for guild_id in ctx.cache.guilds() {
        let guild_name = ctx
            .cache
            .guild_field(guild_id, |g| g.name.clone())
            .unwrap_or_else(|| "Unknown guild".into());

        writeln!(s, "`{}` - {}", guild_id.0, guild_name)?;
    }

    msg.reply(ctx, s).await?;

    Ok(())
}

/*
#[command]
#[owners_only]
async fn query(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let query_str = args.rest();

    if query_str.is_empty() {
        msg.channel_id
            .say(&ctx.http, "Error: No query given")
            .await?;
    }

    let data = ctx.data.read().await;
    let pool = data.get::<DbPool>().unwrap();

    // How to return a generic value like serde_json::Value? Doesn't seem like sqlx supports yet
    // https://github.com/launchbadge/sqlx/issues/182
    let res = sqlx::query(query_str).fetch_all(pool).await;

    match res {
        Ok(rows) => {
            msg.channel_id
                .say(&ctx.http, format!("```{:#?}```", rows))
                .await?;
        }
        Err(e) => {
            msg.channel_id
                .say(&ctx.http, format!("Error: `{}`", e))
                .await?;
        }
    }

    Ok(())
}
*/
