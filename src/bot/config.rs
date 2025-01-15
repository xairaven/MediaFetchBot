use dotenvy::dotenv;
use log::LevelFilter;
use std::{env, fs};
use thiserror::Error;

const WHITELIST_FILE_NAME: &str = "whitelist.json";

pub struct BotConfig {
    pub token: String,
    pub name: String,
    pub log_level: LevelFilter,
    pub log_format: String,
    pub whitelist_enabled: bool,
    pub whitelist: Vec<u64>,

    pub tiktok_api_key: Option<String>,

    pub instagram_photos_api_key: Option<String>,
    pub instagram_reels_api_key: Option<String>,
    pub instagram_stories_api_key: Option<String>,

    pub youtube_api_key: Option<String>,
}

impl BotConfig {
    pub fn from_env() -> Result<BotConfig, EnvError> {
        // Loading .env from the parent folder
        if dotenv().is_err() {
            return Err(EnvError::ConfigNotLoaded);
        }

        // Loading token
        let token = env::var("BOT_TOKEN").map_err(|_| EnvError::TokenNotLoaded)?;

        // Loading bot name
        let name = env::var("BOT_NAME").map_err(|_| EnvError::NameNotLoaded)?;

        // Loading log level
        let log_level = env::var("LOG_LEVEL")
            .map_err(|_| EnvError::LogLevelNotLoaded)?
            .parse::<LevelFilter>()
            .map_err(|_| EnvError::LogLevelUndefined)?;

        // Loading option LOG_FORMAT
        let log_format = match env::var("LOG_FORMAT") {
            Ok(value) if !value.trim().is_empty() => value,
            _ => "[%Y-%m-%D %H-%M-%S %LEVEL %TARGET] %MESSAGE".to_string(),
        };

        // Loading option WHITELIST_ENABLED
        let whitelist_enabled = match env::var("WHITELIST")
            .map_err(|_| EnvError::WhitelistEnabledNotLoaded)?
            .trim()
        {
            "ON" => true,
            "OFF" => false,
            _ => return Err(EnvError::WhitelistEnabledUndefined),
        };

        // Parsing whitelist, if enabled
        let mut whitelist: Vec<u64> = vec![];
        if whitelist_enabled {
            let data = fs::read_to_string(WHITELIST_FILE_NAME)
                .map_err(|_| EnvError::WhitelistFileOpeningFailed)?;
            whitelist = serde_json::from_str(&data)
                .map_err(|_| EnvError::WhitelistParseFailed)?;
        }

        // Loading TikTok API Key
        let tiktok_api_key = match env::var("TIKTOK_API_KEY") {
            Ok(value) if !value.trim().is_empty() => Some(value),
            _ => None,
        };

        // Loading Instagram Photos API Key
        let instagram_photos_api_key = match env::var("INSTAGRAM_PHOTOS_API_KEY") {
            Ok(value) if !value.trim().is_empty() => Some(value),
            _ => None,
        };

        // Loading Instagram Photos API Key
        let instagram_reels_api_key = match env::var("INSTAGRAM_REELS_API_KEY") {
            Ok(value) if !value.trim().is_empty() => Some(value),
            _ => None,
        };

        // Loading Instagram Photos API Key
        let instagram_stories_api_key = match env::var("INSTAGRAM_STORIES_API_KEY") {
            Ok(value) if !value.trim().is_empty() => Some(value),
            _ => None,
        };

        // Loading YouTube API Key
        let youtube_api_key = match env::var("YOUTUBE_API_KEY") {
            Ok(value) if !value.trim().is_empty() => Some(value),
            _ => None,
        };

        // Returning Config var
        Ok(BotConfig {
            token,
            name,
            log_level,
            log_format,
            whitelist_enabled,
            whitelist,

            tiktok_api_key,

            instagram_photos_api_key,
            instagram_reels_api_key,
            instagram_stories_api_key,

            youtube_api_key,
        })
    }
}

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
