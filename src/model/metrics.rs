use metrics::{
    counter, decrement_gauge, increment_gauge,
    register_counter, register_gauge,
};
use metrics_exporter_prometheus::PrometheusBuilder;
use serenity::{model::prelude::*, prelude::*};
use std::net::SocketAddr;

use crate::SushiiConfig;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum UserType {
    user,
    other_bot,
    own,
}

impl UserType {
    fn as_str(&self) -> &'static str {
        match *self {
            Self::user => "user",
            Self::other_bot => "other_bot",
            Self::own => "own",
        }
    }
}

pub struct Metrics;

impl Metrics {
    pub fn new(conf: &SushiiConfig) -> Self {
        let addr: SocketAddr = (conf.metrics_interface, conf.metrics_port).into();
        tracing::info!("Metrics server listening on http://{}", addr);

        // Start metrics server
        PrometheusBuilder::new()
            .listen_address(addr)
            .install()
            .expect("Failed to install Prometheus recorder");

        register_counter!("sushii_messages", "number of messages received");
        register_counter!("sushii_events", "number of events received");
        register_gauge!("sushii_guilds_total", "number of total guilds");
        register_gauge!("sushii_members_total", "number of total members");

        Self
    }

    pub async fn raw_event(&self, ctx: &Context, event: &Event) {
        match event {
            Event::MessageCreate(MessageCreateEvent { message, .. }) => {
                // Regular user
                if !message.author.bot {
                    counter!("sushii_messages", 1, "user_type" => UserType::user.as_str());
                // Sushii messages
                } else if message.is_own(&ctx.cache).await {
                    counter!("sushii_messages", 1, "user_type" => UserType::own.as_str());
                // Other bot messages
                } else {
                    counter!("sushii_messages", 1, "user_type" => UserType::other_bot.as_str());
                }
            }
            Event::GuildCreate(GuildCreateEvent { guild, .. }) => {
                increment_gauge!("sushii_guilds_total", 1.0);
                increment_gauge!("sushii_members_total", guild.member_count as f64);
            }
            Event::GuildDelete(_) => {
                decrement_gauge!("sushii_guilds_total", 1.0);

                // self.members stale value,
                // don't have the guild anymore so don't know how many to sub()
            }
            Event::GuildMemberAdd(_) => {
                increment_gauge!("sushii_members_total", 1.0);
            }
            Event::GuildMemberRemove(_) => {
                decrement_gauge!("sushii_members_total", 1.0);
            }
            _ => {}
        }

        // EventType::name() has weird stuff with 'static borrows, couldn't figure it out
        let event_name = match event.event_type() {
            EventType::ChannelCreate => "CHANNEL_CREATE",
            EventType::ChannelDelete => "CHANNEL_DELETE",
            EventType::ChannelPinsUpdate => "CHANNEL_PINS_UPDATE",
            EventType::ChannelUpdate => "CHANNEL_UPDATE",
            EventType::GuildBanAdd => "GUILD_BAN_ADD",
            EventType::GuildBanRemove => "GUILD_BAN_REMOVE",
            EventType::GuildCreate => "GUILD_CREATE",
            EventType::GuildDelete => "GUILD_DELETE",
            EventType::GuildEmojisUpdate => "GUILD_EMOJIS_UPDATE",
            EventType::GuildIntegrationsUpdate => "GUILD_INTEGRATIONS_UPDATE",
            EventType::GuildMemberAdd => "GUILD_MEMBER_ADD",
            EventType::GuildMemberRemove => "GUILD_MEMBER_REMOVE",
            EventType::GuildMemberUpdate => "GUILD_MEMBER_UPDATE",
            EventType::GuildMembersChunk => "GUILD_MEMBERS_CHUNK",
            EventType::GuildRoleCreate => "GUILD_ROLE_CREATE",
            EventType::GuildRoleDelete => "GUILD_ROLE_DELETE",
            EventType::GuildRoleUpdate => "GUILD_ROLE_UPDATE",
            EventType::InviteCreate => "INVITE_CREATE",
            EventType::InviteDelete => "INVITE_DELETE",
            EventType::GuildUpdate => "GUILD_UPDATE",
            EventType::MessageCreate => "MESSAGE_CREATE",
            EventType::MessageDelete => "MESSAGE_DELETE",
            EventType::MessageDeleteBulk => "MESSAGE_DELETE_BULK",
            EventType::ReactionAdd => "MESSAGE_REACTION_ADD",
            EventType::ReactionRemove => "MESSAGE_REACTION_REMOVE",
            EventType::ReactionRemoveAll => "MESSAGE_REACTION_REMOVE_ALL",
            EventType::MessageUpdate => "MESSAGE_UPDATE",
            EventType::PresenceUpdate => "PRESENCE_UPDATE",
            EventType::PresencesReplace => "PRESENCES_REPLACE",
            EventType::Ready => "READY",
            EventType::Resumed => "RESUMED",
            EventType::TypingStart => "TYPING_START",
            EventType::UserUpdate => "USER_UPDATE",
            EventType::VoiceServerUpdate => "VOICE_SERVER_UPDATE",
            EventType::VoiceStateUpdate => "VOICE_STATE_UPDATE",
            EventType::WebhookUpdate => "WEBHOOKS_UPDATE",
            // Not actually a Discord event name
            EventType::GuildUnavailable => "GUILD_UNAVAILABLE",
            _ => "UNKNOWN",
        };

        counter!("sushii_events", 1, "event_type" => event_name);
    }
}
