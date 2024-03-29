use serenity::prelude::*;
use std::sync::atomic::Ordering;

use crate::error::Result;
use crate::model::sql::*;
use crate::{DbPool, Metrics};

pub async fn update_stats(ctx: &Context) -> Result<()> {
    let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();
    let metrics = ctx.data.read().await.get::<Metrics>().cloned().unwrap();

    let guild_count = ctx.cache.guild_count();

    BotStat::set(&pool, "bot", "guild_count", guild_count as i64).await?;
    BotStat::set(
        &pool,
        "bot",
        "member_count",
        metrics.member_total.load(Ordering::Relaxed) as i64,
    )
    .await?;

    Ok(())
}
