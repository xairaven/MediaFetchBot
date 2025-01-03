use crate::errors::error_type::ErrorType;
use crate::logger;
use crate::rapid_api::RapidApiResults;
use async_trait::async_trait;
use rust_i18n::t;
use teloxide::adaptors::Throttle;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{Message, Requester, ResponseResult};
use teloxide::types::ParseMode;
use teloxide::Bot;

#[async_trait]
pub trait ApiInstance {
    fn base_url(&self) -> String;

    async fn handle_link(
        &self, link: &str, bot: &Throttle<Bot>, msg: &Message,
    ) -> ResponseResult<()> {
        let results = self.get_results(link.to_string()).await;
        self.send_results(results, bot, msg, link).await?;

        Ok(())
    }
    async fn get_results(&self, link: String) -> RapidApiResults;
    async fn send_results(
        &self, results: RapidApiResults, bot: &Throttle<Bot>, msg: &Message, link: &str,
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

                log::info!(
                    "{}",
                    format!("User: {} -> {}", logger::get_sender_identifier(msg), link)
                );
            },
            Err(err) => {
                let error_text = match err {
                    ErrorType::Backend(ref specific_err) => {
                        log::error!(
                            "{}",
                            format!(
                                "User: {}. {} -> ErrQuery: {}",
                                specific_err,
                                logger::get_sender_identifier(msg),
                                link,
                            )
                        );

                        format!("{}", t!(err.to_string()))
                    },
                    ErrorType::User(specific_err) => {
                        log::warn!(
                            "{}",
                            format!(
                                "User: {} -> ErrQuery: {}",
                                logger::get_sender_identifier(msg),
                                link
                            )
                        );

                        format!("{}", t!(specific_err.to_string()))
                    },
                };

                bot.send_message(msg.chat.id, error_text)
                    .parse_mode(ParseMode::Html)
                    .await?;
            },
        }

        Ok(())
    }
}
