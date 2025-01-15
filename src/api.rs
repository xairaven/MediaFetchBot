use crate::api::instagram::core::ContentType;
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
}
mod tiktok {
    pub mod core;
}

pub enum Api {
    TikTok { key: String },
    InstagramPhotos { key: String },
    InstagramReels { key: String },
    InstagramStories { key: String },
}

impl Api {
    pub fn base_url(&self) -> String {
        match self {
            Api::InstagramPhotos { .. } => String::from("instagram.com/p/"),
            Api::InstagramReels { .. } => String::from("instagram.com/reel/"),
            Api::InstagramStories { .. } => String::from("instagram.com/stories/"),
            Api::TikTok { .. } => String::from("tiktok.com"),
        }
    }

    pub fn instances_from_config(config: &BotConfig) -> Vec<Self> {
        let mut instances = vec![];

        if let Some(key) = &config.tiktok_api_key {
            let api = Api::TikTok { key: key.clone() };
            instances.push(api);
        }

        if let Some(key) = &config.instagram_photos_api_key {
            let api = Api::InstagramPhotos { key: key.clone() };
            instances.push(api);
        }

        if let Some(key) = &config.instagram_reels_api_key {
            let api = Api::InstagramReels { key: key.clone() };
            instances.push(api);
        }

        if let Some(key) = &config.instagram_stories_api_key {
            let api = Api::InstagramStories { key: key.clone() };
            instances.push(api);
        }

        instances
    }

    pub async fn handle_link(
        &self, link: String, bot: &Throttle<Bot>, msg: &Message,
    ) -> ResponseResult<()> {
        let response = match self {
            Api::TikTok { key } => tiktok::core::get_response(key, &link).await,
            Api::InstagramPhotos { key } => {
                instagram::core::get_response(key, &link, ContentType::Photos).await
            },
            Api::InstagramReels { key } => {
                instagram::core::get_response(key, &link, ContentType::Reels).await
            },
            Api::InstagramStories { key } => {
                instagram::core::get_response(key, &link, ContentType::Stories).await
            },
        };

        match response {
            Ok(response) => response.send(bot, msg, &link).await?,
            Err(err) => {
                let error_text = match err {
                    Error::Server(ref specific_err) => {
                        log::error!(
                            "{}",
                            format!(
                                "User: {}. Error: {}. Query: {}",
                                logger::get_sender_identifier(msg),
                                specific_err,
                                link,
                            )
                        );

                        format!("{}", t!(err.to_string()))
                    },
                    Error::User(specific_err) => {
                        log::warn!(
                            "{}",
                            format!(
                                "User: {}. Error: {}. Query: {}",
                                logger::get_sender_identifier(msg),
                                specific_err,
                                link,
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
    pub title: Option<String>,
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

        if let Some(title) = self.title {
            bot.send_message(msg.chat.id, title).await?;
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
    #[error("ClientBuildingFailed")]
    ClientBuildingFailed,

    #[error("FailedGetResponse")]
    FailedGetResponse,

    #[error("FailedParseResponse")]
    FailedParseResponse,

    #[error("FailedParseUrl")]
    FailedParseUrl,

    #[error("TiktokQuotaExceeded")]
    TiktokQuotaExceeded,

    #[error("InstagramPhotosQuotaExceeded")]
    InstagramPhotosQuotaExceeded,

    #[error("InstagramReelsQuotaExceeded")]
    InstagramReelsQuotaExceeded,

    #[error("InstagramStoriesQuotaExceeded")]
    InstagramStoriesQuotaExceeded,

    #[error("WrongApiHost")]
    WrongApiHost,

    #[error("WrongApiKey")]
    WrongApiKey,
}
