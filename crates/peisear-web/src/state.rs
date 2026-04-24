//! Application state shared with every handler.

use peisear_storage::Pool;

#[derive(Clone)]
pub struct AppState {
    pub db: Pool,
    pub jwt_secret: String,
    pub cookie_secure: bool,
}
