use color_eyre::eyre::Report;
use thiserror::Error;
use std::io::Error as IoError;
use sqlx::error::Error as SqlxError;
use serde_json::error::Error as SerdeError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("IO error")]
    IoError(#[from] IoError),

    #[error("User nor found error")]
    UserNotFound,

    #[error("SqlxError")]
    SqlxError(#[from] SqlxError),

    #[error("SerdeError")]
    SerdeError(#[from] SerdeError),

    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}