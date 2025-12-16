use reqwest::Client;

pub mod client;
pub mod errors;
pub mod types;

// apis
pub mod data_source;
pub mod database;
pub mod page;

pub struct NotionAPI {
    client: Client,
    base_url: String,
}

pub struct NotionAuthedAPI<'a> {
    api: &'a NotionAPI,
    token: &'a str,
}
