use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub _id: String,
    pub email: String,
    pub password_hash: String,
}
