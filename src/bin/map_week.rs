//! Reads in source_week.json created by 'extract.rs' and given new dates, creates dest_week.json

use std::fs::File;
use std::ops::Add;

use chrono::NaiveDate;
use harvest_time_tracker::model::{TimeEntry, UploadEntry};

fn parse_date(input: &str) -> NaiveDate {
    NaiveDate::parse_from_str(input, "%Y-%m-%d").unwrap()
}

fn main() {
    pretty_env_logger::init();

    // Get the start date from the parameters
    let start_date = std::env::args()
        .skip(1)
        .next()
        .expect("Start date argument. eg. 2021-12-01");
    let start_date = parse_date(&start_date);

    // Load the data
    let f = File::open("source_week.json").unwrap();
    let source: Vec<TimeEntry> = serde_json::de::from_reader(f).unwrap();
    let mut dest: Vec<UploadEntry> = source.into_iter().map(|e| e.into()).collect();

    // Sort it by date
    dest.sort_by(|a, b| a.spent_date.cmp(&b.spent_date));

    // How far ahead in the future are we ?
    let data_start = parse_date(&dest.first().unwrap().spent_date);
    let difference = start_date - data_start;

    // Add the difference to each of the dates
    dest.iter_mut().for_each(|entry| {
        let date = parse_date(&entry.spent_date);
        let new_date = date.add(difference);
        entry.spent_date = new_date.format("%Y-%m-%d").to_string();
    });

    let out_fn = "to_upload.json";
    let f = File::create(&out_fn).unwrap();
    serde_json::ser::to_writer_pretty(f, &dest).unwrap();
    log::info!("Saved to {}", out_fn);
}
