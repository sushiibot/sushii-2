use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::model::sql::*;

#[command]
async fn get(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            msg.channel_id.say(&ctx, "Error: Not in guild").await?;
            return Ok(());
        }
    };

    let tag_name = match args.single::<String>() {
        Ok(n) => n,
        Err(_) => {
            msg.channel_id
                .say(&ctx, "Error: Please give a tag name")
                .await?;
            return Ok(());
        }
    };

    let tag = match Tag::from_name(&ctx, &tag_name, guild_id).await? {
        Some(t) => t,
        None => {
            msg.channel_id
                .say(
                    &ctx,
                    format!("Error: Couldn't find a tag named `{}`", tag_name),
                )
                .await?;

            return Ok(());
        }
    };

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.content(&tag.content);
            m.allowed_mentions(|am| {
                am.empty_parse();
                am
            });

            m
        })
        .await?;

    tag.inc().save(&ctx).await?;

    Ok(())
}

#[command]
async fn random(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            msg.channel_id.say(&ctx, "Error: Not in guild").await?;
            return Ok(());
        }
    };

    let tag = match Tag::random(&ctx, guild_id).await? {
        Some(t) => t,
        None => {
            msg.channel_id
                .say(&ctx, "Error: Couldn't find any tags")
                .await?;

            return Ok(());
        }
    };

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.content(format!("{}: {}", tag.tag_name, tag.content));
            m.allowed_mentions(|am| {
                am.empty_parse();
                am
            });

            m
        })
        .await?;

    Ok(())
}
