use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("User not found: {0}")]
    UserNotFound(String),
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Post with id {0} not found")]
    PostNotFound(i64),
    #[error("Permission denied")]
    Forbidden,
    #[error("Database error: {0}")]
    Db(#[from] sqlx::Error),
    #[error("Internal error")]
    InternalError,
}
