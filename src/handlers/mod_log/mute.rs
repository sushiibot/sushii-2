use serenity::{model::prelude::*, prelude::*};

use super::utils::modlog_handler;
use crate::error::Result;
use crate::model::sql::*;

pub async fn guild_member_update(ctx: &Context, old_member: &Option<Member>, new_member: &Member) {
    if let Err(e) = _guild_member_update(&ctx, &old_member, &new_member).await {
        tracing::error!("Failed to handle mutes member update: {}", e);
    }
}

async fn _guild_member_update(ctx: &Context, old_member: &Option<Member>, new_member: &Member) -> Result<()> {
    let guild_conf = match GuildConfig::from_id(&ctx, &new_member.guild_id).await? {
        Some(c) => c,
        None => {
            tracing::error!(?new_member.guild_id, ?new_member, "No guild config found while handling mute handler");
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
        None => return Ok(())
    };

    let old_has_mute = old_member.roles.contains(&mute_role);
    let new_has_mute = new_member.roles.contains(&mute_role);

    let action = match (old_has_mute, new_has_mute) {
        (false, true) => "mute",
        (true, false) => "unmute",
        // No changes, return
        _ => return Ok(()),
    };

    modlog_handler(ctx, &new_member.guild_id, &new_member.user, action).await
}
