use crate::bot_config::BotConfig;
use crate::errors::error_type::ErrorType;
use crate::instagram::InstagramInstance;
use crate::logger;
use crate::rapid_api::media_format::MediaFormat;
use crate::tiktok::TikTokInstance;
use async_trait::async_trait;
use rust_i18n::t;
use std::collections::HashMap;
use teloxide::adaptors::Throttle;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{Message, Requester, ResponseResult};
use teloxide::types::{InputMedia, ParseMode};
use teloxide::Bot;

pub mod media_format;
pub mod raw_media;

pub type InputMediaMap = HashMap<MediaFormat, Vec<InputMedia>>;
pub type RapidApiResults = Result<(String, InputMediaMap), ErrorType>;
#[async_trait]
pub trait ApiHandler {
    fn link_base(&self) -> String;

    async fn handle_link(
        &self, link: &str, bot: &Throttle<Bot>, msg: &Message,
    ) -> ResponseResult<()> {
        let results = self.get_results(link.to_string()).await;
        self.send_results(results, bot, msg, link).await?;

        Ok(())
    }
    async fn get_results(&self, link: String) -> RapidApiResults;
    async fn send_results(
        &self, results: RapidApiResults, bot: &Throttle<Bot>, msg: &Message,
        link: &str,
    ) -> ResponseResult<()> {
        Ok(send_results(results, bot, msg, link).await?)
    }
}

pub fn api_factory(
    config: &BotConfig,
) -> Vec<Box<dyn ApiHandler + Sync + Send>> {
    let mut structs: Vec<Box<dyn ApiHandler + Sync + Send>> = vec![];

    if let Some(api_key) = &config.tiktok_api_key {
        let instance = TikTokInstance::new(api_key.clone());
        structs.push(Box::new(instance));
    }

    if let Some(api_key) = &config.instagram_api_key {
        let instance = InstagramInstance::new(api_key.clone());
        structs.push(Box::new(instance));
    }

    structs
}

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

            log::info!(
                "{}",
                format!(
                    "User: {} -> {}",
                    logger::get_sender_identifier(msg),
                    link
                )
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
