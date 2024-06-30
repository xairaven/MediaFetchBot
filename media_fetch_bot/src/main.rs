use std::process;
use teloxide::{prelude::*, utils::command::BotCommands};
use media_fetch_bot::config::Config;

static BOT_NAME: &str = "sm_media_fetch_bot";

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

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        handle_message(bot, msg).await;
        respond(())
    })
        .await;
}

async fn handle_message(bot: Bot, msg: Message) {
    let text = msg.text().unwrap_or("");

    // Check if the message is a command
    if let Ok(command) = Command::parse(text, &BOT_NAME) {
        handle_command(bot, msg, command).await
            .log_on_error().await;
    } else {
        // Handle non-command messages
        bot.send_message(msg.chat.id, "Command not found").await
            .log_on_error().await;
    }
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Get assistance and learn about available commands.")]
    Help,
}

async fn handle_command(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
    };

    Ok(())
}