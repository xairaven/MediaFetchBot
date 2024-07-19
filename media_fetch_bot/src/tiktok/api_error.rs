use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Tiktok API Key missing. Tiktok module is not working.")]
    ApiKeyTiktokMissing,

    #[error("FailedGetResponse: Failed to get a response from API.")]
    FailedGetResponse,

    #[error("FailedParseResponse: Failed to parse response.")]
    FailedParseResponse,

    #[error("FailedParseUrl: Failed to parse final url.")]
    FailedParseUrl,

    #[error("WrongApiHost: x-rapidapi-host header is wrong.")]
    WrongApiHost,

    #[error("WrongApiKey: x-rapidapi-key header is wrong.")]
    WrongApiKey,
}