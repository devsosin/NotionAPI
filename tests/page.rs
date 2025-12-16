#[cfg(test)]
mod test {
    use std::env;

    use dotenv::dotenv;
    use notion::{NotionAPI, page::PageClient};
    use serde_json::json;

    #[tokio::test]
    async fn test_update_page() {
        dotenv().ok();

        let api = NotionAPI::from_env();
        let token = env::var("NOTION_KEY").expect("Failed to load env variable: NOTION_KEY");
        let api = api.authed(&token);

        let page_id = "{page_id}";
        let properties = json!({
            "날짜": {
                "date": {
                    "start": "2025-11-27",
                    // "end": "2025-11-11T22:00:00+09:00",
                }
            }
        });
        // vec (property name, property type, value)

        let result = api.update_page(page_id, properties).await.unwrap();

        println!("{:?}", result);
    }
}
