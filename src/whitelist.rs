use crate::error::UserInputError;
use crate::logger;
use rust_i18n::t;
use teloxide::adaptors::Throttle;
use teloxide::prelude::{Requester, ResponseResult};
use teloxide::types::{Message, User};
use teloxide::Bot;

type Whitelist = [u64];

pub fn is_user_whitelisted(sender: &Option<User>, whitelist: &Whitelist) -> bool {
    // Sender, empty for messages sent to channels.
    match sender {
        None => false,
        Some(user) => whitelist.contains(&user.id.0),
    }
}

pub async fn send_denied_access(
    bot: &Throttle<Bot>, user_message: &Message,
) -> ResponseResult<()> {
    bot.send_message(
        user_message.chat.id,
        t!(UserInputError::NotWhitelisted.to_string()),
    )
    .await?;
    let message = format!(
        "User: {} -> NotWhitelisted: {}",
        logger::get_sender_identifier(user_message),
        user_message.text().unwrap_or_default()
    );
    log::info!("{message}");

    Ok(())
}
