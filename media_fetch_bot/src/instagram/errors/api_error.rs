use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("ApiKeyInstagramMissing")]
    ApiKeyInstagramMissing,

    #[error("FailedGetResponse")]
    FailedGetResponse,

    #[error("WrongApiHost")]
    WrongApiHost,

    #[error("WrongApiKey")]
    WrongApiKey,
}