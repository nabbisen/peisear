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
//!
//! # Rendering user-supplied text: two shapes, two remedies
//!
//! `LAYOUT-004`, and the rule `LAYOUT-001`/`LAYOUT-003` are each one
//! instance of. **Any container holding text a user typed can be forced
//! wider than the viewport by a single unbreakable run** — a URL, a
//! token, a checksum. `line-clamp` and `truncate` do not help: they
//! clip lines, they do not introduce a break opportunity. The page then
//! scrolls sideways on a phone.
//!
//! **There are two shapes, and their remedies do not interchange.
//! Applying the wrong one is a silent no-op in both directions**, so
//! classify the site before fixing it — measure whether the element's
//! own *box* is too wide, or whether its *text* overflows a
//! correctly-sized box.
//!
//! - **Shape A — the box.** A flex or grid item defaults to
//!   `min-width: auto`, meaning its content's minimum, which an
//!   unbreakable run sets to the whole run. The item then leaves its
//!   container. Needs **`min-w-0` on the item** *and* `break-words` on
//!   the text: `min-w-0` alone shrinks the box and cuts the text
//!   mid-word, and `break-words` alone does nothing at all, because
//!   `overflow-wrap: break-word` is defined not to affect min-content
//!   intrinsic size. Sites: the account menu (`LAYOUT-001`), the board
//!   column (`LAYOUT-003`), issue detail's `<h1>`.
//!   **The item is not always the element holding the text** — on team
//!   detail it is a wrapper `<div>` — so find it, don't assume it.
//! - **Shape B — the text.** An ordinary block whose box is already
//!   the right width; only the text overflows it. Needs
//!   **`break-words` alone**; `min-w-0` is the no-op here.
//!   `overflow-wrap` inherits, so one class on the container covering
//!   the user text is enough. Sites: the delete interstitials, search
//!   results, the teams list, the sprints list, the `/today` and
//!   `/settings` subtitles.
//! - **Shape C — the container sizes itself to the text.** A
//!   shrink-to-fit box — `inline-flex` at the one site we have;
//!   `inline-block`, a float or a table cell would behave the same —
//!   takes its width from its content's *min-content*. **Both of the
//!   remedies above are inert here**: `min-w-0` does not change what an
//!   item contributes to min-content, and `overflow-wrap: break-word`
//!   is defined not to affect min-content at all. Needs
//!   **`wrap-anywhere`** (`overflow-wrap: anywhere`, defined once in
//!   `style/tailwindcss/input.css`) on the text. `anywhere` breaks in
//!   the same places `break-word` does and differs in exactly one
//!   respect: its soft-wrap opportunities count toward min-content.
//!   Site: the workload hint's chips on the issue forms.
//!
//! **`wrap-anywhere` is not a universal replacement**, and this was
//! measured so nobody re-derives it: with every existing
//! `min-w-0`/`break-words` stripped, it alone clears nine of the
//! thirteen known sites and fails two — the board's `line-clamp-2`
//! title (a `-webkit-box` does not shrink its min-content; the
//! column's `min-w-0` is what bounds it) and the list view's
//! `<select>` (a control, not text). Three shapes, three rows.
//!
//! **This is not guarded from source.** Which text is user-supplied is
//! not a property a scan can read off a class string
//! (`LAYOUT-003-review.md` §4). What catches it is the overflow gate's
//! fixture, which since `BROWSER-002` carries a 64-character unbroken
//! run in the issue title, the project name and the team name — add
//! new user-text surfaces to `browser-checks/overflow-gate.mjs`'s page
//! list rather than trusting review to spot the class.

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
