use crate::errors::api::ApiError;
use crate::errors::user_input::UserInputError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorType {
    #[error("BackendError")]
    Backend(#[from] ApiError),

    #[error("UserError")]
    User(#[from] UserInputError),
}
