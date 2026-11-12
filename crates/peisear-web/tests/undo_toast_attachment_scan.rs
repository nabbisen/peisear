//! `DM-TEST-001` item 4 — undo DOM order, all four direct-manipulation
//! surfaces (`dm.js`, `board.js`, `plan.js`, `calendar.js`).
//!
//! **Rewritten for `VIEW-003`** (round 3 of `060`): the toast these
//! four scripts used to build — a separate `position: fixed` element,
//! created fresh per action — is gone. The undo offer now renders
//! inside the one live region that already sits on the page
//! (`#status-announcements`), relocated beside whichever element the
//! offer is for. The structural guarantee worth pinning changed with
//! it: not "a button is appended into a toast div appended onto the
//! acted-on element" (`alertBox`/`toast`, both gone), but "the undo
//! button and the message text are appended into the shared region,
//! and the region itself is attached to (or immediately after) the
//! exact element `showUndoInline` was called with" — the same Undo-
//! is-the-next-Tab-stop property, built the same way, through a
//! different, smaller DOM shape.
//!
//! The eight tests this file used to carry for `FLOW-001`'s auto-
//! remove timer and `FLOW-001` round 2's dual-source resume guard are
//! deleted, not ported: `VIEW-003` deletes the timer itself (a single,
//! relocatable region cannot suffer the two-toasts-collide defect the
//! timer's pause/resume existed to manage), so there is nothing left
//! for those assertions to describe.
//!
//! **A source scan, not a DOM assertion and not a Tab-walk** (`§10.17`): the
//! live region's content does not reach this shape until the script
//! runs, so there is no server-rendered HTML a Rust integration test
//! could inspect for it (`§10.15`'s own class — nothing executes these
//! files). What a text scan over the source *can* check is the
//! structural guarantee the measurement found: `showUndoInline` always
//! appends the undo button and the message span into `region`, and
//! attaches `region` to (or immediately after) the exact element the
//! function was called with — so wherever that element sits in the
//! real DOM, Undo lands right after it, by construction. These tests
//! assert that chain is still there in each file's source, scoped to
//! `showUndoInline`'s own body so a coincidental match elsewhere in the
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

/// A window from `function {name}(` to the start of the next
/// top-level `function {next_fn}(` declaration that follows it in the
/// same file. Not real brace matching, and not always `showUndoInline`'s
/// own body alone — `calendar.js` declares `syncBlockState` and
/// `attachBlock` between `showUndoInline` and `performUndo`, so the
/// window there is wider than the one function. What makes the window
/// safe regardless is narrower than exact adjacency: every substring
/// these tests check for is written only inside `showUndoInline`'s own
/// body in all four files, so a wider window that happens to include
/// other functions too still cannot match anywhere else.
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
/// after the acted-on element: the message span and the undo button
/// both become `region`'s own children, and `region` itself is
/// attached to (or immediately after) the element the function
/// received as its own first parameter — not some other element, and
/// not detached into, say, `document.body`.
fn assert_undo_is_attached_after_the_acted_on_element(file: &str) {
    let source = read_static(file);
    let body = scoped_function(&source, "showUndoInline", "performUndo");
    let param = first_param_of(body, "showUndoInline");

    assert!(
        body.contains("region.appendChild(text)"),
        "{file}: showUndoInline must append the message span into region: {body}"
    );
    assert!(
        body.contains("region.appendChild(undoButton)"),
        "{file}: showUndoInline must append the undo button into region: {body}"
    );

    let sibling = format!(r#"{param}.insertAdjacentElement("afterend", region)"#);
    let child = format!("{param}.appendChild(region)");
    assert!(
        body.contains(sibling.as_str()) || body.contains(child.as_str()),
        "{file}: showUndoInline must attach region to its own first parameter \
         (`{param}`) -- either as `{param}.appendChild(region)` or \
         `{sibling}` -- so Undo lands in DOM order right after the acted-on \
         element, not detached somewhere else: {body}"
    );
}

/// `dm.js` — the issue list/detail status control. The acted-on
/// element is the status `<form>` itself; since a `<form>` with the
/// live region appended as a child could interact oddly with form
/// semantics, this one attaches the region as the form's own next
/// *sibling* (`insertAdjacentElement("afterend", ...)`) rather than
/// appending it inside the form the other three use for their
/// (non-form) card/row/block.
#[test]
fn dm_js_attaches_the_live_region_after_the_status_form() {
    assert_undo_is_attached_after_the_acted_on_element("dm.js");
}

/// `board.js` — the kanban card.
#[test]
fn board_js_attaches_the_live_region_inside_the_dragged_card() {
    assert_undo_is_attached_after_the_acted_on_element("board.js");
}

/// `plan.js` — the sprint-plan row.
#[test]
fn plan_js_attaches_the_live_region_inside_the_dragged_row() {
    assert_undo_is_attached_after_the_acted_on_element("plan.js");
}

/// `calendar.js` — the day-view block.
#[test]
fn calendar_js_attaches_the_live_region_inside_the_dragged_block() {
    assert_undo_is_attached_after_the_acted_on_element("calendar.js");
}

/// `FOCUS-001`: a source scan can see that a restore call exists; it
/// cannot see where focus actually lands once the browser runs it --
/// that property is `drag-outcome-gate.mjs`'s own fourth assertion
/// (`plan.js`/`calendar.js`) and this round's bespoke measurement
/// (`dm.js`/`board.js`, evidence only, no permanent gate -- the same
/// asymmetry `GATE-006` already drew for Tab order). This is the
/// backstop the same project convention already uses for `FLOW-001`'s
/// own resume guards: proof the wiring cannot be silently dropped,
/// beside the stronger behavioural proof, not instead of it.
///
/// Scoped to `showUndoInline`'s own body (the same window
/// [`assert_undo_is_attached_after_the_acted_on_element`] uses):
/// the undo button's click handler must call `restoreFocus(` guarded
/// by the `hadFocus` check captured before the region is cleared --
/// `if (hadFocus) restoreFocus(` is the exact shape all four files
/// share.
fn assert_undo_click_restores_focus_when_it_had_it(file: &str) {
    let source = read_static(file);
    let body = scoped_function(&source, "showUndoInline", "performUndo");
    let param = first_param_of(body, "showUndoInline");

    assert!(
        body.contains("region.contains(document.activeElement)"),
        "{file}: showUndoInline's undo button must capture whether focus was \
         inside the region before clearing it, so restoration is conditional \
         on that rather than unconditional: {body}"
    );

    let restore_call = format!("if (hadFocus) restoreFocus({param})");
    assert!(
        body.contains(restore_call.as_str()),
        "{file}: showUndoInline's undo button must restore focus to \
         `{param}` when (and only when) focus was inside the region -- \
         expected `{restore_call}`: {body}"
    );
}

#[test]
fn dm_js_restores_focus_when_undo_is_activated_with_it() {
    assert_undo_click_restores_focus_when_it_had_it("dm.js");
}

#[test]
fn board_js_restores_focus_when_undo_is_activated_with_it() {
    assert_undo_click_restores_focus_when_it_had_it("board.js");
}

#[test]
fn calendar_js_restores_focus_when_undo_is_activated_with_it() {
    assert_undo_click_restores_focus_when_it_had_it("calendar.js");
}

#[test]
fn plan_js_restores_focus_when_undo_is_activated_with_it() {
    assert_undo_click_restores_focus_when_it_had_it("plan.js");
}

/// `plan.js` only: its row moves into its destination list inside
/// `performUndo`'s own fetch `.then()` — asynchronously, after the
/// click handler's own immediate restore has already run — and
/// reparenting a focused element can blur it. `performUndo`'s success
/// callback must restore focus a second time, guarded by the same
/// `_focusAfterMove` flag the click handler set, once the row has
/// actually moved.
#[test]
fn plan_js_restores_focus_again_after_the_row_actually_moves() {
    let source = read_static("plan.js");
    let body = scoped_function(&source, "performUndo", "applyMove");
    assert!(
        body.contains("if (row._focusAfterMove) {")
            && body.contains("row._focusAfterMove = false;")
            && body.contains("restoreFocus(row);"),
        "plan.js: performUndo's own success callback must restore focus again, \
         guarded by `row._focusAfterMove`, once the row has actually moved -- \
         a single restore right after onUndo() races the async reparent: {body}"
    );
}
