use std::fmt;
use std::fmt::{Display, Formatter};

pub enum LinkType {
    TikTok,
    Instagram
}

impl Display for LinkType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            LinkType::TikTok => write!(f, "tiktok.com"),
            LinkType::Instagram => write!(f, "instagram.com"),
        }
    }
}