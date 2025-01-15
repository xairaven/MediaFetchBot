use crate::api::ApiError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("ServerError")]
    Server(#[from] ApiError),

    #[error("UserError")]
    User(#[from] UserInputError),
}

#[derive(Debug, Error)]
pub enum UserInputError {
    #[error("EmptyMessage")]
    EmptyMessage,

    #[error("LinkTypeUndefined")]
    LinkTypeUndefined,

    #[error("NoResult")]
    NoResult,

    #[error("NotWhitelisted")]
    NotWhitelisted,

    #[error("InstagramFailedGetContent")]
    InstagramFailedGetContent(Option<String>),
}

impl UserInputError {
    pub fn additional_info(&self) -> Option<String> {
        match self {
            Self::InstagramFailedGetContent(value) => value.clone(),
            _ => None,
        }
    }
}
