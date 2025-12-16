#[cfg(test)]
mod test {
    use std::env;

    use dotenv::dotenv;
    use notion::{NotionAPI, data_source::DataSourceClient};
    use serde_json::json;

    #[tokio::test]
    async fn test_get_ds() {
        dotenv().ok();

        let api = NotionAPI::from_env();
        let token = env::var("NOTION_KEY").expect("Failed to load env variable: NOTION_KEY");
        let api = api.authed(&token);

        let result = api.get_data_source("{data_source_id}").await.unwrap();

        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_query() {
        dotenv().ok();

        let api = NotionAPI::from_env();
        let token = env::var("NOTION_KEY").expect("Failed to load env variable: NOTION_KEY");
        let api = api.authed(&token);

        let result = api
            .query_pages(
                "{data_source_id}",
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
