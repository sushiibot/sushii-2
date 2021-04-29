use language_api_wrapper::error::Error as LanguageApiError;
use std::result::Result as StdResult;
use sushii_model::Error as SushiiModelError;
use thiserror::Error as ThisError;

pub type Result<T> = StdResult<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Event is missing channel ID")]
    MissingChannelId,
    #[error("Event is missing user ID")]
    MissingUserId,
    #[error("Event is missing server ID")]
    MissingGuildId,
    #[error("unknown data store error")]
    Unknown,
    #[error(transparent)]
    LanguageApi(#[from] LanguageApiError),
    #[error("Failed to deserialize event `{0}`, {1}")]
    EventDeserialize(String, serde_json::Error),
    #[error(transparent)]
    SushiiModel(#[from] SushiiModelError),
}
