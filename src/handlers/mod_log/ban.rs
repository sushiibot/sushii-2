use serenity::{model::prelude::*, prelude::*};

use super::utils::guild_ban_handler;

pub async fn guild_ban_addition(ctx: &Context, guild_id: &GuildId, banned_user: &User) {
    if let Err(e) = guild_ban_handler(ctx, guild_id, banned_user, "ban").await {
        tracing::error!("Failed to handle guild_ban_addition: {}", e);
    }
}
