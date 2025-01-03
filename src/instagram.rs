use crate::rapid_api::{ApiHandler, RapidApiResults};
use async_trait::async_trait;

mod content_type;
mod handler;
mod post;
mod story;

pub struct InstagramApi {
    api_key: String,
    base_url: String,
}

impl InstagramApi {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: String::from("instagram.com"),
        }
    }
}

#[async_trait]
impl ApiHandler for InstagramApi {
    fn base_url(&self) -> String {
        self.base_url.clone()
    }

    async fn get_results(&self, link: String) -> RapidApiResults {
        Ok(handler::get_results(&self.api_key, link.to_string()).await?)
    }
}
