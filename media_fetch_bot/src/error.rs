use thiserror::Error;

#[derive(Debug, Error)]
pub enum BotError {
    #[error("Tiktok API Key missing. Tiktok module is not working.")]
    ApiKeyTiktokMissing,

    #[error("EnvIsNotLoaded: Something wrong with .env file. May be, it not exists.")]
    EnvIsNotLoaded,

    #[error("EnvBotToken: Something wrong with .env BOT_TOKEN. May be, there's no field with this name.")]
    EnvBotToken,

    #[error("EnvBotName: Something wrong with .env BOT_NAME. May be, there's no field with this name.")]
    EnvBotName,

    #[error("EnvEnvBotLogLevel: Something wrong with .env LOG_LEVEL. May be, there's no field with this name.")]
    EnvBotLogLevel,

    #[error("FailedGetResponse: Failed to get a response from API.")]
    FailedGetResponse,

    #[error("FailedParseResponse: Failed to parse response.")]
    FailedParseResponse,

    #[error("FailedParseUrl: Failed to parse final url.")]
    FailedParseUrl,

    #[error("NoResult: Unfortunately, there's no result by this query.")]
    NoResult,

    #[error("WrongApiHost: x-rapidapi-host header is wrong.")]
    WrongApiHost,

    #[error("WrongApiKey: x-rapidapi-key header is wrong.")]
    WrongApiKey,
}