//! Leptos server-side-rendered components.
//!
//! Each page-level template under `templates/` in the old askama layout
//! corresponds to a `#[component]` function here. Handlers build props
//! and call one of the `render_*` helpers in this module's submodules,
//! which return an `axum::response::Html<String>` ready to go back to
//! the browser.
//!
//! Why SSR only: the `ssr` feature of Leptos builds only the
//! server-side renderer and does not need the `wasm32-unknown-unknown`
//! target. The app renders HTML on the server on every request, the
//! same way askama did — just with Rust components instead of a DSL.
//! Hydration (`hydrate` feature) would give client-side reactivity but
//! requires a second compile to wasm, which we leave as future work
//! (see the README).

pub mod auth;
pub mod breadcrumb;
pub mod error_page;
pub mod issues;
pub mod layout;
pub mod me;
pub mod notification_preferences;
pub mod notifications;
pub mod projects;
pub mod search;
pub mod settings;
pub mod sprints;
pub mod teams;

use axum::response::Html;
use leptos::prelude::*;

/// Render a Leptos view to a complete HTML document.
///
/// Prepends the `<!DOCTYPE html>` declaration that `<html>` technically
/// requires but that Leptos's `to_html()` does not emit on its own.
pub(crate) fn render_to_html<F, V>(view: F) -> Html<String>
where
    F: FnOnce() -> V,
    V: IntoView,
{
    // `.to_html()` comes from `tachys::view::RenderHtml`, re-exported
    // via `leptos::prelude::*`. Calling `.into_view().to_html()` gives
    // us the full server-rendered HTML for the top-level view.
    let body = view().into_view().to_html();
    Html(format!("<!DOCTYPE html>{body}"))
}

/// Shorthand for `Locale::English.render(key)` — the only locale this
/// crate ever renders (`NFR-LANG-005`). `I18N-005a-review.md` §6:
/// the full call was too long to sit comfortably inline in markup, so
/// components pre-bound every rendered string to a `let` even where
/// it was only used once. Short enough to inline, so pre-binding can
/// go back to being a choice — made when a string is used more than
/// once or needs conditional logic to select, not a requirement for
/// every rendered string.
pub(crate) fn t(key: peisear_i18n::MessageKey) -> String {
    peisear_i18n::Locale::English.render(key)
}

/// Column of issues on the kanban board, grouped by status. Shared
/// between [`issues::ProjectDetailPage`] and its handler.
#[derive(Debug, Clone)]
pub struct Column {
    pub status: peisear_core::IssueStatus,
    pub issues: Vec<peisear_core::Issue>,
}
