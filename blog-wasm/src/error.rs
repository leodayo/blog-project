use gloo_net::Error as GlooError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Gloo error: {0}")]
    Network(#[from] GlooError),

    #[error("Status: {status}. Body: {body}")]
    Http { status: u16, body: String },
}
