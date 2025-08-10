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
