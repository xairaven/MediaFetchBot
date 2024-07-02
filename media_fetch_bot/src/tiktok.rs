use crate::error::BotError;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use serde_json::Value;
use reqwest::{header};
use url::{ParseError, Url};
use teloxide::types::InputFile;

pub async fn process_link(link: String) -> Result<InputFile, BotError> {
    let href = get_href(&link).await?;

    let url : Result<Url, ParseError> = href.parse();
    let url = match url {
        Ok(value) => value,
        Err(_) => { return Err(BotError::FailedParseUrl); }
    };

    let file = InputFile::url(url);

    Ok(file)
}

pub async fn get_href(url: &str) -> Result<String, BotError> {
    let response = get_response(url).await;
    let response = match response {
        Ok(value) => value,
        Err(_) => { return Err(BotError::FailedGetResponse) }
    };

    parse_response(response)
}

// Guide: https://www.youtube.com/watch?v=UsT11sOD1JA
async fn get_response(link: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Predefined headers
    let mut headers = header::HeaderMap::new();
    headers.insert("accept", "*/*".parse().unwrap());
    headers.insert("accept-language", "en-US,en;q=0.9".parse().unwrap());
    headers.insert("content-type", "application/x-www-form-urlencoded; charset=UTF-8".parse().unwrap());
    headers.insert("origin", "https://savetik.co".parse().unwrap());
    headers.insert("priority", "u=1, i".parse().unwrap());
    headers.insert("referer", "https://savetik.co/en2".parse().unwrap());
    headers.insert("sec-ch-ua", "\"Not/A)Brand\";v=\"8\", \"Chromium\";v=\"126\", \"Microsoft Edge\";v=\"126\"".parse().unwrap());
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"Windows\"".parse().unwrap());
    headers.insert("sec-fetch-dest", "empty".parse().unwrap());
    headers.insert("sec-fetch-mode", "cors".parse().unwrap());
    headers.insert("sec-fetch-site", "same-origin".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36 Edg/126.0.0.0".parse().unwrap());
    headers.insert("x-requested-with", "XMLHttpRequest".parse().unwrap());

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let post_body = format!("q={link}&lang=en");

    let response = client.post("https://savetik.co/api/ajaxSearch")
        .headers(headers)
        .body(post_body)
        .send().await?
        .text().await?;

    Ok(response)
}

fn parse_response(response: String) -> Result<String, BotError> {
    // Parse the JSON data
    let parsed_response : serde_json::error::Result<Value> = serde_json::from_str(&response);
    let parsed_response = match parsed_response {
        Ok(value) => value,
        Err(_) => { return Err(BotError::FailedParseResponse) }
    };

    let html_content = parsed_response["data"].as_str();
    let html_content : &str = match html_content {
        None => { return Err(BotError::FailedParseResponse) }
        Some(value) => value
    };

    let document = Document::from(html_content);

    // Find the first <a> element with the specified classes
    if let Some(link) = document.find(Name("a")
        .and(Class("tik-button-dl")).and(Class("button")).and(Class("dl-success"))).next() {

        if let Some(href) = link.attr("href") {
            Ok(href.to_string())
        } else {
            Err(BotError::NoResult)
        }
    } else {
        Err(BotError::NoResult)
    }
}