use chrono::{offset::Utc, Duration};
use serenity::{model::prelude::*, prelude::*};

use crate::error::Result;
use crate::model::moderation::{ModLogReporter, ModLogReporterDb};
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

    let now = Utc::now().naive_local();

    // If mute already expired, just log it and return -- don't need to do anything else
    if let Some(end) = mute.end_time {
        if now > end {
            ModLogEntry::new("unmute", true, guild_id.0, &member.user)
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

    // Add a pending modlog entry for the auto mute reason
    ModLogEntry::new("mute", true, guild_id.0, &member.user)
        .reason(&Some("Automated Mute: User left with a mute role.".into()))
        .save(&ctx)
        .await?;

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

    // If there isn't an prev member then we can't really compare if the role was just added
    let old_member = match old_member {
        Some(m) => m,
        None => return Ok(()),
    };

    let old_has_mute = old_member.roles.contains(&mute_role);
    let new_has_mute = new_member.roles.contains(&mute_role);

    let action = match (old_has_mute, new_has_mute) {
        (false, true) => "mute",
        (true, false) => "unmute",
        // No changes, return
        _ => return Ok(()),
    };

    // Mute entries are to monitor for re-joins and keep track of duration
    let mute_entry = if action == "mute" {
        // Check for a pending mute (e.g. mutes with a command)
        if let Some(mute_entry) =
            Mute::get_pending(&ctx, new_member.guild_id.0, new_member.user.id.0).await?
        {
            tracing::debug!(?mute_entry, "Found pending mute entry");
            // If there's a pending one, update pending to false
            // Save it first in case other stuff fails, since if other stuff
            // fails we don't want this pending still, just throw it out I guess
            Some(mute_entry.pending(false).save(&ctx).await?)
        } else {
            // If there isn't a pending, it's just a regular mute from manually
            // adding roles to a user so just create a new one
            Some(Mute::new(
                new_member.guild_id.0,
                new_member.user.id.0,
                guild_conf.mute_duration.map(Duration::seconds),
            ))
        }
    } else {
        // action == "unmute"
        delete_mute(&ctx, new_member.guild_id.0, new_member.user.id.0).await?;

        None
    };

    // Initial entry executor might NOT be the same person unmuting as manual
    // unmutes can be done by anyone. This is only useful when it is an
    // automated unmute by sushii
    let initial_entry = if let Some(case_id) = mute_entry.as_ref().and_then(|e| e.case_id) {
        Some(ModLogEntry::from_case_id(&ctx, new_member.guild_id.0, case_id as u64).await?)
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

    if let Some(mute_entry) = mute_entry {
        // Add the mod log case id and save it to db
        mute_entry.case_id(entry.case_id).save(&ctx).await?;
    }

    Ok(())
}
