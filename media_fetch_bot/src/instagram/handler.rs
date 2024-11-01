use crate::errors::api::ApiError;
use crate::instagram::content_type::ContentType;
use crate::instagram::{post, story};
use crate::rapid_api::media_format::MediaFormat;
use crate::rapid_api::raw_media::RawMedia;
use crate::rapid_api::{InputMediaMap, RapidApiResults};
use crate::utils::response_processing;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use std::collections::HashMap;
use teloxide::types::{
    InputFile, InputMedia, InputMediaPhoto, InputMediaVideo,
};
use url::Url;

pub async fn get_results(api_key: &str, link: String) -> RapidApiResults {
    let content_type = ContentType::choose(&link);
    let response = get_response(api_key, &content_type, link).await?;
    let json = response_processing::to_json(response)?;

    let (caption, raw_medias) = match content_type {
        ContentType::Post => post::parse_json(json)?,
        ContentType::Story => story::parse_json(json)?,
    };

    let input_media_map = convert_raw_to_input_media(raw_medias)?;

    Ok((caption, input_media_map))
}

async fn get_response(
    api_key: &str, content_type: &ContentType, link: String,
) -> Result<String, ApiError> {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let host_value: HeaderValue =
        "instagram-bulk-scraper-latest.p.rapidapi.com"
            .parse()
            .map_err(|_| ApiError::WrongApiHost)?;
    headers.insert("x-rapidapi-host", host_value);

    let key_value: HeaderValue =
        api_key.parse().map_err(|_| ApiError::WrongApiKey)?;
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

fn convert_raw_to_input_media(
    vec: Vec<RawMedia>,
) -> Result<InputMediaMap, ApiError> {
    let mut files: InputMediaMap = HashMap::new();

    for raw_media in vec {
        let href = raw_media.href;

        let url: Url = href.parse().map_err(|_| ApiError::FailedParseUrl)?;

        let file = InputFile::url(url);
        let file = match &raw_media.format {
            MediaFormat::Image => InputMedia::Photo(InputMediaPhoto::new(file)),
            MediaFormat::Video => InputMedia::Video(InputMediaVideo::new(file)),
            _ => {
                return Err(ApiError::WrongMediaFormat);
            },
        };

        let vector = files.entry(raw_media.format).or_default();
        vector.push(file);
    }

    Ok(files)
}
