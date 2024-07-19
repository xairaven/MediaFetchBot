use thiserror::Error;
use crate::instagram::errors::api_error::ApiError;

#[derive(Error, Debug)]
pub enum ErrorType {
    #[error("InstagramBackendError")]
    Backend(#[from] ApiError),
}