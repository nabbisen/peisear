//! Argon2id password hashing and verification.

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

use crate::{AuthError, AuthResult};

pub fn hash(password: &str) -> AuthResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AuthError::PasswordHash(e.to_string()))?
        .to_string();
    Ok(hash)
}

pub fn verify(password: &str, hash: &str) -> AuthResult<bool> {
    let parsed =
        PasswordHash::new(hash).map_err(|e| AuthError::PasswordHash(e.to_string()))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}
