use crate::errors::env::EnvError;
use dotenvy::dotenv;
use log::LevelFilter;
use std::{env, fs};

const WHITELIST_FILE_NAME: &str = "whitelist.json";

pub struct BotConfig {
    pub token: String,
    pub name: String,
    pub log_level: LevelFilter,
    pub whitelist_enabled: bool,
    pub whitelist: Vec<u64>,
    pub tiktok_api_key: Option<String>,
    pub instagram_api_key: Option<String>,
}

impl BotConfig {
    pub fn from_env() -> Result<BotConfig, EnvError> {
        // Loading .env from the parent folder
        if dotenv().is_err() {
            return Err(EnvError::ConfigNotLoaded);
        }

        // Loading token
        let token =
            env::var("BOT_TOKEN").map_err(|_| EnvError::TokenNotLoaded)?;

        // Loading bot name
        let name = env::var("BOT_NAME").map_err(|_| EnvError::NameNotLoaded)?;

        // Loading log level
        let log_level =
            env::var("LOG_LEVEL").map_err(|_| EnvError::LogLevelNotLoaded)?;
        let log_level = match log_level.to_lowercase().trim() {
            "off" => LevelFilter::Off,
            "error" => LevelFilter::Error,
            "warn" => LevelFilter::Warn,
            "info" => LevelFilter::Info,
            "debug" => LevelFilter::Debug,
            "trace" => LevelFilter::Trace,
            _ => return Err(EnvError::LogLevelUndefined),
        };

        // Loading option WHITELIST_ENABLED
        let whitelist_enabled = env::var("WHITELIST")
            .map_err(|_| EnvError::WhitelistEnabledNotLoaded)?;
        let whitelist_enabled = match whitelist_enabled.trim() {
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
        let tiktok_api_key = env::var("TIKTOK_API_KEY");
        let tiktok_api_key = match tiktok_api_key {
            Ok(value) if !value.trim().is_empty() => Some(value),
            _ => None,
        };

        // Loading Instagram API Key
        let instagram_api_key = env::var("INSTAGRAM_API_KEY");
        let instagram_api_key = match instagram_api_key {
            Ok(value) if !value.trim().is_empty() => Some(value),
            _ => None,
        };

        // Returning Config var
        Ok(BotConfig {
            token,
            name,
            log_level,
            whitelist_enabled,
            whitelist,
            tiktok_api_key,
            instagram_api_key,
        })
    }
}
