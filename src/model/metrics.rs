use chrono::naive::NaiveDateTime;
use chrono::offset::Utc;
use prometheus::{IntCounterVec, Opts, Registry};
use prometheus_static_metric::make_static_metric;
use serenity::{async_trait, model::prelude::*, prelude::*};

make_static_metric! {
    pub label_enum UserType {
        user,
        other_bot,
        own,
    }

    pub label_enum EventType {
        channel_create,
        channel_delete,
        channel_pins_update,
        channel_update,
        guild_ban_add,
        guild_ban_remove,
        guild_create,
        guild_delete,
        guild_emojis_update,
        guild_integrations_update,
        guild_member_add,
        guild_member_remove,
        guild_member_update,
        guild_members_chunk,
        guild_role_create,
        guild_role_delete,
        guild_role_update,
        guild_unavailable,
        guild_update,
        message_create,
        message_delete,
        message_delete_bulk,
        message_update,
        presence_update,
        presences_replace,
        reaction_add,
        reaction_remove,
        reaction_remove_all,
        ready,
        resumed,
        typing_start,
        user_update,
        voice_state_update,
        voice_server_update,
        webhook_update,
        unknown,
    }

    pub struct MessageCounterVec: IntCounter {
        "user_type" => UserType,
    }

    pub struct EventCounterVec: IntCounter {
        "event_type" => EventType,
    }
}

#[async_trait]
pub trait MetricsAsync {
    // Need our own trait since serenity's RawEventHandler doesn't use references
    async fn raw_event(&self, ctx: &Context, event: &Event);
}

pub struct Metrics {
    pub registry: Registry,
    pub start_time: NaiveDateTime,
    pub messages: MessageCounterVec,
    pub events: EventCounterVec,
}

impl Metrics {
    pub fn new() -> Self {
        let messages_vec =
            IntCounterVec::new(Opts::new("messages", "Recieved messages"), &["user_type"]).unwrap();
        let messages_static_vec = MessageCounterVec::from(&messages_vec);

        let events_vec =
            IntCounterVec::new(Opts::new("events", "Gateway events"), &["event_type"]).unwrap();
        let events_static_vec = EventCounterVec::from(&events_vec);

        let registry = Registry::new_custom(Some("sushii".into()), None).unwrap();
        registry.register(Box::new(messages_vec)).unwrap();
        registry.register(Box::new(events_vec)).unwrap();

        Self {
            registry,
            start_time: Utc::now().naive_local(),
            messages: messages_static_vec,
            events: events_static_vec,
        }
    }
}

#[async_trait]
impl MetricsAsync for Metrics {
    async fn raw_event(&self, ctx: &Context, event: &Event) {
        match event {
            Event::MessageCreate(MessageCreateEvent { message, .. }) => {
                // Regular user
                if !message.author.bot {
                    self.messages.user.inc();
                // Sushii messages
                } else if message.is_own(&ctx.cache).await {
                    self.messages.own.inc();
                // Other bot messages
                } else {
                    self.messages.other_bot.inc();
                }
            }
            _ => {}
        }
    }
}
