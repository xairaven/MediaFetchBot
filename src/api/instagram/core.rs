use crate::api::{ApiError, Response};
use crate::error::{Error, UserInputError};
use crate::media::RawMedia;
use reqwest::header;
use reqwest::header::HeaderValue;
use serde_json::Value;

pub enum ContentType {
    Photos,
    Reels,
    Stories,
}

pub async fn get_response(
    api_key: &str, link: &str, content_type: ContentType,
) -> Result<Response, Error> {
    let json_response = request(api_key, link, &content_type).await?;
    let deserialized_json: Value = serde_json::from_str(&json_response)
        .map_err(|_| ApiError::FailedParseResponse)?;
    let response = match content_type {
        ContentType::Photos | ContentType::Reels => {
            parse_response_post_reels(deserialized_json)?
        },
        ContentType::Stories => parse_response_story(deserialized_json)?,
    };

    let mut input_media = vec![];
    for raw in response.media {
        input_media.push(raw.to_input_media()?);
    }

    let response = Response {
        title: response.title,
        media: input_media,
    };
    Ok(response)
}

async fn request(
    api_key: &str, link: &str, content_type: &ContentType,
) -> Result<String, ApiError> {
    let mut headers = header::HeaderMap::new();

    let endpoint = match content_type {
        ContentType::Photos => "photos",
        ContentType::Reels => "reels",
        ContentType::Stories => "story",
    };

    let host_value: HeaderValue =
        format!("instagram-{}-downloader-api.p.rapidapi.com", endpoint)
            .parse()
            .map_err(|_| ApiError::WrongApiHost)?;
    headers.insert("x-rapidapi-host", host_value);

    let key_value: HeaderValue = api_key.parse().map_err(|_| ApiError::WrongApiKey)?;
    headers.insert("x-rapidapi-key", key_value);

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|_| ApiError::ClientBuildingFailed)?;

    let request_body = match content_type {
        ContentType::Photos | ContentType::Reels => format!(
            "https://instagram-{}-downloader-api.p.rapidapi.com/download?url={}",
            endpoint, link
        ),
        ContentType::Stories => format!(
            "https://instagram-{}-downloader-api.p.rapidapi.com/download?link={}",
            endpoint, link
        ),
    };

    let response = client
        .get(request_body)
        .headers(headers)
        .send()
        .await
        .map_err(|_| ApiError::FailedGetResponse)?;

    if response.status().is_client_error() {
        return match content_type {
            ContentType::Photos => Err(ApiError::InstagramPhotosQuotaExceeded),
            ContentType::Reels => Err(ApiError::InstagramReelsQuotaExceeded),
            ContentType::Stories => Err(ApiError::InstagramStoriesQuotaExceeded),
        };
    }

    let response_text = response
        .text()
        .await
        .map_err(|_| ApiError::FailedGetResponse)?;

    Ok(response_text)
}

pub struct ParsedResponse {
    pub title: Option<String>,
    pub media: Vec<RawMedia>,
}

fn parse_response_post_reels(json: Value) -> Result<ParsedResponse, UserInputError> {
    let mut results: Vec<RawMedia> = vec![];

    let data = &json["data"];

    let title: Option<String> = match &data["title"] {
        Value::String(value) => Some(value.to_string()),
        _ => None,
    };

    let medias = match &data["medias"] {
        Value::Array(array) => array,
        _ => {
            return Err(UserInputError::NoResult);
        },
    };

    for value in medias {
        if let (Value::String(url), Value::String(media_type)) =
            (&value["url"], &value["type"])
        {
            if media_type.eq("video") {
                results.push(RawMedia::video(url.to_string()));
            } else if media_type.eq("image") {
                results.push(RawMedia::image(url.to_string()));
            }
        } else {
            return Err(UserInputError::NoResult);
        }
    }

    Ok(ParsedResponse {
        title,
        media: results,
    })
}

fn parse_response_story(json: Value) -> Result<ParsedResponse, UserInputError> {
    let mut results: Vec<RawMedia> = vec![];

    let data = &json["data"];
    let download_info = &data["downloadInfo"];
    let title: Option<String> = if let Value::String(author) = &download_info["author"] {
        Some(format!("Author: {}", author))
    } else {
        None
    };

    if let Value::Bool(error) = &download_info["error"] {
        if *error {
            let additional_info = download_info["message"]
                .as_str()
                .map(|error_message| error_message.to_string());
            return Err(UserInputError::InstagramFailedGetContent(additional_info));
        }
    }

    let medias = match &download_info["medias"] {
        Value::Array(array) => array,
        _ => {
            return Err(UserInputError::NoResult);
        },
    };

    for value in medias {
        if let (Value::String(url), Value::String(media_type)) =
            (&value["url"], &value["type"])
        {
            if media_type.eq("video") {
                results.push(RawMedia::video(url.to_string()));
            } else if media_type.eq("image") {
                results.push(RawMedia::image(url.to_string()));
            }
        } else {
            return Err(UserInputError::NoResult);
        }
    }

    Ok(ParsedResponse {
        title,
        media: results,
    })
}
