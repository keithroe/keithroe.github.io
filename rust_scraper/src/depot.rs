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
    println!("processing The Depot ...");

    let mut shows = Vec::new();

    let mut latest_date = Local::now().naive_local().date();
    loop {
        let url = format!("https://www.depotslc.com/shows?start={}", latest_date);
        println!("url '{}'", url);

        let html = util::get_html(&url).unwrap();
        let selector = scraper::Selector::parse("div.chakra-card__footer").unwrap();
        let html_events = html.select(&selector);

        let mut page_event_count = 0;
        for html_event in html_events {
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

            println!("artist: {}", artist_str);
            println!("date_strs: {:?}", date_strs);
            println!("date: {}", date);
            println!("latest date: {}", latest_date);
            println!("url: {}", url_str);

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
    }

    shows
}
