use std::collections::HashMap;
use crate::error::BotError;
use serde_json::Value;
use reqwest::{header};
use reqwest::header::{HeaderValue};
use url::{ParseError, Url};
use teloxide::types::{InputFile, InputMedia, InputMediaAudio, InputMediaPhoto, InputMediaVideo};
use crate::tiktok::media_format::MediaFormat;
use crate::tiktok::raw_media::RawMedia;

pub async fn process_link(tiktok_api_key: &Option<String>, link: String)
                          -> Result<(String, HashMap<MediaFormat, Vec<InputMedia>>), BotError> {
    let tiktok_api_key: &str = match tiktok_api_key {
        None => { return Err(BotError::ApiKeyTiktokMissing) }
        Some(value) => value
    };

    let response = get_response(tiktok_api_key, link).await;
    let response = match response {
        Ok(value) => value,
        Err(_) => { return Err(BotError::FailedGetResponse) }
    };

    let response_results = parse_response(response)?;
    let mut files: HashMap<MediaFormat, Vec<InputMedia>>  = HashMap::new();

    // Parsing vector of results
    let response_documents = response_results.1;
    for raw_media in response_documents {
        let href = raw_media.href;

        let url: Result<Url, ParseError> = href.parse();
        let url = match url {
            Ok(value) => value,
            Err(_) => { return Err(BotError::FailedParseUrl); }
        };

        let file = InputFile::url(url);
        let file = match &raw_media.format {
            MediaFormat::Image => { InputMedia::Photo(InputMediaPhoto::new(file)) }
            MediaFormat::Music => { InputMedia::Audio(InputMediaAudio::new(file)) }
            MediaFormat::Video => { InputMedia::Video(InputMediaVideo::new(file)) }
        };

        let vector = files.entry(raw_media.format).or_default();
        vector.push(file);
    }

    Ok((response_results.0, files))
}

async fn get_response(tiktok_api_key: &str, link: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("x-rapidapi-host", "tiktok-download-without-watermark.p.rapidapi.com".parse().unwrap());

    let key_value: HeaderValue = match tiktok_api_key.parse() {
        Ok(value) => value,
        Err(_) => { return Err(BotError::WrongApiKey.into()); }
    };
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

fn parse_response(response: String) -> Result<(String, Vec<RawMedia>), BotError> {
    let parsed_response: serde_json::error::Result<Value> = serde_json::from_str(&response);
    let parsed_response = match parsed_response {
        Ok(value) => value,
        Err(_) => { return Err(BotError::FailedParseResponse) }
    };

    let mut results: Vec<RawMedia> = vec![];

    let data = &parsed_response["data"];

    let title: String = match &data["title"] {
        Value::String(value) => value.to_string(),
        _ => String::new()
    };

    let play = match &data["play"] {
        Value::String(value) => value.to_string(),
        _ => { return Err(BotError::NoResult); }
    };

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