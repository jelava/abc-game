use actix_web::error::ResponseError;
use derive_more::{Display, Error};
use std::sync::TryLockError;

#[derive(Debug, Display, Error)]
pub enum Error {
    #[display(fmt = "An internal error occurred on the server.")]
    Internal(InternalError),

    #[display(fmt = "There is no game with ID {}.", "_0")]
    NonexistentGameId(#[error(not(source))] usize),

    #[display(fmt = "There is no user with ID {}.", "_0")]
    NonexistentUserId(#[error(not(source))] usize)
}

impl ResponseError for Error {
    // TODO?
}

#[derive(Debug, Display, Error)]
pub enum InternalError {
    PoisonedMutex
}

impl<T> From<TryLockError<T>> for Error {
    fn from(_: TryLockError<T>) -> Self {
        Error::Internal(InternalError::PoisonedMutex)
    }
}
