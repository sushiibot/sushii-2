use serenity::{model::prelude::*, prelude::*};

use crate::model::moderation::ModLogReporter;

pub async fn guild_ban_addition(ctx: &Context, guild_id: &GuildId, banned_user: &User) {
    if let Err(e) = ModLogReporter::new(guild_id, banned_user, "ban")
        .execute(&ctx)
        .await
    {
        tracing::error!("Failed to handle guild_ban_addition: {}", e);
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
