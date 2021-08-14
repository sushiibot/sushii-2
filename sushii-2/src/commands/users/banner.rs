use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::utils::user::parse_id;

#[command]
async fn banner(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let target_str = args.rest();

    let target_id = match parse_id(target_str) {
        Some(id) => UserId(id),
        None => {
            if !target_str.is_empty() {
                msg.channel_id
                    .say(ctx, "Error: Invalid user given.")
                    .await?;

                return Ok(());
            }

            // If empty use self
            msg.author.id
        }
    };

    let user = match target_id.to_user(&ctx.http).await {
        Ok(u) => u,
        Err(_) => {
            msg.reply(
                &ctx,
                "Error: Failed to fetch user, are you using a correct user ID?",
            )
            .await?;

            return Ok(());
        }
    };

    let banner_url = match user.banner_url() {
        Some(url) => url,
        None => {
            msg.channel_id.say(&ctx, "User has no banner set.").await?;

            return Ok(());
        }
    };

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title(format!("{} banner", user.tag()));
                e.url(&banner_url);

                e.image(&banner_url);

                if let Some(accent_color) = user.accent_color {
                    e.footer(|f| f.text(format!("Accent color #{}", accent_color.hex())));
                }

                e
            });

            m
        })
        .await?;

    Ok(())
}
