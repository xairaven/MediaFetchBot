use crate::bot_commands::BotCommand;
use crate::bot_config::BotConfig;
use crate::errors::error_type::ErrorType;
use crate::errors::user_input::UserInputError;
use crate::link_type::LinkType;
use rust_i18n::t;
use std::process;
use teloxide::adaptors::throttle::Limits;
use teloxide::adaptors::Throttle;
use teloxide::types::ParseMode;
use teloxide::{prelude::*, utils::command::BotCommands};

mod bot_commands;
mod bot_config;
mod errors;
mod link_type;
mod logger;

mod instagram;
mod rapid_api;
mod tiktok;
mod utils;

// Defining folder with locales. Path: media_fetch_bot/locales
rust_i18n::i18n!("locales");

#[tokio::main]
async fn main() {
    rust_i18n::set_locale("en");

    let bot_config = BotConfig::from_env().unwrap_or_else(|err| {
        log::error!("Error: {err}");
        process::exit(1);
    });

    logger::init(bot_config.log_level).unwrap_or_else(|err| {
        log::error!("Error: {err}");
        process::exit(1);
    });

    log::info!("Starting bot...");

    let bot = Bot::new(&bot_config.token).throttle(Limits::default());

    Dispatcher::builder(bot, Update::filter_message().endpoint(handle_message))
        .dependencies(dptree::deps![
            bot_config.name,
            bot_config.tiktok_api_key,
            bot_config.instagram_api_key
        ])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn handle_message(
    bot: Throttle<Bot>, msg: Message, bot_name: String,
    tiktok_api_key: Option<String>, instagram_api_key: Option<String>,
) -> ResponseResult<()> {
    let text = match msg.text() {
        None => {
            bot.send_message(
                msg.chat.id,
                t!(UserInputError::EmptyMessage.to_string()),
            )
            .await?;
            return Ok(());
        },
        Some(value) => value,
    };

    // Check if the message is a command
    if let Ok(command) = BotCommand::parse(text, &bot_name) {
        handle_command(bot, msg, command).await
    } else {
        match text {
            tiktok_link
                if tiktok_link.contains(&LinkType::TikTok.to_string()) =>
            {
                handle_tiktok_link(tiktok_link, tiktok_api_key, &bot, &msg)
                    .await?
            },
            instagram_link
                if instagram_link
                    .contains(&LinkType::Instagram.to_string()) =>
            {
                handle_instagram_link(
                    instagram_link,
                    instagram_api_key,
                    &bot,
                    &msg,
                )
                .await?
            },
            _ => {
                bot.send_message(
                    msg.chat.id,
                    t!(UserInputError::LinkTypeUndefined.to_string()),
                )
                .await?;
                log::info!(
                    "{}",
                    format!("ChatID: {} -> Undefined: {}", msg.chat.id, text)
                );
            },
        }

        Ok(())
    }
}

async fn handle_command(
    bot: Throttle<Bot>, msg: Message, cmd: BotCommand,
) -> ResponseResult<()> {
    match cmd {
        BotCommand::Help => {
            bot.send_message(msg.chat.id, t!(BotCommand::Help.to_string()))
                .parse_mode(ParseMode::Html)
                .await?
        },
        BotCommand::Start => {
            bot.send_message(msg.chat.id, t!(BotCommand::Start.to_string()))
                .parse_mode(ParseMode::Html)
                .await?
        },
    };

    Ok(())
}

async fn handle_tiktok_link(
    link: &str, api_key: Option<String>, bot: &Throttle<Bot>, msg: &Message,
) -> ResponseResult<()> {
    let results = tiktok::handler::get_results(api_key, link.to_string()).await;

    rapid_api::send_results(results, bot, msg, link).await?;

    Ok(())
}

async fn handle_instagram_link(
    link: &str, api_key: Option<String>, bot: &Throttle<Bot>, msg: &Message,
) -> ResponseResult<()> {
    let results =
        instagram::handler::get_results(api_key, link.to_string()).await;

    rapid_api::send_results(results, bot, msg, link).await?;

    Ok(())
}

fn form_error_text(err: ErrorType, chat_id: &ChatId, link: &str) -> String {
    match err {
        ErrorType::Backend(ref specific_err) => {
            log::error!(
                "{}",
                format!(
                    "{}. ChatID: {} -> ErrQuery: {}",
                    specific_err, chat_id, link
                )
            );

            format!("{}", t!(err.to_string()))
        },
        ErrorType::User(specific_err) => {
            log::warn!(
                "{}",
                format!("ChatID: {} -> ErrQuery: {}", chat_id, link)
            );

            format!("{}", t!(specific_err.to_string()))
        },
    }
}
