use std::process;
use teloxide::prelude::*;
use dotenvy::dotenv;
use media_fetch_bot::config::Config;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let config = Config::build().unwrap_or_else(|err| {
        log::error!("Error: {err}");
        eprintln!("Error: {err}");
        process::exit(1);
    });

    log::info!("Starting bot...");

    let bot = Bot::new(&config.bot_token);
}