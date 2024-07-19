use reqwest::header;
use reqwest::header::HeaderValue;
use serde_json::Value;
use std::collections::HashMap;
use teloxide::types::{InputFile, InputMedia, InputMediaAudio, InputMediaPhoto, InputMediaVideo};
use url::{Url};

use crate::errors::api::ApiError;
use crate::errors::error_type::ErrorType;
use crate::errors::user_input::UserInputError;
use crate::tiktok::media_format::MediaFormat;
use crate::tiktok::raw_media::RawMedia;
use crate::utils::response_processing;

type InputMediaMap = HashMap<MediaFormat, Vec<InputMedia>>;

pub async fn get_results(tiktok_api_key: Option<String>, link: String)
                         -> Result<(String, InputMediaMap), ErrorType> {
    let tiktok_api_key: String = tiktok_api_key.ok_or(ApiError::ApiKeyTiktokMissing)?;

    let response = get_response(&tiktok_api_key, link).await
        .map_err(|_| ApiError::FailedGetResponse)?;
    let json = response_processing::to_json(response)?;

    let (post_title, raw_media_documents) = parse_json(json)?;

    let input_media_map = convert_raw_to_input_media(raw_media_documents)?;

    Ok((post_title, input_media_map))
}

async fn get_response(tiktok_api_key: &str, link: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();

    let host_value: HeaderValue = "tiktok-download-without-watermark.p.rapidapi.com".parse()
        .map_err(|_| ApiError::WrongApiHost)?;
    headers.insert("x-rapidapi-host", host_value);

    let key_value: HeaderValue = tiktok_api_key.parse()
        .map_err(|_| ApiError::WrongApiKey)?;
    headers.insert("x-rapidapi-key", key_value);

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let request_body = format!("https://tiktok-download-without-watermark.p.rapidapi.com/analysis?url={}&hd=0", link);

    let response = client.get(request_body)
        .headers(headers)
        .send().await?
        .text().await?;

    Ok(response)
}

fn parse_json(json: Value) -> Result<(String, Vec<RawMedia>), UserInputError> {
    let mut results: Vec<RawMedia> = vec![];

    let data = &json["data"];

    let title: String = match &data["title"] {
        Value::String(value) => value.to_string(),
        _ => String::new()
    };

    let play = match &data["play"] {
        Value::String(value) => value.to_string(),
        _ => { return Err(UserInputError::NoResult); }
    };

    // Supposing that if there "images" field -- then, it's photo-slide format.
    // So, "play" field -- link to mp3 music.
    // Otherwise -- link to mp4 video. No "images" field.
    if let Value::Array(vector) = &data["images"] {
        results.push(RawMedia::new(play, MediaFormat::Music));

        for value in vector {
            if let Value::String(link) = value {
                results.push(RawMedia::new(link.to_string(), MediaFormat::Image));
            }
        }
    } else { results.push(RawMedia::new(play, MediaFormat::Video)); }

    Ok((title, results))
}

fn convert_raw_to_input_media(raw_media_documents: Vec<RawMedia>) -> Result<InputMediaMap, ApiError> {

    let mut files: InputMediaMap = HashMap::new();
    // Parsing vector of results
    for raw_media in raw_media_documents {
        let href = raw_media.href;

        let url: Url = href.parse()
            .map_err(|_| ApiError::FailedParseUrl)?;

        let file = InputFile::url(url);
        let file = match &raw_media.format {
            MediaFormat::Image => { InputMedia::Photo(InputMediaPhoto::new(file)) }
            MediaFormat::Music => { InputMedia::Audio(InputMediaAudio::new(file)) }
            MediaFormat::Video => { InputMedia::Video(InputMediaVideo::new(file)) }
        };

        let vector = files.entry(raw_media.format).or_default();
        vector.push(file);
    }

    Ok(files)
}