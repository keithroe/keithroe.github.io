use chrono::prelude::*;
use std::time;

#[derive(Debug, Ord, PartialOrd)]
pub struct Show {
    pub date: NaiveDate,
    pub artist: String,
    pub venue: String,
    pub city: String,
    pub url: String,
}

impl Show {
    pub fn in_past(self) -> bool {
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
        <tr>
            <pre><td>@DATE@&nbsp;&nbsp;</td></pre>
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
pub fn generate_table_rows(shows: &[Show]) -> String {
    let mut rows: Vec<String> = Vec::new();
    for show in shows {
        let row = ROW_HTML_TEMPLATE;
        let row = row.replace("@DATE@", &show.date.to_string());
        let row = row.replace("@URL@", &show.url);
        let row = row.replace("@ARTIST@", &show.artist);
        let row = row.replace("@CITY@", &show.city);
        let row = row.replace("@VENUE@", &show.venue);
        rows.push(row);
    }

    //rows.join("\n")
    rows.first().unwrap().to_string()
}
