use actix_web::{web::Bytes, ResponseError};
use derive_more::{Display, Error, From};
use futures::channel::mpsc::TrySendError;
use serde_json;

#[derive(Debug, Display, Error, From)]
pub enum Error {
    #[display(fmt = "There is no object with the ID {}.", _0)]
    NonexistentId(#[error(ignore)] usize),

    #[display(fmt = "An internal error occurred on the server.")]
    Internal(InternalError),
}

impl ResponseError for Error {
    // TODO?
}

#[derive(Debug, Display, Error, From)]
pub enum InternalError {
    DuplicateId(#[error(ignore)] usize),
    SseTrySendError(TrySendError<Bytes>),
    JsonSerializationError(serde_json::Error),
}

impl From<TrySendError<Bytes>> for Error {
    fn from(error: TrySendError<Bytes>) -> Self {
        Self::Internal(InternalError::SseTrySendError(error))
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::Internal(InternalError::JsonSerializationError(error))
    }
}
