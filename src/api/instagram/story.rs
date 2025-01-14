use crate::api::instagram::core::ParsedResponse;
use crate::errors::user_input::UserInputError;
use crate::media::RawMedia;
use serde_json::Value;

pub fn parse_response(json: Value) -> Result<ParsedResponse, UserInputError> {
    let mut results: Vec<RawMedia> = Vec::new();

    let data = &json["data"];

    let mut caption = String::new();
    if let Value::Object(owner) = &data["owner"] {
        let username_option = owner.get("username");
        if let Some(Value::String(username)) = username_option {
            caption = format!("Story from {}.", username);
        }
    }

    if let Value::String(video_url) = &data["video_hd"] {
        let raw_media = RawMedia::video(video_url.to_string());
        results.push(raw_media);
        return Ok(ParsedResponse {
            title: caption,
            media: results,
        });
    }

    if let Value::String(img_url) = &data["image_hd"] {
        let raw_media = RawMedia::image(img_url.to_string());
        results.push(raw_media);
        Ok(ParsedResponse {
            title: caption,
            media: results,
        })
    } else {
        Err(UserInputError::NoResult)
    }
}
