//! `DOC-005` §4 (`NFR-A11Y-007`, `DEC-049` as amended by §3.1) — a
//! guard for the alignment property `touch_target_scan` was never
//! built to check. `touch_target_scan` and the overflow gate both
//! measure whether content escapes its box, not whether two labels in
//! a row share a baseline; neither would have caught `DOC-005` §1's
//! defect (a 44px `grow()` box whose plain-text label sat at the
//! box's top edge inside a `flex items-center` row).
//!
//! **One rule, not a taxonomy inside one name** (`§3.1`'s own framing
//! of why this got simpler once `components::grow_text` existed):
//! every `components::grow(...)` call site in `src/components/` must
//! already be one of three things — a flex container (carries
//! `btn`/`badge`/a bare `flex`/`inline-flex` class), a form control
//! (`<input>`, `<select>`, `<textarea>`, which centre their own
//! content natively), or already a block box (`block`/daisyUI's
//! `card`, see [`already_establishes_a_block_box`] for why the first
//! pass missed this third case live) — a plain text element reaching
//! `grow(...)` directly, rather than through `components::grow_text`,
//! is exactly `DOC-005` §1's defect recurring. The complementary
//! direction also holds and costs nothing extra to check: no
//! `components::grow_text(...)` call site is a form control, since
//! `grow_text` composes `inline-flex` unconditionally and `DOC-005`
//! §2 ruled that wrong on a form control's own native centring.
//!
//! **A corrected comment-stripper, written here first.** At `0.51.0`,
//! `touch_target_scan`/`prose_scan`/`test_harness_scan`'s own
//! `strip_line_comments` split a line on the first literal `"//"`,
//! including one inside a real string literal — `xmlns="http://
//! www.w3.org/2000/svg"`, present in five files under
//! `src/components/` (`layout.rs`, `notifications.rs`, `projects.rs`,
//! `sprints.rs`, `teams.rs`). That truncated the line mid-literal,
//! leaving an unterminated quote that misdirected every subsequent
//! `"..."` span's pairing for the rest of the file — found measuring
//! `DOC-005` §2's own call-site count, confirmed live against this
//! tree, and named to the architect as a finding on the shared helper
//! rather than fixed there: that round's scope was the alignment
//! defect, not guard infrastructure generally. This module needed its
//! own correct version to mean anything at the time, so it did not
//! inherit the bug by sharing the sibling's copy. `GRD-001` (`0.52.0`)
//! lifted this function verbatim into `scan_support::strip_line_comments`
//! and switched every scan, this one included, to the one shared copy —
//! see `requirements.md §10.38` for `QA-002`'s duplication call
//! reversed for this helper.
//!
//! **A named limit on the let-bound sites, not a parser.** Five `let`
//! bindings in this tree compose `grow` as
//! `let IDENT = grow(if cond { "…" } else { "…" });`, then reach a
//! tag through a later `class=IDENT` — `touch_target_scan`'s own
//! `class_value_identifier_binding_calls_grow`-style chase could
//! follow that to a tag, but this scan does not: all five are
//! daisyUI `btn`-family toggles (`board_classes`/`list_classes`/`cls`
//! in `issues.rs`, `class` in `calendar.rs`), verified by eye rather
//! than by this guard — already flex containers via `btn`, so
//! `grow()` is already correct for all five independent of whether
//! this scan can see them. A sixth such binding would need the same
//! verification, not an extension to this scan, unless it stops being
//! a handful.
//!
//! **`DISC-001` (`060`) found a fourth way to be correct, and named
//! three call sites rather than widening the rule.** A `<summary>`'s
//! own UA-default `display: list-item` is a second accessibility
//! requirement's own subject (`NFR-A11Y-007`'s own marker, restored by
//! `grow()` specifically *because* it does not add `inline-flex` the
//! way `grow_text()` does) — so neither `grow_text()` (kills the
//! marker) nor an explicit `block`/`card` class (also kills a
//! `<summary>`'s own marker; `list-item` and `block` are different
//! `display` values) can be this call site's answer, and this scan's
//! own three recognised shapes have no fourth name for "already
//! correct because the padding itself, not a flex box, supplies the
//! centring." [`is_padding_centred_disclosure`] is that fourth name,
//! kept as narrow as [`is_named_escalation_exclusion`] in the sibling
//! guard it mirrors: three exact call sites, not a blanket exemption
//! for every `<summary>` — a future `<summary>` reaching `grow()` with
//! `min-h-11` alone and no padding would still have `DOC-005` §1's own
//! defect (text sitting at a `min-height` box's top, no flex to centre
//! it), so this needs the same per-site verification the `let`-bound
//! sites above already get, not a tag-wide pass.
//!
//! **Verified, not assumed**: a real render (`VISUAL`, `DISC-001`'s
//! own completion report) shows the marker and the text sharing one
//! visual line, centred in the row — `py-3`/`py-4`'s own arithmetic
//! (`components/issues.rs`'s own comments at each site) make the
//! padded box's total height exactly the padding plus the context's
//! own line-height, so there is no excess `min-height` space left for
//! the text to float inside.

use std::fs;
use std::path::{Path, PathBuf};

use crate::scan_support::strip_line_comments;

/// Every `.rs` file under `dir`, recursively — the same walk the
/// sibling scans use, duplicated rather than shared for a
/// fifteen-line helper (their own stated reasoning).
fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries {
        let entry = entry.expect("dir entry");
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

/// Every `"..."` string literal's content span in `source` —
/// `(quote_pos, content_start, content_end)` — the same shape and the
/// same named limit (no raw strings, byte strings, or multi-line
/// literals) as the sibling scans' own version.
fn quoted_string_spans(source: &str) -> Vec<(usize, usize, usize)> {
    let bytes = source.as_bytes();
    let mut spans = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'"' {
            let quote_pos = i;
            let content_start = i + 1;
            i = content_start;
            while i < bytes.len() && bytes[i] != b'"' {
                if bytes[i] == b'\\' && i + 1 < bytes.len() {
                    i += 2;
                } else {
                    i += 1;
                }
            }
            spans.push((quote_pos, content_start, i.min(bytes.len())));
        }
        i += 1;
    }
    spans
}

/// Walks backward from `quote_pos` tracking bracket depth to the
/// nearest `(` at depth 0 — the same walk `touch_target_scan`'s
/// `is_grow_call_argument` uses, bounded to 300 bytes back (every
/// real call in this tree resolves well inside that span; an
/// unrelated quote with no enclosing call at all, like `href="/x"`,
/// must not be allowed to walk arbitrarily far and match some
/// distant, coincidental `(` — the bug `DOC-005`'s own measurement
/// tooling hit before this scan existed).
fn open_paren_before(source: &str, quote_pos: usize) -> Option<usize> {
    let bytes = source.as_bytes();
    let mut depth: i32 = 0;
    let mut i = quote_pos;
    let floor = quote_pos.saturating_sub(300);
    while i > floor {
        i -= 1;
        match bytes[i] {
            b')' | b'}' | b']' => depth += 1,
            b'(' => {
                if depth == 0 {
                    return Some(i);
                }
                depth -= 1;
            }
            b'{' | b'[' if depth > 0 => depth -= 1,
            _ => {}
        }
    }
    None
}

/// True if the string literal at `quote_pos` is the sole argument of
/// a call named exactly `name` — i.e. the nearest enclosing `(` at
/// depth 0 is immediately preceded by `name`. Distinguishes
/// `grow("...")` from `grow_text("...")`: `"grow_text".ends_with(
/// "grow")` is false, so the two never cross-match.
fn is_call_argument(source: &str, quote_pos: usize, name: &str) -> bool {
    open_paren_before(source, quote_pos)
        .is_some_and(|paren| source[..paren].trim_end().ends_with(name))
}

/// The tag name enclosing a `class=NAME(...)` call site, and its
/// 1-based line — `None` if the text immediately before `NAME(`'s
/// open paren isn't `class=`, or no unclosed `<tag` precedes it on
/// the same attribute run.
fn enclosing_tag(source: &str, quote_pos: usize, name: &str) -> Option<(String, usize)> {
    let paren = open_paren_before(source, quote_pos)?;
    let before_call = source[..paren].trim_end();
    if !before_call.ends_with(name) {
        return None;
    }
    let before_name = before_call[..before_call.len() - name.len()].trim_end();
    if !(before_name.ends_with("class=") || before_name.ends_with("class =")) {
        return None;
    }
    let class_start = before_name.rfind("class")?;
    let before_class = &before_name[..class_start];
    let lt = before_class.rfind('<')?;
    if before_class[lt..].contains('>') {
        return None;
    }
    let after_lt = &before_class[lt + 1..];
    let name_end = after_lt
        .find(|c: char| !(c.is_alphanumeric() || c == '_' || c == '-'))
        .unwrap_or(after_lt.len());
    let tag = after_lt[..name_end].to_string();
    if tag.is_empty() {
        return None;
    }
    let line = source[..quote_pos].matches('\n').count() + 1;
    Some((tag, line))
}

/// True if `content` (a `class=` value) already makes its element a
/// flex container by some other class it carries — daisyUI's
/// `btn`/`badge` families compose `inline-flex`/`flex` themselves, and
/// a bare `flex`/`inline-flex` utility does it directly.
fn is_flex_container(content: &str) -> bool {
    content.split_whitespace().any(|cls| {
        cls == "btn"
            || cls.starts_with("btn-")
            || cls == "badge"
            || cls.starts_with("badge-")
            || cls == "flex"
            || cls == "inline-flex"
    })
}

/// True if `content` already establishes a non-inline box the §1
/// defect's own premise does not reach — either an explicit `block`
/// utility, or daisyUI's `card` (which always wraps a `<div
/// class="card-body">`, never a bare label). Found live, not derived:
/// the first `grow` → `grow_text` pass at these shapes (`teams.rs`'s
/// and `sprints.rs`'s own team-card links, both `card … block`;
/// `projects.rs`'s project-card link, `card` with no `block`;
/// `issues.rs`'s and `search.rs`'s plain `block` links) turned the
/// overflow gate red at 320/390/768px across five unrelated pages —
/// `static/tailwind.css` emits `.block{display:block}` *before*
/// `.inline-flex{display:inline-flex}`, so the later rule won
/// regardless of class order, same mechanism `§3.1` named for `flex`,
/// and the nested `<div class="card-body">` these links wrap became a
/// flex item with `min-width: auto`, refusing to shrink below its
/// unwrapped content width. A link that already declares its own box
/// type was never the plain single-line label `DOC-005` §1 found
/// floating at the top of its 44px box — it wraps rich block content,
/// not a label expecting a shared flex baseline with a sibling, so
/// `grow()` plus a 44px floor is already correct for it.
fn already_establishes_a_block_box(content: &str) -> bool {
    content
        .split_whitespace()
        .any(|cls| cls == "block" || cls == "card")
}

/// `DISC-001`: the three `<summary>` call sites in
/// `components/issues.rs` this round moved from `grow_text(...)` to
/// `grow(...)` so their native `list-item` display (and `::marker`)
/// survives — centred by padding reaching the context's own
/// line-height exactly, not by `grow_text`'s `inline-flex
/// items-center`, which would undo the very marker this round
/// restores. Keyed on each site's own exact padding suffix, which no
/// other `grow(...)` call site in this tree shares. Closing this is
/// the same kind of decision closing
/// [`is_named_escalation_exclusion`] is in the sibling guard — a
/// verified, per-site call, not this scan's to make by itself.
fn is_padding_centred_disclosure(tag: &str, content: &str) -> bool {
    tag == "summary"
        && (content.ends_with("hover:text-base-content py-3")
            || content.ends_with("hover:text-base-content py-4"))
}

const FORM_CONTROL_TAGS: [&str; 3] = ["input", "select", "textarea"];

#[cfg(test)]
mod tests;
