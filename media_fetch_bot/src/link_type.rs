use std::fmt;
use std::fmt::{Display, Formatter};

pub enum LinkType {
    TikTok,
}

impl Display for LinkType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            LinkType::TikTok => write!(f, "tiktok.com"),
        }
    }
}