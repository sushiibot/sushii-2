use humantime::DurationError;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::parse_role;

use crate::model::sql::*;

#[command]
#[sub_commands(role, duration)]
async fn mute(ctx: &Context, msg: &Message) -> CommandResult {
    let _ = msg
        .channel_id
        .say(
            &ctx.http,
            "Available sub-commands for `mute` are `role`, `duration`",
        )
        .await?;

    Ok(())
}

#[command]
#[num_args(1)]
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

    let role_id = parse_role(role_str)
        .or_else(|| role_str.parse::<u64>().ok())
        .or_else(|| {
            guild
                .roles
                .values()
                .find(|&x| x.name == role_str)
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

fn point_str(s: &str, pos: usize, end: Option<usize>) -> String {
    format!(
        "```\n{}\n{}{}\n```",
        s,
        " ".repeat(pos),
        "^".repeat(end.map_or(1, |e| e - pos)) // End is exclusive
    )
}

#[command]
async fn duration(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut conf = GuildConfig::from_msg_or_respond(&ctx, msg).await?;

    let duration_str = args.rest();

    if duration_str.is_empty() {
        msg.channel_id
            .say(
                &ctx,
                "Error: Please provide a mute duration. Example: `12 hours 30 minutes`",
            )
            .await?;

        return Ok(());
    }

    let duration = match humantime::parse_duration(&duration_str) {
        Ok(d) => d,
        Err(e) => {
            let err_str = match e {
                DurationError::InvalidCharacter(pos) => format!(
                    "Invalid character (only alphanumeric characters are allowed):\n{}",
                    point_str(&duration_str, pos, None),
                ),
                DurationError::NumberExpected(pos) => format!(
                    "Expected a number.\nThis usually means that either the time \
                        unit is separated (e.g. `m in` instead of `min`) \
                        or a number is omitted (e.g. `2 hours min` instead \
                        of `2 hours 1 min`):\n\
                        {}",
                    point_str(&duration_str, pos, None),
                ),
                DurationError::UnknownUnit { start, end, .. } => format!(
                    "Invalid time unit, valid units are:\n\
                    `seconds (second, sec, s),\n\
                    minutes (minute, min, m),\n\
                    hours (hour, hr, h),\n\
                    days (day, d),\n\
                    weeks (week, w),\n\
                    months (month, M)`:\n{}",
                    point_str(&duration_str, start, Some(end)),
                ),
                DurationError::NumberOverflow => "Duration is too long".into(),
                DurationError::Empty => "Duration cannot be empty".into(),
            };

            msg.channel_id
                .say(
                    &ctx.http,
                    format!("Error: Failed to parse duration -- {}", err_str),
                )
                .await?;

            return Ok(());
        }
    };

    conf.mute_duration.replace(duration.as_secs() as i64);

    conf.save(&ctx).await?;

    msg.channel_id
        .say(
            &ctx.http,
            format!(
                "Set the default mute duration to `{}`",
                humantime::format_duration(duration)
            ),
        )
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_str_with_end() {
        let s = point_str("0123456789", 0, Some(9));

        assert_eq!(
            s,
            "```
0123456789
^^^^^^^^^^
```"
        );
    }

    #[test]
    fn point_str_substr() {
        let s = point_str("0123456789", 1, Some(6));

        assert_eq!(
            s,
            "```
0123456789
 ^^^^^^
```"
        );
    }

    #[test]
    fn point_str_single() {
        let s = point_str("0123456789", 7, None);

        assert_eq!(
            s,
            "```
0123456789
       ^
```"
        );
    }
}
