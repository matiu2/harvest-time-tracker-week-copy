use std::future::Future;

use harvest_time_tracker::{model::UploadEntry, RequestData};

struct RequestMaker(RequestData);

impl RequestMaker {
    async fn upload_entry(&self, entry: &UploadEntry) {
        log::info!("Uploading time entry: {:?}", entry);
        let res = self
            .0
            .client
            .post("https://api.harvestapp.com/v2/time_entries")
            .header("Authorization", &format!("Bearer {}", self.0.token))
            .header("Harvest-Account-ID", &self.0.account)
            .header("User-Agent", "Matthew's harvest helper")
            .header("Content-Type", "application/json")
            .json(entry)
            .send()
            .await
            .unwrap();
        log::info!("Response: {}", res.status());
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    // Load the data to insert
    let f = std::fs::File::open("to_upload.json").unwrap();
    let data: Vec<UploadEntry> = serde_json::de::from_reader(f).unwrap();

    // Upload them all
    let request_maker = RequestMaker(RequestData::from_env());
    for entry in data {
        request_maker.upload_entry(&entry).await
    }
}
