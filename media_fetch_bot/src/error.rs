use thiserror::Error;

#[derive(Debug, Error)]
pub enum BotError {
    #[error("Something wrong with .env file. May be, it not exists.")]
    EnvIsNotLoaded,

    #[error("Something wrong with .env BOT_TOKEN. May be, there's no field with this name.")]
    EnvBotToken,

    #[error("Something wrong with .env BOT_NAME. May be, there's no field with this name.")]
    EnvBotName,
}