use std::env;

pub struct Env {
    pub mongodb_uri: String,
    pub db_name: String,
    pub jwt_secret: String,
}

impl Env {
    pub fn load() -> Self {
        dotenvy::dotenv().ok();

        Self {
            mongodb_uri: env::var("MONGODB_URI").unwrap(),
            db_name: env::var("DB_NAME").unwrap(),
            jwt_secret: env::var("JWT_SECRET").unwrap(),
        }
    }
}
