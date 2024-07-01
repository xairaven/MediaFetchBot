use thiserror::Error;

#[derive(Debug, Error)]
pub enum BotError {
    #[error("Something wrong with .env file. May be, it not exists.")]
    EnvIsNotLoaded,

    #[error("Something wrong with .env BOT_TOKEN. May be, there's no field with this name.")]
    EnvBotToken,

    #[error("Something wrong with .env BOT_NAME. May be, there's no field with this name.")]
    EnvBotName,

    #[error("Something wrong with .env SAVE_DIR. May be, there's no field with this name.")]
    EnvSaveDir,

    #[error("Failed to get user message.")]
    FailedGetUserMessage,

    #[error("Failed to get a response from API. Please, send that link to administrator for additional tests.")]
    FailedGetResponse,

    #[error("Failed to extract html. Please, send that link to administrator for additional tests.")]
    FailedExtractingHtml,

    #[error("Failed to parse response. Please, send that link to administrator for additional tests.")]
    FailedParseResponse,

    #[error("Unfortunately, there's no result by this query.")]
    NoResult,
}