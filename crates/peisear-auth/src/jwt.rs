//! JWT issuing and verification.
//!
//! `PRIV-002` (`DEC-058`): [`RequesterId`] is declared in this module, not
//! merely in this crate, and its field is private -- Rust scopes a
//! non-`pub` field to the declaring module and its descendants, not to the
//! crate, so nothing outside `jwt` (including the rest of `peisear-auth`,
//! `peisear-storage`, and every handler in `peisear-web`) can construct one
//! by struct-literal syntax, even though the type itself is `pub` and
//! freely passed around once obtained. [`verify`] is the only function
//! that can see the private field, so the only way to obtain a
//! `RequesterId` is a token that actually passes signature and expiry
//! verification. Proven by plant, not assumed:
//! `.git-exclude/review-request/PRIV-002-identity-type-investigation/evidence/sealing-plant.md`.
//!
//! `Claims` stays private to this module too -- nothing outside it ever
//! read anything but `.sub` (checked: `grep -rn "claims\."` over the
//! workspace before this change touched one call site,
//! `extractors.rs`'s `AuthUser`), so there is no reason for a second,
//! fully-public-fielded, trivially-forgeable struct to keep existing
//! outside `verify`'s own use for signing and decoding.

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::AuthResult;

/// 7 days.
pub const SESSION_TTL_SECS: i64 = 60 * 60 * 24 * 7;

/// Proof that a token was issued by this process and has not expired --
/// obtainable only from [`verify`]. See the module doc comment for why
/// this is a compiler-checked property, not a convention.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RequesterId(String);

impl RequesterId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for RequesterId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    /// Subject (user id).
    sub: String,
    /// Email, included for convenience.
    email: String,
    /// Expiry (seconds since epoch).
    exp: i64,
    /// Issued-at (seconds since epoch).
    iat: i64,
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

pub fn verify(token: &str, secret: &str) -> AuthResult<RequesterId> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(RequesterId(data.claims.sub))
}
