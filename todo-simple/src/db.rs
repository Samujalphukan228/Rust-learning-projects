use mongodb::{Client, Database};

use crate::env::Env;

pub async fn connect_db(env: &Env) -> Database {
    let client = Client::with_uri_str(&env.mongodb_uri)
        .await
        .expect("Failed to initialize MongoDB client");

    client.database(&env.db_name)
}