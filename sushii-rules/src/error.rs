use std::result::Result as StdResult;
use thiserror::Error as ThisError;

pub type Result<T> = StdResult<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Missing channel ID in context data")]
    MissingChannelId,
    #[error("unknown data store error")]
    Unknown,
}
