use thiserror::Error as ThisError;
use twilight_http::error::Error as TwilightHttpError;
use twilight_http::request::channel::message::create_message::CreateMessageError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Invalid ID")]
    InvalidId,
    #[error(transparent)]
    TwilightHttp(#[from] TwilightHttpError),
    #[error(transparent)]
    TwilightCreateMessageError(#[from] CreateMessageError)
}

impl warp::reject::Reject for Error {}
