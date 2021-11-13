use harvest_time_tracker::{model::UploadEntry, HarvestClient};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    // Load the data to insert
    let f = std::fs::File::open("to_upload.json").unwrap();
    let data: Vec<UploadEntry> = serde_json::de::from_reader(f).unwrap();

    // Upload them all
    let harvest_client = HarvestClient::from_env();
    for entry in data {
        harvest_client.upload_entry(&entry).await
    }
}
