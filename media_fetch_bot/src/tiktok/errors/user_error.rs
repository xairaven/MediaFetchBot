use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Unfortunately, there's no content by this link.")]
    NoResult,
}