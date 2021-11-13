//! Downloads time entries from the API to `data.json`

use std::fs::File;

use harvest_time_tracker::HarvestClient;
use serde_json::ser::to_writer_pretty;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    // Get the token and project from the environment variables
    let harvest_client = HarvestClient::from_env();
    // Get all the time_entries
    let entries = harvest_client.get_entries().await;
    // Save what we have to a file
    let file_name = "data.json";
    let f = File::create(file_name).unwrap();
    to_writer_pretty(f, &entries).unwrap();
    log::info!("Saved {} pages of entries to {}", entries.len(), file_name);
}
