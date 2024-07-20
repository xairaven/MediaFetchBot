use crate::instagram::media_format::MediaFormat;

pub struct RawMedia {
    pub href: String,
    pub format: MediaFormat,
}

impl RawMedia {
    pub fn new(href: String, format: MediaFormat) -> Self {
        RawMedia { href, format }
    }
}