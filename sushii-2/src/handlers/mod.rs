use serenity::{async_trait, model::prelude::*, prelude::*};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        tracing::info!("Connected as {}", ready.user.name);
    }

    async fn cache_ready(&self, _ctx: Context, _guild_ids: Vec<GuildId>) {
        tracing::info!("Cache ready!");
    }

    async fn resume(&self, ctx: Context, _: ResumedEvent) {
        tracing::info!("Resumed");
    }
}
