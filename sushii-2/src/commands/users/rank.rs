use num_traits::cast::ToPrimitive;
use serde_json::json;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::AttachmentType;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::borrow::Cow;

use crate::keys::*;
use crate::model::sql::*;
use crate::utils::user::parse_id;
use sushii_model::model::user::UserLevelProgress;

#[command]
#[only_in("guild")]
#[aliases("profile", "rakn", "rnak", "arnk")]
#[bucket("rank")]
async fn rank(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return Ok(()),
    };

    // Get target, or self
    let target_user = match args.single::<String>() {
        Ok(id_str) => {
            let user_id = match parse_id(id_str) {
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

    let level_prog = UserLevelProgress::from_xp(user_level.msg_all_time.0);
    let level_prog_global = UserLevelProgress::from_xp(user_level_global);

    // Get user level or create a new one
    let user_data = if let Some(data) = UserData::from_id(&ctx, target_user.id).await? {
        data
    } else {
        UserData::new(target_user.id).save(&ctx).await?
    };

    // Send typing
    let typing = msg.channel_id.start_typing(&ctx.http)?;

    let reqwest_client = ctx
        .data
        .read()
        .await
        .get::<ReqwestContainer>()
        .cloned()
        .unwrap();

    let sushii_conf = SushiiConfig::get(&ctx).await;

    let (rank_day, rank_day_total) = user_level.get_rank_day();
    let (rank_week, rank_week_total) = user_level.get_rank_week();
    let (rank_month, rank_month_total) = user_level.get_rank_month();
    let (rank_all, rank_all_total) = user_level.get_rank_all_time();

    let res = reqwest_client
        .post(&format!("{}/template", sushii_conf.image_server_url))
        .json(&json!({
            "templateName": "rank",
            "width": 500,
            "height": 400,
            "context": {
                "BASE_URL": sushii_conf.image_server_url,
                "CONTENT_COLOR": "0, 184, 148",
                "CONTENT_OPACITY": "1",
                // Username stuff
                "USERNAME": target_user.name,
                "DISCRIMINATOR": format!("{:0>4}", target_user.discriminator),
                "AVATAR_URL": target_user.face().replace(".gif", ".jpg"),
                // Rep and fishies
                "REP": user_data.rep,
                "REP_LEVEL": user_data.rep_level(),
                "FISHIES": user_data.fishies,
                // Emojis
                "IS_PATRON": user_data.is_patron,
                "PATRON_EMOJI_URL": user_data.profile_data
                    .and_then(|d| d.0.patron_emoji_url)
                    .unwrap_or_else(|| "https://cdn.discordapp.com/emojis/830976556976963644.png".into()),
                // levels
                "LEVEL": level_prog.level,
                "CURR_XP": level_prog.next_level_xp_progress,
                "REQ_XP": level_prog.next_level_xp_required,
                "XP_PROGRESS": level_prog.next_level_xp_percentage,
                // global
                "GLOBAL_LEVEL": level_prog_global.level,
                "GLOBAL_CURR_XP": level_prog_global.next_level_xp_progress,
                "GLOBAL_REQ_XP": level_prog_global.next_level_xp_required,
                "GLOBAL_XP_PROGRESS": level_prog_global.next_level_xp_percentage,
                // ranks
                "RANK_ALL": rank_all,
                "RANK_ALL_TOTAL": rank_all_total,
                "RANK_WEEK": rank_week,
                "RANK_WEEK_TOTAL": rank_week_total,
                "RANK_MONTH": rank_month,
                "RANK_MONTH_TOTAL": rank_month_total,
                "RANK_DAY": rank_day,
                "RANK_DAY_TOTAL": rank_day_total
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
                            format!(
                                "{} (Global: {})",
                                level_prog.level.0, level_prog_global.level.0
                            ),
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

    typing.stop();

    Ok(())
}
