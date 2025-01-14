use crate::bot::config::BotConfig;
use crate::error::Error;
use crate::logger;
use rust_i18n::t;
use teloxide::adaptors::Throttle;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{Message, Requester, ResponseResult};
use teloxide::types::{InputMedia, ParseMode};
use teloxide::Bot;
use thiserror::Error;

mod instagram {
    pub mod core;

    pub mod post;
    pub mod story;
}
mod tiktok {
    pub mod core;
}

pub enum Api {
    TikTok { key: String },
    Instagram { key: String },
}

impl Api {
    pub fn base_url(&self) -> String {
        match self {
            Api::TikTok { .. } => String::from("tiktok.com"),
            Api::Instagram { .. } => String::from("instagram.com"),
        }
    }

    pub fn instances_from_config(config: &BotConfig) -> Vec<Self> {
        let mut instances = vec![];

        if let Some(key) = &config.tiktok_api_key {
            let api = Api::TikTok { key: key.clone() };
            instances.push(api);
        }

        if let Some(key) = &config.instagram_api_key {
            let api = Api::Instagram { key: key.clone() };
            instances.push(api);
        }

        instances
    }

    pub async fn handle_link(
        &self, link: String, bot: &Throttle<Bot>, msg: &Message,
    ) -> ResponseResult<()> {
        let response = match self {
            Api::TikTok { key } => tiktok::core::get_response(key, &link).await,
            Api::Instagram { key } => instagram::core::get_response(key, &link).await,
        };

        match response {
            Ok(response) => response.send(bot, msg, &link).await?,
            Err(err) => {
                let error_text = match err {
                    Error::Server(ref specific_err) => {
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
                    Error::User(specific_err) => {
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

pub struct Response {
    pub title: String,
    pub media: Vec<InputMedia>,
}

impl Response {
    pub async fn send(
        self, bot: &Throttle<Bot>, msg: &Message, link: &str,
    ) -> ResponseResult<()> {
        let (mut images, mut music, mut videos) = (vec![], vec![], vec![]);

        for input_media in self.media {
            match input_media {
                InputMedia::Photo(_) => images.push(input_media),
                InputMedia::Video(_) => videos.push(input_media),
                InputMedia::Audio(_) => music.push(input_media),
                _ => continue,
            }
        }

        if !images.is_empty() {
            bot.send_media_group(msg.chat.id, images).await?;
        }
        if !videos.is_empty() {
            bot.send_media_group(msg.chat.id, videos).await?;
        }
        if !music.is_empty() {
            bot.send_media_group(msg.chat.id, music).await?;
        }

        if !self.title.is_empty() {
            bot.send_message(msg.chat.id, self.title).await?;
        }

        log::info!(
            "{}",
            format!("User: {} -> {}", logger::get_sender_identifier(msg), link)
        );

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("FailedGetResponse")]
    FailedGetResponse,

    #[error("FailedParseResponse")]
    FailedParseResponse,

    #[error("FailedParseUrl")]
    FailedParseUrl,

    #[error("InstagramQuotaExceeded")]
    InstagramQuotaExceeded,

    #[error("TiktokQuotaExceeded")]
    TiktokQuotaExceeded,

    #[error("WrongApiHost")]
    WrongApiHost,

    #[error("WrongApiKey")]
    WrongApiKey,
}
