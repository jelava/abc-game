use actix_web::error::ResponseError;
use derive_more::{Display, Error};
use futures::channel::mpsc::TrySendError;

#[derive(Clone, Copy, Debug, Display, Error)]
pub enum Error {
    #[display(fmt = "An internal error occurred on the server.")]
    Internal(InternalError),

    #[display(fmt = "There is no game with the requested ID.")]
    NonexistentGameId,

    #[display(fmt = "There is no user with the requested ID.")]
    NonexistentUserId,
}

impl ResponseError for Error {
    // TODO?
}

#[derive(Clone, Copy, Debug, Display, Error)]
pub enum InternalError {
    DuplicateGameId,
    DuplicateUserId,
    NoAvailableIds,
    PoisonedMutex,
    SseTrySendError
}

impl<T> From<TrySendError<T>> for Error {
    fn from(_: TrySendError<T>) -> Self {
        Self::Internal(InternalError::SseTrySendError)
    }
}
