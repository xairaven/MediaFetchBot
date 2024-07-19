use thiserror::Error;
use crate::tiktok::errors::api_error::ApiError;
use crate::tiktok::errors::user_error::UserError;

#[derive(Error, Debug)]
pub enum ErrorType {
    #[error("TikTokBackendError")]
    Backend(#[from] ApiError),

    #[error("TikTokUserError")]
    User(#[from] UserError)
}