use serde_json::Value;
use crate::errors::user_input::UserInputError;
use crate::instagram::media_format::MediaFormat;
use crate::instagram::raw_media::RawMedia;

pub fn parse_json(json: Value) -> Result<Vec<RawMedia>, UserInputError>{
    let data = &json["data"];

    let href: Option<String> = match &data["video_hd"] {
        Value::String(value) => Some(value.to_string()),
        _ => None
    };

    let mut result_vector: Vec<RawMedia> = Vec::new();

    if let Some(video_url) = href {
        let raw_media = RawMedia::new(video_url, MediaFormat::Video);
        result_vector.push(raw_media);
        return Ok(result_vector);
    }

    match &data["image_hd"] {
        Value::String(img_url) => {
            let raw_media = RawMedia::new(img_url.to_string(), MediaFormat::Image);
            result_vector.push(raw_media);
            Ok(result_vector)
        },
        _ => Err(UserInputError::NoResult)
    }
}