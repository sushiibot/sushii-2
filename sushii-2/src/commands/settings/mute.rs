use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::parse_role;

use crate::model::sql::*;

#[command]
#[sub_commands(role, defaultduration)]
#[required_permissions("MANAGE_GUILD")]
async fn mute(ctx: &Context, msg: &Message) -> CommandResult {
    let _ = msg
        .channel_id
        .say(
            &ctx.http,
            "Available sub-commands for `mute` are `role`, `defaultduration`",
        )
        .await?;

    Ok(())
}

#[command]
#[required_permissions("MANAGE_GUILD")]
#[description("Sets the mute role")]
#[usage("[role mention, ID, or name]")]
async fn role(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild = match msg.guild(&ctx.cache).await {
        Some(g) => g,
        None => {
            let _ = msg.channel_id.say(&ctx.http, "No guild found").await?;

            return Ok(());
        }
    };

    let mut conf = GuildConfig::from_msg_or_respond(&ctx, msg).await?;

    let role_str = args.rest();

    if role_str.is_empty() {
        msg.channel_id
            .say(&ctx, "Error: Give a role ID or name")
            .await?;

        return Ok(());
    }

    let role_id = parse_role(role_str)
        .or_else(|| role_str.parse::<u64>().ok())
        .or_else(|| {
            guild
                .roles
                .values()
                .find(|&x| x.name.to_lowercase() == role_str.to_lowercase())
                .map(|x| x.id.0)
        });

    if let Some(id) = role_id {
        conf.mute_role.replace(id as i64);
        conf.save(&ctx).await?;

        msg.channel_id
            .say(&ctx.http, format!("Updated mute role to ID {}", id))
            .await?;
    } else {
        msg.channel_id
            .say(&ctx.http, "Invalid role, give a role name, mention, or ID")
            .await?;
    }

    Ok(())
}

#[command]
#[required_permissions("MANAGE_GUILD")]
#[description("Sets the default duration of mutes, default is 24 hours")]
#[usage("[duration]")]
#[example("12 hours 30 minutes")]
async fn defaultduration(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut conf = GuildConfig::from_msg_or_respond(&ctx, msg).await?;

    let duration_str = args.rest();

    if duration_str.is_empty() {
        msg.channel_id
            .say(
                &ctx,
                "Error: Please provide a mute duration or `indefinite` (or 0) to disable. Example: `12 hours 30 minutes`",
            )
            .await?;

        return Ok(());
    }

    let duration = match duration_str {
        "inf" | "indef" | "indefinite" | "0" => None,
        _ => {
            let duration = match crate::utils::duration::parse_duration_std(&duration_str) {
                Ok(d) => d,
                Err(e) => {
                    msg.channel_id
                        .say(
                            &ctx.http,
                            format!("Error: Failed to parse duration -- {}", e),
                        )
                        .await?;

                    return Ok(());
                }
            };

            Some(duration)
        }
    };

    conf.mute_duration = duration.map(|d| d.as_secs() as i64);

    conf.save(&ctx).await?;

    let s = if let Some(dur) = duration {
        format!(
            "Set the default mute duration to `{}`",
            humantime::format_duration(dur)
        )
    } else {
        "Default mute duration set to indefinite (will not expire)".into()
    };

    msg.channel_id.say(&ctx.http, s).await?;

    Ok(())
}
