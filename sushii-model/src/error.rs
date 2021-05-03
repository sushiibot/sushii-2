use base64::DecodeError;
use humantime::DurationError;
use serde_json::Error as SerdeJsonError;
use serenity::Error as SerenityError;
use sqlx::migrate::MigrateError;
use sqlx::Error as SqlxError;
use std::env::VarError;
use std::error::Error as StdError;
use std::fmt::Error as FmtError;
use std::io::Error as IoError;
use std::result::Result as StdResult;
use thiserror::Error as ThisError;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug, ThisError)]
pub enum Error {
    // Sushii errors
    #[error("{0}")]
    Sushii(String),
    #[error(transparent)]
    PgInterval(Box<dyn StdError + Sync + Send>),
    // Crate errors
    #[error(transparent)]
    Decode(#[from] DecodeError),
    #[error(transparent)]
    Fmt(#[from] FmtError),
    #[error(transparent)]
    Io(#[from] IoError),
    #[error(transparent)]
    Json(#[from] SerdeJsonError),
    #[error(transparent)]
    Serenity(#[from] SerenityError),
    #[error(transparent)]
    Sqlx(#[from] SqlxError),
    #[error(transparent)]
    Migrate(#[from] MigrateError),
    #[error(transparent)]
    Var(#[from] VarError),
    #[error(transparent)]
    Duration(#[from] DurationError),
}
