use thiserror::Error;

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
}
