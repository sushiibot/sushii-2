use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::error::Result;
use crate::model::sql::*;

pub async fn check_pending_unmutes(ctx: &Context) -> Result<()> {
    let expired_mutes = Mute::get_expired(&ctx).await?;

    for mute in expired_mutes {
        // Don't use ? since we want to try the rest of the mute entries instead
        // of just stopping on any error
        if let Err(e) = log_unmute(&ctx, &mute).await {
            tracing::error!(?mute, "Failed log unmute: {}", e);
        }
    }

    Ok(())
}

// Logs an unmute in mod log when expired. This does not do anything with the
// member as native time outs expire on their own. There is a possible edge
// case where the end time is not synced with the timeout end time but it does
// not affect the mute, just logging.
pub async fn log_unmute(ctx: &Context, mute: &Mute) -> Result<()> {
    let guild_id = GuildId(mute.guild_id as u64);

    let user = UserId(mute.user_id as u64).to_user(&ctx).await?;

    let reason = format!(
        "Automated Unmute: Mute expired (Duration: {}).",
        mute.get_human_duration().unwrap_or_else(|| "N/A".into()),
    );

    ModLogEntry::new("unmute", true, guild_id.0, user.id.0, &user.tag())
        .reason(&Some(reason))
        .save(&ctx)
        .await?;

    mute.delete(&ctx).await?;

    Ok(())
}
