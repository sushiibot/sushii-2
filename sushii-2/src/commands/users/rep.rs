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
                .say(&ctx, "Error: Invalid user given")
                .await?;

            return Ok(());
        }
    };

    if target_id == msg.author.id.0 {
        msg.reply(&ctx, "Error: You can't rep yourself sorry :(")
            .await?;

        return Ok(());
    }

    let target_user = target_id.to_user(&ctx).await?;

    if target_user.bot {
        msg.channel_id
            .say(&ctx, "Error: You can't give bots rep :(")
            .await?;

        return Ok(());
    }

    let target_user_data = UserData::from_id_or_new(&ctx, target_id).await?;
    target_user_data.inc_rep().save(&ctx).await?;

    author_user_data.reset_last_rep().save(&ctx).await?;

    msg.channel_id
        .say(&ctx, format!("You gave {} a rep!", target_user.tag()))
        .await?;

    Ok(())
}
