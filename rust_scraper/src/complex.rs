use crate::show;
use crate::util;

/*
The shows page renders its listings from an AXS json feed whose url lives in the
data-file attribute of the events container:

<div class="c-axs-events ...">
    <div class="c-axs-events__container"
         data-file="https://aegwebprod.blob.core.windows.net/json/events/376/events.json">

Each entry in that feed looks like:

{
    "title": { "headlinersText": "Have a Nice Life", ... },
    "eventDateTime": "2026-09-05T20:00:00",
    "ticketing": { "url": "https://www.axs.com/events/1392046/have-a-nice-life-tickets", ... },
    ...
}
*/

pub fn scrape() -> Vec<show::Show> {
    println!("processing the complex ...");

    let mut shows = Vec::new();
    let html = util::get_html("https://thecomplexslc.com/shows/").unwrap();

    let events_div = util::select_single(html.root_element(), "div.c-axs-events").unwrap();
    let container_div = util::select_single(events_div, "div.c-axs-events__container").unwrap();
    let data_file = container_div.attr("data-file").unwrap();

    let json = util::get_json(data_file).unwrap();
    let Some(events) = json["events"].as_array() else {
        println!("\tfound 0 shows");
        return shows;
    };

    for event in events {
        let Some(artist_str) = event["title"]["headlinersText"].as_str() else {
            continue;
        };

        let Some(url_str) = event["ticketing"]["url"].as_str() else {
            continue;
        };

        let Some(date_str) = event["eventDateTime"].as_str() else {
            continue;
        };
        let Ok(date) = chrono::naive::NaiveDate::parse_from_str(
            date_str.split('T').next().unwrap(),
            "%Y-%m-%d",
        ) else {
            continue;
        };

        shows.push(show::Show::new(
            date,
            artist_str,
            "the complex",
            "slc",
            url_str,
        ));
    }
    println!("\tfound {} shows", shows.len());
    shows
}
