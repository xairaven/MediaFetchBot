use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("ApiKeyTiktokMissing")]
    ApiKeyTiktokMissing,

    #[error("ApiKeyInstagramMissing")]
    ApiKeyInstagramMissing,

    #[error("FailedGetResponse")]
    FailedGetResponse,

    #[error("FailedParseResponse")]
    FailedParseResponse,

    #[error("FailedParseUrl")]
    FailedParseUrl,

    #[error("WrongApiHost")]
    WrongApiHost,

    #[error("WrongApiKey")]
    WrongApiKey,

    #[error("WrongMediaFormat")]
    WrongMediaFormat,
}