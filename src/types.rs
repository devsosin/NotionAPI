use serde::Deserialize;

use crate::errors::ClientError;

pub type ClientResult<T> = Result<T, ClientError>;

#[derive(Deserialize, Debug)]
pub struct ErrorResponse {
    status: u16,
    code: String,
    object: String,
    message: String,
}

impl ErrorResponse {
    pub fn get_code(&self) -> &str {
        &self.code
    }
    pub fn get_message(&self) -> &str {
        &self.message
    }
}
