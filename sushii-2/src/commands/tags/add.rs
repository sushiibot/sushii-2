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

    let attachments_str = msg
        .attachments
        .iter()
        .map(|a| a.url.as_str())
        .collect::<Vec<&str>>()
        .join("\n");

    if tag_content.is_empty() && attachments_str.is_empty() {
        msg.channel_id
            .say(&ctx, "Error: Please give tag content or attachment(s)")
            .await?;
        return Ok(());
    }

    let content = if !attachments_str.is_empty() && tag_content.is_empty() {
        attachments_str
    } else if !attachments_str.is_empty() && !tag_content.is_empty() {
        format!("{}\n{}", tag_content, attachments_str)
    } else {
        tag_content.to_string()
    };

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

    let tag = Tag::new(msg.author.id, guild_id, &tag_name, content)
        .save(&ctx)
        .await?;

    msg.channel_id
        .send_message(&ctx, |m| {
            m.content(format!(
                "Created a new tag `{}` with content: {}",
                tag.tag_name, tag.content
            ));

            m.allowed_mentions(|am| {
                am.empty_parse();
                am
            });

            m
        })
        .await?;

    Ok(())
}
