use num_traits::cast::ToPrimitive;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::parse_mention;

use crate::model::sql::*;

#[command]
#[only_in("guild")]
async fn rank(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return Ok(()),
    };

    // Get target, or self
    let target_user = match args.single::<String>() {
        Ok(id_str) => {
            let user_id = match id_str.parse::<u64>().ok().or_else(|| parse_mention(id_str)) {
                Some(id) => id,
                None => {
                    msg.channel_id
                        .say(&ctx, "Error: Invalid user ID given")
                        .await?;

                    return Ok(());
                }
            };

            // Get user
            match UserId(user_id).to_user(&ctx).await {
                Ok(u) => u,
                Err(_) => {
                    msg.channel_id
                        .say(&ctx, "Error: Failed to fetch user")
                        .await?;

                    return Ok(());
                }
            }
        }
        Err(_) => msg.author.clone(), // need ownership 
    };

    let user_level = match UserLevelRanked::from_id(&ctx, target_user.id, guild_id).await? {
        Some(level) => level,
        None => {
            msg.channel_id
                .say(&ctx, "Error: No level data found for user")
                .await?;

            return Ok(());
        }
    };

    let user_level_global = UserLevelGlobal::from_id(&ctx, target_user.id)
        .await?
        .and_then(|lvl| lvl.xp)
        .and_then(|xp| xp.to_i64())
        .unwrap_or(0);

    msg.channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.title(format!("Rank for {}", target_user.tag()));
                e.color(0xe67e22);

                e.field("Daily", user_level.fmt_rank_day(), true);
                e.field("Weekly", user_level.fmt_rank_week(), true);
                e.field("Monthly", user_level.fmt_rank_month(), true);
                e.field("All Time", user_level.fmt_rank_all_time(), true);

                e
            })
        })
        .await?;

    Ok(())
}
