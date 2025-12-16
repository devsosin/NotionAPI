#[cfg(test)]
mod test {
    use std::env;

    use dotenv::dotenv;
    use notion::{NotionAPI, database::DatabaseClient};

    #[tokio::test]
    async fn test_get_db() {
        dotenv().ok();

        let api = NotionAPI::from_env();
        let token = env::var("NOTION_KEY").expect("Failed to load env variable: NOTION_KEY");
        let api = api.authed(&token);

        let result = api.get_database("{database_id}").await.unwrap();

        println!("{:?}", result);
    }
}
