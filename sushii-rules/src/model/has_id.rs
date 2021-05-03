use sushii_model::model::sql::RuleScope;
use twilight_model::gateway::event::DispatchEvent;
use twilight_model::gateway::payload;
use twilight_model::{
    channel::message::Message,
    id::{ChannelId, GuildId, MessageId, UserId},
    user::User,
};

use crate::error::{Error, Result};
use crate::model::Event;

pub trait HasScopeId {
    fn scope_id(&self, scope: RuleScope) -> Result<u64>;
}

impl HasScopeId for Event {
    fn scope_id(&self, scope: RuleScope) -> Result<u64> {
        match self {
            Self::Twilight(event) => event.scope_id(scope),
            Self::Counter { counter, .. } => Ok(counter.scope_id as u64),
            // Scope ID for level up is the user ID i guess?
            Self::LevelUp { user_id, .. } => Ok(*user_id),
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

impl HasScopeId for Message {
    fn scope_id(&self, scope: RuleScope) -> Result<u64> {
        match scope {
            RuleScope::Guild => self.guild_id.map(|id| id.0).ok_or(Error::MissingGuildId),
            RuleScope::Channel => Ok(self.channel_id.0),
            RuleScope::User => Ok(self.author.id.0),
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
            Self::Counter { counter, .. } => Ok(GuildId(counter.guild_id as u64)),
            Self::LevelUp { message, .. } => message.guild_id.ok_or(Error::MissingGuildId),
        }
    }
}

impl HasGuildId for DispatchEvent {
    fn guild_id(&self) -> Result<GuildId> {
        match *self {
            Self::BanAdd(payload::BanAdd { guild_id, .. }) => Ok(guild_id),
            Self::BanRemove(payload::BanRemove { guild_id, .. }) => Ok(guild_id),
            // Error since we only consider messages in guilds, if it's missing
            // it's a DM
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
            Self::Counter { original_event, .. } => original_event.channel_id(),
            Self::LevelUp { message, .. } => Ok(message.channel_id),
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
            Self::Counter { original_event, .. } => original_event.message_id(),
            Self::LevelUp { message, .. } => Ok(message.id),
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
            Self::Counter { original_event, .. } => original_event.user_id(),
            Self::LevelUp { message, .. } => Ok(message.author.id),
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

pub trait HasUser {
    fn user(&self) -> Result<&User>;
}

impl HasUser for Event {
    fn user(&self) -> Result<&User> {
        match self {
            Self::Twilight(event) => event.user(),
            Self::Counter { original_event, .. } => original_event.user(),
            Self::LevelUp { message, .. } => Ok(&message.author),
        }
    }
}

impl HasUser for DispatchEvent {
    fn user(&self) -> Result<&User> {
        match *self {
            Self::MessageCreate(ref msg) => Ok(&msg.author),
            _ => Err(Error::MissingUserId),
        }
    }
}
