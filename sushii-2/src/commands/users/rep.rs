use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::model::sql::*;
use crate::utils::user::parse_id;

#[command]
#[only_in("guild")]
async fn rep(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let author_user_data = UserData::from_id_or_new(&ctx, msg.author.id).await?;

    if let Some(duration_str) = author_user_data.rep_humantime_cooldown() {
        msg.channel_id
            .say(&ctx, format!("You can rep again in {}", duration_str))
            .await?;

        return Ok(());
    }

    let target_id = match args.single::<String>().ok().and_then(parse_id) {
        Some(id) => UserId(id),
        None => {
            msg.channel_id
                .say(&ctx, "Give me someone to rep!")
                .await?;

            return Ok(());
        }
    };

    if target_id == msg.author.id.0 {
        msg.reply(&ctx, "Error: You can't rep yourself sorry :(")
            .await?;

        return Ok(());
    }

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

    if target_user.bot {
        msg.channel_id
            .say(&ctx, "Error: You can't give bots rep :(")
            .await?;

        return Ok(());
    }

    let target_user_data = UserData::from_id_or_new(&ctx, target_id).await?;
    let target_user_data = target_user_data.inc_rep().save(&ctx).await?;

    author_user_data.reset_last_rep().save(&ctx).await?;

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.author(|a| {
                    a.name(&target_user.name);
                    a.icon_url(target_user.face());

                    a
                });

                e.colour(0x2F3136);
                e.description(format!(
                    "You gave {} a rep! {} → {} rep",
                    target_user.name,
                    target_user_data.rep - 1,
                    target_user_data.rep
                ));

                e
            })
        })
        .await?;

    Ok(())
}
