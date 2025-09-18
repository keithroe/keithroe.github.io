use crate::show;
use crate::util;

/*
<div class="card-body event-body">
  <div class=k"event-content-mobile">
    <div class="text-muted event-promoter">2025 Twilight Concert Series presents</div>
    <h5 class="card-title event-title">
      <a href="https://www.24tix.com/events/fftiwdjgprcgxiaet3ljdxzbna">
        Japanese Breakfast
      </a>
    </h5>
    <div class="card-row">
      <div class="event-start">Fri, Sep 5 / 06:00PM</div>
    </div>
    <div class="event-venue mt-3">
      <h6 class="m-0 text-capitalize">
        The Gallivan Center
      </h6>
      <small class="card-text event-address" condition="_venue_address">
        <span>Salt Lake City</span>,
        <span>UT</span>
        <span></span>
      </small>
    </div>
  </div>
</div>
*/

pub fn scrape() -> Vec<show::Show> {
    println!("processing 24tix ...");

    let mut page = 0;
    let mut shows = Vec::new();
    let mut venues = std::collections::HashSet::new();
    loop {
        let url = format!("https://www.24tix.com/?batch_page={}", page);
        let html = util::get_html(&url).unwrap();
        let selector = scraper::Selector::parse("div.card-body.event-body").unwrap();
        let html_events = html.select(&selector);

        let mut page_event_count = 0;
        for html_event in html_events {
            page_event_count += 1;

            let link_tag = html_event
                .select(&scraper::Selector::parse("a").unwrap())
                .next()
                .unwrap();
            let link_str = link_tag.attr("href").unwrap();
            let artist_str = link_tag
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string();

            let div_tag = html_event
                .select(&scraper::Selector::parse("div.event-start").unwrap())
                .next()
                .unwrap();
            let date_str = div_tag
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string();
            let date_strs = date_str
                .split_ascii_whitespace()
                .map(|s| s.trim())
                .collect::<Vec<_>>()[1..3]
                .to_vec();

            let date = util::create_date(
                date_strs[1].parse::<u32>().unwrap(),
                util::month_int_from_str(date_strs[0]).unwrap(),
            )
            .unwrap();

            let div_tag = html_event
                .select(&scraper::Selector::parse("div.event-venue").unwrap())
                .next()
                .unwrap();
            let venue_str = div_tag
                .select(&scraper::Selector::parse("h6").unwrap())
                .next()
                .unwrap()
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string()
                .to_lowercase();
            if venue_str.contains("aces high") {
                // These are redundant with Aces High's own website
                continue;
            }
            venues.insert(venue_str.clone());

            let city_strs = div_tag
                .select(&scraper::Selector::parse("small").unwrap())
                .next()
                .unwrap()
                .text()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();

            let city_str = city_strs.first().unwrap().to_lowercase();
            let city_str = city_str.replace("salt lake city", "slc");
            let city_str = city_str.replace(",", "slc");

            shows.push(show::Show {
                date,
                artist: artist_str,
                venue: venue_str,
                city: city_str,
                url: link_str.to_string(),
            });
        }

        if page_event_count == 0 {
            break;
        }
        page += 1;
    }
    println!("\tprocessed {} pages", page);
    println!("\tvenues");
    for v in venues {
        println!("\t\t{}", v);
    }
    println!("\tfound {} shows", shows.len());
    shows
}
