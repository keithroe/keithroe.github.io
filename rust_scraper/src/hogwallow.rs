use crate::show;
use crate::util;

/*
    <div class="col-xs-7 mb30 ml10 pt20">
        <a href="music/live/brazuca-band1255.html" title="Brazuca Band (A blend of Brazilian & American Reggae ) Saturday, 11/29/25 @ The Hog Wallow Pub">
            <img src="/assets/components/phpthumbof/cache/brazucaweb.7d28acb8de3eda9fc03a55b5556f6b63.webP" alt="Brazuca Band @ The Hog Wallow Pub" class="roundCircle dsshadow mb10" loading="lazy" height="150" width="150">
            <br>
            <h3 class="mb0 mt0 pt0">
                <a href="music/live/brazuca-band1255.html" title="Brazuca Band (A blend of Brazilian & American Reggae ) Saturday, 11/29/25" class="link-dark bold">Brazuca Band
            </h3>
            <ul class="list-inline social-links">
                <li>
                    <span class="text-primary">A blend of Brazilian & American Reggae </span>
                    <br>
                </li>
                <li>
                    <span class="text-secondary bold upper">Saturday, Nov 29</span>
                    <br>
                    <span class="text-secondary small"> 9:30PM to 12:30AM</span>
                </li>
            </ul>
        </a>
    </div>
*/

pub fn scrape() -> Vec<show::Show> {
    println!("processing hogs wallow ...");

    let mut shows = Vec::new();
    let html = util::get_html(
        "https://thehogwallow.com/music/all-upcoming-live-music/#all-upcoming-live-music",
    )
    .unwrap();

    let base_url = url::Url::parse("https://thehogwallow.com").unwrap();

    for event in html.select(&scraper::Selector::parse("div.col-xs-7").unwrap()) {
        let link_elmt = util::select_single(event, "a").unwrap();
        let url_str = link_elmt.attr("href").unwrap().to_string();
        let url_str = base_url.join(&url_str).unwrap().to_string();

        let artist_str = util::get_text(util::select_single(event, "h3.mb0").unwrap());

        let date_str = util::get_text(util::select_single(event, "span.text-secondary").unwrap());
        let date_strs = date_str
            .split([' ', ','])
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
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
            venue: "hog wallow".to_string(),
            city: "cottonwood".to_string(),
            url: url_str.to_string(),
        });
    }
    println!("\tfound {} shows", shows.len());
    shows
}
