use std::process;
use teloxide::{prelude::*, utils::command::BotCommands};
use crate::bot_commands::BotCommand;
use crate::bot_config::BotConfig;
use crate::localization::LocalizationCommand;
use rust_i18n::t;
use teloxide::types::ParseMode;

pub mod bot_commands;
pub mod bot_config;
pub mod error;
pub mod localization;
pub mod tiktok;

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

    Dispatcher::builder(bot, Update::filter_message().endpoint(handle_message))
        .dependencies(dptree::deps![bot_name])
        .build()
        .dispatch()
        .await;
}

async fn handle_message(bot: Bot, msg: Message, bot_name: String) -> ResponseResult<()> {
    let text = msg.text().unwrap_or("");

    // Check if the message is a command
    if let Ok(command) = BotCommand::parse(text, &bot_name) {
        handle_command(bot, msg, command).await
    } else {
        let href = tiktok::get_href(msg.text().unwrap()).await;
        let href = href.unwrap();
        let _ = tiktok::download_file_by_link(&href);

        bot.send_message(msg.chat.id,  &href).await?;
        /*
        // Handle non-command messages
        bot.send_message(msg.chat.id, msg.text().unwrap())
            .parse_mode(ParseMode::Html).await?;

        bot.send_message(msg.chat.id,
                         t!(LocalizationCommand::CommandNotFound.into()))
            .parse_mode(ParseMode::Html).await?;
         */

        todo!()
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