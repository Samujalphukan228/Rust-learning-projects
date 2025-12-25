use axum::{extract::State, Json};
use mongodb::bson::doc;
use uuid::Uuid;
use rand::thread_rng;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;

use crate::{state::AppState, models::user::User};

#[derive(serde::Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Json<&'static str> {

    let users = state.db.collection::<User>("users");

    if users.find_one(doc! { "email": &payload.email }, None)
        .await.unwrap().is_some() {
        return Json("User already exists");
    }

    let salt = SaltString::generate(&mut thread_rng());
    let hash = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let user = User {
        _id: Uuid::new_v4().to_string(),
        email: payload.email,
        password_hash: hash,
    };

    users.insert_one(user, None).await.unwrap();

    Json("User registered")
}
