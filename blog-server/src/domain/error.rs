use actix_web::{HttpResponse, ResponseError, http::StatusCode};
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
    #[error("Authentication required")]
    Unauthenticated,
    #[error("Database error: {0}")]
    Db(#[from] sqlx::Error),
    #[error("Internal error")]
    InternalError,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::UserNotFound(_) => StatusCode::NOT_FOUND,
            AppError::UserAlreadyExists => StatusCode::CONFLICT,
            AppError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            AppError::PostNotFound(_) => StatusCode::NOT_FOUND,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::Unauthenticated => StatusCode::UNAUTHORIZED,
            AppError::Db(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        let body = match self {
            AppError::UserNotFound(username) => format!("User not found: {}", username),
            AppError::PostNotFound(id) => format!("Post with id {} not found", id),
            AppError::UserAlreadyExists => "User already exists".to_string(),
            AppError::InvalidCredentials => "Invalid credentials".to_string(),
            AppError::Forbidden => "Permission denied".to_string(),
            AppError::Unauthenticated => "Authentication required".to_string(),
            AppError::Db(e) => {
                tracing::error!("Database error: {}", e);
                "Internal server error".to_string()
            }
            AppError::InternalError => {
                tracing::error!("Internal error occurred: {}", self);
                "Internal server error".to_string()
            }
        };

        HttpResponse::build(status).body(body)
    }
}

impl From<AppError> for tonic::Status {
    fn from(error: AppError) -> Self {
        match error {
            AppError::UserNotFound(_) | AppError::PostNotFound(_) => {
                Self::not_found(error.to_string())
            }
            AppError::UserAlreadyExists => Self::already_exists("User already exists"),
            AppError::InvalidCredentials => Self::unauthenticated("Invalid credentials"),
            AppError::Forbidden => Self::permission_denied("Permission denied"),
            AppError::Unauthenticated => Self::unauthenticated("Authentication required"),
            AppError::Db(e) => {
                tracing::error!("Database error: {}", e);
                Self::internal("Database error")
            }
            AppError::InternalError => {
                tracing::error!("Internal error occurred: {}", error);
                Self::internal("Internal error")
            }
        }
    }
}
