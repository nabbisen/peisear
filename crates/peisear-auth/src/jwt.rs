//! JWT issuing and verification.

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::AuthResult;

/// 7 days.
pub const SESSION_TTL_SECS: i64 = 60 * 60 * 24 * 7;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user id).
    pub sub: String,
    /// Email, included for convenience.
    pub email: String,
    /// Expiry (seconds since epoch).
    pub exp: i64,
    /// Issued-at (seconds since epoch).
    pub iat: i64,
}

pub fn issue(user_id: &str, email: &str, secret: &str) -> AuthResult<String> {
    let now = chrono::Utc::now().timestamp();
    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        iat: now,
        exp: now + SESSION_TTL_SECS,
    };
    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?)
}

pub fn verify(token: &str, secret: &str) -> AuthResult<Claims> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(data.claims)
}
