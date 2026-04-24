//! Runtime configuration pulled from environment variables.

use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub bind_addr: String,
    pub cookie_secure: bool,
}

impl Config {
    pub fn from_env() -> Self {
        // Best-effort load of `.env`; absence is not fatal.
        let _ = dotenvy::dotenv();

        let database_url =
            env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://data/app.db".to_string());

        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| {
            tracing::warn!(
                "JWT_SECRET is not set; falling back to a development-only default. \
                 DO NOT use this configuration in production."
            );
            "dev-only-insecure-change-me".to_string()
        });

        let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string());

        let cookie_secure = env::var("COOKIE_SECURE")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);

        Self {
            database_url,
            jwt_secret,
            bind_addr,
            cookie_secure,
        }
    }
}
