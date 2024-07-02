use thiserror::Error;

#[derive(Debug, Error)]
pub enum BotError {
    #[error("EnvIsNotLoaded: Something wrong with .env file. May be, it not exists.")]
    EnvIsNotLoaded,

    #[error("EnvBotToken: Something wrong with .env BOT_TOKEN. May be, there's no field with this name.")]
    EnvBotToken,

    #[error("EnvBotName: Something wrong with .env BOT_NAME. May be, there's no field with this name.")]
    EnvBotName,

    #[error("EnvSaveDir: Something wrong with .env SAVE_DIR. May be, there's no field with this name.")]
    EnvSaveDir,

    #[error("FailedGetUserMessage: Failed to get user message.")]
    FailedGetUserMessage,

    #[error("FailedGetResponse: Failed to get a response from API.")]
    FailedGetResponse,

    #[error("FailedExtractingHtml: Failed to extract html.")]
    FailedExtractingHtml,

    #[error("FailedParseResponse: Failed to parse response.")]
    FailedParseResponse,

    #[error("FailedParseUrl: Failed to parse final url.")]
    FailedParseUrl,

    #[error("InvalidResult: All steps done, but result is invalid.")]
    InvalidResult,

    #[error("NoResult: Unfortunately, there's no result by this query.")]
    NoResult,
}