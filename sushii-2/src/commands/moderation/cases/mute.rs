use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::fmt::Write;

use crate::model::sql::*;

#[command]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
#[sub_commands(setduration, addduration)]
async fn mute(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(
            &ctx.http,
            "Please use </timeout:996259097202671645> now or right-click the user -> timeout :)",
        )
        .await?;

    return Ok(());
}

#[command]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
async fn unmute(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(
            &ctx.http,
            "Please use </untimeout:1070531459229700147> now or right-click the user -> remove timeout :)",
        )
        .await?;

    return Ok(());
}

#[command]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
#[aliases("s", "set", "setd", "setdur", "settime")]
async fn setduration(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(
            &ctx.http,
            "Please use </timeout:996259097202671645> to update the duration of a mute :)",
        )
        .await?;

    return Ok(());
}

#[command]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
#[aliases("a", "add", "addd", "adddur", "addtime", "extend")]
async fn addduration(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(
            &ctx.http,
            "Please use </timeout:996259097202671645> to update the duration of a mute :)",
        )
        .await?;

    return Ok(());
}

#[command]
#[required_permissions("BAN_MEMBERS")]
#[aliases("listmute", "mutelist", "muteslist")]
#[only_in("guild")]
async fn listmutes(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = match msg.guild(&ctx.cache) {
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

    if guild_conf.mute_role.is_none() {
        msg.channel_id
            .say(&ctx, "Error: There isn't a mute role set")
            .await?;

        return Ok(());
    }

    let ongoing_mutes: Vec<Mute> = Mute::get_ongoing(&ctx, guild.id.0).await?;

    let mut definite_mutes: Vec<&Mute> = ongoing_mutes
        .iter()
        .filter(|x| x.end_time.is_some())
        .collect();

    // Sort by time remaining
    definite_mutes.sort_by_cached_key(|m| m.get_duration_remaining());
    // Sort based on total mute duration (after remaining time)
    definite_mutes.sort_by_cached_key(|m| m.get_duration());

    let indefinite_mutes: Vec<&Mute> = ongoing_mutes
        .iter()
        .filter(|x| x.end_time.is_none())
        .collect();

    let mut s = String::new();

    if !definite_mutes.is_empty() {
        let _ = writeln!(s, "`total` | `remaining`");
    }

    for mute in definite_mutes {
        if let Some(d) = mute.get_human_duration() {
            let _ = write!(s, "`{}`", d);
        }

        if let Some(d) = mute.get_human_duration_remaining() {
            let _ = write!(s, " | `{}`", d);
        }

        let _ = writeln!(s, " <@{}>", mute.user_id as u64);
    }

    if !indefinite_mutes.is_empty() {
        writeln!(s)?;
        writeln!(s, "**Indefinite Mutes**")?;
    }

    for mute in indefinite_mutes {
        if let Some(d) = mute.get_human_duration_elapsed() {
            let _ = write!(s, "`{}` elapsed", d);
        }

        let _ = writeln!(s, ": <@{}>", mute.user_id as u64);
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
