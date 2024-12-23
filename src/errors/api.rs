use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("FailedGetResponse")]
    FailedGetResponse,

    #[error("FailedParseResponse")]
    FailedParseResponse,

    #[error("FailedParseUrl")]
    FailedParseUrl,

    #[error("InstagramQuotaExceeded")]
    InstagramQuotaExceeded,

    #[error("TiktokQuotaExceeded")]
    TiktokQuotaExceeded,

    #[error("WrongApiHost")]
    WrongApiHost,

    #[error("WrongApiKey")]
    WrongApiKey,

    #[error("WrongMediaFormat")]
    WrongMediaFormat,
}
