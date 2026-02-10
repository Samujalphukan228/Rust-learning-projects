use std::env;

pub struct AppConfig {
    pub database_url: String,
    pub database_name: String,
    pub todo_collection: String,
}

impl AppConfig {
    pub fn load() -> Self {
        dotenvy::dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL missing"),
            database_name: env::var("DATABASE_NAME").expect("DATABASE_NAME missing"),
            todo_collection: env::var("TODO_COLLECTION").expect("TODO_COLLECTION missing"),
        }
    }
}