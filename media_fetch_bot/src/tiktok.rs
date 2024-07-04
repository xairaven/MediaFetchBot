use crate::error::BotError;
use serde_json::Value;
use reqwest::{header};
use reqwest::header::{HeaderValue};
use url::{ParseError, Url};
use teloxide::types::{InputFile};

pub async fn process_link(tiktok_api_key: &Option<String>, link: String) -> Result<InputFile, BotError> {
    let tiktok_api_key : &str = match tiktok_api_key {
        None => { return Err(BotError::ApiKeyTiktokMissing)},
        Some(value) => value
    };

    let response = get_response(tiktok_api_key, link).await;
    let response = match response {
        Ok(value) => value,
        Err(_) => { return Err(BotError::FailedGetResponse) }
    };

    let href = parse_response(response)?;

    let url : Result<Url, ParseError> = href.parse();
    let url = match url {
        Ok(value) => value,
        Err(_) => { return Err(BotError::FailedParseUrl); }
    };

    let file = InputFile::url(url);

    Ok(file)
}

async fn get_response(tiktok_api_key: &str, link: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("x-rapidapi-host", "tiktok-download-without-watermark.p.rapidapi.com".parse().unwrap());

    let key_value : HeaderValue = match tiktok_api_key.parse() {
        Ok(value) => value,
        Err(_) => { return Err(BotError::WrongApiKey.into());}
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

fn parse_response(response: String) -> Result<String, BotError> {
    let parsed_response : serde_json::error::Result<Value> = serde_json::from_str(&response);
    let parsed_response = match parsed_response {
        Ok(value) => value,
        Err(_) => { return Err(BotError::FailedParseResponse) }
    };

    let data = &parsed_response["data"];

    let link = &data["play"];
    match link {
        Value::String(value) => Ok(value.to_string()),
        _ => Err(BotError::NoResult)
    }
}