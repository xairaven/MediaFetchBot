use thiserror::Error;
use crate::tiktok::api_error::ApiError;
use crate::tiktok::user_error::UserError;

#[derive(Error, Debug)]
pub enum ErrorType {
    #[error("Backend Error. Something happened with server")]
    Backend(#[from] ApiError),

    #[error("User Error. Something wrong with user input")]
    User(#[from] UserError)
}