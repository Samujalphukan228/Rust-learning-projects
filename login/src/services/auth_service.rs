use mongodb::{Database, bson::doc};
use chrono::Utc;

use crate::{
    models::user::User,
    utils::password::{hash_password, verify_password},
    services::token_service::generate_token,
    errors::AppError,
};

pub async fn register(
    db: &Database,
    email: String,
    password: String,
    jwt_secret: &str,
) -> Result<String, AppError> {

    let users = db.collection::<User>("users");

    // Added None as second argument for options
    if users.find_one(doc! { "email": &email }, None).await?.is_some() {
        return Err(AppError::UserExists);
    }

    let hashed = hash_password(&password)
        .map_err(|_| AppError::HashingError)?;

    let user = User {
        id: None,
        email,
        password_hash: hashed,
        created_at: Utc::now(),
    };

    // Added None as second argument for options
    let result = users.insert_one(user, None).await?;
    let id = result.inserted_id.as_object_id().unwrap().to_hex();

    let token = generate_token(&id, jwt_secret)
        .map_err(|_| AppError::TokenGenerationError)?;

    Ok(token)
}

pub async fn login(
    db: &Database,
    email: String,
    password: String,
    jwt_secret: &str,
) -> Result<String, AppError> {

    let users = db.collection::<User>("users");

    // Added None as second argument for options
    let user = users
        .find_one(doc! { "email": &email }, None)
        .await?
        .ok_or(AppError::InvalidCredentials)?;

    if !verify_password(&user.password_hash, &password) {
        return Err(AppError::InvalidCredentials);
    }

    let id = user.id.unwrap().to_hex();

    let token = generate_token(&id, jwt_secret)
        .map_err(|_| AppError::TokenGenerationError)?;

    Ok(token)
}