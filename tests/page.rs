#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use notion::{NotionAPI, page::PageClient};
    use serde_json::json;

    #[tokio::test]
    async fn test_update_page() {
        dotenv();

        let api = NotionAPI::from_env();

        let page_id = "{page_id}";
        let properties = json!({
            "날짜": {
                "date": {
                    "start": "2025-11-11T20:00:00+09:00",
                    "end": "2025-11-11T22:00:00+09:00",
                }
            }
        });

        let result = api.update_page(page_id, properties).await.unwrap();

        println!("{:?}", result);
    }
}
