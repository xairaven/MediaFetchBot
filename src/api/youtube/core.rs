use crate::api::{ApiError, Response};
use crate::error::{Error, UserOutputError};
use crate::media::RawMedia;
use reqwest::header;
use reqwest::header::HeaderValue;
use serde_json::Value;

pub async fn get_response(api_key: &str, link: &str) -> Result<Response, Error> {
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

    let host_value: HeaderValue = "youtube-video-mp3-downloader-api.p.rapidapi.com"
        .parse()
        .map_err(|_| ApiError::WrongApiHost)?;
    headers.insert("x-rapidapi-host", host_value);

    let key_value: HeaderValue = api_key.parse().map_err(|_| ApiError::WrongApiKey)?;
    headers.insert("x-rapidapi-key", key_value);

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|_| ApiError::ClientBuildingFailed)?;

    let request_body = format!(
        "https://youtube-video-mp3-downloader-api.p.rapidapi.com/download?url={}",
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
    pub title: Option<String>,
    pub media: Vec<RawMedia>,
}

fn parse_response(json: Value) -> Result<ParsedResponse, UserOutputError> {
    let mut results: Vec<RawMedia> = vec![];

    let data = &json["data"];

    let title = data["title"].as_str().map(|s| s.to_string());

    let medias = match &data["medias"] {
        Value::Array(array) => array,
        _ => {
            return Err(UserOutputError::NoResult);
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
            return Err(UserOutputError::NoResult);
        }
    }

    Ok(ParsedResponse {
        title,
        media: results,
    })
}
