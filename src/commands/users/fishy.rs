use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::parse_mention;

use crate::model::sql::*;

#[command]
#[aliases("fwishy")]
#[only_in("guild")]
async fn fishy(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // Check cooldown before checking args
    let mut author_user_data = UserData::from_id_or_new(&ctx, msg.author.id).await?;

    if let Some(duration_str) = author_user_data.fishies_humantime_cooldown() {
        msg.channel_id
            .say(&ctx, format!("You can fishy again in {}", duration_str))
            .await?;

        return Ok(());
    }

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

    let target_user = target_id.to_user(&ctx).await?;

    if target_user.bot {
        msg.channel_id
            .say(&ctx, "Error: Bots don't need fishies :(")
            .await?;

        return Ok(());
    }

    let is_self = target_id == msg.author.id;

    let target_user_data = if !is_self {
        Some(UserData::from_id_or_new(&ctx, target_id).await?)
    } else {
        None
    };

    let (fishies, is_golden) = match target_user_data {
        Some(mut target) => {
            // Someone else
            let fishies_tup = target.inc_fishies(is_self);
            target.save(&ctx).await?;
            // So we need to save author separately
            author_user_data.reset_last_fishy().save(&ctx).await?;

            fishies_tup
        }
        None => {
            let fishies_tup = author_user_data.inc_fishies(is_self);

            author_user_data.reset_last_fishy().save(&ctx).await?;

            fishies_tup
        }
    };

    let name_str = if is_self {
        "".to_string()
    } else {
        format!(" for {}", target_user.tag())
    };

    let s = if is_golden {
        format!(
            "You caught a golden fishy{}!!! ({} fishies)",
            name_str, fishies
        )
    } else {
        format!("You caught {} fishies{}!", fishies, name_str)
    };

    msg.channel_id.say(&ctx, s).await?;

    Ok(())
}
