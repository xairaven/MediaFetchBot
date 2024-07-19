use thiserror::Error;

#[derive(Debug, Error)]
pub enum EnvError {
    #[error("ConfigNotLoaded")]
    ConfigNotLoaded,

    #[error("LogLevelNotLoaded")]
    LogLevelNotLoaded,

    #[error("LogLevelUndefined")]
    LogLevelUndefined,

    #[error("NameNotLoaded")]
    NameNotLoaded,

    #[error("TokenNotLoaded")]
    TokenNotLoaded,
}