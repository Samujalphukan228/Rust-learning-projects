use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

impl Claims {
    pub fn new(user_id: String) -> Self {
        let exp = (Utc::now() + Duration::hours(1)).timestamp() as usize;

        Self { sub: user_id, exp }
    }
}
