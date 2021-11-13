use crate::model::{TimeEntries, UploadEntry};
use reqwest::Client;
use std::env;

pub struct HarvestClient {
    pub client: Client,
    pub token: String,
    pub account: String,
}

const TOKEN_NAME: &str = "token";
const ACCOUNT_NAME: &str = "account_id";

impl HarvestClient {
    /// Create the data from environment variables
    pub fn from_env() -> HarvestClient {
        HarvestClient {
            client: reqwest::Client::new(),
            token: env::var(TOKEN_NAME).expect("`token` environment variable"),
            account: env::var(ACCOUNT_NAME).expect("`account` environment variable"),
        }
    }

    /// Upload a single time entry
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

    /// Download a single page of entries from the harvest API
    async fn get_one_page(&self, url: &str) -> TimeEntries {
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

    /// Downloads all the time entries that the API can give us
    /// (The API doesn't have any search functionality)
    pub async fn get_entries(&self) -> Vec<TimeEntries> {
        let mut downloader = Downloader::new(self);
        downloader.get_all().await
    }
}

/// Small abstraction over downloading multiple pages
pub struct Downloader<'a> {
    client: &'a HarvestClient,
    url: Option<String>,
}

impl<'a> Downloader<'a> {
    pub fn new(client: &HarvestClient) -> Downloader {
        Downloader {
            client,
            url: Some("https://api.harvestapp.com/v2/time_entries".to_string()),
        }
    }

    /// Download the next page if there is one
    pub async fn next(&mut self) -> Option<TimeEntries> {
        match &self.url {
            Some(url) => {
                let entries = self.client.get_one_page(url).await;
                self.url = entries.links.next.clone();
                Some(entries)
            }
            None => None,
        }
    }

    /// Gets all the time entries
    pub async fn get_all(&mut self) -> Vec<TimeEntries> {
        let mut out = Vec::new();
        while let Some(next) = self.next().await {
            out.push(next)
        }
        out
    }
}
