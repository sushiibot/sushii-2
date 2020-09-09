use serenity::{async_trait, model::prelude::*, prelude::*};

pub mod mod_log;
pub mod raw_event_handler;
pub mod roles;

pub use raw_event_handler::RawHandler;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        tracing::info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        tracing::info!("Resumed");
    }

    async fn message(&self, ctx: Context, msg: Message) {
        roles::message(&ctx, &msg).await;
    }

    async fn guild_ban_addition(&self, ctx: Context, guild_id: GuildId, banned_user: User) {
        mod_log::ban::guild_ban_addition(&ctx, &guild_id, &banned_user).await;
    }

    async fn guild_ban_removal(&self, ctx: Context, guild_id: GuildId, unbanned_user: User) {
        mod_log::unban::guild_ban_removal(&ctx, &guild_id, &unbanned_user).await;
    }
}
