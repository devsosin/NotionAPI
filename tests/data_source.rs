#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use notion::{NotionAPI, data_source::DataSourceClient};
    use serde_json::json;

    #[tokio::test]
    async fn test_get_ds() {
        dotenv();

        let api = NotionAPI::from_env();

        let result = api
            .get_data_source("28714e4a-5157-80cb-8961-000b4d54c831")
            .await
            .unwrap();

        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_query() {
        dotenv();

        let api = NotionAPI::from_env();

        let result = api
            .query_pages(
                "28714e4a-5157-80cb-8961-000b4d54c831",
                vec!["title"],
                json!({"and": [
                    {
                        "property": "날짜",
                        "date": {
                            // "equals": "2025-11-07T00:00:00+09:00"
                            // "this_week": {}
                            "after": "2025-12-15T00:00:00+09:00"
                        }
                    },
                    {
                        "property": "날짜",
                        "date": {
                            "before": "2025-12-16T00:00:00+09:00"
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
