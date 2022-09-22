use crate::tasks;
use serenity::{async_trait, model::prelude::*, prelude::*};

mod bans;
mod cache;
mod join_msg;
mod member_log;
mod mention;
mod mod_log;
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
        tasks::start(&ctx).await;
        bans::start(&ctx, ready.guilds.iter().map(|g| g.id).collect::<Vec<_>>()).await;
    }

    async fn cache_ready(&self, _ctx: Context, _guild_ids: Vec<GuildId>) {
        tracing::info!("Cache ready!");
    }

    async fn resume(&self, ctx: Context, _: ResumedEvent) {
        tracing::info!("Resumed");
        ctx.set_activity(Activity::playing("sushii.xyz")).await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        tokio::join!(
            roles::message(&ctx, &msg),
            // Disable user levels handler, now handled in sushii-ts-services
            // user_levels::message(&ctx, &msg),
            msg_log::message(&ctx, &msg),
            mention::message(&ctx, &msg),
            cache::cache_user::message(&ctx, &msg),
            notification::message(&ctx, &msg),
        );
    }

    async fn message_delete(
        &self,
        ctx: Context,
        channel_id: ChannelId,
        msg_id: MessageId,
        guild_id: Option<GuildId>,
    ) {
        msg_log::message_delete(&ctx, channel_id, msg_id, guild_id).await;
    }

    async fn message_delete_bulk(
        &self,
        ctx: Context,
        channel_id: ChannelId,
        msg_ids: Vec<MessageId>,
        guild_id: Option<GuildId>,
    ) {
        msg_log::message_delete_bulk(&ctx, channel_id, msg_ids, guild_id).await;
    }

    async fn message_update(
        &self,
        ctx: Context,
        old_msg: Option<Message>,
        new_msg: Option<Message>,
        event: MessageUpdateEvent,
    ) {
        msg_log::message_update(&ctx, &old_msg, &new_msg, &event).await;
    }

    async fn guild_ban_addition(&self, ctx: Context, guild_id: GuildId, banned_user: User) {
        mod_log::ban::guild_ban_addition(&ctx, &guild_id, &banned_user).await;
        bans::guild_ban_addition(&ctx, guild_id, &banned_user).await;
    }

    async fn guild_ban_removal(&self, ctx: Context, guild_id: GuildId, unbanned_user: User) {
        mod_log::ban::guild_ban_removal(&ctx, &guild_id, &unbanned_user).await;
        bans::guild_ban_removal(&ctx, guild_id, &unbanned_user).await;
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: bool) {
        cache::cache_guild::guild_create(&ctx, &guild, is_new).await;
        bans::guild_create(&ctx, &guild, is_new).await;
    }

    async fn guild_update(
        &self,
        ctx: Context,
        old_guild_if_avail: Option<Guild>,
        partial_guild: PartialGuild,
    ) {
        cache::cache_guild::guild_update(&ctx, &old_guild_if_avail, &partial_guild).await;
    }

    async fn guild_member_update(
        &self,
        ctx: Context,
        old_member: Option<Member>,
        new_member: Member,
    ) {
        mod_log::mute::guild_member_update(&ctx, &old_member, &new_member).await;
    }

    async fn guild_member_addition(&self, ctx: Context, mut member: Member) {
        // TODO: Run these concurrently instead of one by one
        mod_log::mute::guild_member_addition(&ctx, member.guild_id, &mut member).await;

        tokio::join!(
            join_msg::guild_member_addition(&ctx, &member.guild_id, &member),
            member_log::guild_member_addition(&ctx, &member.guild_id, &member),
        );
    }

    async fn guild_member_removal(
        &self,
        ctx: Context,
        guild_id: GuildId,
        user: User,
        member: Option<Member>,
    ) {
        member_log::guild_member_removal(&ctx, &guild_id, &user, &member).await;
    }

    async fn interaction_create(&self, _ctx: Context, interaction: Interaction) {
        tracing::debug!(?interaction, "Received interaction");
    }
}
