use mongodb::{Client, Database};

pub async fn connect() -> Database {
    dotenvy::dotenv().ok();
    
    let uri = std::env::var("MONGODB_URI")
        .expect("MONGODB_URI must be set");
    
    let db_name = std::env::var("DATABASE_NAME")
        .expect("DATABASE_NAME must be set");

    let client = Client::with_uri_str(&uri)
        .await
        .expect("Failed to connect to MongoDB");

    println!("✅ Connected to MongoDB");
    
    client.database(&db_name)
}