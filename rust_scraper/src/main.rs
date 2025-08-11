use chrono::prelude::*;
mod show;

const HTML_TEMPLATE: &str = r###" 
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="description" content="Listing of Salt Lake City Music Concerts Shows">
    <meta name="keywords" content="Music, Concerts, Shows, Salt, Lake">
    <meta name="author" content="keith">

    <title>slc shows</title>
    <style>
      body {
          background-color:black;
          color:silver;
          font-family:"Courier New";
          font-size:large; 
      }
      a {
          /*text-decoration: none;*/
      }
      a:link {
          color:silver;
      }
      a:visited {
          color:gray;
      }
      tr td{
          max-width: 48ch;
          overflow: hidden;
          text-overflow: ellipsis;
          white-space: nowrap;
      }
   </style>
  </head>
  <body>
    <main>
      <p>
        <a href="#about">about this page</a>
      </p>
      <table id="events_table">
          @TABLE_ROWS@
      </table>
      <div id="about"></div>
      <p id="about">
        <pre class="about"> 
          @ABOUT@
        </pre>
      </p>
    </main>
  </body>
</html>
"###;

fn generate_html_page(shows: &[show::Show]) {
    let html = HTML_TEMPLATE.to_string();

    let date_str = Local::now().naive_local().date().to_string();
    let pre_string = "\nabout this page:\n".to_string();
    let pre_string = pre_string + "\tmissing venues or feedback: slcshowsnet AT gmail DOT com\n";
    let pre_string = pre_string + &format!("\tgenerated on: {}\n---\n\n", &date_str);
    let html = html.replace("@ABOUT@", &pre_string);

    let html = html.replace("@TABLE_ROWS@", &show::generate_table_rows(shows));
    std::fs::write("index.html", &html).expect("failed to write index.html");
}

fn get_html(url: &str) -> scraper::html::Html {
    println!("Loading html '{}' ...", url);

    let response = reqwest::blocking::get(url).unwrap();
    let html = response.text().unwrap();

    scraper::Html::parse_document(&html)
}

fn month_int_from_str(month_str: &str) -> u32 {
    let month_str = month_str.to_lowercase();
    if month_str.starts_with("ja") {
        1
    } else if month_str.starts_with("f") {
        2
    } else if month_str.starts_with("mar") {
        3
    } else if month_str.starts_with("ap") {
        4
    } else if month_str.starts_with("may") {
        5
    } else if month_str.starts_with("jun") {
        6
    } else if month_str.starts_with("jul") {
        7
    } else if month_str.starts_with("au") {
        8
    } else if month_str.starts_with("s") {
        9
    } else if month_str.starts_with("o") {
        10
    } else if month_str.starts_with("n") {
        11
    } else if month_str.starts_with("d") {
        12
    } else {
        println!(
            "ERROR: month_int_from_str given invalid string '{}'",
            month_str
        );
        0
    }
}

fn create_date(day: u32, month: u32) -> chrono::NaiveDate {
    let today = chrono::Local::now().naive_local().date();
    let mut date = NaiveDate::from_ymd_opt(today.year(), month, day).unwrap();
    if date < today {
        date = date.checked_add_months(chrono::Months::new(12)).unwrap();
    }
    date
}

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

fn scrape_24tix() -> Vec<show::Show> {
    println!("processing 24tix ...");

    let mut page = 0;
    let mut shows = Vec::new();
    loop {
        let url = format!("https://www.24tix.com/?batch_page={}", page);
        let html = get_html(&url);
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

            let date = create_date(
                date_strs[1].parse::<u32>().unwrap(),
                month_int_from_str(date_strs[0]),
            );

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

            println!("{}", link_str);
            println!("{}", artist_str);
            println!("{}", date_str);
            println!("{}", date);
            println!("{}", venue_str);
            println!("{}", city_str);

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
    shows
}

fn main() {
    println!("Scraping ...");

    let mut shows = Vec::new();
    shows.append(&mut scrape_24tix());
    generate_html_page(&shows);
}
