use super::*;

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
