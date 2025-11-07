use reqwest::Client;

pub mod client;
pub mod errors;
pub mod types;

// apis
pub mod datasource;

pub struct NotionAPI {
    client: Client,
    base_url: String,
}
