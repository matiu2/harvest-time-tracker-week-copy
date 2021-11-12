//! Downloads time entries from the API to `data.json`

use std::fs::File;

use harvest_time_tracker::{model::TimeEntries, RequestData};
use serde_json::ser::to_writer_pretty;

struct RequestMaker(RequestData);

impl RequestMaker {
    async fn get_entries(&self, url: &str) -> TimeEntries {
        log::info!("Downloading time entries from: {}", url);
        let res = self
            .0
            .client
            .get(url)
            .header("Authorization", &format!("Bearer {}", self.0.token))
            .header("Harvest-Account-ID", &self.0.account)
            .header("User-Agent", "Matthew's harvest helper")
            .send()
            .await
            .unwrap();
        res.json().await.unwrap()
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    // Get the token and project from the environment variables
    let request_maker = RequestMaker(RequestData::from_env());
    // Get all the time_entries
    let mut entries = Vec::new();
    // The last (most recent) page read
    let mut last = request_maker
        .get_entries("https://api.harvestapp.com/v2/time_entries")
        .await;
    loop {
        let next_page = last.links.next.clone();
        // Save the last page we read
        entries.push(last);
        // While there's a new page..
        if let Some(next_page) = next_page {
            // ..read it
            last = request_maker.get_entries(&next_page).await;
        } else {
            break;
        }
    }
    // Save what we have to a file
    let file_name = "data.json";
    let f = File::create(file_name).unwrap();
    to_writer_pretty(f, &entries).unwrap();
    log::info!("Saved {} pages of entries to {}", entries.len(), file_name);
}
