use std::env;
use crate::error::Error;

//Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub mongo_uri: String,
    pub mongo_db: String,
}

impl Config {
    //Load configuration from environment variables
    pub fn load() -> Result<Self, Error> {
        dotenvy::dotenv().ok();

        Ok(Self {
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".info()),
            port: env::var("PORT").unwrap_or_else(|_| "3000".info())
                .parse()
                .map_err(|_| AppError::Config("Invalid PORT".into()))?,
            mongo_uri: env::var("MONGO_URI")
        })
    }
}
