use serde_json::Value;
use crate::errors::user_input::UserInputError;
use crate::instagram::raw_media::RawMedia;

pub fn parse_json(json: Value) -> Result<Vec<RawMedia>, UserInputError>{
    let mut result_vector: Vec<RawMedia> = Vec::new();

    let data = &json["data"];

    if let Value::String(video_url) = &data["video_hd"] {
        let raw_media = RawMedia::Video(video_url.to_string());
        result_vector.push(raw_media);
        return Ok(result_vector);
    }

    if let Value::String(img_url) = &data["image_hd"] {
        let raw_media = RawMedia::Image(img_url.to_string());
        result_vector.push(raw_media);
        return Ok(result_vector);
    } else {
        Err(UserInputError::NoResult)
    }
}