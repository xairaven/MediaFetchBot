use crate::rapid_api::{ApiHandler, RapidApiResults};
use async_trait::async_trait;

pub mod handler;

pub struct TikTokInstance {
    api_key: String,
    link_base: String,
}

impl TikTokInstance {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            link_base: String::from("tiktok.com"),
        }
    }
}

#[async_trait]
impl ApiHandler for TikTokInstance {
    fn link_base(&self) -> String {
        self.link_base.clone()
    }

    async fn get_results(&self, link: String) -> RapidApiResults {
        Ok(handler::get_results(&self.api_key, link.to_string()).await?)
    }
}
