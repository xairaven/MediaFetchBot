use reqwest::header;
use crate::error::BotError;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use serde_json::Value;

pub async fn get_href(url: &str) -> Result<(String), Box<dyn std::error::Error>> {
    let response = get_response(url).await;
    let href = parse_response(response.unwrap()).unwrap();

    Ok(href)
}

// https://www.youtube.com/watch?v=UsT11sOD1JA
async fn get_response(link: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("accept", "*/*".parse().unwrap());
    headers.insert("accept-language", "en-US,en;q=0.9".parse().unwrap());
    headers.insert("content-type", "application/x-www-form-urlencoded; charset=UTF-8".parse().unwrap());
    headers.insert("origin", "https://savetik.co".parse().unwrap());
    headers.insert("priority", "u=1, i".parse().unwrap());
    headers.insert("referer", "https://savetik.co/en2".parse().unwrap());
    headers.insert("sec-ch-ua", "\"Opera\";v=\"111\", \"Chromium\";v=\"125\", \"Not.A/Brand\";v=\"24\"".parse().unwrap());
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"Windows\"".parse().unwrap());
    headers.insert("sec-fetch-dest", "empty".parse().unwrap());
    headers.insert("sec-fetch-mode", "cors".parse().unwrap());
    headers.insert("sec-fetch-site", "same-origin".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36 OPR/111.0.0.0".parse().unwrap());
    headers.insert("x-requested-with", "XMLHttpRequest".parse().unwrap());

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let post_body = format!("q={link}&lang=en");

    let res = client.post("https://savetik.co/api/ajaxSearch")
        .headers(headers)
        .body(post_body)
        .send().await?
        .text().await?;

    Ok(res)
}

fn parse_response(response: String) -> Result<String, Box<dyn std::error::Error>> {
    // Parse the JSON data
    let api_response : Value = serde_json::from_str(&response).unwrap();

    let html_content = api_response["data"].as_str()
        .expect("Failed to extract HTML content");

    let document = Document::from(html_content);

    // Find the first <a> element with the specified classes
    if let Some(link) = document.find(Name("a")
        .and(Class("tik-button-dl")).and(Class("button")).and(Class("dl-success"))).next() {

        if let Some(href) = link.attr("href") {
            Ok(href.to_string())
        } else {
            Err(BotError::NoResult.into())
        }
    } else {
        Err(BotError::NoResult.into())
    }
}

pub fn download_file_by_link(href: &str)  {
    println!("{href}");

    todo!()
}