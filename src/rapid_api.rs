use crate::bot::config::BotConfig;
use crate::errors::error_type::ErrorType;
use crate::instagram::InstagramApi;
use crate::rapid_api::instance::ApiInstance;
use crate::rapid_api::media::MediaFormat;
use crate::tiktok::TikTokApi;
use std::collections::HashMap;
use teloxide::types::InputMedia;

pub mod instance;
pub mod media;

pub type InputMediaMap = HashMap<MediaFormat, Vec<InputMedia>>;
pub type RapidApiResults = Result<(String, InputMediaMap), ErrorType>;

pub fn api_factory(config: &BotConfig) -> Vec<Box<dyn ApiInstance + Sync + Send>> {
    let mut structs: Vec<Box<dyn ApiInstance + Sync + Send>> = vec![];

    if let Some(api_key) = &config.tiktok_api_key {
        structs.push(Box::new(TikTokApi::new(api_key.clone())));
    }

    if let Some(api_key) = &config.instagram_api_key {
        structs.push(Box::new(InstagramApi::new(api_key.clone())));
    }

    structs
}
