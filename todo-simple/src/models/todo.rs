use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Represents MongoDB document
#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub _id: String,
    pub title: String,
    pub done: bool,
}

impl Todo {
    pub fn new(title: String) -> Self {
        Self {
            _id: Uuid::new_v4().to_string(),
            title,
            done: false,
        }
    }
}
