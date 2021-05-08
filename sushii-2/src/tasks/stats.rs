use serenity::prelude::*;

use crate::error::Result;
use crate::model::sql::*;
use crate::DbPool;

pub async fn update_stats(ctx: &Context) -> Result<()> {
    let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();
    let guild_count = ctx.cache.guild_count().await;

    BotStat::set(&pool, "bot", "guild_count", guild_count as i64).await?;

    Ok(())
}
