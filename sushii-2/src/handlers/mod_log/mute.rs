use chrono::{offset::Utc, Duration};
use serenity::{model::prelude::*, prelude::*};
use std::collections::HashSet;
use std::fmt::Write;

use crate::error::Result;
use crate::model::moderation::ModLogReporter;
use crate::model::sql::*;

pub async fn guild_member_addition(ctx: &Context, guild_id: GuildId, mut member: &mut Member) {
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
    let guild_conf = match GuildConfig::from_id(&ctx, &new_member.guild_id).await? {
        Some(c) => c,
        None => {
            tracing::error!(?new_member.guild_id, ?new_member, "No guild config found while handling mute guild_member_update");
            return Ok(());
        }
    };

    let mute_role = match guild_conf.mute_role {
        Some(role) => RoleId(role as u64),
        None => return Ok(()),
    };

    let mute_entry =
        Mute::from_id_any_pending(ctx, new_member.guild_id.0, new_member.user.id.0).await?;

    if let Some(old_member) = old_member {
        let old: HashSet<_> = old_member.roles.iter().collect();
        let new: HashSet<_> = new_member.roles.iter().collect();

        if old == new {
            return Ok(());
        }
    }

    let new_has_mute = new_member.roles.contains(&mute_role);

    let mute_entry = match (mute_entry, new_has_mute) {
        // Role added manually: No mute entry and has mute role
        (None, true) => {
            // If there isn't a pending OR there isn't an existing mute, it's
            // a NEW regular mute from manually adding roles to a user so just
            // create a new one
            Some(Mute::new(
                new_member.guild_id.0,
                new_member.user.id.0,
                guild_conf.mute_duration.map(Duration::seconds),
            ))
        }
        // Role added for s!!mute command: Has pending mute entry, so use existing
        (Some(entry), true) if entry.pending => {
            // If there's a pending one, update pending to false
            // Save it first in case other stuff fails, since if other stuff
            // fails we don't want this pending still, just throw it out I guess
            Some(entry.pending(false).save(ctx).await?)
        }
        // Role removed: Has mute entry and does not have mute role
        (Some(_), false) => {
            delete_mute(&ctx, new_member.guild_id.0, new_member.user.id.0).await?;

            None
        }
        // Role added but not pending, just whatever maybe rejoin
        _ => return Ok(()),
    };

    let action = if mute_entry.is_none() {
        "unmute"
    } else {
        "mute"
    };

    // Initial entry executor might NOT be the same person unmuting as manual
    // unmutes can be done by anyone. This is only useful when it is an
    // automated unmute by sushii
    let initial_entry = if let Some(case_id) = mute_entry.as_ref().and_then(|e| e.case_id) {
        ModLogEntry::from_case_id(&ctx, new_member.guild_id.0, case_id as u64).await?
    } else {
        None
    };

    // Add a mod log entry
    let entry = ModLogReporter::new(&new_member.guild_id, &new_member.user, action)
        .mute_duration(mute_entry.as_ref().and_then(|m| m.get_std_duration()))
        .initial_entry(initial_entry)
        .execute(&ctx)
        .await?;

    tracing::debug!(?mute_entry, ?entry, "Added mod log entry");

    let mute_entry = if let Some(mute_entry) = mute_entry {
        // Add the mod log case id and save it to db
        // Return again to use it below
        Some(mute_entry.case_id(entry.case_id).save(&ctx).await?)
    } else {
        None
    };

    // If dm isn't enabled skip the rest
    if !guild_conf.mute_dm_enabled {
        return Ok(());
    }

    let guild_name = new_member
        .guild_id
        .to_guild_cached(&ctx)
        .as_ref()
        .map(|g| g.name.clone())
        .unwrap_or_else(|| format!("Unknown Guild (ID: {})", new_member.guild_id.0));

    // Dm user
    let mut s = String::new();

    writeln!(s, "You have been {}d in {}", action, guild_name)?;

    if action == "mute" {
        if let Some(ref reason) = entry.reason {
            writeln!(s, "Reason: {}", reason)?;
        }
    }

    if let Some(dur_str) = mute_entry.and_then(|m| m.get_human_duration()) {
        write!(s, "Duration: {}", dur_str)?;
    }

    // Ignore if dm fails, could be disabled
    let _ = new_member.user.dm(ctx, |m| m.content(s)).await;

    Ok(())
}
