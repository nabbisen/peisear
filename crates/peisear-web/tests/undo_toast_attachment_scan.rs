//! `DM-TEST-001` item 4 — undo DOM order, all four direct-manipulation
//! surfaces (`dm.js`, `board.js`, `plan.js`, `calendar.js`). `FR-DM-006`
//! was just recorded Met on all four on the strength of a reading
//! (`FR-DM-002-measurement-review.md` §4): *"a real `<button
//! type="button">` with the undo label and a `setTimeout(…, 5000)`...
//! all four append the toast as the acted-on element's own DOM
//! child, so Undo is the next Tab stop."* This is what keeps that
//! reading pinned.
//!
//! **A DOM-order assertion, not a Tab-walk** (`§10.17`): the toast
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
//! via `CARGO_MANIFEST_DIR/../../static`.

use std::path::{Path, PathBuf};

fn static_file(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
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

/// `dm.js` — the issue list/detail status control. The acted-on
/// element is the status `<form>` itself; since a `<form>` with a
/// toast appended as a child could interact oddly with form
/// semantics, this one attaches the toast as the form's own next
/// *sibling* (`insertAdjacentElement("afterend", ...)`) rather than
/// appending it inside the form the other three use for their
/// (non-form) card/row/block.
#[test]
fn dm_js_undo_lands_right_after_the_status_form() {
    assert_undo_is_appended_after_the_acted_on_element("dm.js");
}

/// `board.js` — the kanban card.
#[test]
fn board_js_undo_lands_right_after_the_dragged_card() {
    assert_undo_is_appended_after_the_acted_on_element("board.js");
}

/// `plan.js` — the sprint-plan row.
#[test]
fn plan_js_undo_lands_right_after_the_dragged_row() {
    assert_undo_is_appended_after_the_acted_on_element("plan.js");
}

/// `calendar.js` — the day-view block.
#[test]
fn calendar_js_undo_lands_right_after_the_dragged_block() {
    assert_undo_is_appended_after_the_acted_on_element("calendar.js");
}
