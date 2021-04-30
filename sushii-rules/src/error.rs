use language_api_wrapper::error::Error as LanguageApiError;
use std::result::Result as StdResult;
use sushii_model::Error as SushiiModelError;
use thiserror::Error as ThisError;

use crate::model::Trigger;

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
}
