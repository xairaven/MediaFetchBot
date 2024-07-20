use crate::rapid_api::media_format::MediaFormat;

pub struct RawMedia {
    pub href: String,
    pub format: MediaFormat,
}

impl RawMedia {
    pub fn new(href: String, format: MediaFormat) -> Self {
        Self { href, format }
    }

    pub fn video(href: String) -> Self {
        Self::new(href, MediaFormat::Video)
    }

    pub fn image(href: String) -> Self {
        Self::new(href, MediaFormat::Image)
    }

    pub fn music(href: String) -> Self {
        Self::new(href, MediaFormat::Music)
    }
}