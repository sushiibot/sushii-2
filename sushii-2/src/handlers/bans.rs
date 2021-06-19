use serenity::{model::prelude::*, prelude::*};
use tokio::time::{sleep, Duration};

use crate::error::Result;
use sushii_model::keys::DbPool;
use sushii_model::model::sql::GuildBan;

pub async fn cache_ready(ctx: &Context, guild_ids: &[GuildId]) {
    if let Err(e) = _cache_ready(ctx, guild_ids).await {
        tracing::error!("Failed to handle bans cache_ready: {}", e);
    }
}

async fn _cache_ready(ctx: &Context, guild_ids: &[GuildId]) -> Result<()> {
    let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

    for guild_id in guild_ids {
        let bans = guild_id.bans(ctx).await?;
        tracing::debug!(
            "Fetched guild ID {} bans, found {} bans",
            guild_id.0,
            bans.len()
        );

        GuildBan::update_guild_bans(&pool, *guild_id, &bans).await?;

        // Wait between each one as to not spam api even though it should be rate limited
        sleep(Duration::from_secs(2)).await;
    }

    Ok(())
}

pub async fn guild_ban_addition(ctx: &Context, guild_id: GuildId, banned_user: &User) {
    if let Err(e) = _guild_ban_addition(ctx, guild_id, banned_user).await {
        tracing::error!("Failed to handle bans guild_ban_addition: {}", e);
    }
}

async fn _guild_ban_addition(ctx: &Context, guild_id: GuildId, banned_user: &User) -> Result<()> {
    let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

    GuildBan::add_ban(&pool, guild_id, banned_user.id).await?;

    Ok(())
}

pub async fn guild_ban_removal(ctx: &Context, guild_id: GuildId, unbanned_user: &User) {
    if let Err(e) = _guild_ban_removal(&ctx, guild_id, unbanned_user).await {
        tracing::error!("Failed to handle bans guild_ban_removal: {}", e);
    }
}

async fn _guild_ban_removal(ctx: &Context, guild_id: GuildId, unbanned_user: &User) -> Result<()> {
    let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

    GuildBan::remove_ban(&pool, guild_id, unbanned_user.id).await?;

    Ok(())
}
