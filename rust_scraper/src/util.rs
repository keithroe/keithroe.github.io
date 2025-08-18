use chrono::prelude::*;
use anyhow::{anyhow, Result, Context};

pub fn get_html(url: &str) -> Result<scraper::html::Html> {
    
    println!("Loading html '{}' ...", url);

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT, 
        reqwest::header::HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36"
        )
    );
    headers.insert(
        reqwest::header::CONTENT_TYPE, 
        reqwest::header::HeaderValue::from_static(
            "text/html; charset=utf-8"
        )
    );

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .headers(headers)
        .send().context(format!("Failed to get '{}'", url))?;

    let html = response.text().context(format!("Failed to get text '{}'", url))?;

    Ok(scraper::Html::parse_document(&html))
}

pub fn month_int_from_str(month_str: &str) -> Result<u32> {
    let month_str = month_str.to_lowercase();
    if month_str.starts_with("ja") {
        Ok(1)
    } else if month_str.starts_with("f") {
        Ok(2)
    } else if month_str.starts_with("mar") {
        Ok(3)
    } else if month_str.starts_with("ap") {
        Ok(4)
    } else if month_str.starts_with("may") {
        Ok(5)
    } else if month_str.starts_with("jun") {
        Ok(6)
    } else if month_str.starts_with("jul") {
        Ok(7)
    } else if month_str.starts_with("au") {
        Ok(8)
    } else if month_str.starts_with("s") {
        Ok(9)
    } else if month_str.starts_with("o") {
        Ok(10)
    } else if month_str.starts_with("n") {
        Ok(11)
    } else if month_str.starts_with("d") {
        Ok(12)
    } else {
        Err(anyhow!("month_int_from_str given invalid string '{}'", month_str))
    }
}

pub fn create_date(day: u32, month: u32) -> Result<chrono::NaiveDate> {
    let today = chrono::Local::now().naive_local().date();
    let mut date = NaiveDate::from_ymd_opt(
        today.year(), 
        month, 
        day).context(format!("failed to create date from {}, {}", day, month))?;
    if date < today {
        date = date.checked_add_months(chrono::Months::new(12)).context("Failed to add year to date")?;
    }
    Ok(date)
}
            

fn select_single(:tag: &str) -> Result<scraper::ElementRef<'a>> {
    let selector = scraper::Selector::parse(tag)?;
    let elem = html_event.select(&selector).next()?;
}
