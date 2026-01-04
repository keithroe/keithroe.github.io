use crate::show;
use crate::util;

/*
<!-- Text area -->
    <div class="flex flex-col gap-2 flex-1 px-5 pb-5
                md:group-[.view-list]:px-0
                md:group-[.view-list]:justify-center">

        <a href="/e/saving-abel" class="text-base font-bold
                    md:group-[.view-grid]:text-clip
                    md:group-[.view-grid]:max-w-120">
            Saving Abel
        </a>

        <div class="text-sm md:group-[.view-grid]:text-base">
            Sunday, Jan 18, 2026 at 7:00 PM
        </div>

        <div class="text-xs">
            <span class="data-location-address">Liquid Joe&#039;s, </span><span class="data-location-city">Salt Lake City, </span><span class="data-location-state">UT</span>
        </div>

        <!-- More details link shows only in list -->
        <a href="/e/saving-abel" class="text-[#999999] text-sm max-md:hidden md:group-[.view-grid]:hidden">
            More details
        </a>
    </div>
*/

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

    for event in html.select(&scraper::Selector::parse("div.flex-col").unwrap()) {
        let link_elmt = match util::select_single(event, "a") {
            Ok(elmt) => elmt,
            Err(_) => continue,
        };

        let url_str = link_elmt.attr("href").unwrap().to_string();
        let url_str = base_url.join(&url_str).unwrap().to_string();
        let artist_str = util::get_text(link_elmt);
        if artist_str.is_empty() {
            continue;
        }

        let date_str = util::get_text(util::select_single(event, "div.text-sm").unwrap());
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

        /*
        println!("artist: '{}'", artist_str);
        println!("url   : {}", url_str);
        println!("date  : {}", date);
        */

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
