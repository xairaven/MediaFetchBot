use strum_macros::{Display, EnumString, IntoStaticStr};

#[derive(Display, EnumString, IntoStaticStr)]
pub enum LocalizationCommand {
    #[strum(serialize = "help")]
    Help,

    #[strum(serialize = "command_not_found")]
    CommandNotFound
}