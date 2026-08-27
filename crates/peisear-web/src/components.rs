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
pub mod calendar;
pub mod confirmation;
pub mod error_page;
pub mod issues;
pub mod layout;
pub mod me;
pub mod notification_preferences;
pub mod notifications;
pub mod projects;
pub mod search;
pub mod settings;
pub mod sprint_plan;
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

/// `NFR-A11Y-007`'s 44px touch target (`DEC-049`), resolved:
/// `min-h-11`/`min-w-11` = `2.75rem` = 44px each, verified against the
/// pinned `daisyui@4.12.14` bundle `TT-001` located at
/// `.git-exclude/tmp/daisy.css`. One name, composed at each of
/// `TT-002`'s 136 call sites, rather than the pair repeated as many
/// times — the same "move the fact to where it can be checked" shape
/// as [`t`] itself, `RFC 006`, `QA-019`, `HLT-001`, and `JS-003`.
///
/// `pub`, not `pub(crate)`: `TT-002` §7.2 wants a test proving the
/// *constant* drives the rendered page, not a hardcoded copy of its
/// current value — which means an integration test (a separate crate
/// under `tests/`) needs to read this symbol directly.
pub const TOUCH_TARGET: &str = "min-h-11 min-w-11";

/// Append [`TOUCH_TARGET`] to `base`'s existing classes — `TT-002`'s
/// one call site for the `Grow` mechanism (`DEC-049` as amended by
/// `TT-001-review.md` §2.1: `Grow` is the default, and a `Grow`
/// cluster inside a positive-`gap` container is presumed to satisfy
/// clause (2) with no further verification). `TT-003`'s guard checks
/// for [`TOUCH_TARGET`] as one symbol rather than pattern-matching a
/// class pair at 136 sites.
pub(crate) fn grow(base: &str) -> String {
    format!("{base} {TOUCH_TARGET}")
}

/// Column of issues on the kanban board, grouped by status. Shared
/// between [`issues::ProjectDetailPage`] and its handler.
#[derive(Debug, Clone)]
pub struct Column {
    pub status: peisear_core::IssueStatus,
    pub issues: Vec<peisear_core::Issue>,
}

/// `CAL-002`'s three calendar view modes. Shared between
/// `handlers::calendar` (parses `?view=`, computes the window) and
/// `components::calendar` (renders it) — same reason [`Column`]
/// lives here rather than in either side alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalendarView {
    Day,
    Week,
    Month,
}

impl CalendarView {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Week => "week",
            Self::Month => "month",
        }
    }

    pub fn to_i18n_label(self) -> peisear_i18n::CalendarViewLabel {
        match self {
            Self::Day => peisear_i18n::CalendarViewLabel::Day,
            Self::Week => peisear_i18n::CalendarViewLabel::Week,
            Self::Month => peisear_i18n::CalendarViewLabel::Month,
        }
    }
}

/// One day's worth of calendar blocks, for the week/month grids. An
/// issue spanning multiple days appears once per [`CalendarDay`] it
/// overlaps, clipped to the visible window.
#[derive(Debug, Clone)]
pub struct CalendarDay {
    pub date: chrono::NaiveDate,
    pub blocks: Vec<peisear_core::Issue>,
}

/// The closed set of notification kinds `notification_preferences`'s
/// and `notifications`'s pages render — always resolves in practice
/// (`kind::all_user_facing()` is the only source of kind ids reaching
/// either page today). Defensive `Option` return matches
/// `IssueStatus::parse`'s shape for an id that could, in principle,
/// be unrecognised; callers fall back to the raw id.
///
/// `I18N-005d-review.md` §2.1: this used to be defined identically in
/// both `notifications.rs` and `notification_preferences.rs` — the
/// fourth instance this release of the same "second copy invisible
/// until something consolidates it" pattern (two back-link casings,
/// three nav-destination casings, a hand-rolled issue-status word in
/// `sprints.rs`). One mapping, one place, now.
pub(crate) fn kind_label_for(kind_id: &str) -> Option<peisear_i18n::NotificationKindLabel> {
    use peisear_core::notifications::kind;
    use peisear_i18n::NotificationKindLabel;
    match kind_id {
        kind::BURNOUT_OVERLOAD => Some(NotificationKindLabel::BurnoutOverload),
        kind::BURNOUT_STALLED => Some(NotificationKindLabel::BurnoutStalled),
        kind::PROJECT_TREND_DECLINE => Some(NotificationKindLabel::ProjectTrendDecline),
        _ => None,
    }
}

/// See [`kind_label_for`] — same shape, same reason, same
/// deduplication. Notification rows can carry any channel string a
/// future release adds; unrecognised channels fall back to their raw
/// id.
pub(crate) fn channel_label_for(
    channel_id: &str,
) -> Option<peisear_i18n::NotificationChannelLabel> {
    use peisear_core::notifications::channel;
    use peisear_i18n::NotificationChannelLabel;
    match channel_id {
        channel::IN_APP => Some(NotificationChannelLabel::InApp),
        channel::EMAIL => Some(NotificationChannelLabel::Email),
        channel::WEBHOOK => Some(NotificationChannelLabel::Webhook),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `I18N-005d-review.md` §2.2: `kind_label_for`/`channel_label_for`
    /// end in `_ => None`, so there is no type for the compiler to be
    /// exhaustive over at this seam — adding a notification kind or
    /// channel in `peisear-core` without updating the map here would
    /// ship a raw id to a user's inbox with no compile error, no
    /// guard rejection. This test converts that hole into a failing
    /// test the moment a kind/channel is declared without a label.
    #[test]
    fn every_declared_notification_kind_has_a_label() {
        for kind_id in peisear_core::notifications::kind::all_user_facing() {
            assert!(
                kind_label_for(kind_id).is_some(),
                "notifications::kind::{kind_id:?} has no NotificationKindLabel mapping in \
                 kind_label_for — add one so it doesn't ship its raw id to a user's inbox"
            );
        }
    }

    #[test]
    fn every_declared_notification_channel_has_a_label() {
        for channel_id in peisear_core::notifications::channel::all() {
            assert!(
                channel_label_for(channel_id).is_some(),
                "notifications::channel::{channel_id:?} has no NotificationChannelLabel \
                 mapping in channel_label_for — add one so it doesn't ship its raw id"
            );
        }
    }
}
