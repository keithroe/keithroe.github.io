use crate::show;
use crate::util;
use chrono::prelude::*;

/*
*/

pub fn scrape() -> Vec<show::Show> {
    println!("processing the depot ...");

    let mut shows = Vec::new();

    let mut latest_date = Local::now().naive_local().date();
    loop {
        let url = format!("https://www.depotslc.com/shows?start={}", latest_date);
        let html = util::get_html(&url).unwrap();

        let mut page_event_count = 0;
        for script_elmt in html.select(&scraper::Selector::parse("script").unwrap()) {
            if let Some(type_) = script_elmt.value().attr("type") {
                if type_ != "application/ld+json" {
                    continue;
                }
            } else {
                continue;
            }

            let json_string = util::get_text(script_elmt);
            let json_map: serde_json::Value =
                serde_json::from_str(&json_string).expect("Failed to parse json");

            let artist_str = json_map["name"].as_str().unwrap();
            let url_str = json_map["url"].as_str().unwrap();
            let date_str = json_map["startDate"]
                .as_str()
                .unwrap()
                .split("T")
                .next()
                .unwrap()
                .to_string();
            let date = chrono::naive::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").unwrap();
            if date < latest_date {
                continue;
            }
            latest_date = date.checked_add_days(chrono::Days::new(1)).unwrap();

            shows.push(show::Show {
                date,
                artist: artist_str.to_string(),
                venue: "the depot".to_string(),
                city: "slc".to_string(),
                url: url_str.to_string(),
            });
            page_event_count += 1;
        }
        if page_event_count == 0 {
            break;
        }

        /*
        let selector = scraper::Selector::parse("div.chakra-card__footer").unwrap();
        let html_events = html.select(&selector);

        let mut page_event_count = 0;
        for html_event in html_events {
            println!("here 0");
            page_event_count += 1;

            let selector = scraper::Selector::parse("p").unwrap();
            let mut ps = html_event.select(&selector);
            let artist_str = ps
                .next()
                .unwrap()
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string();

            let date_strs = ps
                .next()
                .unwrap()
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .split([' ', ','])
                .map(|s| s.to_string())
                .collect::<Vec<_>>();

            let date = util::create_date(
                date_strs[2].parse::<u32>().unwrap(),
                util::month_int_from_str(&date_strs[1]).unwrap(),
            )
            .unwrap();
            latest_date = date.checked_add_days(chrono::Days::new(1)).unwrap();

            let url_str;
            if let Some(url_string) = html_event
                .select(&scraper::Selector::parse("a").unwrap())
                .next()
                .unwrap()
                .attr("href")
            {
                url_str = url_string;
            } else {
                continue;
            }

            shows.push(show::Show {
                date,
                artist: artist_str,
                venue: "the depot".to_string(),
                city: "slc".to_string(),
                url: url_str.to_string(),
            });
        }

        if page_event_count == 0 {
            break;
        }
        */
    }

    println!("\tfound {} shows", shows.len());
    shows
}
