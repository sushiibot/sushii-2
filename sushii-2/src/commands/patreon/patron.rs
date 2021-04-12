use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use sqlx::types::Json;

use crate::model::sql::*;
use crate::utils::user::parse_id;

#[command]
#[owners_only]
async fn patron(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let target_id = match args.single::<String>().ok().and_then(parse_id) {
        Some(id) => UserId(id),
        None => {
            msg.channel_id.say(&ctx, "Give me a user").await?;

            return Ok(());
        }
    };

    let target_user = match target_id.to_user(&ctx).await {
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

    let action = match args.single::<String>() {
        Ok(s) => s,
        Err(_) => {
            msg.reply(&ctx, "Error: Give an action, `add` or `remove`")
                .await?;

            return Ok(());
        }
    };

    let cents = match args.single::<i64>() {
        Ok(c) => c,
        Err(_) => {
            msg.reply(&ctx, "Error: Give patron cents").await?;

            return Ok(());
        }
    };

    let emoji_url = args.single::<String>().ok();
    let mut user_data = match UserData::from_id(&ctx, target_user.id).await? {
        Some(u) => u,
        None => {
            msg.reply(&ctx, "Error: No user data found").await?;

            return Ok(());
        }
    };

    if action == "add" {
        user_data.is_patron = true;

        if let Some(ref mut data) = user_data.profile_data {
            data.0.patron_cents.replace(cents);
            data.0.patron_emoji_url = emoji_url.clone();
        } else {
            user_data.profile_data = Some(Json(UserProfileData {
                patron_cents: Some(cents),
                patron_emoji_url: emoji_url.clone(),
            }));
        }
    } else if action == "remove" {
        user_data.is_patron = false;

        if let Some(ref mut data) = user_data.profile_data {
            data.0.patron_cents = None;
            data.0.patron_emoji_url = None;
        } else {
            user_data.profile_data = Some(Json(UserProfileData {
                patron_cents: None,
                patron_emoji_url: None,
            }));
        }
    }

    user_data.save(ctx).await?;

    msg.channel_id
        .say(
            &ctx,
            format!(
                "{}ed {} as patron with {} cents and emoji url {}",
                action,
                target_user.tag(),
                cents,
                emoji_url.unwrap_or_else(|| "N/A".into())
            ),
        )
        .await?;

    Ok(())
}
