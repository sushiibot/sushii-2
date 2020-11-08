use chrono::Duration;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::parse_mention;
use std::fmt::Write;

use crate::model::moderation::{ModActionExecutor, ModActionExecutorDb, ModActionType};
use crate::model::sql::*;
use crate::utils::duration::parse_duration;
use crate::utils::user::get_user;

#[command]
#[only_in("guild")]
#[sub_commands(setduration, addduration)]
async fn mute(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            msg.channel_id.say(&ctx.http, "No guild found").await?;

            return Ok(());
        }
    };

    let conf = GuildConfig::from_msg_or_respond(&ctx, msg).await?;
    if conf.mute_role.is_none() {
        let _ = msg
            .channel_id
            .say(&ctx.http, "There is no mute command set");

        return Ok(());
    }

    ModActionExecutor::from_args(args, ModActionType::Mute)
        .execute(&ctx, &msg, &guild_id)
        .await?;

    Ok(())
}

#[command]
#[only_in("guild")]
async fn unmute(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            msg.channel_id.say(&ctx.http, "No guild found").await?;

            return Ok(());
        }
    };

    let conf = GuildConfig::from_msg_or_respond(&ctx, msg).await?;
    if conf.mute_role.is_none() {
        let _ = msg
            .channel_id
            .say(&ctx.http, "There is no mute command set");

        return Ok(());
    }

    ModActionExecutor::from_args(args, ModActionType::Unmute)
        .execute(&ctx, &msg, &guild_id)
        .await?;

    Ok(())
}

enum DurationModifyAction {
    Set,
    Add,
}

enum DurationOption {
    Duration(Duration),
    Indefinite,
}

async fn modify_duration(
    action: DurationModifyAction,
    ctx: &Context,
    msg: &Message,
    mut args: Args,
) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            msg.channel_id.say(&ctx.http, "No guild ID found").await?;

            return Ok(());
        }
    };

    let user_id_str = match args.single::<String>() {
        Ok(s) => s,
        Err(_) => {
            msg.channel_id
                .say(&ctx.http, "Please give a user ID")
                .await?;

            return Ok(());
        }
    };

    let user_id = match user_id_str
        .parse::<u64>()
        .ok()
        .or_else(|| parse_mention(user_id_str))
    {
        Some(id) => id,
        None => {
            msg.channel_id
                .say(&ctx.http, "Invalid user ID given")
                .await?;

            return Ok(());
        }
    };

    // or is indefinite
    let duration_str = args.rest();

    if duration_str.is_empty() {
        msg.channel_id
            .say(&ctx.http, "Error: Give a duration or `indefinite`")
            .await?;

        return Ok(());
    }

    let duration_opt = match duration_str.to_lowercase().as_ref() {
        "indefinite" | "indef" | "inf" | "none" => DurationOption::Indefinite,
        _ => match parse_duration(duration_str) {
            Ok(d) => DurationOption::Duration(d),
            Err(e) => {
                msg.channel_id
                        .say(
                            &ctx.http,
                            format!("Error: Failed to parse duration, give a duration or `indefinite` -- {}", e),
                        )
                        .await?;

                return Ok(());
            }
        },
    };

    let mut mute = match Mute::from_id(&ctx, guild_id.0, user_id).await? {
        Some(m) => m,
        None => {
            msg.channel_id
                .say(
                    &ctx,
                    "Error: This member is not muted, or a mute entry was not found",
                )
                .await?;

            return Ok(());
        }
    };

    let user = match get_user(&ctx, mute.user_id as u64).await {
        Some(u) => u,
        None => {
            msg.channel_id
                .say(&ctx, "Error: Failed to fetch user")
                .await?;

            return Ok(());
        }
    };

    let old_duration = mute
        .get_human_duration()
        .unwrap_or_else(|| "Indefinite".into());

    let s = match action {
        DurationModifyAction::Add => {
            match duration_opt {
                DurationOption::Duration(duration) => {
                    // End time = end + new duration
                    // or if indefinite / no end time
                    // End time = start + new duration
                    mute.end_time = mute
                        .end_time
                        .and_then(|t| t.checked_add_signed(duration))
                        .or_else(|| mute.start_time.checked_add_signed(duration));

                    format!(
                        "Mute duration for {} extended by `{}`, new duration is now `{}` (old duration: `{}`)",
                        user.tag(),
                        humantime::format_duration(duration.to_std().unwrap()),
                        mute.get_std_duration()
                            .map(|d| humantime::format_duration(d).to_string())
                            .unwrap_or_else(|| "N/A".to_string()),
                        old_duration
                    )
                }
                DurationOption::Indefinite => {
                    mute.end_time = None;

                    format!(
                        "Mute for {} is now indefinite (old duration: `{}`)",
                        user.tag(),
                        old_duration
                    )
                }
            }
        }
        DurationModifyAction::Set => {
            match duration_opt {
                DurationOption::Duration(duration) => {
                    // End time = start + new duration
                    mute.end_time = mute.start_time.checked_add_signed(duration);

                    format!(
                        "Mute duration for {} set to `{}` (old duration: `{}`)",
                        user.tag(),
                        mute.get_std_duration()
                            .map(|d| humantime::format_duration(d).to_string())
                            .unwrap_or_else(|| "N/A".to_string()),
                        old_duration
                    )
                }
                DurationOption::Indefinite => {
                    mute.end_time = None;

                    format!(
                        "Mute for {} is now indefinite (old duration: `{}`)",
                        user.tag(),
                        old_duration
                    )
                }
            }
        }
    };

    mute.save(&ctx).await?;

    msg.channel_id.say(&ctx, s).await?;

    Ok(())
}

#[command]
#[only_in("guild")]
#[aliases("s", "set", "setd", "setdur", "settime")]
async fn setduration(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    modify_duration(DurationModifyAction::Set, ctx, msg, args).await
}

#[command]
#[only_in("guild")]
#[aliases("a", "add", "addd", "adddur", "addtime", "extend")]
async fn addduration(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    modify_duration(DurationModifyAction::Add, ctx, msg, args).await
}

#[command]
#[only_in("guild")]
async fn listmutes(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = match msg.guild(&ctx.cache).await {
        Some(g) => g,
        None => {
            msg.channel_id.say(&ctx.http, "No guild found").await?;

            return Ok(());
        }
    };

    let guild_conf = match GuildConfig::from_id(&ctx, &guild.id).await? {
        Some(c) => c,
        None => {
            tracing::error!(?msg, "No guild config found while listing mutes");
            return Ok(());
        }
    };

    let mute_role = match guild_conf.mute_role {
        Some(role) => RoleId(role as u64),
        None => {
            msg.channel_id
                .say(&ctx, "Error: There isn't a mute role set")
                .await?;

            return Ok(());
        }
    };

    let ongoing_mutes: Vec<Mute> = Mute::get_ongoing(&ctx, guild.id.0).await?;

    let mut definite_mutes: Vec<&Mute> = ongoing_mutes
        .iter()
        .filter(|x| x.end_time.is_some())
        .collect();

    // Sort based on mute duration
    definite_mutes
        .sort_by_cached_key(|m| m.end_time.map(|t| t.signed_duration_since(m.start_time)));

    let indefinite_mutes: Vec<&Mute> = ongoing_mutes
        .iter()
        .filter(|x| x.end_time.is_none())
        .collect();

    // Merge definite and indefinites
    let ongoing_mutes_sorted: Vec<&Mute> = definite_mutes
        .into_iter()
        .chain(indefinite_mutes.into_iter())
        .collect();

    let mut s = String::new();

    for mute in ongoing_mutes_sorted {
        let user = match UserId(mute.user_id as u64).to_user(&ctx).await {
            Ok(u) => u,
            Err(e) => {
                tracing::error!(?mute, "Failed to get user: {}", e);

                continue;
            }
        };

        if let Ok(member) = guild.member(&ctx, mute.user_id as u64).await {
            // Double check if member has a mute role or not
            if !member.roles.contains(&mute_role) {
                // Delete this mute entry if the member is missing a mute role
                if let Err(e) = mute.delete(&ctx).await {
                    tracing::error!(?mute, "Failed to delete mute: {}", e);
                }

                continue;
            }
        }

        let _ = write!(
            s,
            "`{}` - `{}`",
            mute.start_time.format("%Y-%m-%d %H:%M:%S"),
            // Need to do to_string() since indefinite is a string too
            mute.end_time.map_or_else(
                || "indefinite".into(),
                |m| m.format("%Y-%m-%d %H:%M:%S").to_string()
            )
        );

        if let Some(d) = mute.get_human_duration() {
            let _ = write!(s, " (`{}` total", d);
        }

        if let Some(d) = mute.get_human_duration_remaining() {
            let _ = write!(s, ", `{}` remaining)", d);
        }

        let _ = writeln!(s, ": {} {} (ID: {})", user.mention(), user.tag(), user.id.0);
    }

    if s.is_empty() {
        let _ = writeln!(s, "There are no ongoing mutes");
    }

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Ongoing Mutes");
                e.description(&s);

                e.color(0xe67e22);

                e
            })
        })
        .await?;

    Ok(())
}
