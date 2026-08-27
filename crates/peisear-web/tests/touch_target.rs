//! `TT-002` (RFC 012 step 3) §7 — proving what a source scan cannot.
//! `TT-003`'s future guard can check that `components::grow`'s call
//! sites exist in source; it cannot check that the composed classes
//! actually reach a rendered page, or that the wrapped checkboxes'
//! `aria-label` survives the `<label>` wrap. These tests render real
//! pages and assert the markup, not the source.

mod common;

use axum::http::StatusCode;
use common::auth::{TestUser, register_and_login};
use common::server::TestApp;
use peisear_web::components::TOUCH_TARGET;

/// Mechanism 1 (`Grow`), a button: the project list's "New Project"
/// link (`projects.rs:27`, `btn btn-primary btn-sm`) always renders,
/// empty-state or not (it's in the page header, not the empty-state
/// card), composed with [`TOUCH_TARGET`] by `components::grow`.
#[tokio::test]
async fn grown_button_reaches_the_rendered_page() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    register_and_login(&app, &user).await;

    let resp = app.server.get("/projects").await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    assert!(
        body.contains(&format!("btn btn-primary btn-sm {TOUCH_TARGET}")),
        "the project list's \"New Project\" button must carry {TOUCH_TARGET:?} \
         alongside its btn-sm class; body: {body}"
    );
}

/// Mechanism 1 (`Grow`), an input: the login form's email field
/// (`auth.rs:42`, `input input-bordered input-sm w-full`). No auth
/// needed — `/login` is the one page guaranteed to render for an
/// anonymous request.
///
/// Anchored on `name="email"` and scoped to that one `<input>` tag
/// rather than a bare `body.contains(class-string)` — the login
/// page's password field carries the *identical* class string, so an
/// unscoped check would still pass with the email field's own `grow`
/// call removed (caught planting this: `STATUS-001` test 6's own
/// lesson, cited by `TT-002` §7 — one compound assertion hid a
/// defect a scoped one would have caught).
#[tokio::test]
async fn grown_input_reaches_the_rendered_page() {
    let app = TestApp::spawn().await;

    let resp = app.server.get("/login").await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    let email_tag = tag_containing(&body, r#"name="email""#);
    assert!(
        email_tag.contains(&format!(
            "input input-bordered input-sm w-full {TOUCH_TARGET}"
        )),
        "the login form's email input specifically must carry {TOUCH_TARGET:?} \
         alongside its input-sm class; email <input> tag: {email_tag}"
    );
}

/// The `<input ...>` tag containing `needle`, from its `<input` start
/// up to (not including) the tag's own closing `>`. Scopes an
/// assertion to one element instead of the whole rendered body, which
/// matters whenever two elements can carry the same class string.
fn tag_containing<'a>(body: &'a str, needle: &str) -> &'a str {
    let needle_at = body
        .find(needle)
        .unwrap_or_else(|| panic!("{needle:?} not found in body: {body}"));
    let tag_start = body[..needle_at]
        .rfind("<input")
        .unwrap_or_else(|| panic!("no <input tag precedes {needle:?} in body: {body}"));
    let tag_end = body[tag_start..]
        .find('>')
        .unwrap_or_else(|| panic!("<input tag at {tag_start} has no closing '>': {body}"));
    &body[tag_start..tag_start + tag_end]
}

/// Mechanism 2 (`Expand` via a `<label>` wrap) — the sanctioned
/// technique `DEC-049` as amended permits, and the one this handoff
/// uses for the three notification-preference checkboxes. The box
/// itself stays at its native 24px (`class="checkbox"`, untouched);
/// the wrapping `<label>` reaches 44px and participates in normal
/// layout, which is what keeps it inside a positive-gap container's
/// overlap protection (`TT-001-review.md` §2.1).
#[tokio::test]
async fn wrapped_checkbox_reaches_the_rendered_page() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    register_and_login(&app, &user).await;

    let resp = app.server.get("/settings/notifications").await;
    resp.assert_status(StatusCode::OK);
    let body = resp.text();

    let wrap_marker =
        format!("<label class=\"inline-flex items-center justify-center {TOUCH_TARGET}\">");
    let wrap_count = body.matches(&wrap_marker).count();
    // TT-001 counted 3 checkbox *source sites* (in-app/email/webhook,
    // one Rust line each); each renders once per notification kind at
    // runtime. `kind::all_user_facing()` is 3 kinds, so the rendered
    // page carries 3 x 3 = 9 wrapped checkboxes, not 3 -- verified by
    // rendering rather than assumed from the source count. Checking
    // for exactly 9, not merely >=1, is the point: a plain `contains`
    // would still pass with most of the 9 left unwrapped.
    assert_eq!(
        wrap_count, 9,
        "expected all 9 rendered notification-preference checkboxes (3 kinds \
         x 3 channels) wrapped in a 44px label, found {wrap_count}; body: {body}"
    );
    assert!(
        body.contains(r#"class="checkbox""#),
        "the checkbox's own box must stay at its native 24px -- only the \
         wrapping label grows, per DEC-049 as amended; body: {body}"
    );
}

/// `TT-002` §4/§6 — the `<label>` wrap must not change the checkboxes'
/// accessible name. All three carry a distinct `aria-label`
/// (kind/channel-specific); if wrapping had accidentally introduced
/// visible label text, or dropped the `aria-label`, this is where it
/// would show. `aria-label` on the input always outranks a wrapping
/// `<label>`'s own content in accessible-name computation, but that
/// reasoning is only worth anything if the attribute is still there.
#[tokio::test]
async fn wrapped_checkboxes_keep_their_aria_label() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    register_and_login(&app, &user).await;

    let resp = app.server.get("/settings/notifications").await;
    let body = resp.text();

    // Every wrapping <label>'s opening tag must be followed immediately
    // by `<input` -- no text, no icon, nothing that could compete with
    // the input's own aria-label in accessible-name computation.
    let wrap_open =
        format!("<label class=\"inline-flex items-center justify-center {TOUCH_TARGET}\">");
    let mut checked = 0;
    for (i, _) in body.match_indices(&wrap_open) {
        let after = &body[i + wrap_open.len()..];
        assert!(
            after.starts_with("<input"),
            "expected the wrapping <label> at byte {i} to contain nothing \
             but the bare <input>, found: {:?}; body: {body}",
            &after[..after.len().min(60)]
        );
        checked += 1;
    }
    assert_eq!(
        checked, 9,
        "expected to check all 9 rendered checkbox wraps; body: {body}"
    );

    // Every checkbox still carries its own distinct aria-label (three
    // channel labels x three kinds = nine, one per rendered checkbox).
    for prefix in ["In-app for", "Email for", "Webhook for"] {
        assert!(
            body.matches(&format!("aria-label=\"{prefix}")).count() == 3,
            "expected exactly 3 occurrences of aria-label=\"{prefix}...\" \
             (one per notification kind); body: {body}"
        );
    }
}

/// `TT-002-review.md` §2 (round 2, correction 1) — the five tests above
/// verify that [`TOUCH_TARGET`] and the rendered page agree with each
/// other. None of them verify that the value they agree on is the one
/// `NFR-A11Y-007` actually demands. A `TOUCH_TARGET` of
/// `"min-h-8 min-w-8"` (32px) would pass every other test in this file
/// — confirmed by planting it during review. This test resolves the
/// constant to real pixels via Tailwind's own default spacing scale
/// and checks it against the 44px floor directly, the same shape
/// `contrast_scan` uses against WCAG's 4.5:1 rather than against a
/// copy of itself (baseline `§9.5`: the arithmetic and the scale are
/// facts outside the requirement, not a restatement of it).
#[test]
fn touch_target_resolves_to_at_least_44px_on_both_axes() {
    let height_px = resolve_min_utility_px(TOUCH_TARGET, "min-h-");
    let width_px = resolve_min_utility_px(TOUCH_TARGET, "min-w-");

    assert!(
        height_px >= 44,
        "TOUCH_TARGET's height resolves to {height_px}px, below NFR-A11Y-007's \
         44px floor -- TOUCH_TARGET is {TOUCH_TARGET:?}"
    );
    assert!(
        width_px >= 44,
        "TOUCH_TARGET's width resolves to {width_px}px, below NFR-A11Y-007's \
         44px floor -- TOUCH_TARGET is {TOUCH_TARGET:?}"
    );
}

/// Resolve a `{prefix}{n}` Tailwind utility (e.g. `min-h-11`) to CSS
/// pixels via the default spacing scale: `n * 0.25rem`, `1rem = 16px`,
/// so `n * 4px`. `TT-001` §7 confirmed no project-level
/// `tailwind.config` override exists in this tree, so the default
/// scale is the one that actually applies.
fn resolve_min_utility_px(classes: &str, prefix: &str) -> u32 {
    let n = classes
        .split_whitespace()
        .find_map(|class| class.strip_prefix(prefix))
        .unwrap_or_else(|| panic!("no {prefix:?} utility found in {classes:?}"));
    let n: u32 = n.parse().unwrap_or_else(|e| {
        panic!("{prefix:?} utility {n:?} in {classes:?} isn't a plain integer: {e}")
    });
    n * 4
}

/// `TT-002` §7.2 — the constant is the source of the rendered value,
/// not a hardcoded copy of it. This reads [`TOUCH_TARGET`] itself
/// rather than the literal `"min-h-11 min-w-11"`: if the constant's
/// value ever changes, this test tracks it automatically. A hardcoded
/// literal would not — it would keep passing even after the constant
/// and the rendered page had silently diverged, which is the exact
/// failure mode `TT-002` §2 exists to close (`components::grow` is
/// the one call site that can drift, and only one).
#[tokio::test]
async fn the_touch_target_constant_is_what_the_page_actually_renders() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    register_and_login(&app, &user).await;

    let resp = app.server.get("/projects").await;
    let body = resp.text();

    assert!(
        body.contains(TOUCH_TARGET),
        "the rendered page must contain components::TOUCH_TARGET's current \
         value ({TOUCH_TARGET:?}) verbatim; body: {body}"
    );
}
