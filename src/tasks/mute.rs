use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::error::Result;
use crate::model::sql::*;

pub async fn check_pending_unmutes(ctx: &Context) -> Result<()> {
    let expired_mutes = Mute::get_expired(&ctx).await?;
    tracing::debug!("Found {} expired mute entries", expired_mutes.len());

    for mute in expired_mutes {
        // Don't use ? since we want to try the rest of the mute entries instead
        // of just stopping on any error
        if let Err(e) = unmute_member(&ctx, &mute).await {
            tracing::error!(?mute, "Failed to unmute member: {}", e);
        }
    }

    Ok(())
}

pub async fn unmute_member(ctx: &Context, mute: &Mute) -> Result<()> {
    let guild_id = GuildId(mute.guild_id as u64);
    // Possibly inefficient here since there can be the same guild config
    // fetched here, but it's likely that there aren't many entries at a single
    // time since the check happens every 10 seconds
    let guild_conf = match GuildConfig::from_id(&ctx, &guild_id).await? {
        Some(c) => c,
        None => {
            tracing::warn!("Guild config not found handling mute expirey");
            return Ok(());
        }
    };

    let mute_role = match guild_conf.mute_role {
        Some(role) => role as u64,
        None => {
            tracing::warn!(
                ?guild_conf,
                "Guild mute role not found handling mute expirey"
            );
            return Ok(());
        }
    };

    let mut member = guild_id.member(&ctx, mute.user_id as u64).await?;
    member.remove_role(&ctx, mute_role).await?;

    mute.delete(&ctx).await?;

    Ok(())
}
