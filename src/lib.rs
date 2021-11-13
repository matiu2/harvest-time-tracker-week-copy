pub mod model;
use chrono::NaiveDate;
use model::{TimeEntries, UploadEntry};
use reqwest::Client;
use std::env;

/// The data needed to make requests to the harvest time tracker API
pub struct HarvestClient {
    pub client: Client,
    pub token: String,
    pub account: String,
}

impl HarvestClient {
    /// Create the data from environment variables
    pub fn from_env() -> HarvestClient {
        HarvestClient {
            client: reqwest::Client::new(),
            token: env::var("token").expect("`token` environment variable"),
            account: env::var("account").expect("`account` environment variable"),
        }
    }

    /// Download a single page of entries from the harvest API
    /// The start URL will probably be: "https://api.harvestapp.com/v2/time_entries"
    pub async fn get_entries(&self, url: &str) -> TimeEntries {
        log::info!("Downloading time entries from: {}", url);
        let res = self
            .client
            .get(url)
            .header("Authorization", &format!("Bearer {}", self.token))
            .header("Harvest-Account-ID", &self.account)
            .header("User-Agent", "Matthew's harvest helper")
            .send()
            .await
            .unwrap();
        res.json().await.unwrap()
    }
    pub async fn upload_entry(&self, entry: &UploadEntry) {
        log::info!("Uploading time entry: {:?}", entry);
        let res = self
            .client
            .post("https://api.harvestapp.com/v2/time_entries")
            .header("Authorization", &format!("Bearer {}", self.token))
            .header("Harvest-Account-ID", &self.account)
            .header("User-Agent", "Matthew's harvest helper")
            .header("Content-Type", "application/json")
            .json(entry)
            .send()
            .await
            .unwrap();
        log::info!("Response: {}", res.status());
    }
}

pub fn parse_date(input: &str) -> NaiveDate {
    NaiveDate::parse_from_str(input, "%Y-%m-%d").unwrap()
}
