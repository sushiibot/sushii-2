use language_api_wrapper::error::Error as LanguageApiError;
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
    #[error("Guild config does not have {0:?} set")]
    ConfigMissingField(&'static str),
    #[error("Unknown data store error")]
    Unknown,
    #[error("Invalid event constraint, {0:?} is not applicable to event {1:?}")]
    InvalidEventConstraint(&'static str, Trigger),
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
}
