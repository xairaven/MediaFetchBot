use crate::api::Api;
use crate::bot::commands::BotCommand;
use crate::bot::config::BotConfig;
use crate::error::UserOutputError;
use rust_i18n::t;
use std::process;
use std::sync::Arc;
use teloxide::adaptors::Throttle;
use teloxide::adaptors::throttle::Limits;
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
            bot.send_message(msg.chat.id, t!(UserOutputError::EmptyMessage.to_string()))
                .await?;
            return Ok(());
        },
        Some(value) => value.trim(),
    };

    // Check if user whitelisted, if whitelist is enabled.
    if bot_config.whitelist_enabled {
        let whitelisted =
            whitelist::is_user_whitelisted(&msg.from, &bot_config.whitelist);

        if !whitelisted {
            return whitelist::send_denied_access(&bot, &msg).await;
        }
    }

    // Check if the message is a command
    if let Ok(command) = BotCommand::parse(text, &bot_config.name) {
        return command.handle(bot, msg).await;
    }

    let api_instances = Api::instances_from_config(&bot_config);
    let instance = api_instances
        .iter()
        .find(|instance| instance.matches_url(text));

    match instance {
        Some(instance) => {
            instance.handle_link(text.to_string(), &bot, &msg).await?;
        },
        None => {
            bot.send_message(
                msg.chat.id,
                t!(UserOutputError::LinkTypeUndefined.to_string()),
            )
            .await?;

            let log_message = format!(
                "User: {} -> Undefined: {}",
                logger::get_sender_identifier(&msg),
                text
            );
            log::info!("{log_message}");
        },
    }

    Ok(())
}

mod bot {
    pub mod commands;
    pub mod config;
}
mod api;
mod error;
mod logger;
mod media;
mod whitelist;
