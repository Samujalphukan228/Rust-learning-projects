use axum::{extract::State, Json};
use mongodb::bson::doc;
use argon2::{Argon2, PasswordVerifier};
use argon2::password_hash::PasswordHash;
use jsonwebtoken::{encode, Header, EncodingKey};

use crate::{
    state::AppState,
    models::{user::User, claims::Claims},
};

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct LoginResponse {
    pub access_token: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Json<Option<LoginResponse>> {

    let users = state.db.collection::<User>("users");

    let user = users
        .find_one(doc! { "email": &payload.email }, None)
        .await
        .unwrap();

    // 👇 HANDLE Option MANUALLY
    let Some(user) = user else {
        return Json(None);
    };

    let parsed = PasswordHash::new(&user.password_hash).unwrap();

    if Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed)
        .is_err() {
        return Json(None);
    }

    let claims = Claims::new(user._id);

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    ).unwrap();

    Json(Some(LoginResponse { access_token: token }))
}
