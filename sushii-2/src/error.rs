use config::ConfigError;
use dotenv::Error as DotenvError;
use humantime::DurationError;
use serde_json::Error as SerdeJsonError;
use serenity::model::timestamp::InvalidTimestamp;
use serenity::Error as SerenityError;
use sqlx::migrate::MigrateError;
use sqlx::Error as SqlxError;
use std::env::VarError;
use std::fmt::Error as FmtError;
use std::io::Error as IoError;
use std::result::Result as StdResult;
use sushii_model::Error as SushiiModelError;
use thiserror::Error as ThisError;

pub type Result<T> = StdResult<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    // Sushii errors
    #[error("{0}")]
    Sushii(String),
    #[error(transparent)]
    Model(#[from] SushiiModelError),
    // Crate errors
    #[error(transparent)]
    Dotenv(#[from] DotenvError),
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
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error(transparent)]
    InvalidTimestamp(#[from] InvalidTimestamp),
}
