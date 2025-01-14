use crate::api::instagram::{post, story};
use crate::api::Response;
use crate::errors::api::ApiError;
use crate::errors::error_type::ErrorType;
use crate::media::RawMedia;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use serde_json::Value;

pub enum ContentType {
    Post,
    Story,
}

impl ContentType {
    pub fn choose(link: &str) -> Self {
        if link.contains("stories") {
            Self::Story
        } else {
            Self::Post
        }
    }
}

pub async fn get_response(api_key: &str, link: &str) -> Result<Response, ErrorType> {
    let content_type = ContentType::choose(link);
    let json_response = request(api_key, &content_type, link).await?;
    let deserialized_json: Value = serde_json::from_str(&json_response)
        .map_err(|_| ApiError::FailedParseResponse)?;

    let response = match content_type {
        ContentType::Post => post::parse_response(deserialized_json)?,
        ContentType::Story => story::parse_response(deserialized_json)?,
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
    api_key: &str, content_type: &ContentType, link: &str,
) -> Result<String, ApiError> {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let host_value: HeaderValue = "instagram-bulk-scraper-latest.p.rapidapi.com"
        .parse()
        .map_err(|_| ApiError::WrongApiHost)?;
    headers.insert("x-rapidapi-host", host_value);

    let key_value: HeaderValue = api_key.parse().map_err(|_| ApiError::WrongApiKey)?;
    headers.insert("x-rapidapi-key", key_value);

    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let endpoint = match content_type {
        ContentType::Post => "media_download_from_url",
        ContentType::Story => "download_story_from_url",
    };
    let endpoint_url = format!(
        "https://instagram-bulk-scraper-latest.p.rapidapi.com/{}",
        endpoint
    );
    let request_body = format!("{{\"url\":\"{}\"}}", link);

    let response = client
        .post(endpoint_url)
        .headers(headers)
        .body(request_body)
        .send()
        .await
        .map_err(|_| ApiError::FailedGetResponse)?;

    if response.status().is_client_error() {
        return Err(ApiError::InstagramQuotaExceeded);
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
