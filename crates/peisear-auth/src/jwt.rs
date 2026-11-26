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
//! verification. Proven by plant, not assumed.
//!
//! `Claims` stays private to this module too -- nothing outside it ever
//! read anything but `.sub` (checked: `grep -rn "claims\."` over the
//! workspace before this change touched one call site,
//! `extractors.rs`'s `AuthUser`), so there is no reason for a second,
//! fully-public-fielded, trivially-forgeable struct to keep existing
//! outside `verify`'s own use for signing and decoding.
//!
//! `SES-001` (`RFC 024` §4): [`verify`] now surrenders a second
//! value, the session generation the token was issued under -- **a
//! value, not the struct**, so the shape above survives unchanged.
//! Nothing outside this module can see `Claims` itself; `verify`'s
//! own two return values are the only things it ever gives up.

use argon2::password_hash::rand_core::{OsRng, RngCore};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::AuthResult;

/// 7 days.
pub const SESSION_TTL_SECS: i64 = 60 * 60 * 24 * 7;

/// 32 random bytes from the OS CSPRNG (`RFC 017` §4) -- a signing key an
/// instance generates once, when no `JWT_SECRET` is configured. Same
/// `OsRng` import path as [`crate::password::hash`]'s salt generation,
/// reused rather than adding a second randomness source to this crate.
pub fn generate_signing_key() -> [u8; 32] {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    bytes
}

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
    /// `SES-001` (`RFC 024` §4.3): the session generation this token
    /// was issued under. `#[serde(default)]` so a token minted before
    /// this claim existed (any `0.70.0` token) deserialises as `0` --
    /// the column's own default -- rather than failing to parse.
    /// Without the default, upgrading to `0.71.0` would refuse every
    /// outstanding token at once, which is exactly the silent mass
    /// logout `RFC 024` §4.3 names as the cost of getting this wrong.
    #[serde(default)]
    ses: i64,
}

pub fn issue(
    user_id: &str,
    email: &str,
    session_generation: i64,
    secret: &str,
) -> AuthResult<String> {
    let now = chrono::Utc::now().timestamp();
    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        iat: now,
        exp: now + SESSION_TTL_SECS,
        ses: session_generation,
    };
    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?)
}

/// Verifies signature and expiry, same as before `SES-001`, and
/// additionally surrenders the token's own session generation as a
/// plain `i64` -- the caller (`verified_requester_id`, the one place
/// `RFC 024` §4.2 puts this check) compares it against the subject's
/// current generation. Returning a value rather than `Claims` itself
/// keeps this module's own `DEC-058` shape: nothing outside `jwt` has
/// ever been able to name `Claims`, and that stays true.
pub fn verify(token: &str, secret: &str) -> AuthResult<(RequesterId, i64)> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok((RequesterId(data.claims.sub), data.claims.ses))
}
