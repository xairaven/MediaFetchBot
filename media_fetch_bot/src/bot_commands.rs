use teloxide::macros::BotCommands;

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum BotCommand {
    #[command(description = "Instructions for use.")]
    Help,
}