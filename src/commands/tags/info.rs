use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::model::sql::*;

#[command]
async fn info(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
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

    let d = format!(
        "**Content:** {}\n**Use Count:** {}\n**Owner:** <@{}>",
        tag.content, tag.use_count, tag.owner_id
    );

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(tag_name);
                e.color(0xe67e22);

                e.description(d);

                e
            })
        })
        .await?;

    Ok(())
}
