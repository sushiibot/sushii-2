use chrono::{offset::Utc, Duration};
use serenity::{model::prelude::*, prelude::*};
use std::fmt::Write;

use crate::error::Result;
use crate::model::moderation::ModLogReporter;
use crate::model::sql::*;

pub async fn guild_member_addition(ctx: &Context, guild_id: &GuildId, mut member: &mut Member) {
    if let Err(e) = _guild_member_addition(&ctx, &guild_id, &mut member).await {
        tracing::error!("Failed to handle mutes guild_member_addition: {}", e);
    }
}

async fn _guild_member_addition(
    ctx: &Context,
    guild_id: &GuildId,
    member: &mut Member,
) -> Result<()> {
    let mute = match Mute::from_id(&ctx, guild_id.0, member.user.id.0).await? {
        Some(m) => m,
        None => return Ok(()),
    };

    let now = Utc::now().naive_utc();

    // If mute already expired, just log it and return -- don't need to do anything else
    // TODO: Check if this is needed, don't think this ever happens, since if it's already expired it would be deleted
    if let Some(end) = mute.end_time {
        if now > end {
            ModLogEntry::new(
                "unmute",
                true,
                guild_id.0,
                member.user.id.0,
                &member.user.tag(),
            )
            .reason(&Some(
                "Automated Unmute: User re-joined after mute expired.".into(),
            ))
            .save(&ctx)
            .await?;

            return Ok(());
        }
    }

    let guild_conf = match GuildConfig::from_id(&ctx, &member.guild_id).await? {
        Some(c) => c,
        None => {
            tracing::error!(?member.guild_id, ?member, "No guild config found while handling mute guild_member_addition");
            return Ok(());
        }
    };

    let mute_role = match guild_conf.mute_role {
        Some(role) => RoleId(role as u64),
        None => return Ok(()),
    };

    // Re-add mute role if there's a mute entry
    member.add_role(&ctx.http, mute_role).await?;

    Ok(())
}

pub async fn guild_member_update(ctx: &Context, old_member: &Option<Member>, new_member: &Member) {
    if let Err(e) = _guild_member_update(&ctx, &old_member, &new_member).await {
        tracing::error!("Failed to handle mutes member update: {}", e);
    }
}

async fn _guild_member_update(
    ctx: &Context,
    old_member: &Option<Member>,
    new_member: &Member,
) -> Result<()> {
    let old_member = match old_member {
        Some(old_member) => old_member,
        None => return Ok(()),
    };

    // No modification to mute/unmute/duration change
    if old_member.communication_disabled_until == new_member.communication_disabled_until {
        return Ok(());
    }

    let duration = new_member
        .communication_disabled_until
        .map(|until| until.signed_duration_since(Utc::now()))
        .map(|d| Duration::seconds(d.num_seconds()));

    let guild_conf = match GuildConfig::from_id(&ctx, &new_member.guild_id).await? {
        Some(c) => c,
        None => {
            tracing::error!(
                ?new_member.guild_id,
                ?new_member,
                "No guild config found while handling mute guild_member_update",
            );
            return Ok(());
        }
    };

    let now = Utc::now();
    let action = match (
        old_member.communication_disabled_until,
        new_member.communication_disabled_until,
    ) {
        // Timeout removed
        (Some(_), None) => MuteAction::Unmute,
        // If disabled_until has not changed but timestamp is in the past
        (Some(_), Some(until)) if until < now => MuteAction::Unmute,
        // Timeout newly added
        (None, Some(_)) => MuteAction::Mute,
        // Timeout changed and is still valid
        (Some(_), Some(_)) => MuteAction::ChangeDuration,
        // No timeout to no timeout
        (None, None) => return Ok(()),
    };

    let mute_entry =
        Mute::from_id_any_pending(ctx, new_member.guild_id.0, new_member.user.id.0).await?;

    let mute_entry = match (mute_entry, action) {
        // Timed out manually: No mute entry and is timed out
        (None, MuteAction::Mute) => {
            // If there isn't a pending OR there isn't an existing mute, it's a
            // NEW regular mute from manually adding timing out a user to a user
            // so just create a new one
            Some(Mute::new(
                new_member.guild_id.0,
                new_member.user.id.0,
                duration,
            ))
        }
        // s!!mute command: Has pending mute entry, so use existing
        (Some(entry), MuteAction::Mute) if entry.pending => {
            // If there's a pending one, update pending to false
            // Save it first in case other stuff fails, since if other stuff
            // fails we don't want this pending still, just throw it out I guess
            Some(entry.pending(false).save(ctx).await?)
        }
        // Timeout removed: Has mute entry and is no longer muted
        (Some(_), MuteAction::Unmute) => {
            delete_mute(&ctx, new_member.guild_id.0, new_member.user.id.0).await?;

            None
        }
        // Timeout changed: Has mute entry and mute duration changed
        (Some(mut entry), MuteAction::ChangeDuration) => {
            // Update end time to the new end time
            entry.end_time = new_member
                .communication_disabled_until
                .map(|d| d.naive_utc());

            entry.save(ctx).await?;

            None
        }
        // timeout added but no pending mute entry
        _ => return Ok(()),
    };

    // Add a mod log entry
    let entry = ModLogReporter::new(&new_member.guild_id, &new_member.user, &action.to_string())
        .mute_duration(mute_entry.as_ref().and_then(|m| m.get_std_duration()))
        .execute(&ctx)
        .await?;

    tracing::debug!(?entry, "Added mod log entry");

    // If dm isn't enabled skip the rest
    if !guild_conf.mute_dm_enabled {
        return Ok(());
    }

    let guild_name = new_member
        .guild_id
        .to_guild_cached(&ctx)
        .await
        .as_ref()
        .map(|g| g.name.clone())
        .unwrap_or_else(|| format!("Unknown Guild (ID: {})", new_member.guild_id.0));

    // Dm user
    let mut s = String::new();

    writeln!(s, "{} in {}", action.to_dm_message(), guild_name)?;

    if action == MuteAction::Mute {
        if let Some(ref reason) = entry.reason {
            writeln!(s, "Reason: {}", reason)?;
        }
    }

    // Ignore if dm fails, could be disabled
    let _ = new_member.user.dm(ctx, |m| m.content(s)).await;

    Ok(())
}
