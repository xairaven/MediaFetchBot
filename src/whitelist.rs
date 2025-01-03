use crate::errors::user_input::UserInputError;
use crate::logger;
use rust_i18n::t;
use teloxide::adaptors::Throttle;
use teloxide::prelude::Requester;
use teloxide::types::{Message, User};
use teloxide::Bot;

type Whitelist = [u64];

pub async fn validate_user_access(
    bot: &Throttle<Bot>, msg: &Message, whitelist: &Whitelist,
) -> bool {
    let user = &msg.from;
    let whitelisted = is_user_whitelisted(user, whitelist);

    if !whitelisted {
        let _ = bot
            .send_message(msg.chat.id, t!(UserInputError::NotWhitelisted.to_string()))
            .await;
        let sender_identifier = logger::get_sender_identifier(msg);
        let message = format!(
            "User: {} -> NotWhitelisted: {}",
            sender_identifier,
            msg.text().unwrap_or_default()
        );
        log::info!("{message}");
    }

    whitelisted
}

fn is_user_whitelisted(sender: &Option<User>, whitelist: &Whitelist) -> bool {
    // Sender, empty for messages sent to channels.
    match sender {
        None => false,
        Some(user) => whitelist.contains(&user.id.0),
    }
}
