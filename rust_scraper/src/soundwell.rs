use crate::show;
use crate::util;

/*
<div class="event-wrapper" >
    <div class="event event-start-date">
        <h4 class="date month">Nov</h4>
        <h3 class="date number">22</h3>
    </div>
    <div class="event-image-container">
        <a href="https://tixr.com/e/96551" class="event event-image" target="_blank">
            <img src="https://soundwellslc.com/wp-content/uploads/2024/02/005_Matthew_Yoscary23-copy_600x600_acf_cropped-1.jpg" loading="lazy"/>
        </a>
    </div>
    <div class="info">
        <div class="event event-info">
            <h3 class="event-name">HOMESHAKE</h3>
            <h4 class="event-feature">GREEN-HOUSE</h4>
            <p>
                <br/>
                <h4>
                    <span class="event-feature event-age">AA</span>
                    <span class="event-feature"> | Doors At </span>
                    <span class="event-feature event-time">7:00 pm</span>
                    <span class="event-feature event-price"> | $22+</span>
                </h4>
                <br/>
            </p>
            <div class="description">
            </div>
        </div>
        <br/>
        <div class="event event-tickets">
            <a href="https://tixr.com/e/96551" class="button event-link on-sale" target="_blank">
                GET TICKETS!                                                                                                                                                                                                                                <span></span>
            </a>
        </div>
    </div>
</div>
*/

pub fn scrape() -> Vec<show::Show> {
    println!("processing soundwell ...");

    let mut shows = Vec::new();
    let html = util::get_html("https://soundwellslc.com/events/").unwrap();

    //for html_event in html.select(&scraper::Selector::parse("div.event-wrapper").unwrap()) {
    let event_selector = scraper::Selector::parse("div.event-wrapper").unwrap();
    let html_events = html.select(&event_selector);

    for html_event in html_events {
        let header_elmt = util::select_single(html_event, "h3.event-name").unwrap();
        let artist_str = util::get_text(header_elmt);

        let div_elmt = util::select_single(html_event, "div.event-tickets").unwrap();
        let link_elem = util::select_single(div_elmt, "a").unwrap();
        let url_str = link_elem.attr("href").unwrap();

        let div_elmt = util::select_single(html_event, "div.event-start-date").unwrap();
        let month_str = util::get_text(util::select_single(div_elmt, "h4").unwrap());
        let day_str = util::get_text(util::select_single(div_elmt, "h3").unwrap());
        let date = util::create_date(
            day_str.parse::<u32>().unwrap(),
            util::month_int_from_str(&month_str).unwrap(),
        )
        .unwrap();

        shows.push(show::Show {
            date,
            artist: artist_str.to_string(),
            venue: "soundwell".to_string(),
            city: "slc".to_string(),
            url: url_str.to_string(),
        });
    }
    println!("\tfound {} shows", shows.len());
    shows
}
