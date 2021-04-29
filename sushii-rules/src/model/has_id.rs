use twilight_model::gateway::event::DispatchEvent;
use twilight_model::gateway::payload;
use twilight_model::id::{ChannelId, GuildId, UserId};

pub trait HasGuildId {
    fn guild_id(&self) -> Option<GuildId>;
}

impl HasGuildId for DispatchEvent {
    fn guild_id(&self) -> Option<GuildId> {
        match *self {
            Self::BanAdd(payload::BanAdd { guild_id, .. }) => Some(guild_id),
            Self::BanRemove(payload::BanRemove { guild_id, .. }) => Some(guild_id),
            Self::MessageCreate(ref msg) => msg.guild_id,
            Self::MemberAdd(ref member) => Some(member.guild_id),
            _ => None,
        }
    }
}

pub trait HasChannelId {
    fn channel_id(&self) -> Option<ChannelId>;
}

impl HasChannelId for DispatchEvent {
    fn channel_id(&self) -> Option<ChannelId> {
        match *self {
            Self::MessageCreate(ref msg) => Some(msg.channel_id),
            _ => None,
        }
    }
}

pub trait HasUserId {
    fn user_id(&self) -> Option<UserId>;
}

impl HasUserId for DispatchEvent {
    fn user_id(&self) -> Option<UserId> {
        match *self {
            Self::MessageCreate(ref msg) => Some(msg.author.id),
            _ => None,
        }
    }
}
