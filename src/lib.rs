use std::{env, time::Duration};

use reqwest::{Client, ClientBuilder, header};
use serde::Serialize;
use serde_json::Value;

use crate::types::{ClientResult, ErrorResponse};

pub mod errors;
pub mod types;

pub struct NotionAPI {
    client: Client,
    base_url: String,
}

// TODO: move to client.rs
impl NotionAPI {
    pub fn from_env() -> Self {
        let token = env::var("NOTION_KEY").expect("Failed to load env variable: NOTION_KEY");
        let version = env::var("NOTION_API_VERSION")
            .expect("Failed to load env variable: NOTION_API_VERSION");

        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            format!("Bearer {}", &token).parse().unwrap(),
        );
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );
        headers.insert("Notion-Version", version.parse().unwrap());

        Self {
            client: ClientBuilder::new()
                .user_agent("Notion-Rust/1.0.0")
                .default_headers(headers)
                .danger_accept_invalid_certs(true)
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to build HTTP client"),
            base_url: "https://api.notion.com/v1".to_string(),
        }
    }

    // TODO: response type , U: for<'a> Deserialize<'a>
    pub async fn send<T: Serialize>(&self, endpoint: &str, body: T) -> ClientResult<()> {
        let url = format!("{}/{}", &self.base_url, endpoint);

        // TODO: get, post, patch, delete
        let res = self.client.get(url).json(&body).send().await?;

        match res.status().is_success() {
            true => {
                let res_body = res.json::<Value>().await.unwrap();
                println!("{:?}", res_body);
            }
            false => {
                let err_response = res.json::<ErrorResponse>().await.unwrap();
                println!("{:?}", err_response);
                let err = err_response.into();
                return Err(err);
            }
        }

        Ok(())
    }
}
