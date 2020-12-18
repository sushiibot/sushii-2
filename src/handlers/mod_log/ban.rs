use serenity::{model::prelude::*, prelude::*};

use crate::model::moderation::ModLogReporter;
use crate::model::sql::delete_mute;

pub async fn guild_ban_addition(ctx: &Context, guild_id: &GuildId, banned_user: &User) {
    if let Err(e) = ModLogReporter::new(guild_id, banned_user, "ban")
        .execute(&ctx)
        .await
    {
        tracing::error!("Failed to handle guild_ban_addition: {}", e);
    }

    // Delete any mute entries if any
    if let Err(e) = delete_mute(&ctx, guild_id.0, banned_user.id.0).await {
        tracing::error!("Failed to delete mute: {}", e);
    }
}

pub async fn guild_ban_removal(ctx: &Context, guild_id: &GuildId, unbanned_user: &User) {
    if let Err(e) = ModLogReporter::new(guild_id, unbanned_user, "unban")
        .execute(&ctx)
        .await
    {
        tracing::error!("Failed to handle guild_ban_removal: {}", e);
    }
}
