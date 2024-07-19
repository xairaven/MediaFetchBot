use teloxide::macros::BotCommands;
use strum_macros::{Display};

#[derive(Display, BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum BotCommand {
    #[command(description = "Instructions for use.")]
    Help,
}