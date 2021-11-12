pub mod model;
use reqwest::Client;
use std::env;

/// The data needed to make requests to the harvest time tracker API
pub struct RequestData {
    pub client: Client,
    pub token: String,
    pub account: String,
}

impl RequestData {
    /// Create the data from environment variables
    pub fn from_env() -> RequestData {
        RequestData {
            client: reqwest::Client::new(),
            token: env::var("token").expect("`token` environment variable"),
            account: env::var("account").expect("`account` environment variable"),
        }
    }
}