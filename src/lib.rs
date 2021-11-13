pub mod harvest_client;
pub mod model;
use chrono::NaiveDate;

/// The data needed to make requests to the harvest time tracker API

pub fn parse_date(input: &str) -> NaiveDate {
    NaiveDate::parse_from_str(input, "%Y-%m-%d").unwrap()
}

pub use harvest_client::HarvestClient;
