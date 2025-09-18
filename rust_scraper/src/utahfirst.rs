use crate::show;
use crate::util;

/*
<script type="application/ld+json">
{
  "@context":"https://schema.org",
  "@type":"MusicEvent",
  "name":"Judas Priest & Alice Cooper Live",
  "image":"https://s1.ticketm.net/dam/a/56f/472e7aa6-c353-41c3-aeed-3f09a519056f_TABLET_LANDSCAPE_LARGE_16_9.jpg",
  "startDate":"2025-10-12T18:45:00-06:00",
  "url":"https://www.ticketmaster.com/judas-priest-alice-cooper-live-west-valley-city-utah-10-12-2025/event/1E00628AD8B74495",
  ...
}
*/

pub fn scrape() -> Vec<show::Show> {
    println!("processing utah first ...");

    let mut shows = Vec::new();

    let url = "https://www.utahfirstamp.com/shows";
    let html = util::get_html(url).unwrap();

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
        if json_map["@type"].as_str().unwrap() != "MusicEvent" {
            continue;
        }
        let artist_str = json_map["name"].as_str().unwrap();
        if artist_str.contains("Tickets Wait List") {
            continue;
        }
        let url_str = json_map["url"].as_str().unwrap();
        let date_str = json_map["startDate"]
            .as_str()
            .unwrap()
            .split("T")
            .next()
            .unwrap()
            .to_string();
        let date = chrono::naive::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").unwrap();

        shows.push(show::Show {
            date,
            artist: artist_str.to_string(),
            venue: "utah first".to_string(),
            city: "west valley".to_string(),
            url: url_str.to_string(),
        });
    }

    println!("\tfound {} shows", shows.len());
    shows
}
