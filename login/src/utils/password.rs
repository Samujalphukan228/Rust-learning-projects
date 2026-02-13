use argon2::{
    password_hash::{SaltString, PasswordHasher, PasswordVerifier, rand_core::OsRng},
    Argon2, PasswordHash,
};

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(hash.to_string())
}

pub fn verify_password(hash: &str, password: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash);
    match parsed_hash {
        Ok(h) => Argon2::default().verify_password(password.as_bytes(), &h).is_ok(),
        Err(_) => false,
    }
}