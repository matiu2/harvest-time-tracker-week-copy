//! Reads in source_week.json created by 'extract.rs' and given new dates, creates dest_week.json

use std::fs::File;

use harvest_time_tracker::{
    date::{FromHarvestDate, ToHarvestDate},
    model::{TimeEntry, UploadEntry},
};

fn main() {
    pretty_env_logger::init();

    // Get the start date from the parameters
    let start_date = std::env::args()
        .nth(2)
        .expect("Start date argument. eg. 2021-12-01");
    let start_date = start_date.from_harvest_date();

    // Load the data
    let f = File::open("source_week.json").unwrap();
    let source: Vec<TimeEntry> = serde_json::de::from_reader(f).unwrap();
    let mut dest: Vec<UploadEntry> = source.into_iter().map(|e| e.into()).collect();

    // Sort it by date
    dest.sort_by(|a, b| a.spent_date.cmp(&b.spent_date));

    // How far ahead in the future are we ?
    let data_start = dest.first().unwrap().spent_date.from_harvest_date();
    let difference = start_date - data_start;

    // Add the difference to each of the dates
    dest.iter_mut().for_each(|entry| {
        let date = entry.spent_date.from_harvest_date();
        let new_date = date + difference;
        entry.spent_date = new_date.to_harvest_date();
    });

    let out_fn = "to_upload.json";
    let f = File::create(&out_fn).unwrap();
    serde_json::ser::to_writer_pretty(f, &dest).unwrap();
    log::info!("Saved to {}", out_fn);
}
