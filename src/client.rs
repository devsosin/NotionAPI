use std::{env, fmt, time::Duration};

use reqwest::{ClientBuilder, RequestBuilder, header};
use serde::{Deserialize, Serialize};

use crate::{
    NotionAPI, NotionAuthedAPI,
    types::{ClientResult, ErrorResponse, Method, NotionResponse},
};

impl NotionAPI {
    pub fn from_env() -> Self {
        let version = env::var("NOTION_API_VERSION")
            .expect("Failed to load env variable: NOTION_API_VERSION");

        let mut headers = header::HeaderMap::new();
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

    pub fn authed<'a>(&'a self, token: &'a str) -> NotionAuthedAPI<'a> {
        NotionAuthedAPI { api: self, token }
    }

    fn build_request(&self, method: Method, endpoint: &str) -> RequestBuilder {
        let url = format!("{}/{}", &self.base_url, endpoint);

        let builder = match method {
            Method::Get => self.client.get(url),
            Method::Post => self.client.post(url),
            Method::Patch => self.client.patch(url),
            Method::Delete => self.client.delete(url),
        };

        builder
    }
}

impl<'a> NotionAuthedAPI<'a> {
    pub async fn send<T: Serialize, U: for<'d> Deserialize<'d> + fmt::Debug>(
        &self,
        endpoint: &str,
        method: Method,
        body: T,
    ) -> ClientResult<NotionResponse<U>> {
        let res = self
            .api
            .build_request(method, endpoint)
            .bearer_auth(self.token)
            .json(&body)
            .send()
            .await?;

        if res.status().is_success() == false {
            let err_response = res.json::<ErrorResponse>().await.unwrap();
            // TODO? log.error
            println!("{:?}", err_response);
            let err = err_response.into();
            return Err(err);
        }

        let res_body = res.json::<NotionResponse<U>>().await.unwrap();
        Ok(res_body)
    }
}
