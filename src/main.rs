use crate::bot::commands::BotCommand;
use crate::bot::config::BotConfig;
use crate::errors::user_input::UserInputError;
use rust_i18n::t;
use std::process;
use std::sync::Arc;
use teloxide::adaptors::throttle::Limits;
use teloxide::adaptors::Throttle;
use teloxide::{prelude::*, utils::command::BotCommands};

// Defining folder with locales. Path: media_fetch_bot/locales
rust_i18n::i18n!("locales");

#[tokio::main]
async fn main() {
    rust_i18n::set_locale("en");

    let bot_config = Arc::new(BotConfig::from_env().unwrap_or_else(|err| {
        let error = t!(err.to_string());
        eprintln!("Error: {error}");
        process::exit(1);
    }));

    logger::init(&bot_config.log_level, bot_config.log_format.clone()).unwrap_or_else(
        |err| {
            eprintln!("Error: {err}");
            process::exit(1);
        },
    );

    let bot = Bot::new(&bot_config.token).throttle(Limits::default());

    log::info!("Started bot.");

    Dispatcher::builder(bot, Update::filter_message().endpoint(handle_message))
        .dependencies(dptree::deps![Arc::clone(&bot_config)])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn handle_message(
    bot: Throttle<Bot>, msg: Message, bot_config: Arc<BotConfig>,
) -> ResponseResult<()> {
    let text = match msg.text() {
        None => {
            bot.send_message(msg.chat.id, t!(UserInputError::EmptyMessage.to_string()))
                .await?;
            return Ok(());
        },
        Some(value) => value,
    };

    // Check if user whitelisted, if whitelist is enabled.
    if bot_config.whitelist_enabled {
        let whitelisted =
            whitelist::validate_user_access(&bot, &msg, &bot_config.whitelist).await;

        if !whitelisted {
            return Ok(());
        }
    }

    // Check if the message is a command
    if let Ok(command) = BotCommand::parse(text, &bot_config.name) {
        command.handle(bot, msg).await?;
        return Ok(());
    }

    let api_instances = rapid_api::api_factory(&bot_config);
    let mut link_defined = false;
    for instance in &api_instances {
        if text.contains(&instance.link_base()) {
            instance.handle_link(text, &bot, &msg).await?;
            link_defined = true;
        }
    }
    if !link_defined {
        bot.send_message(
            msg.chat.id,
            t!(UserInputError::LinkTypeUndefined.to_string()),
        )
        .await?;
        log::info!(
            "{}",
            format!(
                "User: {} -> Undefined: {}",
                logger::get_sender_identifier(&msg),
                text
            )
        );
    }

    Ok(())
}

mod bot {
    pub mod commands;
    pub mod config;
}
mod errors;
mod logger;

mod instagram;
mod rapid_api;
mod tiktok;
mod utils;
mod whitelist;
