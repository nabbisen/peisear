//! `JS-002` (RFC 011, step 1b) — pins the shape `dm.js`'s fallback
//! boundary is made of, not that the boundary behaves correctly.
//!
//! `JS-001` traced the boundary — *"falling back to a native form
//! submit is correct before the server has applied the change and
//! wrong after"* — and found it has no boolean anywhere. The property
//! holds because `applyChange` carries its own `try { … } catch { … }`:
//! an exception raised inside it unwinds into *that* handler and
//! therefore never reaches the outer `.catch()` that calls
//! `fallback()`. It is a fact about which handler is nested where, so
//! no amount of Rust-side policy testing touches it — `HLT-001`'s "one
//! authority" move and `QA-019`'s before it both assume there is a
//! *value* to relocate; here there is only a *shape*.
//!
//! **What this guard does not claim.** It does not execute `dm.js` and
//! it does not prove the fallback boundary works — that would need a
//! browser, which is exactly what `§10.15` records this project cannot
//! run yet. It proves three narrower, textual things: that a function
//! named `applyChange` still exists, that its body still contains its
//! own `try`/`catch`, and that `fallback(` is never called from inside
//! that body. Losing any one of those is the specific regression
//! `STATUS-002`'s review caught by reading — wrapping the whole
//! promise chain in one fallback `catch`, which re-submits an
//! already-applied change with a stale lock value. This guard makes
//! that regression fail loudly and mechanically instead of needing a
//! second careful reading. `§10.15` stays open; this narrows what is
//! unguarded within it, the same claim `test_harness_scan` makes about
//! clock-derived temp paths — not that the harness is correct, but
//! that the shape which made it incorrect cannot return.
//!
//! **A sibling module, not an addition to `static_js_scan`** — the
//! same reasoning `dec_007_ci_scan` gave for splitting off
//! `dec_007_scan`: this guard's failure means something structurally
//! different from a hardcoded-prose violation, and keeping them apart
//! keeps failure attribution clear. Nothing here is shared with
//! `static_js_scan` beyond the same `CARGO_MANIFEST_DIR`-relative file
//! resolution and the same `//`-only comment-stripping — both
//! duplicated rather than imported, matching every other scan module
//! in this crate's own stated precedent for that choice.
//!
//! **The name is pinned first and separately, before anything about
//! the body** — `QA-021`'s lesson: a guard that only searches for
//! `fallback(` and `try`/`catch` inside "whatever function comes after
//! this marker" would go silently green if `applyChange` were renamed
//! out from under it, because the marker itself would stop matching
//! and the body-search would simply find nothing to complain about.
//! `apply_change_exists_by_name` fails loudly and specifically in that
//! case instead.
//!
//! **Named textual limits**, in the same spirit as `dec_007_ci_scan`'s
//! `job_is_disabled` and `peisear-core`'s `find_matching_block`:
//!
//! - **Brace-depth counting, not a parser.** `find_function_body`
//!   counts `{`/`}` from `applyChange`'s opening brace to its match.
//!   `QA-008` and `QA-012` both drew the line at a real parser for one
//!   assertion; this stays a text scan.
//! - **A `{` or `}` inside a string literal would defeat the brace
//!   count.** `dm.js` has none inside `applyChange`'s body today (its
//!   only string-shaped content there is `copy.movedTo[newStatus]`, a
//!   property lookup, and the reload call, neither of which contains a
//!   literal brace) — a real limit, not a theoretical one dodged by
//!   luck, and not handled here.
//! - **`/* */` block comments are not stripped**, only `//` line
//!   comments — `strip_line_comments` here is a byte-for-byte copy of
//!   `static_js_scan.rs`'s, same documented gap. `dm.js` has no block
//!   comments today; introducing one containing a brace inside
//!   `applyChange` would defeat the depth count silently.
//! - **The name match requires the exact text `function applyChange(`**
//!   with no space before the parenthesis, matching this file's own
//!   consistent style. Restructuring `applyChange` into an arrow
//!   function or a method shorthand would fail
//!   `apply_change_exists_by_name` — correctly, since that is exactly
//!   the kind of change this guard exists to force a deliberate
//!   decision about, not a false negative to work around.
//! - **Closed in round 2** (`JS-002-review.md` §2): a `try`/`catch`
//!   nested inside `applyChange`'s own `showUndoToast` callback used
//!   to satisfy `apply_change_carries_its_own_try_catch`'s bare
//!   substring search without protecting anything — the function's
//!   own top-level statements (`setPressed`, `announcePolite`, …)
//!   would still throw straight out to the outer chain's `fallback()`.
//!   Unlike the three limits above, this was not a shape with no
//!   reason to appear — moving error handling closer to what throws is
//!   ordinary good instinct, and someone would plausibly make this
//!   change on purpose. `contains_top_level_try` fixes it by requiring
//!   the `try` at brace-depth 0 of `applyChange`'s own body.
//! - **Open, recorded in RFC 011 only (`JS-003` §7): a *narrowed*
//!   top-level `try` still passes.** `contains_top_level_try` proves a
//!   `try {` exists at depth 0 — it does not prove that `try` extends
//!   to `applyChange`'s own closing brace, only that one starts
//!   somewhere at the top level. A rewrite that closes the `try` early
//!   and leaves later top-level statements unprotected —
//!   ```js
//!   try { setPressed(form, newStatus); } catch (e) { ...; return; }
//!   var message = copy.movedTo[newStatus]; // now outside the try
//!   showUndoToast(form, message, function () { ... });
//!   ```
//!   — would still satisfy both current assertions. Closing this
//!   needs the `try`'s own matching close brace located (the same
//!   depth-counting `find_function_body` already does, applied a
//!   second time to the `try` block itself) and a rule that nothing
//!   but the `catch` follows it — a second depth-counting pass, not a
//!   parser, but more than a bare substring check, which is why it is
//!   named here rather than built in this round.

use std::fs;
use std::path::Path;

const DM_JS_RELATIVE_PATH: &str = "dm.js";

/// Same resolution `static_js_scan.rs` uses for the whole `static/`
/// directory, narrowed to the one file this guard cares about.
fn read_dm_js() -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir
        .join("..")
        .join("..")
        .join("static")
        .join(DM_JS_RELATIVE_PATH);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

/// Strips `//`-style line comments. Byte-for-byte the same as
/// `static_js_scan.rs`'s `strip_line_comments` and the same documented
/// limitation: line-based, does not account for `//` inside a string
/// literal. Duplicated rather than shared, matching every other scan
/// module's own choice for this exact helper.
fn strip_line_comments(source: &str) -> String {
    source
        .lines()
        .map(|line| line.split("//").next().unwrap_or(""))
        .collect::<Vec<_>>()
        .join("\n")
}

/// `function_name`'s body, brace-depth counted from its opening `{` to
/// the matching `}` so a nested `{ }` inside it (a callback passed to
/// another function, exactly what `applyChange` does when it hands
/// `showUndoToast` a closure) does not end the scan early. Panics with
/// a message naming the function if it cannot be found — a rename or
/// removal must fail this guard loudly, never silently return an
/// empty or wrong body. See the module doc's "named textual limits"
/// for what this brace count does not account for.
fn find_function_body<'a>(source: &'a str, function_name: &str) -> &'a str {
    let marker = format!("function {function_name}(");
    let start = source.find(&marker).unwrap_or_else(|| {
        panic!(
            "no `{marker}` found in dm.js -- if `{function_name}` was renamed or removed, \
             this guard (JS-002) needs a deliberate update, not to silently stop checking \
             the fallback boundary's shape"
        )
    });
    let after_signature = &source[start + marker.len()..];
    let brace_offset = after_signature
        .find('{')
        .unwrap_or_else(|| panic!("`{function_name}`'s signature in dm.js never reaches a `{{`"));
    let body_start = brace_offset + 1;
    let bytes = after_signature.as_bytes();
    let mut depth = 1i32;
    let mut i = body_start;
    while i < bytes.len() {
        match bytes[i] {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return &after_signature[body_start..i];
                }
            }
            _ => {}
        }
        i += 1;
    }
    panic!("`{function_name}`'s body in dm.js never closes");
}

/// Whether `try {` appears in `body` at brace-depth 0 -- as one of the
/// function's own top-level statements, not nested inside a callback
/// or any other brace pair defined within it.
///
/// `JS-002` round 2 (the architect's own plant, `JS-002-review.md`
/// §2): a bare `body.contains("try {")` still matches a `try`/`catch`
/// moved *inside* the callback `applyChange` already passes to
/// `showUndoToast` -- the substring is present, but it no longer
/// protects anything. An exception in `setPressed`/`announcePolite`,
/// now outside any handler, would propagate straight to the outer
/// chain's `fallback()` catch after the mutation has already landed --
/// `STATUS-002`'s defect, reopened, with the scan passing. Depth 0
/// here is `applyChange`'s own body, one level inside the function's
/// opening brace and no deeper; a `try` written inside the
/// `showUndoToast` callback's own `{ }` is at depth 1 and does not
/// count.
fn contains_top_level_try(body: &str) -> bool {
    let bytes = body.as_bytes();
    let mut depth: i32 = 0;
    let mut i = 0;
    while i < bytes.len() {
        if depth == 0 && body[i..].starts_with("try {") {
            return true;
        }
        match bytes[i] {
            b'{' => depth += 1,
            b'}' => depth -= 1,
            _ => {}
        }
        i += 1;
    }
    false
}

/// The name pin (`QA-021`'s precedent), independent of and checked
/// before either assertion about the body below.
#[test]
fn apply_change_exists_by_name() {
    let source = strip_line_comments(&read_dm_js());
    assert!(
        source.contains("function applyChange("),
        "dm.js no longer defines a function named `applyChange` -- if it was renamed, \
         this guard (JS-002) needs a deliberate update. It exists to pin the shape \
         `STATUS-002`'s fallback boundary is made of; do not let it go silently green \
         on a function that no longer matches its own name."
    );
}

/// §2.1 of `JS-002`: `applyChange` must carry its own `try`/`catch` —
/// the handler that turns a post-mutation UI failure into a reload
/// instead of letting it propagate to the outer chain's `fallback()`.
/// The `try` must be at `applyChange`'s own top level (`JS-002` round
/// 2, see `contains_top_level_try`'s doc comment) -- a `try` nested
/// inside a callback the function happens to pass elsewhere satisfies
/// a bare substring search without protecting the statements that run
/// before it.
#[test]
fn apply_change_carries_its_own_try_catch() {
    let source = strip_line_comments(&read_dm_js());
    let body = find_function_body(&source, "applyChange");
    assert!(
        contains_top_level_try(body) && body.contains("catch ("),
        "applyChange no longer carries its own top-level try/catch -- this is the \
         handler that keeps a post-mutation UI failure from reaching the outer \
         fallback() catch. A try/catch nested inside a callback (e.g. the one passed \
         to showUndoToast) does not count: it leaves applyChange's own statements \
         (setPressed, announcePolite, ...) unprotected, and an exception there would \
         propagate to the outer chain and call fallback() after the mutation already \
         landed -- STATUS-002's defect, reopened. Body scanned:\n{body}"
    );
}

/// §2.2 of `JS-002`: `fallback(` must never be called from inside
/// `applyChange`'s body -- the mutation has already landed by the time
/// this function runs, and nothing past that point may resubmit.
#[test]
fn fallback_is_never_called_inside_apply_change() {
    let source = strip_line_comments(&read_dm_js());
    let body = find_function_body(&source, "applyChange");
    assert!(
        !body.contains("fallback("),
        "fallback( is called inside applyChange's body -- the mutation has already \
         landed by the time applyChange runs, so nothing here may resubmit. This is \
         the exact shape STATUS-002's review corrected: the first handoff wrapped the \
         whole promise chain in one fallback catch, and a UI failure after a \
         successful change resubmitted it with a stale client_updated_at, handing the \
         user a conflict for something that already worked. Body scanned:\n{body}"
    );
}
