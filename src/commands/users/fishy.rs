use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::parse_mention;

use crate::model::sql::*;

#[command]
#[only_in("guild")]
async fn fishy(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let target_str = args.rest();

    let target_id = match target_str
        .parse::<u64>()
        .ok()
        .or_else(|| parse_mention(target_str))
    {
        Some(id) => UserId(id),
        None => {
            if target_str != "self" {
                msg.channel_id
                .say(&ctx, "Error: Invalid user given. Fishy for someone else to catch more fishies, or fish for yourself by passing `self` as an argument.")
                .await?;

                return Ok(());
            }

            msg.author.id
        }
    };

    let is_self = target_id == msg.author.id;

    let author_user_data = UserData::from_id_or_new(&ctx, msg.author.id).await?;

    if let Some(duration_str) = author_user_data.fishies_humantime_cooldown() {
        msg.channel_id
            .say(&ctx, format!("You can fishy again in {}", duration_str))
            .await?;

        return Ok(());
    }

    let target_user_data = if !is_self {
        Some(UserData::from_id_or_new(&ctx, target_id).await?)
    } else {
        None
    };

    match target_user_data {
        Some(target) => {
            // Someone else
            target.inc_fishies(is_self).save(&ctx).await?;
            // So we need to save author separately
            author_user_data.reset_last_rep().save(&ctx).await?;
        }
        None => {
            author_user_data
                .inc_fishies(is_self)
                .reset_last_fishy()
                .save(&ctx)
                .await?;
        }
    }

    Ok(())
}
