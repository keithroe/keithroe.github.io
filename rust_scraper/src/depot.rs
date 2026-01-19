use crate::show;
use crate::util;
use chrono::prelude::*;
use regex::Regex;

/*
// other method stopped working for a while -- could use this if fails again
use chrono::Datelike;
<div class="css-vlhl7f">
    <time class="chakra-text css-18djj88" dateTime="2026-01-17">17</time>
    <div class="css-18te69i">
        <a target="_blank" class="chakra-link css-1auf7pg" title="Pearl Jam Experience @ 7:00PM" role="group" href="https://www.ticketmaster.com/pearl-jam-experience-salt-lake-city-utah-01-17-2026/event/1E006388AADE6721">
            <span class="chakra-text css-83q39x">Pearl Jam Experience</span>
            <span class="chakra-text css-18nnj73">7:00PM</span>
        </a>
    </div>
</div>

https://www.depotslc.com/shows/calendar/2026-05
*/

pub fn scrape() -> Vec<show::Show> {
    println!("processing the depot ...");

    let date_re = Regex::new(r"[0-9]{2}-[0-9]{2}-[0-9]{4}").unwrap();
    let mut shows = Vec::new();

    let mut latest_date = Local::now().naive_local().date();
    //let month = latest_date.month();
    //let year = latest_date.year();
    //println!("{}-{}", year, month);
    loop {
        let url = format!("https://www.depotslc.com/shows?start={}", latest_date);
        let html = util::get_html(&url).unwrap();

        let mut page_event_count = 0;
        for script_elmt in html.select(&scraper::Selector::parse("script").unwrap()) {
            // TODO: match
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
            let date_str;
            match date_re.find(url_str) {
                Some(match_obj) => {
                    date_str = match_obj.as_str();
                }
                None => continue,
            }
            let Ok(date) = chrono::naive::NaiveDate::parse_from_str(&date_str, "%m-%d-%Y") else {
                continue;
            };
            /*
            let date_str = json_map["startDate"]
                .as_str()
                .unwrap()
                .split("T")
                .next()
                .unwrap()
                .to_string();
            let date_str = json_map["start_date_local"].as_str().unwrap();
            // thedepot has started listing some shows as "Date TBA"
            let Ok(date) = chrono::naive::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") else {
                continue;
            };
            */

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
