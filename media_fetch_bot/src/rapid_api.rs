use crate::errors::error_type::ErrorType;
use crate::form_error_text;
use crate::rapid_api::media_format::MediaFormat;
use std::collections::HashMap;
use teloxide::adaptors::Throttle;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{Message, Requester, ResponseResult};
use teloxide::types::{InputMedia, ParseMode};
use teloxide::Bot;

pub mod media_format;
pub mod raw_media;

type RapidApiResults =
    Result<(String, HashMap<MediaFormat, Vec<InputMedia>>), ErrorType>;
pub async fn send_results(
    results: RapidApiResults, bot: &Throttle<Bot>, msg: &Message, link: &str,
) -> ResponseResult<()> {
    match results {
        Ok(tuple) => {
            // This hashmap logic needed because library can group documents only by the same type.
            // But API returns just links.

            let title = tuple.0;
            let files = tuple.1;
            let keys = files.keys();
            for key in keys {
                let vector = files.get(key);
                if let Some(vector) = vector {
                    bot.send_media_group(msg.chat.id, vector.clone()).await?;
                }
            }

            if !title.is_empty() {
                bot.send_message(msg.chat.id, title).await?;
            }

            log::info!("{}", format!("ChatID: {} -> {}", msg.chat.id, link));
        },
        Err(err) => {
            let error_text = form_error_text(err, &msg.chat.id, link);

            bot.send_message(msg.chat.id, error_text)
                .parse_mode(ParseMode::Html)
                .await?;
        },
    }

    Ok(())
}
