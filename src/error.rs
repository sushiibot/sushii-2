use dotenv::Error as DotenvError;
use sqlx::Error as SqlxError;
use std::env::VarError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;
use std::result::Result as StdResult;
use twilight::gateway::cluster::error::Error as GatewayError;
use twilight::http::error::Error as TwilightError;
use twilight::http::request::channel::message::{
    create_message::CreateMessageError,
    get_channel_messages::GetChannelMessagesError
};

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    // Sushii errors
    Sushii(String),
    UserError(String),
    IsBot,
    // Crate errors
    Dotenv(DotenvError),
    /// `env::VarError`
    Var(VarError),
    /// `std::io` error
    Io(IoError),
    /// `sqlx` error
    Sqlx(SqlxError),
    /// `twilight` error
    Twilight(TwilightError),
    CreateMessage(CreateMessageError),
    GetChannelMessages(GetChannelMessagesError),
    Gateway(GatewayError),
}

impl From<DotenvError> for Error {
    fn from(err: DotenvError) -> Error {
        Error::Dotenv(err)
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

impl From<TwilightError> for Error {
    fn from(err: TwilightError) -> Error {
        Error::Twilight(err)
    }
}

impl From<GatewayError> for Error {
    fn from(err: GatewayError) -> Error {
        Error::Gateway(err)
    }
}

impl From<CreateMessageError> for Error {
    fn from(err: CreateMessageError) -> Error {
        Error::CreateMessage(err)
    }
}

impl From<GetChannelMessagesError> for Error {
    fn from(err: GetChannelMessagesError) -> Error {
        Error::GetChannelMessages(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            // Sushii
            Error::IsBot => write!(f, "IsBot"),
            Error::Sushii(ref inner) => inner.fmt(f),
            Error::UserError(ref inner) => inner.fmt(f),
            // Crates
            Error::Dotenv(ref inner) => inner.fmt(f),
            Error::Io(ref inner) => inner.fmt(f),
            Error::Sqlx(ref inner) => inner.fmt(f),
            Error::Var(ref inner) => inner.fmt(f),
            // Twilight
            Error::Twilight(ref inner) => inner.fmt(f),
            Error::CreateMessage(ref inner) => inner.fmt(f),
            Error::Gateway(ref inner) => inner.fmt(f),
            Error::GetChannelMessages(ref inner) => inner.fmt(f),
        }
    }
}
