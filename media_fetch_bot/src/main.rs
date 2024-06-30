use std::process;
use teloxide::{prelude::*, utils::command::BotCommands};
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

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Get assistance and learn about available commands.")]
    Help,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
    };

    Ok(())
}