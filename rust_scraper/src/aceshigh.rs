use crate::show;
use crate::util;

/*
<div  class="tribe-common-g-row tribe-events-calendar-list__event-row" >
    <div class="tribe-events-calendar-list__event-date-tag tribe-common-g-col">
        <time class="tribe-events-calendar-list__event-date-tag-datetime" datetime="2024-12-05" aria-hidden="true">
            <span class="tribe-events-calendar-list__event-date-tag-weekday">
                Thu
            </span>
            <span class="tribe-events-calendar-list__event-date-tag-daynum tribe-common-h5 tribe-common-h4--min-medium">
                5
            </span>
        </time>
    </div>

    <div class="tribe-events-calendar-list__event-wrapper tribe-common-g-col">
        <article  class="tribe-events-calendar-list__event tribe-common-g-row tribe-common-g-row--gutters post-23925 tribe_events type-tribe_events status-publish hentry" >
            <div class="tribe-events-calendar-list__event-details tribe-common-g-col">
                <header class="tribe-events-calendar-list__event-header">
                    <div class="tribe-events-calendar-list__event-datetime-wrapper tribe-common-b2">
                        <time class="tribe-events-calendar-list__event-datetime" datetime="2024-12-05">
                            <span class="tribe-event-date-start">December 5 @ 8:00 pm</span>
                            -
                            <span class="tribe-event-time">11:59 pm</span>
                            <span class='timezone'> MST </span>
                        </time>
                    </div>
                    <h3 class="tribe-events-calendar-list__event-title tribe-common-h6 tribe-common-h4--min-medium">
                        <a
                            href="https://aceshighsaloon.com/event/public-serpent/"
                            title="Public Serpent"
                            rel="bookmark"
                            class="tribe-events-calendar-list__event-title-link tribe-common-anchor-thin"
                        >
                             Public Serpent
                        </a>
                    </h3>
                </header>
            </div>
        </article>
    </div>
</div>
*/

pub fn scrape() -> Vec<show::Show> {
    println!("processing Aces High ...");

    let mut shows = Vec::new();
    // TODO: website is bad -- but can do this:
    //let url_template = https://aceshighsaloon.com/events/list/page/{}";
    let mut page_idx = 1;
    loop {
        let html = util::get_html(&format!(
            "https://aceshighsaloon.com/events/list/page/{}",
            page_idx
        ))
        .unwrap();
        page_idx += 1;

        let mut page_event_count = 0;
        for html_event in html
            .select(&scraper::Selector::parse("div.tribe-events-calendar-list__event-row").unwrap())
        {
            page_event_count += 1;
            let link_selector = scraper::Selector::parse("a").unwrap();
            let link_elem = html_event.select(&link_selector).next().unwrap();
            let url_str = link_elem.attr("href").unwrap();
            let artist_str = link_elem
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string();

            let time_selector = scraper::Selector::parse("time").unwrap();
            let time_elem = html_event.select(&time_selector).next().unwrap();
            let date_str = time_elem.attr("datetime").unwrap();
            let date = chrono::naive::NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();

            shows.push(show::Show {
                date,
                artist: artist_str.to_string(),
                venue: "aces high".to_string(),
                city: "slc".to_string(),
                url: url_str.to_string(),
            });
        }

        if page_event_count == 0 {
            break;
        }
    }

    shows
}
