use thiserror::Error;

#[derive(Debug, Error)]
pub enum EnvError {
    #[error("ConfigNotLoaded")]
    ConfigNotLoaded,

    #[error("LogLevelNotLoaded")]
    LogLevelNotLoaded,

    #[error("LogLevelUndefined")]
    LogLevelUndefined,

    #[error("WhitelistEnabledNotLoaded")]
    WhitelistEnabledNotLoaded,

    #[error("WhitelistEnabledUndefined")]
    WhitelistEnabledUndefined,

    #[error("WhitelistFileOpeningFailed")]
    WhitelistFileOpeningFailed,

    #[error("WhitelistParseFailed")]
    WhitelistParseFailed,

    #[error("NameNotLoaded")]
    NameNotLoaded,

    #[error("TokenNotLoaded")]
    TokenNotLoaded,
}
