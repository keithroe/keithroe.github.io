use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result};
use chrono::NaiveDate;

use crate::show::Show;

// Remembers the date each show was first scraped, across runs, via a json file
// kept next to index.html.
pub struct Store {
    first_seen: HashMap<(String, NaiveDate), NaiveDate>,
    previous: Vec<Show>,
}

impl Store {
    pub fn load(path: &Path) -> Result<Store> {
        let json = match std::fs::read_to_string(path) {
            Ok(json) => json,
            // no store yet: this is the first run, everything scraped is new
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                println!("\tno existing '{}', starting fresh", path.display());
                return Ok(Store {
                    first_seen: HashMap::new(),
                    previous: Vec::new(),
                });
            }
            Err(err) => return Err(err).context(format!("failed to read '{}'", path.display())),
        };

        // A parse failure is deliberately fatal. Falling back to an empty store
        // would restamp every show with today's date and silently destroy the
        // entire history.
        let shows: Vec<Show> =
            serde_json::from_str(&json).context(format!("failed to parse '{}'", path.display()))?;

        let mut first_seen = HashMap::new();
        for show in &shows {
            if let Some(date_created) = show.date_created {
                first_seen.insert(show.key(), date_created);
            }
        }

        println!("\tloaded {} shows from '{}'", first_seen.len(), path.display());
        Ok(Store {
            first_seen,
            previous: shows,
        })
    }

    // Fills in date_created on every show: the remembered date if we have seen
    // it before, otherwise today.
    pub fn stamp(&self, shows: &mut [Show], today: NaiveDate) {
        let mut new_count = 0;
        for show in shows.iter_mut() {
            show.date_created = Some(match self.first_seen.get(&show.key()) {
                Some(date_created) => *date_created,
                None => {
                    // past shows are pruned from the store, so they look new
                    // every run; only count the ones that will be listed
                    if show.date >= today {
                        new_count += 1;
                    }
                    today
                }
            });
        }
        println!("\t{} shows seen for the first time today", new_count);
    }

    pub fn save(&self, shows: &[Show], path: &Path, today: NaiveDate) -> Result<()> {
        // drop shows that have already happened so the file stays bounded
        let mut shows: Vec<Show> = shows.iter().filter(|s| s.date >= today).cloned().collect();

        // Carry forward anything we knew about but did not scrape this run. A
        // venue site being down makes get_html return no shows at all, and
        // without this those shows would lose their first-seen date and come
        // back looking new tomorrow.
        let scraped: std::collections::HashSet<_> = shows.iter().map(|s| s.key()).collect();
        let mut carried = 0;
        for show in &self.previous {
            if show.date >= today && !scraped.contains(&show.key()) {
                shows.push(show.clone());
                carried += 1;
            }
        }
        if carried > 0 {
            println!("\tcarried forward {} shows not seen this run", carried);
        }

        // sort and dedup so the committed file diffs cleanly between runs
        shows.sort();
        shows.dedup();

        let json = serde_json::to_string_pretty(&shows)?;

        // write to a temp file and rename, so an interrupted run leaves the
        // previous store intact rather than a truncated one
        let tmp_path = path.with_extension("json.tmp");
        std::fs::write(&tmp_path, &json)
            .context(format!("failed to write '{}'", tmp_path.display()))?;
        std::fs::rename(&tmp_path, path)
            .context(format!("failed to replace '{}'", path.display()))?;

        println!("\twrote {} shows to '{}'", shows.len(), path.display());
        Ok(())
    }
}
