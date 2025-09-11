use crate::show;
use crate::util;
use regex;

/*
<div class="content">
    <a href="https://www.thecomplexslc.com/event-2562.htm" class="image-link" title="Mark Ambor - The Rockwood Tour">
        <h3>Mark Ambor - The Rockwood Tour</h3>
        <h4>Tuesday Nov 19th</h4>
        <h4>The Grand</h4>
        <p>Indie</p>
    </a>
</div>
*/

pub fn scrape() -> Vec<show::Show> {
    println!("processing The Complex ...");

    let mut shows = Vec::new();
    let html = util::get_html("https://www.thecomplexslc.com/").unwrap();
    let date_re = regex::Regex::new(r"[a-zA-Z]+\s+([a-zA-Z]+)\s+(\d+).+").unwrap();

    let event_selector = scraper::Selector::parse("a.image-link").unwrap();
    let html_events = html.select(&event_selector);
    for html_event in html_events {
        let artist_str;
        if let Some(artist_string) = html_event.attr("title") {
            artist_str = artist_string;
        } else {
            continue;
        }

        let url_str;
        if let Some(url_string) = html_event.attr("href") {
            url_str = url_string;
        } else {
            continue;
        }

        let date;
        if let Some(date_elem) = html_event
            .select(&scraper::Selector::parse("h4").unwrap())
            .next()
        {
            let date_str = date_elem
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string()
                .to_lowercase();

            if let Some(matches) = date_re.captures(&date_str) {
                let month = util::month_int_from_str(&matches[1]).unwrap();
                let day = matches[2].parse::<u32>().unwrap();
                date = util::create_date(day, month).unwrap();
            } else {
                continue;
            }
        } else {
            continue;
        }

        shows.push(show::Show {
            date,
            artist: artist_str.to_string(),
            venue: "the complex".to_string(),
            city: "slc".to_string(),
            url: url_str.to_string(),
        });
    }
    println!("\tfound {} shows", shows.len());
    shows
}
