use mongodb::{Client, Database};
use crate::env::AppConfig;

pub async fn connect_db(config: &AppConfig) -> Database {
    let client = Client::with_uri_str(&config.database_url)
        .await
        .expect("MongoDB connection failed");

    client.database(&config.database_name)
}
