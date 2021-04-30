use sushii_model::model::sql::RuleScope;
use twilight_model::gateway::event::DispatchEvent;
use twilight_model::gateway::payload;
use twilight_model::id::{ChannelId, GuildId, MessageId, UserId};

use crate::error::{Error, Result};
use crate::model::Event;

pub trait HasScopeId {
    fn scope_id(&self, scope: RuleScope) -> Result<u64>;
}

impl HasScopeId for Event {
    fn scope_id(&self, scope: RuleScope) -> Result<u64> {
        match self {
            Self::Twilight(event) => event.scope_id(scope),
            Self::Counter(counter, ..) => Ok(counter.scope_id as u64),
        }
    }
}

impl HasScopeId for DispatchEvent {
    fn scope_id(&self, scope: RuleScope) -> Result<u64> {
        match scope {
            RuleScope::Guild => self.guild_id().map(|id| id.0),
            RuleScope::Channel => self.channel_id().map(|id| id.0),
            RuleScope::User => self.user_id().map(|id| id.0),
        }
    }
}

pub trait HasGuildId {
    fn guild_id(&self) -> Result<GuildId>;
}

impl HasGuildId for Event {
    fn guild_id(&self) -> Result<GuildId> {
        match self {
            Self::Twilight(event) => event.guild_id(),
            Self::Counter(counter, ..) => Ok(GuildId(counter.guild_id as u64)),
        }
    }
}

impl HasGuildId for DispatchEvent {
    fn guild_id(&self) -> Result<GuildId> {
        match *self {
            Self::BanAdd(payload::BanAdd { guild_id, .. }) => Ok(guild_id),
            Self::BanRemove(payload::BanRemove { guild_id, .. }) => Ok(guild_id),
            Self::MessageCreate(ref msg) => msg.guild_id.ok_or(Error::MissingGuildId),
            Self::MemberAdd(ref member) => Ok(member.guild_id),
            _ => Err(Error::MissingGuildId),
        }
    }
}

pub trait HasChannelId {
    fn channel_id(&self) -> Result<ChannelId>;
}

impl HasChannelId for Event {
    fn channel_id(&self) -> Result<ChannelId> {
        match self {
            Self::Twilight(event) => event.channel_id(),
            Self::Counter(_counter, event) => event.channel_id(),
        }
    }
}

impl HasChannelId for DispatchEvent {
    fn channel_id(&self) -> Result<ChannelId> {
        match *self {
            Self::MessageCreate(ref msg) => Ok(msg.channel_id),
            _ => Err(Error::MissingChannelId),
        }
    }
}

pub trait HasMessageId {
    fn message_id(&self) -> Result<MessageId>;
}

impl HasMessageId for Event {
    fn message_id(&self) -> Result<MessageId> {
        match self {
            Self::Twilight(event) => event.message_id(),
            Self::Counter(_counter, event) => event.message_id(),
        }
    }
}

impl HasMessageId for DispatchEvent {
    fn message_id(&self) -> Result<MessageId> {
        match *self {
            Self::MessageCreate(ref msg) => Ok(msg.id),
            _ => Err(Error::MissingMessageId),
        }
    }
}

pub trait HasUserId {
    fn user_id(&self) -> Result<UserId>;
}

impl HasUserId for Event {
    fn user_id(&self) -> Result<UserId> {
        match self {
            Self::Twilight(event) => event.user_id(),
            Self::Counter(_counter, event) => event.user_id(),
        }
    }
}

impl HasUserId for DispatchEvent {
    fn user_id(&self) -> Result<UserId> {
        match *self {
            Self::MessageCreate(ref msg) => Ok(msg.author.id),
            _ => Err(Error::MissingUserId),
        }
    }
}
