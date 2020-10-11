use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::fmt::Write;

use crate::model::moderation::{ModActionExecutor, ModActionExecutorDb, ModActionType};
use crate::model::sql::*;

#[command]
#[only_in("guild")]
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
