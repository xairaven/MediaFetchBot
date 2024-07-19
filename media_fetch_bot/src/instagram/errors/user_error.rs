use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("NoResult")]
    NoResult,
}