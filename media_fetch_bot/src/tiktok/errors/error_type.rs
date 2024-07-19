use thiserror::Error;
use crate::tiktok::errors::api_error::ApiError;
use crate::tiktok::errors::user_error::UserError;

#[derive(Error, Debug)]
pub enum ErrorType {
    #[error("TikTokBackendErrorMessage")]
    Backend(#[from] ApiError),

    #[error("User Error. Something wrong with user input")]
    User(#[from] UserError)
}