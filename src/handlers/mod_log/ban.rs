use serenity::{model::prelude::*, prelude::*};

use super::utils::modlog_handler;

pub async fn guild_ban_addition(ctx: &Context, guild_id: &GuildId, banned_user: &User) {
    if let Err(e) = modlog_handler(ctx, guild_id, banned_user, "ban", &None).await {
        tracing::error!("Failed to handle guild_ban_addition: {}", e);
    }
}

pub async fn guild_ban_removal(ctx: &Context, guild_id: &GuildId, unbanned_user: &User) {
    if let Err(e) = modlog_handler(ctx, guild_id, unbanned_user, "unban", &None).await {
        tracing::error!("Failed to handle guild_ban_removal: {}", e);
    }
}
