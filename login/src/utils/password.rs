use argon2::{
    password_hash::{SaltString, PasswordHasher, PasswordVerifier, rand_core::OsRng},
    Argon2,
};

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(hash.to_string())
}

pub fn verify_password(hash: &str, password: &str) -> bool {
    let pared_hash = argon2::PasswordHash::new(hash).unwrap();
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}
