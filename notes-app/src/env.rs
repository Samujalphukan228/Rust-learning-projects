use std::env;

pub struct AppConfig {
    pub database_url: String,
    pub database_name: String,
    // Remove this: pub notes_collection: String,
}

impl AppConfig {
    pub fn load() -> Self {
        dotenvy::dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL missing"),
            database_name: env::var("DATABASE_NAME")
                .expect("DATABASE_NAME missing"),
            // Remove this: notes_collection: env::var("NOTES_COLLECTION")...
        }
    }
}