use crate::show;
use crate::util;

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

pub fn scrape() -> Vec<show::Show> {
    println!("processing maverik center ...");

    let mut shows = Vec::new();
    let html = util::get_html("https://maverikcenter.com/events-tickets/upcoming-events/").unwrap();

    for event in html.select(&scraper::Selector::parse("div.each-event").unwrap()) {
        let data_div_elmt = util::select_single(event, "div.data-info").unwrap();

        let date_str = util::get_text(util::select_single(data_div_elmt, "h5").unwrap());
        let artist_str = util::get_text(util::select_single(data_div_elmt, "h4").unwrap());

        if artist_str.contains("Grizzlies") {
            continue;
        }

        let link_elmt = util::select_single(event, "div.buttons > a").unwrap();
        let url_str = link_elmt.attr("href").unwrap().to_string();

        let date_tokens: Vec<_> = date_str
            .split([' ', ','])
            .filter(|x| !x.is_empty())
            .collect();

        let date = util::create_date(
            date_tokens[1].parse::<u32>().unwrap(),
            util::month_int_from_str(date_tokens[0]).unwrap(),
        )
        .unwrap();

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
