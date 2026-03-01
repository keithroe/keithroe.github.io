use crate::show;
use crate::util;
use regex::Regex;

/*

<div class="each-event clearfix">
    <div class="image">
         <img src="http://maverikcenter.com.ismmedia.com/ISM3/std-content/repos/Top/Calendar/grizzvsicemen.png" alt=""/>
    </div>
    <div class="data">

    <div class="data-info">
         <h5>October 30, 2025</h5>
         <h4>Grizzlies vs Jacksonville Icemen </h4>
         <h5>07:10 PM &#47; Doors 6:00PM </h5>
     </div>

     <div class="buttons">
         <a class="button" href="https://www.ticketmaster.com/event/1E0062F889D02593" target="_blank">Buy Tickets</a>
         <a class="button" href="/events-tickets/upcoming-events/details/?event_id=10557">More Info</a>
     </div>
</div>

*/

/*
<div class="event-info">
    <div class="event-date"> March 14, 7:10PM </div>
    <div class="event-title"> Utah Grizzlies vs. Tulsa Oilers </div>
    <div class="sub-title"></div>
    <div class="event-desc"> </div>
    <div class="event-action">
        <a href="https://maverikcenter.com/event/utah-grizzlies-vs-tulsa-oilers-2/?event_source=featured-events" class="btn-link">Event Info</a>
        <a href="https://www.ticketmaster.com/utah-grizzlies-vs-tulsa-oilers-west-valley-city-utah-03-14-2026/event/1E0062F889AA237E" target="blank" class="btn btn-default home_get_tickets" data-type="Sports" data-genre="Hockey" data-subgenre="Minor League" data-organizer="PROMOTED BY VENUE">Get Tickets</a>
    </div>
</div>
*/

pub fn scrape() -> Vec<show::Show> {
    println!("processing maverik center ...");

    let mut shows = Vec::new();
    //let html = util::get_html("https://maverikcenter.com/events-tickets/upcoming-events/").unwrap();
    let html = util::get_html("https://maverikcenter.com/events/").unwrap();
    let date_re = Regex::new(r"[0-9]{2}-[0-9]{2}-[0-9]{4}").unwrap();

    for event in html.select(&scraper::Selector::parse("div.event-info").unwrap()) {
        let date_div_elmt = util::select_single(event, "div.event-date").unwrap();
        /*
        let date_str = util::get_text(date_div_elmt);
        let date_tokens: Vec<_> = date_str
            .split([' ', ','])
            .filter(|x| !x.is_empty())
            .collect();

        let date = util::create_date(
            date_tokens[1].parse::<u32>().unwrap(),
            util::month_int_from_str(date_tokens[0]).unwrap(),
        )
        .unwrap();
        */

        let artist_div_elmt = util::select_single(event, "div.event-title").unwrap();
        let artist_str = util::get_text(artist_div_elmt);
        if artist_str.contains("Grizzlies") {
            continue;
        }

        let action_div_elmt = if let Ok(val) = util::select_single(event, "div.event-action") {
            val
        } else {
            continue;
        };

        let link_elmt = if let Ok(val) = util::select_single(action_div_elmt, "a.home_get_tickets")
        {
            val
        } else {
            continue;
        };

        let url_str = link_elmt.attr("href").unwrap().to_string();

        let date_str;
        match date_re.find(&url_str) {
            Some(match_obj) => {
                date_str = match_obj.as_str();
            }
            None => continue,
        }
        let Ok(date) = chrono::naive::NaiveDate::parse_from_str(&date_str, "%m-%d-%Y") else {
            continue;
        };

        shows.push(show::Show {
            date,
            artist: artist_str,
            venue: "maverik center".to_string(),
            city: "west valley".to_string(),
            url: url_str.to_string(),
        });
    }
    println!("\tfound {} shows", shows.len());
    shows
}
