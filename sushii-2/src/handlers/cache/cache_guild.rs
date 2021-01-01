use serenity::{model::prelude::*, prelude::*};

use crate::error::Result;
use crate::model::sql::*;

pub async fn guild_create(ctx: &Context, guild: &Guild, is_new: bool) {
    if let Err(e) = _guild_create(ctx, guild, is_new).await {
        tracing::error!(?guild, "Failed to run cache_guild create handler: {}", e);
    }
}

async fn _guild_create(ctx: &Context, guild: &Guild, _is_new: bool) -> Result<()> {
    CachedGuild::update(ctx, guild).await?;

    Ok(())
}
pub async fn guild_update(
    ctx: &Context,
    old_guild_if_avail: &Option<Guild>,
    partial_guild: &PartialGuild,
) {
    if let Err(e) = _guild_update(ctx, old_guild_if_avail, partial_guild).await {
        tracing::error!(
            ?partial_guild,
            "Failed to run cache_guild update handler: {}",
            e
        );
    }
}

async fn _guild_update(
    ctx: &Context,
    _old_guild_if_avail: &Option<Guild>,
    partial_guild: &PartialGuild,
) -> Result<()> {
    CachedGuild::update_from_partial(ctx, partial_guild).await?;

    Ok(())
}
