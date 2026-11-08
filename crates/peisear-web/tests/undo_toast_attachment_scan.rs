//! `DM-TEST-001` item 4 — undo DOM order, all four direct-manipulation
//! surfaces (`dm.js`, `board.js`, `plan.js`, `calendar.js`). `FR-DM-006`
//! was just recorded Met on all four on the strength of a reading
//! (`FR-DM-002-measurement-review.md` §4): *"a real `<button
//! type="button">` with the undo label and a `setTimeout(…, 5000)`...
//! all four append the toast as the acted-on element's own DOM
//! child, so Undo is the next Tab stop."* This is what keeps that
//! reading pinned.
//!
//! **A source scan, not a DOM assertion and not a Tab-walk** (`§10.17`): the toast
//! these four scripts build does not exist until the script runs, so
//! there is no server-rendered HTML a Rust integration test could
//! inspect for it (`§10.15`'s own class — nothing executes these
//! files). What a text scan over the source *can* check is the
//! structural guarantee the measurement found: `showUndoToast` always
//! appends the undo button into `alertBox`, `alertBox` into `toast`,
//! and `toast` onto (or immediately after) the exact element the
//! function was called with — so wherever that element sits in the
//! real DOM, Undo lands right after it, by construction. These tests
//! assert that chain is still there in each file's source, scoped to
//! `showUndoToast`'s own body so a coincidental match elsewhere in the
//! file cannot pass them.
//!
//! No `mod common`, no `TestApp` — nothing here renders a page or
//! starts a server; it reads `static/*.js` the same way
//! `static_js_scan` (`crates/peisear-web/src/static_js_scan.rs`) does,
//! via `CARGO_MANIFEST_DIR/static`.

use std::path::{Path, PathBuf};

fn static_file(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("static")
        .join(name)
}

fn read_static(name: &str) -> String {
    let path = static_file(name);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

/// Scope to one function's body: from `function {name}(` to the start
/// of the next top-level `function {next_fn}(` declaration that
/// follows it in the same file. Not real brace matching — a
/// substring window — but sufficient here because every one of the
/// four files declares `performUndo` as the very next function after
/// `showUndoToast`, and everything this module asserts on sits well
/// before that boundary in all four.
fn scoped_function<'a>(source: &'a str, name: &str, next_fn: &str) -> &'a str {
    let start_marker = format!("function {name}(");
    let start = source
        .find(&start_marker)
        .unwrap_or_else(|| panic!("no `{start_marker}` found"));
    let end_marker = format!("function {next_fn}(");
    let end = source[start..]
        .find(&end_marker)
        .map(|rel| start + rel)
        .unwrap_or_else(|| panic!("no `{end_marker}` found after `{start_marker}`"));
    &source[start..end]
}

/// The first parameter name out of `function {name}(PARAM, ...) {`.
fn first_param_of<'a>(scoped_body: &'a str, name: &str) -> &'a str {
    let start_marker = format!("function {name}(");
    let after = &scoped_body[start_marker.len()..];
    let close = after.find(')').expect("function signature has a `)`");
    let params = &after[..close];
    params
        .split(',')
        .next()
        .expect("function has at least one parameter")
        .trim()
}

/// Assert the structural chain that puts Undo in DOM order right
/// after the acted-on element: the button becomes `alertBox`'s child,
/// `alertBox` becomes `toast`'s child, and `toast` itself is attached
/// to (or immediately after) the element the function received as
/// its own first parameter — not some other element, and not
/// detached into, say, `document.body`.
fn assert_undo_is_appended_after_the_acted_on_element(file: &str) {
    let source = read_static(file);
    let body = scoped_function(&source, "showUndoToast", "performUndo");
    let param = first_param_of(body, "showUndoToast");

    assert!(
        body.contains("alertBox.appendChild(undoButton)"),
        "{file}: showUndoToast must append the undo button into alertBox: {body}"
    );
    assert!(
        body.contains("toast.appendChild(alertBox)"),
        "{file}: showUndoToast must append alertBox into toast: {body}"
    );

    let sibling = format!(r#"{param}.insertAdjacentElement("afterend", toast)"#);
    let child = format!("{param}.appendChild(toast)");
    assert!(
        body.contains(sibling.as_str()) || body.contains(child.as_str()),
        "{file}: showUndoToast must attach toast to its own first parameter \
         (`{param}`) -- either as `{param}.appendChild(toast)` or \
         `{sibling}` -- so Undo lands in DOM order right after the acted-on \
         element, not detached somewhere else: {body}"
    );
}

/// `FLOW-001` (WCAG 2.2.1 Timing Adjustable) — the four auto-remove
/// pause/resume handlers must be attached to `toast` itself, inside
/// `showUndoToast`'s own body. A source-level fact, the same shape
/// [`assert_undo_is_appended_after_the_acted_on_element`] already
/// checks: it cannot see that hovering actually clears the pending
/// timeout (that needs the browser, see `drag-outcome-gate.mjs`), but
/// it can see that all four listeners are wired to the right element.
fn assert_auto_remove_pauses_on_hover_and_focus(file: &str) {
    let source = read_static(file);
    let body = scoped_function(&source, "showUndoToast", "performUndo");

    for (event, handler) in [
        ("mouseenter", "pauseAutoRemove"),
        ("focusin", "pauseAutoRemove"),
        ("mouseleave", "resumeIfFocusNotInside"),
        ("focusout", "resumeIfPointerNotOver"),
    ] {
        let needle = format!(r#"toast.addEventListener("{event}", {handler})"#);
        assert!(
            body.contains(needle.as_str()),
            "{file}: showUndoToast must attach `{needle}` so the five-second \
             auto-remove pauses while the reader is on the toast (pointer or \
             keyboard focus) and resumes when they leave: {body}"
        );
    }
}

#[test]
fn dm_js_pauses_auto_remove_on_hover_and_focus() {
    assert_auto_remove_pauses_on_hover_and_focus("dm.js");
}

#[test]
fn board_js_pauses_auto_remove_on_hover_and_focus() {
    assert_auto_remove_pauses_on_hover_and_focus("board.js");
}

#[test]
fn plan_js_pauses_auto_remove_on_hover_and_focus() {
    assert_auto_remove_pauses_on_hover_and_focus("plan.js");
}

#[test]
fn calendar_js_pauses_auto_remove_on_hover_and_focus() {
    assert_auto_remove_pauses_on_hover_and_focus("calendar.js");
}

/// A nested function's own body, scoped from `function {name}(` to the
/// next `    function ` declaration within `within` (or to `within`'s
/// own end, if none follows) — the same windowed, not brace-matched,
/// approach [`scoped_function`] uses for a whole file, narrowed to an
/// already-scoped substring so it can isolate one of several sibling
/// nested functions inside `showUndoToast`'s own body.
fn scoped_inner_function<'a>(within: &'a str, name: &str) -> &'a str {
    let start_marker = format!("function {name}(");
    let start = within
        .find(&start_marker)
        .unwrap_or_else(|| panic!("no `{start_marker}` found"));
    let after_start = start + start_marker.len();
    let end = within[after_start..]
        .find("\n    function ")
        .map(|rel| after_start + rel)
        .unwrap_or(within.len());
    &within[start..end]
}

/// `FLOW-001` round 2 — each resume handler must check the *other*
/// source's own live DOM truth before resuming (so releasing one does
/// not resume the countdown while the other still holds it), and must
/// clear any timer still running before starting a new one. A source
/// scan cannot see behaviour — the browser assertion in
/// `drag-outcome-gate.mjs` is the primary evidence for the property
/// itself — this is the backstop that a future edit cannot drop one of
/// the two guards while leaving the listener wiring untouched.
fn assert_resume_checks_the_other_source(file: &str) {
    let source = read_static(file);
    let body = scoped_function(&source, "showUndoToast", "performUndo");

    let focus_guard = scoped_inner_function(body, "resumeIfFocusNotInside");
    assert!(
        focus_guard.contains("toast.contains(document.activeElement)"),
        "{file}: resumeIfFocusNotInside must not resume while focus is still \
         inside the toast: {focus_guard}"
    );
    assert!(
        focus_guard.contains("clearTimeout(current.timer)"),
        "{file}: resumeIfFocusNotInside must clear any timer still running \
         before starting a new one: {focus_guard}"
    );

    let pointer_guard = scoped_inner_function(body, "resumeIfPointerNotOver");
    assert!(
        pointer_guard.contains(r#"toast.matches(":hover")"#),
        "{file}: resumeIfPointerNotOver must not resume while the pointer is \
         still over the toast: {pointer_guard}"
    );
    assert!(
        pointer_guard.contains("clearTimeout(current.timer)"),
        "{file}: resumeIfPointerNotOver must clear any timer still running \
         before starting a new one: {pointer_guard}"
    );
}

#[test]
fn dm_js_resume_checks_the_other_source() {
    assert_resume_checks_the_other_source("dm.js");
}

#[test]
fn board_js_resume_checks_the_other_source() {
    assert_resume_checks_the_other_source("board.js");
}

#[test]
fn plan_js_resume_checks_the_other_source() {
    assert_resume_checks_the_other_source("plan.js");
}

#[test]
fn calendar_js_resume_checks_the_other_source() {
    assert_resume_checks_the_other_source("calendar.js");
}

/// `dm.js` — the issue list/detail status control. The acted-on
/// element is the status `<form>` itself; since a `<form>` with a
/// toast appended as a child could interact oddly with form
/// semantics, this one attaches the toast as the form's own next
/// *sibling* (`insertAdjacentElement("afterend", ...)`) rather than
/// appending it inside the form the other three use for their
/// (non-form) card/row/block.
#[test]
fn dm_js_appends_the_undo_toast_after_the_status_form() {
    assert_undo_is_appended_after_the_acted_on_element("dm.js");
}

/// `board.js` — the kanban card.
#[test]
fn board_js_appends_the_undo_toast_after_the_dragged_card() {
    assert_undo_is_appended_after_the_acted_on_element("board.js");
}

/// `plan.js` — the sprint-plan row.
#[test]
fn plan_js_appends_the_undo_toast_after_the_dragged_row() {
    assert_undo_is_appended_after_the_acted_on_element("plan.js");
}

/// `calendar.js` — the day-view block.
#[test]
fn calendar_js_appends_the_undo_toast_after_the_dragged_block() {
    assert_undo_is_appended_after_the_acted_on_element("calendar.js");
}
