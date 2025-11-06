use thiserror::Error;

use crate::types::ErrorResponse;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Unauthorized Error")]
    UnauthorizedError,

    #[error("RateLimited Error")]
    RateLimitedError,

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<ErrorResponse> for ClientError {
    fn from(res: ErrorResponse) -> Self {
        match res.get_code().to_lowercase().as_str() {
            "unauthorized" => ClientError::UnauthorizedError,
            _ => ClientError::InternalError(res.get_message().to_string()),
        }
    }
}
