use crate::instagram::content_type::ContentType::{Post, Story};

pub enum ContentType {
    Post,
    Story
}

impl ContentType {
    pub fn choose(link: &str) -> ContentType {
        if link.contains("stories") {
            Story
        } else {
            Post
        }
    }
}