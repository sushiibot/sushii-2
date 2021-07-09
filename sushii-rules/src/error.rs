use language_api_wrapper::error::Error as LanguageApiError;
use lapin::Error as LapinError;
use std::borrow::Cow;
use std::num::TryFromIntError;
use std::result::Result as StdResult;
use sushii_model::Error as SushiiModelError;
use thiserror::Error as ThisError;

use crate::model::Trigger;

pub type Result<T> = StdResult<T, Error>;

/// Errors from triggering a rule
#[derive(ThisError, Debug)]
pub enum RuleError {
    #[error("Event is missing channel ID")]
    MissingChannelId,
    #[error("Event is missing user ID")]
    MissingUserId,
    #[error("Event is missing server ID")]
    MissingGuildId,
    #[error("Event is missing message ID")]
    MissingMessageId,
    #[error("Event is missing member data")]
    MissingMember,
    #[error("Unknown data store error")]
    Unknown,
    #[error("Invalid event constraint, {0:?} is not applicable to event {1:?}")]
    InvalidEventConstraint(&'static str, Trigger),
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Event is missing channel ID")]
    MissingChannelId,
    #[error("Event is missing user ID")]
    MissingUserId,
    #[error("Event is missing server ID")]
    MissingGuildId,
    #[error("Event is missing message ID")]
    MissingMessageId,
    #[error("Event is missing member data")]
    MissingMember,
    #[error("Rule config does not have {0:?} set")]
    RuleConfigMissingField(Cow<'static, str>),
    #[error("Rule config field {0:?} is not the correct type {1:?}")]
    RuleConfigMismatchedType(Cow<'static, str>, Cow<'static, str>),
    #[error("Guild config does not have {0:?} set")]
    GuildConfigMissingField(Cow<'static, str>),
    #[error("Unknown data store error")]
    Unknown,
    #[error("Unknown word list {0:?}")]
    UnknownWordList(Cow<'static, str>),
    #[error("There are no word lists added")]
    NoWordLists,
    #[error("Invalid event constraint, {0:?} is not applicable to event {1:?}")]
    InvalidEventConstraint(&'static str, Trigger),
    #[error("Unsupported gateway event")]
    UnsupportedEvent,
    #[error(transparent)]
    LanguageApi(#[from] LanguageApiError),
    #[error("Failed to deserialize event `{0}`, {1}")]
    EventDeserialize(String, serde_json::Error),
    #[error(transparent)]
    SushiiModel(#[from] SushiiModelError),
    #[error("Failed to convert from integer {0}")]
    TryFromInt(#[from] TryFromIntError),
    #[error("Failed to parse DateTime {0}")]
    DateTimeParse(#[from] chrono::ParseError),
    #[error(transparent)]
    Lapin(#[from] LapinError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    Config(#[from] config::ConfigError),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    RedisPool(#[from] deadpool_redis::PoolError),
    #[error(transparent)]
    Redis(#[from] redis::RedisError),
}
