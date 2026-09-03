//! `QA-015` §5 (`NFR-A11Y-007`, RFC 005 §6) — `checkbox-xs` resolves to
//! `16×16px` in `daisyui@4.12.14`'s pinned CSS
//! (`https://cdn.jsdelivr.net/npm/daisyui@4.12.14/dist/full.min.css`,
//! the version `components/layout.rs` loads), below WCAG 2.2's own AA
//! floor (2.5.8: 24×24) — the only class `QA-014`'s survey found doing
//! that, everything else in `src/components/` is 24px or larger.
//! `QA-015` removed it from the three real checkbox controls that
//! carried it (`notification_preferences.rs`), reaching the bare
//! `.checkbox` class's own `24px` (`height:1.5rem;width:1.5rem`). This
//! keeps it from coming back.
//!
//! **Sibling to `contrast_scan`, not folded into it.** Same reasoning
//! `dec_007_scan`/`dec_007_ci_scan` already established: contrast
//! (`WCAG` 1.4.3) and touch-target size (`WCAG` 2.5.8) are different
//! properties measured against a different pinned CSS fact, and a
//! regression in one should not read as a failure of the other's own
//! name. `contrast_scan`'s own doc comment names the same DaisyUI
//! version and a disjoint set of resolved values; this module names its
//! own.
//!
//! **One needle, not a class of needles — until `TT-002` made it
//! safe to be more.** `QA-014`'s survey found six other classes below
//! 44px (`btn-sm`/`-xs`, `input-sm`/`-xs`, `select-sm`/`-xs`) still in
//! deliberate use pending `0.30.0`'s touch-target design pass —
//! banning any of those then would have failed on the current,
//! correct tree and gotten weakened until it passed, worse than no
//! guard (`QA-013`'s own reasoning for why `contrast_scan` stops at
//! `/60` and not higher). `checkbox-xs` was different from the start:
//! `QA-015` removed every use, and the class serves no purpose this
//! codebase still needs — `checkbox-sm` (`20px`, still below AA) and
//! the bare `checkbox` (`24px`, the class this project now uses) are
//! the only smaller-than-default sizes with any legitimate reason to
//! exist here, and neither is `checkbox-xs`.
//!
//! **`TT-002` (RFC 012 step 3) made the other six safe to enforce
//! too** — every sizing-class site in `src/components/` now composes
//! `components::TOUCH_TARGET` via `components::grow`, so
//! [`every_sizing_class_site_composes_the_touch_target`] makes
//! `NFR-A11Y-007`'s whole size clause unconstructible, not just one
//! class of it. Reads `components::TOUCH_TARGET`/`grow`'s own
//! behaviour rather than a hardcoded copy — the guard lives in this
//! crate and has no excuse for one (`TT-002-round2-review.md` §4: the
//! 44px value already has one home in production and seven in test
//! code; this guard must not be an eighth).
//!
//! **No exception list.** `TT-002` round 2 converted the last three
//! hardcoded `min-h-11 min-w-11` literals specifically so this guard
//! would need none (`TT-002-round2-review.md` §2). If a future change
//! needs one, that is a finding to report, not a line to add — this
//! module's own `checkbox-xs` history already states why: a rule that
//! fails on a correct tree gets weakened until it passes, and an
//! exception list is how that weakening looks in practice.
//!
//! **Scans string-literal contents, not lines and not raw file
//! text.** `QA-004` found a guard satisfied by a doc-comment mention;
//! `QA-005` found one satisfied by a commented-out line. The inverse
//! matters here too: a doc comment reading *"`btn-sm` is 32px"* must
//! not **fail** this guard either. [`quoted_string_spans`] walks the
//! source char-by-char and only yields the span between an actual
//! `"..."` Rust string literal's quotes — a `///`/`//` comment is
//! never inside quotes, so prose is invisible to it in both
//! directions, with no need to special-case comment syntax at all.
//!
//! **A named limit, not a parser** — the same boundary `JS-002` hit
//! and the same answer: [`quoted_string_spans`] handles the one string
//! form every `class=` site in `src/components/` actually uses today
//! (a plain `"..."` literal, `\"` escapes recognised so an escaped
//! quote can't prematurely close a span) and does not attempt raw
//! strings (`r#"..."#`), byte strings, or multi-line literals. None
//! appear in a `class=` attribute in this tree; if one ever does, this
//! scan misses it rather than mis-parsing it — the same trade
//! `dm_fallback_boundary_scan`'s own doc comment makes for its class
//! of gap, named here rather than reached for a parser dependency.
//!
//! **Scope, stated because a green result reads as more than it
//! is** (`TT-001` §5, now a named limit on `NFR-A11Y-007` itself):
//! this guard keys off sizing classes, so it covers class-carrying
//! controls only. A plain `<a>` link, a breadcrumb, a whole-card link
//! — none carry `btn-*`/`input-*`/`select-*`, so none are counted,
//! checked, or claimed compliant by this scan. They sit **outside**
//! `TT-002`'s 139-control counting method entirely, not inside it and
//! passing. A green result here is a claim about class-carrying
//! controls, not about every interactive element in the tree.

use std::fs;
use std::path::{Path, PathBuf};

/// Every `.rs` file under `dir`, recursively — the same walk
/// `prose_scan`/`contrast_scan` use, duplicated rather than shared for
/// a fifteen-line helper.
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

#[test]
fn checkbox_xs_appears_nowhere() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let src_dir = manifest_dir.join("src");
    let mut files = Vec::new();
    collect_rs_files(&src_dir, &mut files);
    assert!(
        !files.is_empty(),
        "found no .rs files under src/ -- the workspace layout assumption \
         this scan depends on may have changed"
    );

    let mut offenders = Vec::new();
    for path in files
        .iter()
        .filter(|p| p.file_name().and_then(|n| n.to_str()) != Some("touch_target_scan.rs"))
    {
        let source =
            fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        if source.contains("checkbox-xs") {
            offenders.push(path.display().to_string());
        }
    }

    assert!(
        offenders.is_empty(),
        "checkbox-xs resolves to 16x16px in the pinned DaisyUI CSS, below WCAG \
         2.2's AA touch-target floor (2.5.8: 24x24) -- QA-015 removed every use; \
         use the bare `checkbox` class (24px) instead:\n{}",
        offenders
            .iter()
            .map(|o| format!("  {o}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// The six sizing classes `TT-001`'s survey found below 44px and
/// `TT-002` made every site of compose `TOUCH_TARGET`.
const SIZING_CLASSES: [&str; 6] = [
    "btn-sm",
    "btn-xs",
    "input-sm",
    "input-xs",
    "select-sm",
    "select-xs",
];

/// Every `"..."` string-literal's content span in `source` —
/// `(quote_pos, content_start, content_end)`, where `quote_pos` is the
/// byte offset of the opening `"` and `content_start`/`content_end`
/// bound the text between the quotes. Handles `\"` escapes so an
/// escaped quote can't prematurely close a span; does not handle raw
/// strings, byte strings, or any other Rust string form (module doc's
/// named limit). All three offsets land on `"` or `\` byte positions,
/// both single-byte ASCII, so slicing `source` at them is always a
/// valid UTF-8 boundary regardless of non-ASCII content inside a
/// literal.
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

/// True if the string literal whose opening quote sits at `quote_pos`
/// in `source` is the sole argument of a `grow(...)` call — i.e. the
/// text immediately before the quote, whitespace trimmed, ends with
/// `grow(`. This is `components::grow`'s exact call shape
/// (`class=grow("...")`); `TT-002` never composes a sizing class any
/// other way.
fn is_grow_call_argument(source: &str, quote_pos: usize) -> bool {
    source[..quote_pos].trim_end().ends_with("grow(")
}

/// True if `content` (a string literal's contents) carries one of
/// [`SIZING_CLASSES`] as a whole space-separated token — an HTML
/// `class` attribute's own delimiter, so this needs no word-boundary
/// regex the way free text would: `"btn-small"` splits to one token,
/// `"btn-small"`, which is never equal to `"btn-sm"`.
fn carries_a_sizing_class(content: &str) -> bool {
    content
        .split_whitespace()
        .any(|token| SIZING_CLASSES.contains(&token))
}

#[test]
fn every_sizing_class_site_composes_the_touch_target() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let components_dir = manifest_dir.join("src").join("components");
    let mut files = Vec::new();
    collect_rs_files(&components_dir, &mut files);
    assert!(
        !files.is_empty(),
        "found no .rs files under src/components/ -- the workspace layout \
         assumption this scan depends on may have changed"
    );

    let mut offenders = Vec::new();
    for path in &files {
        let source =
            fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        for (quote_pos, content_start, content_end) in quoted_string_spans(&source) {
            let content = &source[content_start..content_end];
            if carries_a_sizing_class(content) && !is_grow_call_argument(&source, quote_pos) {
                let line = source[..quote_pos].matches('\n').count() + 1;
                offenders.push(format!(
                    "{}:{line}: {content:?} carries a sizing class but is not the \
                     argument of a components::grow(...) call",
                    path.display()
                ));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "every sizing-class site in src/components/ must reach a 44px target via \
         components::grow(...) (NFR-A11Y-007, DEC-049 as amended) -- TT-002 already \
         converted every site this scan found on the tree it shipped against, so a \
         new offender means either a new control shipped without grow() or an \
         existing one lost it:\n{}",
        offenders
            .iter()
            .map(|o| format!("  {o}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
