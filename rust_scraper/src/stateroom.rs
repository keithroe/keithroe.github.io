use crate::show;
use crate::util;

/*
<div class="allevents-event my-3 shadow col col-sm-4">
    <div class="allevents-img">
        <a href="/state-room-presents/pigeons-playing-ping-pong-3">
            <span class="acfup-item">
                <img src="https://thestateroompresents.com/images/acfupload/Pigeons-Playing-Ping-Pong_03-11-2025_v2_Facebook_1200x628.jpg"/>
            </span>
        </a>
    </div>
    <div class="p-3">
        <h3 class="allevents-title">
            <a href="/state-room-presents/pigeons-playing-ping-pong-3">Pigeons Playing Ping Pong</a>
        </h3>
        <div class="allevents-date">
            Tue Mar 11
        </div>
        <div class="allevents-venue2">
            The Commonwealth Room
        </div>
        <div class="allevents-link">
            <a id="acf_url_1307_11" href="https://www.axs.com/events/751925/pigeons-playing-ping-pong-tickets?skin=stateroom" class="acf_url btn" target="_blank" rel="noopener">
                On Sale Fri 11/15
            </a>
        </div>
    </div>
</div>
*/

fn query_city_stateroom(venue_str: &str) -> String {
    let venue_str = venue_str.to_lowercase();
    if venue_str.contains("presents") {
        "slc?".to_string()
    } else if venue_str.contains("commonwealth") {
        "slc".to_string()
    } else if venue_str.contains("deer") {
        "park city".to_string()
    } else if venue_str.contains("eccles") {
        "slc".to_string()
    } else if venue_str.contains("state") {
        "slc".to_string()
    } else {
        "slc?".to_string()
    }
}

pub fn scrape() -> Vec<show::Show> {
    println!("processing State Room Presents ...");

    let mut shows = Vec::new();
    let html = util::get_html("https://thestateroompresents.com/state-room-presents").unwrap();

    let event_selector = scraper::Selector::parse("div.p-3").unwrap();
    let html_events = html.select(&event_selector);
    for html_event in html_events {
        let artist_str;
        if let Some(artist_elem) = html_event
            .select(&scraper::Selector::parse("h3 > a").unwrap())
            .next()
        {
            artist_str = artist_elem
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string();
        } else {
            continue;
        }

        let url_str;
        if let Some(link_elem) = html_event
            .select(&scraper::Selector::parse("div.allevents-link > a").unwrap())
            .next()
        {
            url_str = link_elem.attr("href").unwrap().to_string();
        } else {
            continue;
        }

        let date;
        if let Some(date_elem) = html_event
            .select(&scraper::Selector::parse("div.allevents-date").unwrap())
            .next()
        {
            let date_strings = date_elem
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            //let date_strings = vec![date_strings[1].clone(), date_strings[2].clone()];
            let date_strings = [date_strings[1].clone(), date_strings[2].clone()];
            date = util::create_date(
                date_strings[1].parse::<u32>().unwrap(),
                util::month_int_from_str(&date_strings[0]).unwrap(),
            )
            .unwrap();
        } else {
            continue;
        }

        let venue_str;
        let city_str;
        if let Some(venue_elem) = html_event
            .select(&scraper::Selector::parse("div.allevents-venue2").unwrap())
            .next()
        {
            venue_str = venue_elem
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string()
                .to_lowercase();
            city_str = query_city_stateroom(&venue_str);
        } else {
            continue;
        }

        shows.push(show::Show {
            date,
            artist: artist_str,
            venue: venue_str,
            city: city_str,
            url: url_str.to_string(),
        });
    }
    println!("\tfound {} shows", shows.len());
    shows
}
