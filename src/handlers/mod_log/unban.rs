use serenity::{model::prelude::*, prelude::*};

use super::utils::guild_ban_handler;

pub async fn guild_ban_removal(ctx: &Context, guild_id: &GuildId, unbanned_user: &User) {
    if let Err(e) = guild_ban_handler(ctx, guild_id, unbanned_user, "unban").await {
        tracing::error!("Failed to handle guild_ban_removal: {}", e);
    }
}
