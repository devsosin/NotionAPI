#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use serde_json::json;

    use notion::NotionAPI;

    #[tokio::test]
    async fn test_send() {
        dotenv();

        let api = NotionAPI::from_env();

        let res = api
            .send("databases/{database_id}", json!({}))
            .await
            .map_err(|e| println!("에러 발생 : {:?}", e))
            .unwrap();
    }
}
