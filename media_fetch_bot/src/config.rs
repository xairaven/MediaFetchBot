use std::env;
use dotenvy::dotenv;
use crate::error::BotError;

pub struct Config {
    pub bot_token: String
}

impl Config {
    pub fn build() -> Result<Config, BotError> {
        // Loading .env from the parent folder
        if let Err(_) = dotenv() {
            return Err(BotError::EnvIsNotLoaded);
        }

        // Loading token
        let token = match env::var("BOT_TOKEN") {
            Ok(value) => value,
            Err(_) => return Err(BotError::EnvBotToken)
        };

        // Returning Config var
        Ok(Config {
          bot_token: token
        })
    }
}