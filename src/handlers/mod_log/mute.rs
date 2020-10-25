use chrono::{offset::Utc, Duration};
use serenity::{model::prelude::*, prelude::*};

use super::utils::modlog_handler;
use crate::error::Result;
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

    // Add a mod log entry
    let entry = modlog_handler(ctx, &new_member.guild_id, &new_member.user, action).await?;

    if action == "mute" {
        // Mute entries are to monitor for re-joins and keep track of duration

        // Check for a pending mute (e.g. mutes with a command)
        if let Some(mute_entry) =
            Mute::get_pending(&ctx, new_member.guild_id.0, new_member.user.id.0).await?
        {
            // If there's a pending one, update the mod log case id and set
            // pending to false
            mute_entry
                .case_id(entry.case_id)
                .pending(false)
                .save(&ctx)
                .await?;
        } else {
            // If there isn't a pending, it's just a regular mute from manually
            // adding roles to a user so just create a new one and save it
            Mute::new(
                new_member.guild_id.0,
                new_member.user.id.0,
                guild_conf.mute_duration.map(Duration::seconds),
            )
            .case_id(entry.case_id)
            .save(&ctx)
            .await?;
        }
    } else if action == "unmute" {
        delete_mute(&ctx, new_member.guild_id.0, new_member.user.id.0).await?;
    }

    Ok(())
}
