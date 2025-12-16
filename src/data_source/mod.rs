use serde_json::{Value, json};

use crate::{
    NotionAuthedAPI,
    data_source::dto::{
        request::{PropertyFilters, QueryBody},
        response::{GetDataSourceResponse, QueryPageListResponse},
    },
    types::{ClientResult, Method, NotionResponse},
};

pub mod dto;
pub mod types;

pub trait DataSourceClient: Send + Sync {
    // create
    // update
    fn get_data_source(
        &self,
        data_source_id: &str,
    ) -> impl Future<Output = ClientResult<NotionResponse<GetDataSourceResponse>>>;
    fn query_pages(
        &self,
        data_source_id: &str,
        properties: Vec<&str>,
        filters: Value,
        sorts: Vec<(&str, &str)>,
    ) -> impl Future<Output = ClientResult<NotionResponse<QueryPageListResponse>>>;
    // get_templates
}

impl<'a> DataSourceClient for NotionAuthedAPI<'a> {
    async fn get_data_source(
        &self,
        data_source_id: &str,
    ) -> ClientResult<NotionResponse<GetDataSourceResponse>> {
        let endpoint = format!("data_sources/{}", data_source_id);

        let response = self
            .send::<Value, GetDataSourceResponse>(&endpoint, Method::Get, json!({}))
            .await?;

        Ok(response)
    }

    /// * `sorts` - Sorting property name and direction (asc, desc)
    async fn query_pages(
        &self,
        data_source_id: &str,
        properties: Vec<&str>,
        filters: Value,
        sorts: Vec<(&str, &str)>,
    ) -> ClientResult<NotionResponse<QueryPageListResponse>> {
        let endpoint = format!(
            "data_sources/{}/query?{}",
            data_source_id,
            PropertyFilters::as_query(properties)
        );

        let body = QueryBody::new(filters, sorts.iter().map(|s| s.into()).collect());

        let response = self
            .send::<QueryBody, QueryPageListResponse>(&endpoint, Method::Post, body)
            .await?;

        Ok(response)
    }
}
