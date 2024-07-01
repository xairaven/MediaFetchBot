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

    #[error("Failed to get a response from API.")]
    FailedGetResponse,

    #[error("Failed to extract html.")]
    FailedExtractingHtml,

    #[error("Failed to parse response.")]
    FailedParseResponse,

    #[error("All steps done, but result is invalid.")]
    InvalidResult,

    #[error("Unfortunately, there's no result by this query.")]
    NoResult,

    #[error("Server problem, unable to create file for downloading.")]
    UnableToCreateFile,

    #[error("Server problem, failed to copy content.")]
    UnableToCopyContent,
}