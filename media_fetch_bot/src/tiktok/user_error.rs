use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("NoResult: Unfortunately, there's no result by this query.")]
    NoResult,
}