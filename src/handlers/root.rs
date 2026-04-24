//! Miscellaneous root routes.

use axum::response::{IntoResponse, Redirect};

use crate::auth::MaybeAuthUser;

pub async fn index(MaybeAuthUser(user): MaybeAuthUser) -> impl IntoResponse {
    match user {
        Some(_) => Redirect::to("/projects"),
        None => Redirect::to("/login"),
    }
}

pub async fn health() -> &'static str {
    "ok"
}
