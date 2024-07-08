use crate::error::BotError;
use dotenvy::dotenv;
use std::env;
use log::LevelFilter;

pub struct BotConfig {
    pub token: String,
    pub name: String,
    pub log_level: LevelFilter,
    pub tiktok_api_key: Option<String>,
}

impl BotConfig {
    pub fn build() -> Result<BotConfig, BotError> {
        // Loading .env from the parent folder
        if dotenv().is_err() {
            return Err(BotError::EnvIsNotLoaded);
        }

        // Loading token
        let token = match env::var("BOT_TOKEN") {
            Ok(value) => value,
            Err(_) => return Err(BotError::EnvBotToken)
        };

        // Loading bot name
        let name = match env::var("BOT_NAME") {
            Ok(value) => value,
            Err(_) => return Err(BotError::EnvBotName)
        };

        // Loading log level
        let log_level = match env::var("LOG_LEVEL") {
            Ok(value) => value,
            Err(_) => return Err(BotError::EnvBotLogLevel)
        };
        let log_level = match log_level.to_lowercase().trim() {
            "off" => LevelFilter::Off,
            "error" => LevelFilter::Error,
            "warn" => LevelFilter::Warn,
            "info" => LevelFilter::Info,
            "debug" => LevelFilter::Debug,
            "trace" => LevelFilter::Trace,
            _ => return Err(BotError::EnvBotLogLevel)
        };

        // Loading tiktok_api_key
        let tiktok_api_key = match env::var("TIKTOK_API_KEY") {
            Ok(value) => Some(value),
            Err(_) => None
        };

        // Returning Config var
        Ok(BotConfig {
            token,
            name,
            log_level,
            tiktok_api_key,
        })
    }
}