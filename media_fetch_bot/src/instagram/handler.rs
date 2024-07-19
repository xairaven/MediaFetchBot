use reqwest::{Client};
use reqwest::header::{HeaderMap, HeaderValue};
use crate::instagram::content_type::ContentType;
use crate::instagram::errors::api_error::ApiError;
use crate::instagram::errors::error_type::ErrorType;

pub async fn get_results(api_key: Option<String>, link: String) -> Result<(String), ErrorType> {
    let api_key: String = api_key.ok_or(ApiError::ApiKeyInstagramMissing)?;

    let content_type = ContentType::choose(&link);
    let response = get_response(&api_key, content_type, link).await
        .map_err(|_| ApiError::FailedGetResponse)?;

    Ok(response)
}

async fn get_response(api_key: &str, content_type: ContentType, link: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let host_value: HeaderValue = "instagram-bulk-scraper-latest.p.rapidapi.com".parse()
        .map_err(|_| ApiError::WrongApiHost)?;
    headers.insert("x-rapidapi-host", host_value);

    let key_value: HeaderValue = api_key.parse()
        .map_err(|_| ApiError::WrongApiKey)?;
    headers.insert("x-rapidapi-key", key_value);

    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let endpoint = match content_type {
        ContentType::Post => "media_download_from_url",
        ContentType::Story => "download_story_from_url"
    };
    let endpoint_url = format!("https://instagram-bulk-scraper-latest.p.rapidapi.com/{}", endpoint);
    let request_body = format!("{{\"url\":\"{}\"}}", link);

    let response = client.post(endpoint_url)
        .headers(headers)
        .body(request_body)
        .send().await?
        .text().await?;

    Ok(response)
}