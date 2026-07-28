use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlogClientError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("gRPC error: {0}")]
    Grpc(tonic::Status),
    #[error("Transport error: {0}")]
    Transport(#[from] tonic::transport::Error),
    #[error("Invalid URI: {0}")]
    InvalidUri(#[from] http::uri::InvalidUri),
    #[error("Not found")]
    NotFound,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
    #[error("Invalid request")]
    InvalidRequest,
    #[error("Invalid response")]
    InvalidResponse,
}

impl From<tonic::Status> for BlogClientError {
    fn from(status: tonic::Status) -> Self {
        match status.code() {
            tonic::Code::NotFound => BlogClientError::NotFound,
            tonic::Code::Unauthenticated => BlogClientError::Unauthorized,
            tonic::Code::PermissionDenied => BlogClientError::Forbidden,
            tonic::Code::InvalidArgument => BlogClientError::InvalidRequest,
            _ => BlogClientError::Grpc(status),
        }
    }
}
