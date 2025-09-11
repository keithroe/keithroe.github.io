use crate::show;
use crate::util;
use url;

/*
<div class="eventlist-column-info">
  <h1 class="eventlist-title">
    <a href="/upcomingevents/finneas" class="eventlist-title-link">
      FINNEAS - For Cryin' Out Loud!: The Tour
    </a>
  </h1>
  <ul class="eventlist-meta event-meta">
    <li class="eventlist-meta-item eventlist-meta-date event-meta-item">
      <time class="event-date" datetime="2025-03-02">Sunday, March 2, 2025</time>
    </li>
    <li class="eventlist-meta-item eventlist-meta-time event-meta-item">
      <span class="event-time-12hr">
        <time class="event-time-12hr-start" datetime="2025-03-02">6:30 PM</time>
        <span class="event-datetime-divider"></span>
        <time class="event-time-12hr-end" datetime="2025-03-02">10:30 PM</time>
      </span>
      <span class="event-time-24hr">
        <time class="event-time-24hr-start" datetime="2025-03-02">18:30</time>
        <span class="event-datetime-divider"></span>
        <time class="event-time-12hr-end" datetime="2025-03-02">22:30</time>
      </span>
    </li>
      <li class="eventlist-meta-item eventlist-meta-address event-meta-item">
          The Union Event Center
        <a href="http://maps.google.com?q=235 North 500 West Salt Lake City, Utah, 84116 United States" class="eventlist-meta-address-maplink" target="_blank">(map)</a>
      </li>
    <li class="eventlist-meta-item eventlist-meta-export event-meta-item">
      <a href="http://www.google.com/calendar/event?action=TEMPLATE&text=FINNEAS%20-%20For%20Cryin%27%20Out%20Loud%21%3A%20The%20Tour&dates=20250303T013000Z/20250303T053000Z&location=235%20North%20500%20West%2C%20Salt%20Lake%20City%2C%20Utah%2C%2084116%2C%20United%20States" class="eventlist-meta-export-google">Google Calendar</a>
      <span class="eventlist-meta-export-divider"></span>
      <a href="/upcomingevents/finneas?format=ical" class="eventlist-meta-export-ical">ICS</a>
    </li>
  </ul>
  <div class="eventlist-excerpt"><p class=""><strong>ALL AGES</strong></p></div>
  <a href="/upcomingevents/finneas" class="eventlist-button sqs-button-element--primary">
    View Event &#8594;
  </a>
  <div class="eventlist-actions">
    <span class="sqs-simple-like" data-item-id="66fec350d7a785280bdefe8e" data-like-count="0">
      <span class="like-icon"></span>
      <span class="like-count"></span>
    </span>
    <span class="squarespace-social-buttons inline-style" data-system-data-id="1728408791646-BGQO7U76G82Q40JHBM35" data-asset-url="https://images.squarespace-cdn.com/content/v1/5a48752132601ea2c0890ed8/1728408791646-BGQO7U76G82Q40JHBM35/Static_Social-Instagram_1080x1080_Finneas_2025_Regional_TheUnionEventCenter_0302.jpg" data-record-type="12" data-full-url="/upcomingevents/finneas" data-title="FINNEAS - For Cryin' Out Loud!: The Tour">
    </span>
  </div>
</div>
*/

pub fn scrape() -> Vec<show::Show> {
    println!("processing the union ...");

    let mut shows = Vec::new();
    let union_url = url::Url::parse("https://theunioneventcenter.com/").unwrap();
    let html = util::get_html(&union_url.to_string()).unwrap();

    let event_selector = scraper::Selector::parse("div.eventlist-column-info").unwrap();
    let html_events = html.select(&event_selector);
    for html_event in html_events {
        let link_selector = scraper::Selector::parse("a").unwrap();
        let link_elem = html_event.select(&link_selector).next().unwrap();
        let url_str = link_elem.attr("href").unwrap();
        let url_str = union_url.join(url_str).unwrap().to_string();
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

        // the union lists past events in the html (they are not displayed)
        if date < chrono::Local::now().naive_local().date() {
            continue;
        }

        shows.push(show::Show {
            date,
            artist: artist_str.to_string(),
            venue: "the union".to_string(),
            city: "slc".to_string(),
            url: url_str.to_string(),
        });
    }

    println!("\tfound {} shows", shows.len());
    shows
}
