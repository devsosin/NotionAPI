#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use notion::{NotionAPI, database::DatabaseClient};

    #[tokio::test]
    async fn test_get_db() {
        dotenv();

        let api = NotionAPI::from_env();

        let result = api.get_database("{database_id}").await.unwrap();

        println!("{:?}", result);
    }
}
