use chrono::prelude::*;
use anyhow::{anyhow, Result, Context};
/*
            'authority': 'www.google.com',
            'accept-language': 'en-US,en;q=0.9',
            'cache-control': 'max-age=0',
            #'cookie': 'SID=ZAjX93QUU1NMI2Ztt_dmL9YRSRW84IvHQwRrSe1lYhIZncwY4QYs0J60X1WvNumDBjmqCA.; __Secure- #..,
            'sec-ch-ua': '"Not/A)Brand";v="99", "Google Chrome";v="115", "Chromium";v="115"',
            'sec-ch-ua-arch': '"x86"',
            'sec-ch-ua-bitness': '"64"',
            'sec-ch-ua-full-version': '"115.0.5790.110"',
            'sec-ch-ua-full-version-list': '"Not/A)Brand";v="99.0.0.0", "Google Chrome";v="115.0.5790.110", "Chromium";v="115.0.5790.110"',
            'sec-ch-ua-mobile': '?0',
            'sec-ch-ua-model': '""',
            'sec-ch-ua-platform': 'Windows',
            'sec-ch-ua-platform-version': '15.0.0',
            'sec-ch-ua-wow64': '?0',
            'sec-fetch-dest': 'document',
            'sec-fetch-mode': 'navigate',
            'sec-fetch-site': 'same-origin',
            'sec-fetch-user': '?1',
            'upgrade-insecure-requests': '1',
            'user-agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36',
            'x-client-data': '#..',
*/
pub fn get_html(url: &str) -> Result<scraper::html::Html> {
    
    println!("Loading html '{}' ...", url);

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::HeaderName::from_static("authority"),
        reqwest::header::HeaderValue::from_static("www.google.com")
    );
    headers.insert(
        reqwest::header::AUTHORIZATION, 
        reqwest::header::HeaderValue::from_static("www.google.com")
    );
    headers.insert(
        reqwest::header::ACCEPT, 
        reqwest::header::HeaderValue::from_static(
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"
        )
    );
    headers.insert(
        reqwest::header::ACCEPT_LANGUAGE , 
        reqwest::header::HeaderValue::from_static(
            "en-US,en;q=0.9"
        )
    );
    headers.insert(
        reqwest::header::CACHE_CONTROL, 
        reqwest::header::HeaderValue::from_static(
            "max-age=0"
        )
    );
    headers.insert(
        reqwest::header::UPGRADE_INSECURE_REQUESTS, 
        reqwest::header::HeaderValue::from_static(
            "1"
        )
    );
    headers.insert(
        reqwest::header::USER_AGENT, 
        reqwest::header::HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36"
        )
    );
    headers.insert(
        reqwest::header::HeaderName::from_static("x-client-data"),
        reqwest::header::HeaderValue::from_static("#..")
    );

    headers.insert(
        reqwest::header::HeaderName::from_static("cookie"),
        reqwest::header::HeaderValue::from_static("SID=ZAjX93QUU1NMI2Ztt_dmL9YRSRW84IvHQwRrSe1lYhIZncwY4QYs0J60X1WvNumDBjmqCA.; __Secure- #..,")
    );


    headers.insert(
        reqwest::header::HeaderName::from_static("sec-fetch-dest"),
        reqwest::header::HeaderValue::from_static("document")
    );
    headers.insert(
        reqwest::header::HeaderName::from_static("sec-fetch-mode" ),
        reqwest::header::HeaderValue::from_static("navigate")
    );
    headers.insert(
        reqwest::header::HeaderName::from_static("sec-fetch-site" ),
        reqwest::header::HeaderValue::from_static("same-origin")
    );
    headers.insert(
        reqwest::header::HeaderName::from_static("sec-fetch-user"),
        reqwest::header::HeaderValue::from_static("?1")
    );
    /*
    headers.insert(
        reqwest::header::HeaderName::from_static(
),
        reqwest::header::HeaderValue::from_static(
)
    );
    */



    headers.insert(
        reqwest::header::HeaderName::from_static( "sec-ch-ua"),
        reqwest::header::HeaderValue::from_static( r#""Not/A)Brand";v="99" "Google Chrome";v="115" "Chromium";v="115""#)
    );
    headers.insert(
        reqwest::header::HeaderName::from_static( "sec-ch-ua-arch"),
        reqwest::header::HeaderValue::from_static( r#""x86""#)
    );
    headers.insert(
        reqwest::header::HeaderName::from_static( "sec-ch-ua-bitness"),
        reqwest::header::HeaderValue::from_static( r#""64""#)
    );
    headers.insert(
        reqwest::header::HeaderName::from_static( "sec-ch-ua-full-version"),
        reqwest::header::HeaderValue::from_static( r#""115.0.5790.110""#)
    );
    headers.insert(
        reqwest::header::HeaderName::from_static( "sec-ch-ua-full-version-list"),
        reqwest::header::HeaderValue::from_static( r#""Not/A)Brand";v="99.0.0.0" "Google Chrome";v="115.0.5790.110" "Chromium";v="115.0.5790.110""#)
    );
    headers.insert(
        reqwest::header::HeaderName::from_static( "sec-ch-ua-mobile"),
        reqwest::header::HeaderValue::from_static( "?0")
    );
    headers.insert(
        reqwest::header::HeaderName::from_static( "sec-ch-ua-model"),
        reqwest::header::HeaderValue::from_static( r#""""#)
    );
    headers.insert(
        reqwest::header::HeaderName::from_static( "sec-ch-ua-platform"),
        reqwest::header::HeaderValue::from_static( "Windows")
    );
    headers.insert(
        reqwest::header::HeaderName::from_static( "sec-ch-ua-platform-version"),
        reqwest::header::HeaderValue::from_static( "15.0.0")
    );
    headers.insert(
        reqwest::header::HeaderName::from_static( "sec-ch-ua-wow64"),
        reqwest::header::HeaderValue::from_static( "?0")
    );


    /*
    headers.insert(
        reqwest::header::CONTENT_TYPE, 
        reqwest::header::HeaderValue::from_static(
            "text/html; charset=utf-8"
        )
    );
    headers.insert(
        reqwest::header::REFERER, 
        reqwest::header::HeaderValue::from_static(
            "http://www.google.com/"
        )
    );
*/
    for (key, value) in headers.iter() {
        println!("'{:?}': '{:?}'", key, value);
    }

    let client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .build()?;
    let response = client
        .get(url)
        .headers(headers)
        .send()
        .context(format!("Failed to get '{}'", url))?;

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
            

pub fn select_single<'a>(elmt: scraper::ElementRef<'a>, tag: &str) -> Result<scraper::ElementRef<'a>> {
    let selector = scraper::Selector::parse(tag).unwrap();
    let child_elmt = elmt.select(&selector).next().context("No elements selected")?;
    Ok(child_elmt)
}
        

pub fn get_text<'a>(elmt: scraper::ElementRef<'a>) -> String {
    elmt.text().collect::<Vec<_>>().join(" ").trim().to_string()
}
