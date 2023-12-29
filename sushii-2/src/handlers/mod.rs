use crate::tasks;
use serenity::{async_trait, model::prelude::*, prelude::*};

mod bans;
mod cache;
mod join_msg;
mod member_log;
mod mention;
mod msg_log;
mod notification;
mod raw_event_handler;
mod roles;
mod user_levels;

pub use raw_event_handler::RawHandler;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        tracing::info!("Connected as {}", ready.user.name);
        ctx.set_activity(Activity::playing("sushii.xyz")).await;

        // Start tasks and ban fetching
        // These both only run once even if ready is called multiple times
        // This is here instead of cache_ready as a single unavailable guild will
        // prevent any of it from starting
        // tasks::start(&ctx).await;

        // Disabled ban fetching, now handled in sushii-ts-services
        // bans::start(&ctx, ready.guilds.iter().map(|g| g.id).collect::<Vec<_>>()).await;
    }

    async fn cache_ready(&self, _ctx: Context, _guild_ids: Vec<GuildId>) {
        tracing::info!("Cache ready!");
    }

    async fn resume(&self, ctx: Context, _: ResumedEvent) {
        tracing::info!("Resumed");
        ctx.set_activity(Activity::playing("sushii.xyz")).await;
    }
}
