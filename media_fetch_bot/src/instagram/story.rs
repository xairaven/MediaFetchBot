use serde_json::Value;
use crate::errors::user_input::UserInputError;
use crate::rapid_api::raw_media::RawMedia;

pub fn parse_json(json: Value) -> Result<(String, Vec<RawMedia>), UserInputError>{
    let mut result_vector: Vec<RawMedia> = Vec::new();

    let data = &json["data"];

    let mut caption = String::new();
    if let Value::Object(owner) = &data["owner"] {
        let username = owner.get("username");
        if let Some(value) = username {
            if let Value::String(username) = value {
                caption = format!("Story from {}.", username);
            }
        }
    }

    if let Value::String(video_url) = &data["video_hd"] {
        let raw_media = RawMedia::video(video_url.to_string());
        result_vector.push(raw_media);
        return Ok((caption, result_vector));
    }

    if let Value::String(img_url) = &data["image_hd"] {
        let raw_media = RawMedia::image(img_url.to_string());
        result_vector.push(raw_media);
        return Ok((caption, result_vector));
    } else {
        Err(UserInputError::NoResult)
    }
}