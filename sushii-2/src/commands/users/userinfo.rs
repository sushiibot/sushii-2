use chrono::{DateTime, Duration, Utc};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::fmt::Write;

use crate::utils::user::parse_id;

#[command]
#[aliases("user")]
#[only_in("guild")]
async fn userinfo(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            msg.channel_id.say(&ctx.http, "No guild found").await?;

            return Ok(());
        }
    };

    let target_str = args.rest();

    let target_id = match parse_id(target_str) {
        Some(id) => UserId(id),
        None => {
            if !target_str.is_empty() {
                msg.channel_id
                    .say(ctx, "Error: Invalid user given.")
                    .await?;

                return Ok(());
            }

            // If empty use self
            msg.author.id
        }
    };

    let user = target_id.to_user(ctx).await?;
    let member = guild_id.member(ctx, target_id).await;
    let now = Utc::now();

    let mut user_str = String::new();

    write!(user_str, "**User Tag:** {}", user.tag())?;
    if user.bot {
        writeln!(user_str, "(Bot)")?;
    } else {
        writeln!(user_str)?;
    }

    writeln!(user_str, "**ID:** {}", user.id.0)?;
    writeln!(
        user_str,
        "**Created at:** {} ({} ago)",
        user.created_at().format("%Y-%m-%d %H:%M:%S"),
        format_duration(&now, &user.created_at())
    )?;

    let mut colour = None;

    if let Ok(member) = member {
        writeln!(user_str)?;
        writeln!(user_str, "**Member Info**")?;

        if let Some(ref nick) = member.nick {
            writeln!(user_str, "**Server Nickname:** {}", nick)?;
        }

        if let Some(ref joined_at) = member.joined_at {
            writeln!(
                user_str,
                "**Joined at:** {} ({} ago)",
                joined_at.format("%Y-%m-%d %H:%M:%S"),
                format_duration(&now, joined_at)
            )?;
        }

        if !member.roles.is_empty() {
            write!(user_str, "**Roles:**")?;
        }

        for role in &member.roles {
            write!(user_str, "{} ", role.mention())?;
        }

        if !member.roles.is_empty() {
            writeln!(user_str)?;
        }

        colour = member.colour(ctx).await;
    }

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.author(|a| {
                    a.name(&user.tag());
                    a.url(user.face());

                    a
                });

                if let Some(colour) = colour {
                    e.colour(colour);
                }

                e.thumbnail(user.face());
                e.description(user_str);

                e
            })
        })
        .await?;

    Ok(())
}

fn format_duration(now: &DateTime<Utc>, before: &DateTime<Utc>) -> String {
    let dur_secs = Duration::seconds(now.signed_duration_since(*before).num_seconds());
    let days = dur_secs.num_days();

    if days >= 1 {
        format!("{} days", days)
    } else {
        humantime::format_duration(dur_secs.to_std().unwrap()).to_string()
    }
}
