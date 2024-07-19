use thiserror::Error;

#[derive(Debug, Error)]
pub enum EnvError {
    #[error("ConfigNotLoaded: Something wrong with .env file. May be, it not exists.")]
    ConfigNotLoaded,

    #[error("LogLevelNotLoaded: Something wrong with .env LOG_LEVEL. May be, there's no field with this name.")]
    LogLevelNotLoaded,

    #[error("LogLevelUndefined: Undefined LOG_LEVEL. Possible levels: error, warn, info, debug, trace, off")]
    LogLevelUndefined,

    #[error("NameNotLoaded: Something wrong with .env BOT_NAME. May be, there's no field with this name.")]
    NameNotLoaded,

    #[error("TokenNotLoaded: Something wrong with .env BOT_TOKEN. May be, there's no field with this name.")]
    TokenNotLoaded,
}