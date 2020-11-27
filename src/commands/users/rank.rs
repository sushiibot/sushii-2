use num_traits::cast::ToPrimitive;
use serde_json::json;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::http::AttachmentType;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::parse_mention;
use std::borrow::Cow;

use crate::keys::*;
use crate::model::sql::*;
use crate::model::user::*;

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

    let level_prog = UserLevelProgress::from_xp(user_level.msg_all_time);
    let level_prog_global = UserLevelProgress::from_xp(user_level_global);

    let reqwest_client = ctx
        .data
        .read()
        .await
        .get::<ReqwestContainer>()
        .cloned()
        .unwrap();

    let sushii_conf = SushiiConfig::get(&ctx).await;

    let res = reqwest_client
        .post(&format!("{}/template", sushii_conf.image_server_url))
        .json(&json!({
            "name": "rank",
            "width": 500,
            "height": 400,
            "ctx": {
                "CONTENT_COLOR": "0, 184, 148",
                "CONTENT_OPACITY": "1",
                "AVATAR_URL": target_user.face(),
                "REP": "123",
                "FISHIES": "456",
                "USERNAME": target_user.tag(),
                "PATRON_EMOJI": "",
                "XP_PROGRESS": level_prog.next_level_xp_percentage,
                "LEVEL": level_prog.level,
                "GLOBAL_LEVEL": level_prog_global.level,
                "CURR_LEVEL_XP": level_prog.next_level_xp_progress,
                "LEVEL_XP_REQ": level_prog.next_level_xp_required,
                "DAILY": user_level.fmt_rank_day(),
                "WEEKLY": user_level.fmt_rank_week(),
                "MONTHLY": user_level.fmt_rank_month(),
                "ALL": user_level.fmt_rank_all_time()
            }
        }))
        .send()
        .await
        .and_then(|r| r.error_for_status());

    let bytes = match res {
        Ok(r) => r.bytes().await?,
        Err(e) => {
            tracing::warn!("Image server responded with error: {}", e);

            msg.channel_id
                .send_message(&ctx, |m| {
                    m.embed(|e| {
                        e.title(format!("Rank for {}", target_user.tag()));
                        e.color(0xe67e22);

                        e.field(
                            "Level",
                            format!("{} (Global: {})", level_prog.level, level_prog_global.level),
                            false,
                        );

                        e.field("Daily", user_level.fmt_rank_day(), true);
                        e.field("Weekly", user_level.fmt_rank_week(), true);
                        e.field("Monthly", user_level.fmt_rank_month(), true);
                        e.field("All Time", user_level.fmt_rank_all_time(), true);

                        e
                    })
                })
                .await?;

            return Ok(());
        }
    };

    let files = AttachmentType::Bytes {
        data: Cow::from(bytes.as_ref()),
        filename: "level.png".into(),
    };

    msg.channel_id.send_files(&ctx, vec![files], |m| m).await?;

    Ok(())
}
