use crate::show;
use crate::util;

/*
        <div class="event-info">
//          <a href="/e/cup-of-aloha-tour" class="name">Cup of Aloha Tour</a>
//          <div class="date date-long">Saturday, Oct 11, 2025 at 7:00 PM to Sunday, Oct 12, 2025</div>
            <div class="date date-short">Saturday, Oct 11 to Sunday, Oct 12, 2025</div>
            <div class="location">
                <span class="data-location-address">Liquid Joe&#039;s, </span>
                <span class="data-location-city">Salt Lake City, </span>
                <span class="data-location-state">UT</span>
            </div>
            <a id="toggle-summary-btn-6" class="toggle-summary-btn" href="javascript:void(0);" onclick="toggleSummary('6');">
                <span class="more-text">More Details</span>
                <span class="fewer-text" style="display:none">Fewer Details</span>
            </a>
            <div id="summary-6" class="summary" style="display:none">
                <p>Cup of Aloha Tour</p>
                <p>Featuring</p>
                <p>Nuff Sedd</p>
                <p>Sione Liti</p>
            </div>
        </div>
*/

pub fn scrape() -> Vec<show::Show> {
    println!("processing liquid joes...");

    let mut shows = Vec::new();
    let base_url = url::Url::parse("https://liquidjoes.ticketsauce.com/").unwrap();
    let html = util::get_html(&base_url.to_string()).unwrap();

    for event in html.select(&scraper::Selector::parse("div.event-info").unwrap()) {
        let link_elmt = util::select_single(event, "a.name").unwrap();
        let url_str = link_elmt.attr("href").unwrap().to_string();
        let url_str = base_url.join(&url_str).unwrap().to_string();
        let artist_str = util::get_text(link_elmt);

        let date_str = util::get_text(util::select_single(event, "div.date-long").unwrap());
        let date_strs = date_str
            .split([' ', ','])
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        // <div class="date date-long">Saturday, Oct 11, 2025 at 7:00 PM to Sunday, Oct 12, 2025</div>
        let day_str = date_strs[2].clone();
        let month_str = date_strs[1].clone();

        let date = util::create_date(
            day_str.parse::<u32>().unwrap(),
            util::month_int_from_str(&month_str).unwrap(),
        )
        .unwrap();

        shows.push(show::Show {
            date,
            artist: artist_str.to_string(),
            venue: "liquid joes".to_string(),
            city: "slc".to_string(),
            url: url_str.to_string(),
        });
    }
    println!("\tfound {} shows", shows.len());
    shows
}
