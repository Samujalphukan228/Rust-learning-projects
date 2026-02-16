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
            
        })
    }
}
