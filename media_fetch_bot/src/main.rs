use crate::bot_commands::BotCommand;
use crate::bot_config::BotConfig;
use crate::link_type::LinkType;
use crate::localization::LocalizationCommand;
use rust_i18n::t;
use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::types::{InputFile, ParseMode};
use std::{fs, process};

pub mod bot_commands;
pub mod bot_config;
pub mod error;
mod link_type;
pub mod localization;
pub mod tiktok;
pub mod utils;

// Defining folder with locales. Path: media_fetch_bot/locales
rust_i18n::i18n!("locales");

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    log::info!("Started executable...");

    rust_i18n::set_locale("en");
    log::info!("Set 'en' locale...");

    let bot_config = BotConfig::build().unwrap_or_else(|err| {
        log::error!("Error: {err}");
        process::exit(1);
    });

    log::info!("Starting bot...");

    let bot = Bot::new(&bot_config.token);

    let bot_name = bot_config.name.clone();
    let save_dir = bot_config.save_dir.clone();

    Dispatcher::builder(bot, Update::filter_message().endpoint(handle_message))
        .dependencies(dptree::deps![bot_name])
        .dependencies(dptree::deps![save_dir])
        .build()
        .dispatch()
        .await;
}

async fn handle_message(bot: Bot, msg: Message, bot_name: String, save_dir: String)
                        -> ResponseResult<()> {
    let text = match msg.text() {
        None => {
            bot.send_message(msg.chat.id,
                             t!(LocalizationCommand::EmptyMessage.into())).await?;
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
                let file_path
                    = tiktok::process_link(text.to_string(), &save_dir).await;
                match file_path {
                    Ok(file_path) => {
                        let file = InputFile::file(&file_path);

                        bot.send_video(msg.chat.id, file).await?;

                        let _ = fs::remove_file(file_path);
                    }
                    Err(err) => {
                        let error_text = format!("{}\n\n<i>{}</i>",
                                                 t!("error_text"), err.to_string());

                        bot.send_message(msg.chat.id, error_text)
                            .parse_mode(ParseMode::Html)
                            .await?;

                        return Ok(());
                    }
                };
            }
            _ => {
                bot.send_message(msg.chat.id,
                                 t!(LocalizationCommand::LinkTypeUndefined.into())).await?;
            }
        }

        Ok(())
    }
}

async fn handle_command(bot: Bot, msg: Message, cmd: BotCommand) -> ResponseResult<()> {
    match cmd {
        BotCommand::Help => bot.send_message(msg.chat.id,
                                             t!(LocalizationCommand::Help.into()))
            .parse_mode(ParseMode::Html)
            .await?,
    };

    Ok(())
}