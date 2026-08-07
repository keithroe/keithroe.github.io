use chrono::prelude::*;
use serde::{Deserialize, Serialize};

// date_created must stay last: Ord is derived and generate_table_rows sorts on it
#[derive(Debug, Ord, PartialOrd, Clone, Serialize, Deserialize)]
pub struct Show {
    pub date: NaiveDate,
    pub artist: String,
    pub venue: String,
    pub city: String,
    pub url: String,
    pub date_created: Option<NaiveDate>,
}

// Venue pages are inconsistent about non-breaking spaces and runs of
// whitespace, and the same listing can gain or lose one between runs. Without
// this a show would change identity and look newly discovered.
fn normalize(text: impl Into<String>) -> String {
    text.into().split_whitespace().collect::<Vec<_>>().join(" ")
}

impl Show {
    // scrapers leave date_created empty; store::Store::stamp fills it in
    pub fn new(
        date: NaiveDate,
        artist: impl Into<String>,
        venue: impl Into<String>,
        city: impl Into<String>,
        url: impl Into<String>,
    ) -> Self {
        Show {
            date,
            artist: normalize(artist),
            venue: normalize(venue),
            city: normalize(city),
            url: url.into().trim().to_string(),
            date_created: None,
        }
    }

    // identity as used by PartialEq, so the store agrees with dedup.
    // Normalizes again because deserializing an older shows.json bypasses new().
    pub fn key(&self) -> (String, NaiveDate) {
        (normalize(&self.artist).to_lowercase(), self.date)
    }

    pub fn in_past(&self) -> bool {
        self.date < Local::now().naive_local().date()
    }
}

impl PartialEq for Show {
    fn eq(&self, other: &Self) -> bool {
        self.artist.to_lowercase() == other.artist.to_lowercase() && self.date == other.date
    }
}

impl Eq for Show {}

const ROW_HTML_TEMPLATE: &str = r###"
        <tr class="@ROW_CLASS@">
            <pre><td class="date">@DATE@&nbsp;&nbsp;</td></pre>
            <td>
                <a href="@URL@">
                    @ARTIST@  
                </a>
            </td>
            <pre><td>&nbsp;&nbsp;</td></pre>
            <td>
                @CITY@ 
            </td>
            <pre><td>&nbsp;&nbsp;</td></pre>
            <td>
                @VENUE@
            </td>
        </tr>
"###;

// Highlights recently discovered shows: the css classes are defined alongside
// the page template in main.rs.
fn row_class(show: &Show, today: NaiveDate) -> &'static str {
    let Some(date_created) = show.date_created else {
        return "";
    };
    match (today - date_created).num_days() {
        0..=6 => "added-this-week",
        7..=13 => "added-last-week",
        _ => "",
    }
}

fn date_string(date: NaiveDate) -> String {
    let month_strs = &[
        "???", "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
    ];

    format!("{} {:02}", month_strs[date.month() as usize], date.day())
}

pub fn generate_table_rows(shows: &[Show]) -> String {
    let mut shows = shows.to_vec();
    shows.sort();
    shows.dedup();
    let mut last_date = Option::None;
    let today = Local::now().naive_local().date();

    let mut rows: Vec<String> = Vec::new();
    for show in shows {
        if show.in_past() {
            continue;
        }

        let date_str = match Some(show.date) {
            last if last == last_date => "      ".to_string(),
            _ => {
                last_date = Some(show.date);
                date_string(show.date)
            }
        };

        let row = ROW_HTML_TEMPLATE;
        //let row = row.replace("@DATE@", &show.date.to_string());
        let row = row.replace("@ROW_CLASS@", row_class(&show, today));
        let row = row.replace("@DATE@", &date_str);
        let row = row.replace("@URL@", &show.url);
        let row = row.replace("@ARTIST@", &show.artist);
        let row = row.replace("@CITY@", &show.city);
        let row = row.replace("@VENUE@", &show.venue);
        rows.push(row);
    }

    rows.join("\n")
    //rows.first().unwrap().to_string()
}
