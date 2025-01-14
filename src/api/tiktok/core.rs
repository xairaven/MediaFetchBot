use crate::api::Response;
use crate::errors::api::ApiError;
use crate::errors::error_type::ErrorType;
use crate::errors::user_input::UserInputError;
use crate::media::RawMedia;
use reqwest::header;
use reqwest::header::HeaderValue;
use serde_json::Value;

pub async fn get_response(api_key: &str, link: &str) -> Result<Response, ErrorType> {
    let json_response = request(api_key, link).await?;
    let deserialized_json: Value = serde_json::from_str(&json_response)
        .map_err(|_| ApiError::FailedParseResponse)?;
    let response = parse_response(deserialized_json)?;

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

async fn request(api_key: &str, link: &str) -> Result<String, ApiError> {
    let mut headers = header::HeaderMap::new();

    let host_value: HeaderValue = "tiktok-download-without-watermark.p.rapidapi.com"
        .parse()
        .map_err(|_| ApiError::WrongApiHost)?;
    headers.insert("x-rapidapi-host", host_value);

    let key_value: HeaderValue = api_key.parse().map_err(|_| ApiError::WrongApiKey)?;
    headers.insert("x-rapidapi-key", key_value);

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let request_body = format!(
        "https://tiktok-download-without-watermark.p.rapidapi.com/analysis?url={}&hd=0",
        link
    );

    let response = client
        .get(request_body)
        .headers(headers)
        .send()
        .await
        .map_err(|_| ApiError::FailedGetResponse)?;

    if response.status().is_client_error() {
        return Err(ApiError::TiktokQuotaExceeded);
    }

    let response_text = response
        .text()
        .await
        .map_err(|_| ApiError::FailedGetResponse)?;

    Ok(response_text)
}

pub struct ParsedResponse {
    pub title: String,
    pub media: Vec<RawMedia>,
}

fn parse_response(json: Value) -> Result<ParsedResponse, UserInputError> {
    let mut results: Vec<RawMedia> = vec![];

    let data = &json["data"];

    let title: String = match &data["title"] {
        Value::String(value) => value.to_string(),
        _ => String::new(),
    };

    let play = match &data["play"] {
        Value::String(value) => value.to_string(),
        _ => {
            return Err(UserInputError::NoResult);
        },
    };

    // Supposing that if there "images" field -- then, it's photo-slide format.
    // So, "play" field -- link to mp3 music.
    // Otherwise -- link to mp4 video. No "images" field.
    if let Value::Array(vector) = &data["images"] {
        results.push(RawMedia::music(play));

        for value in vector {
            if let Value::String(link) = value {
                results.push(RawMedia::image(link.to_string()));
            }
        }
    } else {
        results.push(RawMedia::video(play));
    }

    Ok(ParsedResponse {
        title,
        media: results,
    })
}
