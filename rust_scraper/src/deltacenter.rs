use crate::show;
use crate::util;

/*
  <article class=" mec-event-article mec-clear " itemscope>
    <div class="mec-event-image">
      <a data-event-id="7328" href="https://www.deltacenter.com/events/xavi/" target="_self" rel="noopener">
        <img width="900" height="500" alt="Xavi at Delta Center" data-mec-postid="7328" sizes="(max-width: 900px) 100vw, 900px" nitro-lazy-srcset="https://cdn-ilchegh.nitrocdn.com/vkSMQxJlMubOkFHBODwmZDCiQRtlwTUe/assets/images/optimized/rev-080c957/www.deltacenter.com/wp-content/uploads/2025/08/Xavi_web_900x500.jpg 900w, https://cdn-ilchegh.nitrocdn.com/vkSMQxJlMubOkFHBODwmZDCiQRtlwTUe/assets/images/optimized/rev-080c957/www.deltacenter.com/wp-content/uploads/2025/08/Xavi_web_900x500-300x167.jpg 300w, https://cdn-ilchegh.nitrocdn.com/vkSMQxJlMubOkFHBODwmZDCiQRtlwTUe/assets/images/optimized/rev-080c957/www.deltacenter.com/wp-content/uploads/2025/08/Xavi_web_900x500-768x427.jpg 768w" nitro-lazy-src="https://cdn-ilchegh.nitrocdn.com/vkSMQxJlMubOkFHBODwmZDCiQRtlwTUe/assets/images/optimized/rev-080c957/www.deltacenter.com/wp-content/uploads/2025/08/Xavi_web_900x500.jpg" class="attachment-full size-full wp-post-image nitro-lazy" decoding="async" nitro-lazy-empty id="MTcyMDo2OTI=-1" src="data:image/svg+xml;nitro-empty-id=MTcyMDo2OTI=-1;base64,PHN2ZyB2aWV3Qm94PSIwIDAgOTAwIDUwMCIgd2lkdGg9IjkwMCIgaGVpZ2h0PSI1MDAiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+PC9zdmc+" />
      </a>
    </div>

    <div class="mec-event-content">
      <h3 class="mec-event-title">
//      <a class="mec-color-hover" data-event-id="7328" href="https://www.deltacenter.com/events/xavi/" target="_self" rel="noopener">Xavi </a>
        <span class="event-color" style=""></span>
      </h3>
      <div class="event_date_grids">
        <ul>
          <li>
//          <span class="mec-start-date-label" itemprop="startDate">16 Jan 2026</span>
          </li>
          <li>
            <div class="mec-time-details">
              <span class="mec-start-time">8:00 pm</span>
            </div>
          </li>
        </ul>
      </div>
      <p class="mec-grid-event-location">Delta Center, Salt Lake City, Utah</p>
      <div class="mec-event-footer">
        <ul class="mec-event-sharing-wrap">
          <li class="mec-event-share">
            <a href="#" class="mec-event-share-icon">
              <i class="mec-sl-share"></i>
            </a>
          </li>
          <li>
            <ul class="mec-event-sharing">
              <li class="mec-event-social-icon">
                <a class="facebook" href="https://www.facebook.com/sharer/sharer.php?u=https%3A%2F%2Fwww.deltacenter.com%2Fevents%2Fxavi%2F" onclick="javascript:window.open(this.href, '', 'menubar=no,toolbar=no,resizable=yes,scrollbars=yes,height=500,width=600'); return false;" title="Share on Facebook">
                  <i class="mec-fa-facebook"></i>
                </a>
              </li>
              <li class="mec-event-social-icon">
                <a class="twitter" href="https://twitter.com/share?url=https%3A%2F%2Fwww.deltacenter.com%2Fevents%2Fxavi%2F" onclick="javascript:window.open(this.href, '', 'menubar=no,toolbar=no,resizable=yes,scrollbars=yes,height=600,width=500'); return false;" target="_blank" title="Tweet">
                  <i class="mec-fa-twitter"></i>
                </a>
              </li>
              <li class="mec-event-social-icon">
                <a class="linkedin" href="https://www.linkedin.com/shareArticle?mini=true&url=https%3A%2F%2Fwww.deltacenter.com%2Fevents%2Fxavi%2F" onclick="javascript:window.open(this.href, '', 'menubar=no,toolbar=no,resizable=yes,scrollbars=yes,height=600,width=500'); return false;" target="_blank" title="Linkedin">
                  <i class="mec-fa-linkedin"></i>
                </a>
              </li>
              <li class="mec-event-social-icon">
                <a class="email" href="/cdn-cgi/l/email-protection#d1eea2a4b3bbb4b2a5ec89b0a7b8f7b3beb5a8ecb9a5a5a1a2f4e290f4e397f4e397a6a6a6ffb5b4bda5b0b2b4bfa5b4a3ffb2bebcf4e397b4a7b4bfa5a2f4e397a9b0a7b8f4e397" title="Email">
                  <i class="mec-fa-envelope"></i>
                </a>
              </li>
            </ul>
          </li>
        </ul>
        <div class="single_custom_mec event_listing_btn">
          <a class="mec-bg-color" target="_self" href="https://seatgeek.com/xavi-tickets/salt-lake-city-utah-delta-center-1-2026-01-16-8-pm/concert/17651833?aid=16139&#038;pid=website&#038;rid=82525&#038;utm_medium=partnership&#038;utm_source=delta_center_ticketing&#038;utm_campaign=website" aria-label="Buy tickets for Xavi">Buy Tickets</a>
        </div>
        <div class="view__detail_btn">
          <a class="mec-booking-button" aria-label="View Details for Xavi" data-event-id="7328" href="https://www.deltacenter.com/events/xavi/" target="_self" rel="noopener">Details</a>
        </div>
      </div>
    </div>
  </article>
*/

pub fn scrape() -> Vec<show::Show> {
    println!("processing delta center ...");

    let mut shows = Vec::new();
    let html = util::get_html("https://www.deltacenter.com/event-type/concert/").unwrap();

    for event in html.select(&scraper::Selector::parse("div.mec-event-content").unwrap()) {
        let link_elmt = util::select_single(event, "a.mec-color-hover").unwrap();
        let url_str = link_elmt.attr("href").unwrap().to_string();
        let artist_str = util::get_text(link_elmt);

        let date_elmt = util::select_single(event, "span.mec-start-date-label").unwrap();
        let date_strs = util::get_text(date_elmt)
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        /*
        println!("artist: {}", artist_str);
        println!("url   : {}", url_str);
        println!("date  : {:?}", date_strs);
        */
        let date = util::create_date(
            date_strs[0].parse::<u32>().unwrap(),
            util::month_int_from_str(&date_strs[1]).unwrap(),
        )
        .unwrap();

        shows.push(show::Show {
            date,
            artist: artist_str,
            venue: "delta center".to_string(),
            city: "slc".to_string(),
            url: url_str.to_string(),
        });
    }
    println!("\tfound {} shows", shows.len());
    shows
}
