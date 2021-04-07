use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::model::sql::*;
use crate::utils::user::parse_id;

#[command]
#[aliases("fwishy", "foshy")]
#[only_in("guild")]
async fn fishy(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // Check cooldown before checking args
    let mut author_user_data = UserData::from_id_or_new(&ctx, msg.author.id).await?;

    if let Some(duration_str) = author_user_data.fishies_humantime_cooldown() {
        msg.channel_id
            .say(&ctx, format!("You can fishy again in {}", duration_str))
            .await?;

        return Ok(());
    }

    let target_str = match args.single::<String>() {
        Ok(s) => s,
        Err(_) => {
            msg.channel_id
                .say(&ctx, "Error: Give me someone to fish for! Fishy for someone else to catch more fishies, or fish for yourself by passing `self` as an argument.")
                .await?;

            return Ok(());
        }
    };

    let target_id = match parse_id(&target_str) {
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

    // Total fishies is before adding more fishies
    let total_fishies = target_user_data
        .as_ref()
        .map(|d| d.fishies)
        .unwrap_or(author_user_data.fishies);

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
        format!(" for {}", target_user.name)
    };

    let s = if is_golden {
        format!(
            "You caught a **golden fishy**{}!!! <:goldenFishy:418676324157227008> ({} fishies)",
            name_str, fishies
        )
    } else {
        format!("You caught **{} fishies**{}!", fishies, name_str)
    };

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
                    "{} {} → {} fishies",
                    s,
                    total_fishies,
                    total_fishies + fishies,
                ));

                e
            })
        })
        .await?;

    Ok(())
}
