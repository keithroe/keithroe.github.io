use chrono::prelude::*;

pub fn get_html(url: &str) -> scraper::html::Html {
    
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
        .send()
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            println!("\tSuccess");
        } other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    };



    //let response = reqwest::blocking::get(url).unwrap();
    let html = response.text().unwrap();

    scraper::Html::parse_document(&html)
}

pub fn month_int_from_str(month_str: &str) -> u32 {
    let month_str = month_str.to_lowercase();
    if month_str.starts_with("ja") {
        1
    } else if month_str.starts_with("f") {
        2
    } else if month_str.starts_with("mar") {
        3
    } else if month_str.starts_with("ap") {
        4
    } else if month_str.starts_with("may") {
        5
    } else if month_str.starts_with("jun") {
        6
    } else if month_str.starts_with("jul") {
        7
    } else if month_str.starts_with("au") {
        8
    } else if month_str.starts_with("s") {
        9
    } else if month_str.starts_with("o") {
        10
    } else if month_str.starts_with("n") {
        11
    } else if month_str.starts_with("d") {
        12
    } else {
        println!(
            "ERROR: month_int_from_str given invalid string '{}'",
            month_str
        );
        0
    }
}

pub fn create_date(day: u32, month: u32) -> chrono::NaiveDate {
    let today = chrono::Local::now().naive_local().date();
    let mut date = NaiveDate::from_ymd_opt(today.year(), month, day).unwrap();
    if date < today {
        date = date.checked_add_months(chrono::Months::new(12)).unwrap();
    }
    date
}
