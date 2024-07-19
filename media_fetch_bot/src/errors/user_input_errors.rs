use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserInputError {
    #[error("Empty message.")]
    EmptyMessage,

    #[error("Link type undefined.")]
    LinkTypeUndefined
}