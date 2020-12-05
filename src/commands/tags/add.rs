use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::model::sql::*;

#[command]
async fn add(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            msg.channel_id.say(&ctx, "Error: Not in guild").await?;
            return Ok(());
        }
    };

    // TODO: Disallow tag subcommands as tag names
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

    if Tag::from_name(&ctx, &tag_name, guild_id).await?.is_some() {
        msg.channel_id
            .say(
                &ctx,
                format!(
                    "Error: The tag `{}` already exists, use a different name.",
                    tag_name
                ),
            )
            .await?;
        return Ok(());
    }

    let tag = Tag::new(msg.author.id, guild_id, &tag_name, tag_content)
        .save(&ctx)
        .await?;

    msg.channel_id
        .say(
            &ctx,
            format!(
                "Created a new tag `{}` with content: {}",
                tag.tag_name, tag.content
            ),
        )
        .await?;

    Ok(())
}
