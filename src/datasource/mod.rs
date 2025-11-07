pub mod types;

use serde_json::{Value, json};

use crate::{
    NotionAPI,
    datasource::types::{GetDatabaseResponse, NotionListResult, QueryBody, Sort, SortDirection},
    types::{ClientResult, Method},
};

pub trait DatabaseClient: Send + Sync {
    fn get_database(&self, database_id: &str) -> impl Future<Output = ClientResult<()>>;
}

pub trait DataSourceClient: Send + Sync {
    // fn get_data_source() -> properties
    fn query(&self, data_source_id: &str) -> impl Future<Output = ClientResult<()>>;
}

impl DatabaseClient for NotionAPI {
    async fn get_database(&self, database_id: &str) -> ClientResult<()> {
        let endpoint = format!("databases/{}", database_id);

        let response = self
            .send::<Value, GetDatabaseResponse>(&endpoint, Method::Get, json!({}))
            .await?;

        println!("{:?}", response);

        Ok(())
    }
}

impl DataSourceClient for NotionAPI {
    async fn query(&self, data_source_id: &str) -> ClientResult<()> {
        let endpoint = format!("data_sources/{}/query", data_source_id);

        // TODO: Filter 구조화

        let body = QueryBody::new(
            json!({"and": [
                {
                    "property": "날짜",
                    "date": {
                        // "equals": "2025-11-07T00:00:00+09:00"
                        // "this_week": {}
                        "after": "2025-11-07T00:00:00+09:00"
                    }
                },
                {
                    "property": "날짜",
                    "date": {
                        "before": "2025-11-08T00:00:00+09:00"
                    }
                }
            ]}),
            vec![Sort::new("날짜", SortDirection::Ascending)],
        );

        let response = self
            .send::<QueryBody, NotionListResult>(&endpoint, Method::Post, body)
            .await?;

        println!("{:?}", response);

        Ok(())
    }
}
