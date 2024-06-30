use std::process;
use teloxide::{prelude::*, utils::command::BotCommands};
use media_fetch_bot::bot_commands::BotCommand;
use media_fetch_bot::config::Config;
use media_fetch_bot::localization::LocalizationCommand;
use rust_i18n::t;
use teloxide::types::ParseMode;

rust_i18n::i18n!("locales");

static BOT_NAME: &str = "sm_media_fetch_bot";

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    log::info!("Started executable...");

    rust_i18n::set_locale("en");
    log::info!("Set 'en' locale...");

    let config = Config::build().unwrap_or_else(|err| {
        log::error!("Error: {err}");
        eprintln!("Error: {err}");
        process::exit(1);
    });

    log::info!("Starting bot...");

    let bot = Bot::new(&config.bot_token);

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        handle_message(bot, msg).await;
        respond(())
    }).await;
}

async fn handle_message(bot: Bot, msg: Message) {
    let text = msg.text().unwrap_or("");

    // Check if the message is a command
    if let Ok(command) = BotCommand::parse(text, &BOT_NAME) {
        handle_command(bot, msg, command).await
            .log_on_error().await;
    } else {
        // Handle non-command messages
        bot.send_message(msg.chat.id,
                         t!(LocalizationCommand::CommandNotFound.into()))
            .parse_mode(ParseMode::Html).await
            .log_on_error().await;
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