#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use notion::{NotionAPI, data_source::DataSourceClient, database::DatabaseClient};
    use serde_json::json;

    #[tokio::test]
    async fn test_get_db() {
        dotenv();

        let api = NotionAPI::from_env();

        let result = api.get_database("{database_id}").await.unwrap();

        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_get_ds() {
        dotenv();

        let api = NotionAPI::from_env();

        let result = api.get_data_source("{data_source_id}").await.unwrap();

        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_query() {
        dotenv();

        let api = NotionAPI::from_env();

        let result = api
            .query_pages(
                "{data_source_id}",
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
                vec![("날짜", "asc")],
            )
            .await
            .unwrap();

        println!("{:?}", result);
    }
}
