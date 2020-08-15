use serenity::{async_trait, model::prelude::*, prelude::*};

pub mod mod_log;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        tracing::info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        tracing::info!("Resumed");
    }

    async fn guild_ban_addition(&self, _ctx: Context, _guild_id: GuildId, _banned_user: User) {}
}
