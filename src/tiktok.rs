use crate::rapid_api::manager::ApiInstance;
use crate::rapid_api::RapidApiResults;
use async_trait::async_trait;

pub mod handler;

pub struct TikTokApi {
    api_key: String,
    base_url: String,
}

impl TikTokApi {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: String::from("tiktok.com"),
        }
    }
}

#[async_trait]
impl ApiInstance for TikTokApi {
    fn base_url(&self) -> String {
        self.base_url.clone()
    }

    async fn get_results(&self, link: String) -> RapidApiResults {
        Ok(handler::get_results(&self.api_key, link.to_string()).await?)
    }
}
