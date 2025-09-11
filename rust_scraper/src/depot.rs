use crate::show;
use crate::util;
use chrono::prelude::*;

/*
<div class="chakra-card__footer css-6nvkkc">
 <div class="css-1cbuemh">
  <div class="css-1u2896s">
   <p class="chakra-text css-zvlevn">
    Nero
   </p>
   <p class="chakra-text css-1g5zdf0">
    Sat Nov 16, 2024
   </p>
   <div class="css-7timbt">
    <div class="css-0">
     <a class="chakra-button css-1asqcxu" href="https://concerts.livenation.com/nero-salt-lake-city-utah-11-16-2024/event/1E0060BFC2A05247" target="_blank">
      Buy Tickets
     </a>
    </div>
    <div class="css-14dycje">
     <button class="chakra-button css-16uafp" type="button">
      More Info
     </button>
    </div>
   </div>
  </div>
 </div>
 <div class="css-27cwld">
  <button aria-label="More Info" class="css-1v6uy3d">
   <i aria-hidden="true" class="icn icn-ellipses __className_f3e3e6 css-1eqjgjs" focusable="false">
   </i>
  </button>
 </div>
 <time class="css-kfisjo">
  <p class="chakra-text css-1yp7tc1">
   Sat
  </p>
  <p class="chakra-text date-box-date css-go0khb">
   16
  </p>
  <p class="chakra-text css-1yp7tc1">
   Nov
  </p>
 </time>
 <div class="css-l1pvlg">
  <a class="chakra-linkbox__overlay css-a0kmza" href="https://concerts.livenation.com/nero-salt-lake-city-utah-11-16-2024/event/1E0060BFC2A05247" target="_blank">
   Nero
  </a>
 </div>
</div>
*/

pub fn scrape() -> Vec<show::Show> {
    println!("processing the depot ...");

    let mut shows = Vec::new();

    let mut latest_date = Local::now().naive_local().date();
    loop {
        let url = format!("https://www.depotslc.com/shows?start={}", latest_date);
        let html = util::get_html(&url).unwrap();

        let mut page_event_count = 0;
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

            let artist_str = json_map["name"].as_str().unwrap();
            let url_str = json_map["url"].as_str().unwrap();
            let date_str = json_map["startDate"]
                .as_str()
                .unwrap()
                .split("T")
                .next()
                .unwrap()
                .to_string();
            let date = chrono::naive::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").unwrap();
            if date < latest_date {
                continue;
            }
            latest_date = date.checked_add_days(chrono::Days::new(1)).unwrap();

            shows.push(show::Show {
                date,
                artist: artist_str.to_string(),
                venue: "the depot".to_string(),
                city: "slc".to_string(),
                url: url_str.to_string(),
            });
            page_event_count += 1;
        }
        if page_event_count == 0 {
            break;
        }

        /*
        let selector = scraper::Selector::parse("div.chakra-card__footer").unwrap();
        let html_events = html.select(&selector);

        let mut page_event_count = 0;
        for html_event in html_events {
            println!("here 0");
            page_event_count += 1;

            let selector = scraper::Selector::parse("p").unwrap();
            let mut ps = html_event.select(&selector);
            let artist_str = ps
                .next()
                .unwrap()
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string();

            let date_strs = ps
                .next()
                .unwrap()
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .split([' ', ','])
                .map(|s| s.to_string())
                .collect::<Vec<_>>();

            let date = util::create_date(
                date_strs[2].parse::<u32>().unwrap(),
                util::month_int_from_str(&date_strs[1]).unwrap(),
            )
            .unwrap();
            latest_date = date.checked_add_days(chrono::Days::new(1)).unwrap();

            let url_str;
            if let Some(url_string) = html_event
                .select(&scraper::Selector::parse("a").unwrap())
                .next()
                .unwrap()
                .attr("href")
            {
                url_str = url_string;
            } else {
                continue;
            }

            shows.push(show::Show {
                date,
                artist: artist_str,
                venue: "the depot".to_string(),
                city: "slc".to_string(),
                url: url_str.to_string(),
            });
        }

        if page_event_count == 0 {
            break;
        }
        */
    }

    println!("\tfound {} shows", shows.len());
    shows
}
