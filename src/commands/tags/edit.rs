use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::model::sql::*;

#[command]
async fn edit(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
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

    let tag_content = args.rest();

    if tag_content.is_empty() {
        msg.channel_id
            .say(&ctx, "Error: Please give tag content")
            .await?;
        return Ok(());
    }

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

    let member = msg.member(&ctx).await?;

    if !tag.can_edit(&ctx, &member).await? {
        msg.channel_id
            .say(
                &ctx,
                "Error: You can't edit this tag, it doesn't belong to you",
            )
            .await?;

        return Ok(());
    }

    let tag = tag.edit(tag_content).save(&ctx).await?;

    msg.channel_id
        .say(
            &ctx,
            format!(
                "Edited `{}` with new content: {}",
                tag.tag_name, tag.content
            ),
        )
        .await?;

    Ok(())
}

#[command]
async fn rename(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
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

    let new_tag_name = match args.single::<String>() {
        Ok(n) => n,
        Err(_) => {
            msg.channel_id
                .say(&ctx, "Error: Please give a new tag name")
                .await?;
            return Ok(());
        }
    };

    let mut tag = match Tag::from_name(&ctx, &tag_name, guild_id).await? {
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

    let member = msg.member(&ctx).await?;

    if !tag.can_edit(&ctx, &member).await? {
        msg.channel_id
            .say(
                &ctx,
                "Error: You can't rename this tag, it doesn't belong to you",
            )
            .await?;

        return Ok(());
    }

    if !tag.rename(&ctx, &new_tag_name).await? {
        msg.channel_id
            .say(
                &ctx,
                format!(
                    "Error: The tag `{}` already exists, use a different name.",
                    new_tag_name
                ),
            )
            .await?;

        return Ok(());
    }

    let tag = tag.save(&ctx).await?;

    msg.channel_id
        .say(
            &ctx,
            format!("Renamed `{}` to `{}`", tag_name, tag.tag_name),
        )
        .await?;

    Ok(())
}

#[command]
async fn delete(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
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

    let member = msg.member(&ctx).await?;

    if !tag.can_edit(&ctx, &member).await? {
        msg.channel_id
            .say(
                &ctx,
                "Error: You can't rename this tag, it doesn't belong to you",
            )
            .await?;

        return Ok(());
    }

    tag.delete(&ctx).await?;

    msg.channel_id
        .say(
            &ctx,
            format!("Deleted tag `{}`", tag_name),
        )
        .await?;

    Ok(())
}
