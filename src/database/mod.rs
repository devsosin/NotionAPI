pub mod dto;
pub mod types;

use serde_json::{Value, json};

use crate::{
    NotionAuthedAPI,
    database::dto::response::GetDatabaseResponse,
    types::{ClientResult, Method, NotionResponse},
};

pub trait DatabaseClient: Send + Sync {
    // create
    // update
    fn get_database(
        &self,
        database_id: &str,
    ) -> impl Future<Output = ClientResult<NotionResponse<GetDatabaseResponse>>>;
}

impl<'a> DatabaseClient for NotionAuthedAPI<'a> {
    async fn get_database(
        &self,
        database_id: &str,
    ) -> ClientResult<NotionResponse<GetDatabaseResponse>> {
        let endpoint = format!("databases/{}", database_id);

        let response = self
            .send::<Value, GetDatabaseResponse>(&endpoint, Method::Get, json!({}))
            .await?;

        Ok(response)
    }
}
