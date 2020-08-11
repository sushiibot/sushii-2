mod cacher;

use std::sync::Arc;
use twilight::gateway::Event;
use twilight::model::gateway::payload::MessageCreate;

use crate::error::Result;

use crate::model::context::SushiiContext;

pub async fn handle_middleware<'a>(
    msg: &Box<MessageCreate>,
    ctx: Arc<SushiiContext<'a>>,
) -> Result<()> {
    cacher::cache_guild_config(&msg, &ctx).await;

    Ok(())
}

pub async fn handle_event<'a>(
    _shard_id: u64,
    event: &Event,
    ctx: Arc<SushiiContext<'a>>,
) -> Result<()> {
    match event {
        Event::MessageCreate(msg) => {
            if let Err(e) = handle_middleware(msg, ctx.clone()).await {
                tracing::error!(?msg, "Failed to handle middleware: {}", e);
            };
        }
        _ => {}
    }

    Ok(())
}