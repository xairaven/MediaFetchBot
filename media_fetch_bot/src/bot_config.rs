use std::env;
use dotenvy::dotenv;
use crate::error::BotError;

pub struct BotConfig {
    pub token: String,
    pub name: String,
    pub save_dir: String
}

impl BotConfig {
    pub fn build() -> Result<BotConfig, BotError> {
        // Loading .env from the parent folder
        if let Err(_) = dotenv() {
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

        // Loading save directory
        let save_dir = match env::var("SAVE_DIR") {
            Ok(value) => value,
            Err(_) => return Err(BotError::EnvSaveDir)
        };

        // Returning Config var
        Ok(BotConfig {
            token,
            name,
            save_dir
        })
    }
}