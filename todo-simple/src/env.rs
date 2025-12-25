use std::env;

pub struct Env {
    pub mongodb_uri: String,
    pub db_name: String,
}

impl Env {
    pub fn load() -> Self {
        dotenvy::dotenv().ok();

        Self {
            mongodb_uri: env::var("MONGODB_URI").expect("MONGODB_URI not set"),
            db_name: env::var("DB_NAME").expect("DB_NAME not set"),
        }
    }
}
