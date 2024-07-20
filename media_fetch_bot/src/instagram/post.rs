use serde_json::Value;
use crate::errors::user_input::UserInputError;
use crate::instagram::raw_media::RawMedia;

pub fn parse_json(json: Value) -> Result<(String, Vec<RawMedia>), UserInputError> {
    let mut results: Vec<RawMedia> = vec![];

    let data = &json["data"];

    let mut caption = String::new();
    if let Value::String(value) = &data["caption"] {
        caption = value.to_string();
    }

    let main_media = form_raw_media(&data["main_media_type"],
                                    &data["main_media_hd"])?;
    results.push(main_media);
    
    if let Value::Array(child_media_vector) = &data["child_medias_hd"] {
        for value in child_media_vector {
            let raw_media = form_raw_media(&value["type"],
                                           &value["url"])?;

            results.push(raw_media);
        }
    }

    Ok((caption, results))
}

fn form_raw_media(media_type: &Value, url: &Value) -> Result<RawMedia, UserInputError> {
    let media_type_unpacked = match media_type {
        Value::String(main_media_type) => Ok(main_media_type),
        _ => Err(UserInputError::NoResult)
    }?;

    let media_url = match url {
        Value::String(url) => Ok(url),
        _ => Err(UserInputError::NoResult)
    }?;

    let raw_media = if media_type_unpacked == "video" {
        RawMedia::video(media_url.to_string())
    } else if media_type_unpacked == "image" {
        RawMedia::image(media_url.to_string())
    } else {
        return Err(UserInputError::NoResult);
    };

    Ok(raw_media)
}