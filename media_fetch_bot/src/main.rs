use crate::bot_commands::BotCommand;
use crate::bot_config::BotConfig;
use crate::errors::error_type::ErrorType;
use crate::errors::user_input::UserInputError;
use crate::link_type::LinkType;
use rust_i18n::t;
use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::types::{ParseMode};
use std::{process};
use pretty_env_logger::env_logger::Target;

mod bot_commands;
mod bot_config;
mod errors;
mod link_type;

mod tiktok;
mod instagram;
mod utils;


// Defining folder with locales. Path: media_fetch_bot/locales
rust_i18n::i18n!("locales");

#[tokio::main]
async fn main() {
    rust_i18n::set_locale("en");

    let bot_config = BotConfig::build().unwrap_or_else(|err| {
        log::error!("Error: {err}");
        process::exit(1);
    });

    pretty_env_logger::formatted_builder()
        .filter_level(bot_config.log_level)
        .target(Target::Stdout)
        .init();

    log::info!("Starting bot...");

    let bot = Bot::new(&bot_config.token);

    Dispatcher::builder(bot, Update::filter_message().endpoint(handle_message))
        .dependencies(dptree::deps![bot_config.name,
            bot_config.tiktok_api_key,
            bot_config.instagram_api_key])
        .build()
        .dispatch()
        .await;
}

async fn handle_message(bot: Bot, msg: Message,
                        bot_name: String,
                        tiktok_api_key: Option<String>,
                        instagram_api_key: Option<String>) -> ResponseResult<()> {
    let text = match msg.text() {
        None => {
            bot.send_message(msg.chat.id, t!(&UserInputError::EmptyMessage.to_string())).await?;
            return Ok(());
        }
        Some(value) => value
    };

    // Check if the message is a command
    if let Ok(command) = BotCommand::parse(text, &bot_name) {
        handle_command(bot, msg, command).await
    } else {
        match text {
            tiktok_link if tiktok_link.contains(&LinkType::TikTok.to_string()) => {
                handle_tiktok_link(tiktok_link, tiktok_api_key,
                                   &bot, &msg).await?
            }
            instagram_link if instagram_link.contains(&LinkType::Instagram.to_string()) => {
                handle_instagram_link(instagram_link, instagram_api_key,
                                      &bot, &msg).await?
            }
            _ => {
                bot.send_message(msg.chat.id,
                                 t!(&UserInputError::LinkTypeUndefined.to_string())).await?;
                log::info!("{}", format!("ChatID: {} -> Undefined: {}", msg.chat.id, text));
            }
        }

        Ok(())
    }
}

async fn handle_command(bot: Bot, msg: Message, cmd: BotCommand) -> ResponseResult<()> {
    match cmd {
        BotCommand::Help => bot.send_message(msg.chat.id, t!(&BotCommand::Help.to_string()))
            .parse_mode(ParseMode::Html)
            .await?,
    };

    Ok(())
}

async fn handle_tiktok_link(link: &str, api_key: Option<String>,
                            bot: &Bot, msg: &Message) -> ResponseResult<()> {
    let results
        = tiktok::handler::get_results(api_key, link.to_string()).await;

    match results {
        Ok(tuple) => {
            // This hashmap logic needed because library can group documents only by the same type.
            // But API returns just links.

            let title = tuple.0;
            let files = tuple.1;
            let keys = files.keys();
            for key in keys {
                let vector = files.get(key);
                if let Some(vector) = vector {
                    bot.send_media_group(msg.chat.id, vector.clone()).await?;
                }
            }

            if !title.is_empty() {
                bot.send_message(msg.chat.id, title).await?;
            }

            log::info!("{}", format!("ChatID: {} -> Tiktok: {}", msg.chat.id, link));
        }
        Err(err) => {
            let error_text = match err {
                ErrorType::Backend(ref specific_err) => {
                    log::error!("{}", format!("{}. ChatID: {} -> ErrQuery: {}",
                            specific_err, msg.chat.id, link));

                    format!("{}", t!(&err.to_string()))
                }
                ErrorType::User(err) => {
                    log::warn!("{}", format!("ChatID: {} -> ErrQuery: {}",
                            msg.chat.id, link));

                    format!("{}", t!(&err.to_string()))
                }
            };

            bot.send_message(msg.chat.id, error_text)
                .parse_mode(ParseMode::Html)
                .await?;
        }
    };

    Ok(())
}

async fn handle_instagram_link(link: &str, api_key: Option<String>,
                               bot: &Bot, msg: &Message) -> ResponseResult<()> {
    let results = instagram::handler::get_results(api_key, link.to_string()).await;
    match results {
        Ok(result) => {
            bot.send_message(msg.chat.id, result).await?;

            log::info!("{}", format!("ChatID: {} -> Instagram: {}", msg.chat.id, link));
        }
        Err(err) => {
            bot.send_message(msg.chat.id, err.to_string()).await?;
        }
    }

    Ok(())
}