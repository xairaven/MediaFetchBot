use strum_macros::Display;
use teloxide::macros::BotCommands;

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
