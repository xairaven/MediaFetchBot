use crate::api::ApiError;
use teloxide::types::{
    InputFile, InputMedia, InputMediaAudio, InputMediaPhoto, InputMediaVideo,
};
use url::Url;

#[derive(PartialEq, Eq, Hash)]
pub enum MediaFormat {
    Image,
    Music,
    Video,
}

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

    pub fn to_input_media(&self) -> Result<InputMedia, ApiError> {
        let url: Url = self.href.parse().map_err(|_| ApiError::FailedParseUrl)?;
        let file = InputFile::url(url);
        let media = match self.format {
            MediaFormat::Image => InputMedia::Photo(InputMediaPhoto::new(file)),
            MediaFormat::Music => InputMedia::Audio(InputMediaAudio::new(file)),
            MediaFormat::Video => InputMedia::Video(InputMediaVideo::new(file)),
        };

        Ok(media)
    }
}
