use serenity::{model::prelude::*, prelude::*};

use crate::error::Result;
use crate::model::sql::*;

pub async fn message(ctx: &Context, msg: &Message) {
    if let Err(e) = _message(ctx, msg).await {
        tracing::error!(?msg, "Failed to run user_levels handler: {}", e);
    }
}

async fn _message(ctx: &Context, msg: &Message) -> Result<()> {
    // Ignore bots
    if msg.author.bot {
        return Ok(());
    }

    // Ignore DMs
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return Ok(()),
    };

    // Get user level or create a new one
    let user_level = if let Some(lvl) = UserLevel::from_id(&ctx, msg.author.id, guild_id).await? {
        lvl
    } else {
        // No data found, make a new one and save it
        // Won't be eligible for the following inc() so it doesn't matter about the extra save below
        UserLevel::new(msg.author.id, guild_id).save(&ctx).await?
    };

    // < 1 minute since last XP inc
    if !user_level.eligible() {
        return Ok(());
    }

    // Increment XP and save to DB
    user_level.inc().save(&ctx).await?;

    Ok(())
}
