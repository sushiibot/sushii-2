use lapin::Error as LapinError;
use std::borrow::Cow;
use std::num::TryFromIntError;
use std::result::Result as StdResult;
use thiserror::Error as ThisError;
use twilight_http::error::Error as TwilightHttpError;
use twilight_http::response::DeserializeBodyError;

pub type Result<T> = StdResult<T, Error>;

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
    #[error("Unsupported gateway event")]
    UnsupportedEvent,
    #[error("Failed to deserialize event `{0}`, {1}")]
    EventDeserialize(String, serde_json::Error),
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
    TwilightHttp(#[from] TwilightHttpError),
    #[error(transparent)]
    TwilightDeserializeBodyError(#[from] DeserializeBodyError),
}
