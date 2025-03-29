use rust_i18n::t;
use strum_macros::Display;
use teloxide::Bot;
use teloxide::adaptors::Throttle;
use teloxide::macros::BotCommands;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{Message, Requester, ResponseResult};
use teloxide::types::ParseMode;

#[derive(Display, BotCommands, Clone, Debug)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum BotCommand {
    #[command(description = "Welcome message.")]
    Start,

    #[command(description = "Instructions for use.")]
    Help,
}

impl BotCommand {
    pub async fn handle(&self, bot: Throttle<Bot>, msg: Message) -> ResponseResult<()> {
        match self {
            BotCommand::Start => {
                bot.send_message(msg.chat.id, t!(BotCommand::Start.to_string()))
                    .parse_mode(ParseMode::Html)
                    .await?;
            },
            BotCommand::Help => {
                bot.send_message(msg.chat.id, t!(BotCommand::Help.to_string()))
                    .parse_mode(ParseMode::Html)
                    .await?;
            },
        }

        Ok(())
    }
}
