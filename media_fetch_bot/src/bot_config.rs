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
        let token = env::var("BOT_TOKEN")
            .map_err(|_| BotError::EnvBotToken)?;

        // Loading bot name
        let name = env::var("BOT_NAME")
            .map_err(|_| BotError::EnvBotName)?;

        // Loading log level
        let log_level = env::var("LOG_LEVEL")
            .map_err(|_| BotError::EnvBotLogLevel)?;
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
        let tiktok_api_key = env::var("TIKTOK_API_KEY").ok();

        // Returning Config var
        Ok(BotConfig {
            token,
            name,
            log_level,
            tiktok_api_key,
        })
    }
}