use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::model::sql::*;
use crate::utils::user::parse_id;

#[command]
async fn transfer(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
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

    let new_owner_id = match args.single::<String>().ok().and_then(|s| parse_id(s)) {
        Some(id) => UserId(id),
        None => {
            msg.channel_id
                .say(ctx, "Error: Invalid user given.")
                .await?;

            return Ok(());
        }
    };

    let new_owner_user = match new_owner_id.to_user(&ctx).await {
        Ok(u) => u,
        Err(_) => {
            msg.reply(
                &ctx,
                "Error: Failed to fetch user, are you using a correct user ID? (Could be a message ID)",
            )
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
                "Error: You can't edit this tag, it doesn't belong to you",
            )
            .await?;

        return Ok(());
    }

    let tag = tag.transfer(new_owner_id).save(&ctx).await?;

    msg.channel_id
        .say(
            &ctx,
            format!("Transfered `{}` to {}", tag.tag_name, new_owner_user.tag()),
        )
        .await?;

    Ok(())
}
