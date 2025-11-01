use crate::show;
use crate::util;

/*
<div class="event-wrapper" >
    <div class="event event-start-date">
        <h4 class="date month">Sep</h4>
        <h3 class="date number">23</h3>
    </div>
    <div class="event-image-container">
        <a href="https://tixr.com/e/149466 " target="_blank">
            <div class="event event-image">
                <img src="https://granarylive.com/wp-content/uploads/2025/06/EmpireOfTheSun_WebSlider-600x600.png" loading="lazy" alt="9.23 Empire of the Sun at Granary Live Event Photo"/>
            </div>
        </a>
    </div>
    <div class="info">
        <div class="event event-info">
            <h3 class="event-name">Empire of the Sun: Ask That God Tour</h3>
            <h4 class="event-feature">ROI TURBO</h4>
            <p>
                <br/>
                <h4>
                    <span class="event-feature event-age">AA</span>
                    <span class="event-feature"> | Doors At </span>
                    <span class="event-feature event-time">6:00 pm</span>
                </h4>
                <br/>
            </p>
            <div class="description"></div>
        </div>
        <div class="event event-tickets">
            <a href="https://tixr.com/e/149466 " class="button event-link on-sale" target="_blank">
                GET TICKETS! <span></span>
            </a>
        </div>
    </div>
</div>
*/

pub fn scrape() -> Vec<show::Show> {
    println!("processing granary ...");

    let mut shows = Vec::new();
    let venue_url = "https://granarylive.com/events/";
    let Ok(html) = util::get_html(venue_url) else {
        println!("\tWARNING: Failed to open '{}'", venue_url);
        return shows;
    };

    for event in html.select(&scraper::Selector::parse("div.event-wrapper").unwrap()) {
        let event_date_elmt = util::select_single(event, "div.event-start-date").unwrap();
        let day_str = util::get_text(util::select_single(event_date_elmt, "h3.number").unwrap());
        let month_str = util::get_text(util::select_single(event_date_elmt, "h4.month").unwrap());

        let artist_str = util::get_text(util::select_single(event, "h3.event-name").unwrap());
        let link_elmt = util::select_single(event, "a.event-link").unwrap();
        let url_str = link_elmt.attr("href").unwrap().to_string();
        let date = util::create_date(
            day_str.parse::<u32>().unwrap(),
            util::month_int_from_str(&month_str).unwrap(),
        )
        .unwrap();

        shows.push(show::Show {
            date,
            artist: artist_str.to_string(),
            venue: "granary live".to_string(),
            city: "slc".to_string(),
            url: url_str.to_string(),
        });
    }
    println!("\tfound {} shows", shows.len());
    shows
}
