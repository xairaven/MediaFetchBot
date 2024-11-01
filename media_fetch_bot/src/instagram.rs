use crate::rapid_api::{ApiHandler, RapidApiResults};
use async_trait::async_trait;

mod content_type;
mod handler;
mod post;
mod story;

pub struct InstagramInstance {
    api_key: String,
    link_base: String,
}

impl InstagramInstance {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            link_base: String::from("instagram.com"),
        }
    }
}

#[async_trait]
impl ApiHandler for InstagramInstance {
    fn link_base(&self) -> String {
        self.link_base.clone()
    }

    async fn get_results(&self, link: String) -> RapidApiResults {
        Ok(handler::get_results(&self.api_key, link.to_string()).await?)
    }
}
