use crate::show;
use crate::util;

/*
<div class="eventlist-column-info">
    <h1 class="eventlist-title"><a href="/events/wm3n9ghfe7ufwxal1su764jz1ssg0b" class="eventlist-title-link">Spillkit and Puddle Jumping’s Album Release</a></h1>

    <ul class="eventlist-meta event-meta" data-animation-role="date">
        <li class="eventlist-meta-item eventlist-meta-date event-meta-item">
            <time class="event-date" datetime="2026-04-25">Saturday, April 25, 2026</time>
        </li>

        <li class="eventlist-meta-item eventlist-meta-time event-meta-item">
            <span class="event-time-localized">
                <time class="event-time-localized-start" datetime="2026-04-25">6:30 PM</time>
                <span class="event-datetime-divider"></span>
                <time class="event-time-localized-end" datetime="2026-04-25">10:00 PM</time>
            </span>
        </li>
        <li class="eventlist-meta-item eventlist-meta-export event-meta-item">
            <a href="http://www.google.com/calendar/event?action=TEMPLATE&text=Spillkit%20and%20Puddle%20Jumping%E2%80%99s%20Album%20Release&dates=20260426T003000Z/20260426T040000Z" class="eventlist-meta-export-google">Google Calendar</a>
            <span class="eventlist-meta-export-divider"></span>
            <a href="/events/wm3n9ghfe7ufwxal1su764jz1ssg0b?format=ical" class="eventlist-meta-export-ical">ICS</a>
        </li>

    </ul>
    ...
</div>
*/

pub fn scrape() -> Vec<show::Show> {
    println!("processing hypha ...");

    let base_url = url::Url::parse("https://www.hyphaproductions.com/").unwrap();
    let html = util::get_html(base_url.as_str()).unwrap();

    let mut shows = Vec::new();

    let event_selector = scraper::Selector::parse("div.eventlist-column-info").unwrap();
    for event in html.select(&event_selector) {
        let link_elmt = match util::select_single(event, "a") {
            Ok(elmt) => elmt,
            Err(_) => {
                println!("\tWARNING: eventlist found without link elmt");
                continue;
            }
        };
        let url_str = link_elmt.attr("href").unwrap().to_string();
        let url_str = base_url.join(&url_str).unwrap().to_string();
        let artist_str = util::get_text(link_elmt);
        if artist_str.is_empty() {
            println!("\tWARNING: link elmt missing artist text");
            continue;
        }
        let time_elmt = match util::select_single(event, "time.event-time-localized-start") {
            Ok(elmt) => elmt,
            Err(_) => {
                println!("\tWARNING: event-time not found");
                continue;
            }
        };

        let date_str = time_elmt.attr("datetime").unwrap();
        let date = chrono::naive::NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();

        // TODO: try to parse location from generic html text?
        let venue_str = "SEE EVENT LISTING";
        let city_str = "slc";

        shows.push(show::Show {
            date,
            artist: artist_str,
            venue: venue_str.into(),
            city: city_str.into(),
            url: url_str.to_string(),
        });
    }

    println!("\tfound {} shows", shows.len());
    shows
}
