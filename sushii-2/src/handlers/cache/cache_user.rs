use serenity::{model::prelude::*, prelude::*};

use crate::error::Result;
use crate::model::sql::*;

pub async fn message(ctx: &Context, msg: &Message) {
    if let Err(e) = _message(ctx, msg).await {
        tracing::error!(?msg, "Failed to run cache_users handler: {}", e);
    }
}

async fn _message(ctx: &Context, msg: &Message) -> Result<()> {
    // Ignore bots
    if msg.author.bot {
        return Ok(());
    }

    CachedUser::update(ctx, &msg.author).await?;

    Ok(())
}
