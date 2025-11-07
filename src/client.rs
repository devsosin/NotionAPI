use std::{env, fmt, time::Duration};

use reqwest::{ClientBuilder, header};
use serde::{Deserialize, Serialize};

use crate::{
    NotionAPI,
    types::{ClientResult, ErrorResponse, Method},
};

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

    pub async fn send<T: Serialize, U: for<'a> Deserialize<'a> + fmt::Debug>(
        &self,
        endpoint: &str,
        method: Method,
        body: T,
    ) -> ClientResult<U> {
        let url = format!("{}/{}", &self.base_url, endpoint);

        let builder = match method {
            Method::Get => self.client.get(url),
            Method::Post => self.client.post(url),
            Method::Patch => self.client.patch(url),
            Method::Delete => self.client.delete(url),
        };

        let res = builder.json(&body).send().await?;

        if res.status().is_success() == false {
            let err_response = res.json::<ErrorResponse>().await.unwrap();
            println!("{:?}", err_response);
            let err = err_response.into();
            return Err(err);
        }

        let res_body = res.json::<U>().await.unwrap();
        Ok(res_body)
    }
}
