use humantime::DurationError;
use serde_json::Error as SerdeJsonError;
use serenity::Error as SerenityError;
use sqlx::migrate::MigrateError;
use sqlx::Error as SqlxError;
use std::env::VarError;
use std::error::Error as StdError;
use std::fmt::{Display, Error as FmtError, Formatter, Result as FmtResult};
use std::io::Error as IoError;
use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    // Sushii errors
    Sushii(String),
    // Crate errors
    Fmt(FmtError),
    Io(IoError),
    Json(SerdeJsonError),
    Serenity(SerenityError),
    Sqlx(SqlxError),
    Migrate(MigrateError),
    Var(VarError),
    Duration(DurationError),
}

impl From<SerdeJsonError> for Error {
    fn from(err: SerdeJsonError) -> Error {
        Error::Json(err)
    }
}

impl From<SerenityError> for Error {
    fn from(err: SerenityError) -> Error {
        Error::Serenity(err)
    }
}

impl From<FmtError> for Error {
    fn from(err: FmtError) -> Error {
        Error::Fmt(err)
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Io(err)
    }
}

impl From<VarError> for Error {
    fn from(err: VarError) -> Error {
        Error::Var(err)
    }
}

impl From<SqlxError> for Error {
    fn from(err: SqlxError) -> Error {
        Error::Sqlx(err)
    }
}

impl From<MigrateError> for Error {
    fn from(err: MigrateError) -> Error {
        Error::Migrate(err)
    }
}

impl From<DurationError> for Error {
    fn from(err: DurationError) -> Error {
        Error::Duration(err)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            // Sushii
            Error::Sushii(ref inner) => inner.fmt(f),
            // Crates
            Error::Fmt(ref inner) => inner.fmt(f),
            Error::Io(ref inner) => inner.fmt(f),
            Error::Json(ref inner) => inner.fmt(f),
            Error::Serenity(ref inner) => inner.fmt(f),
            Error::Sqlx(ref inner) => inner.fmt(f),
            Error::Migrate(ref inner) => inner.fmt(f),
            Error::Var(ref inner) => inner.fmt(f),
            Error::Duration(ref inner) => inner.fmt(f),
        }
    }
}
