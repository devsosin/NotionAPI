use serde_json::Value;

use crate::{
    NotionAPI,
    page::{
        dto::request::{PageCreateBody, PageUpdateBody},
        types::{Page, PageParent},
    },
    types::{ClientResult, Method, NotionResponse},
};

pub mod dto;
pub mod types;

/// * parent - (data_source, database) and id
pub trait PageClient: Send + Sync {
    fn create_page(
        &self,
        parent: (&str, &str),
        title: &str,
    ) -> impl Future<Output = ClientResult<NotionResponse<Page>>>;
    fn update_page(
        &self,
        page_id: &str,
        properties: Value,
    ) -> impl Future<Output = ClientResult<NotionResponse<Page>>>;
}

impl PageClient for NotionAPI {
    async fn create_page(
        &self,
        parent: (&str, &str),
        title: &str,
    ) -> ClientResult<NotionResponse<Page>> {
        let parent = PageParent::new(parent.0, parent.1);
        let body = PageCreateBody::new(parent, title);

        let response = self
            .send::<PageCreateBody, Page>("pages", Method::Post, body)
            .await?;

        Ok(response)
    }

    async fn update_page(
        &self,
        page_id: &str,
        properties: Value,
    ) -> ClientResult<NotionResponse<Page>> {
        let endpoint = format!("pages/{}", page_id);
        let body = PageUpdateBody::new(properties);

        let response = self
            .send::<PageUpdateBody, Page>(&endpoint, Method::Patch, body)
            .await?;

        Ok(response)
    }
}
