//! Extracts data from data.json

use std::fs::File;

use chrono::Duration;
use harvest_time_tracker::{
    date::FromHarvestDate,
    model::{TimeEntries, TimeEntry},
};
fn main() {
    pretty_env_logger::init();

    // Get the start of the week
    let start = std::env::args()
        .skip(1)
        .take(1)
        .next()
        .expect("Start of week to copy from required");
    let start = start.from_harvest_date();
    let end = start + Duration::days(5);

    // Read the data
    let f = File::open("data.json").unwrap();
    let data: Vec<TimeEntries> = serde_json::de::from_reader(&f).unwrap();
    let entries = || data.iter().flat_map(|entries| &entries.time_entries);

    // Now get all the entries that are in our week
    let week: Vec<&TimeEntry> = entries()
        .filter(|e| {
            let date = e.spent_date.from_harvest_date();
            date >= start && date <= end
        })
        .inspect(|entry| log::debug!("Saving entry: {:?}", entry))
        .collect();
    let file_name = "source_week.json";
    let f = File::create(file_name).unwrap();
    serde_json::ser::to_writer_pretty(f, &week).unwrap();
    log::info!("{} saved", file_name);
}
