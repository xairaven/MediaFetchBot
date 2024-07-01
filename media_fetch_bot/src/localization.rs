use strum_macros::{Display, EnumString, IntoStaticStr};

#[derive(Display, EnumString, IntoStaticStr)]
pub enum LocalizationCommand {
    #[strum(serialize = "command_not_found")]
    CommandNotFound,

    #[strum(serialize = "empty_message")]
    EmptyMessage,

    #[strum(serialize = "help")]
    Help,

    #[strum(serialize = "link_type_undefined")]
    LinkTypeUndefined,
}