use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserInputError {
    #[error("Command not found.")]
    CommandNotFound,

    #[error("Empty message.")]
    EmptyMessage,

    #[error("Link type undefined.")]
    LinkTypeUndefined
}