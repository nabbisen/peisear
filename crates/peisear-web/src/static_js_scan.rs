//! `BOARD-001` (RFC 004b / D-2) — a guard over `static/*.js`, the
//! twin of `prose_scan.rs` for JavaScript instead of Rust. Exists
//! because `board.js` carried three user-visible English sentences,
//! authored as literal `var` assignments, for as long as this project
//! has had a vocabulary guard — `prose_scan` covers `components/` and
//! `handlers/` only, so `static/*.js` was never scanned. Not excluded
//! on purpose; simply unexamined.
//!
//! **Signature**: a quoted string literal with two or more
//! space-separated "word" tokens — a token that, after trimming
//! leading/trailing non-alphabetic characters, is at least two
//! characters and entirely ASCII-alphabetic. `BOARD-001-review.md`
//! §3: an earlier version of this guard required a `.` too, on the
//! theory that this codebase's copy is complete sentences — which
//! held for the three sentences it was built to catch, but not for
//! copy in general (`UndoButtonLabel` is `"Undo"`; a great deal of
//! this project's copy is a short label, not a sentence). Planting
//! `"Move to Done"` and `"Another member changed this issue first"`
//! (both period-less) proved the `.` requirement was hiding exactly
//! the more likely kind of violation — a button label typed straight
//! into a `.js` file — so it was dropped from the signature.
//!
//! The per-token trim is what survives contact with CSS class lists
//! and directives without a period requirement: `"toast toast-end
//! toast-bottom z-50"` has one qualifying token (`toast`; the
//! hyphenated ones keep their hyphen through the trim and so are not
//! "entirely alphabetic", and `z-50` trims down to the single
//! character `z`) — one word is below the two-word threshold, so it
//! passes. `"use strict"` is two genuine words and needed its own
//! exact-text exception (below) — the one JS pragma this codebase
//! reuses verbatim, not a general pattern. If a future string needs
//! more than that exception plus `search.js`'s file-level exclusion
//! to clear this guard, the heuristic is wrong, not the codebase
//! (§3's own calibration standard, same as `prose_scan`'s).
//!
//! **`search.js` is the one named exclusion** — RFC 006's standing
//! position since 0.21.0: it needs a JS-side rendering mechanism that
//! does not exist, and inventing one for a type-ahead dropdown is
//! disproportionate.
//!
//! **Comments are stripped before scanning**, same choice
//! `prose_scan.rs`/`test_harness_scan.rs` made and for the same
//! reason: otherwise a doc comment that quotes UI copy — this file's
//! own module doc above, or `dm.js`'s comment describing its "moved
//! to" announcement — would be mistaken for a real literal.

use std::fs;
use std::path::{Path, PathBuf};

/// Files under `static/` this guard does not scan, each with a
/// reason that is a decision, not "not converted yet" (`I18N-007`
/// §3's standard, carried over unchanged).
const ALLOWLIST: &[(&str, &str)] = &[(
    "search.js",
    "needs a JS-side rendering mechanism that does not exist yet -- \
     RFC 006's named, standing exclusion since 0.21.0; inventing one \
     for a type-ahead dropdown is disproportionate",
)];

fn is_allowlisted(filename: &str) -> bool {
    ALLOWLIST.iter().any(|(name, _)| *name == filename)
}

/// Strips `//`-style line comments. Same documented limitation as
/// `prose_scan.rs`/`test_harness_scan.rs`: line-based, does not
/// account for `//` inside a string literal -- no file in scope has
/// one today.
fn strip_line_comments(source: &str) -> String {
    source
        .lines()
        .map(|line| line.split("//").next().unwrap_or(""))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Byte offset (relative to `s`) of the first `quote` not preceded
/// by an odd number of backslashes. `quote` is always ASCII (`"` or
/// `'`), so this byte-level scan is UTF-8-safe: an ASCII byte value
/// never occurs as part of a multi-byte codepoint, and the position
/// right after it is always a valid `str` boundary.
fn find_unescaped_quote_end(s: &str, quote: u8) -> Option<usize> {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == quote {
            let mut backslashes = 0;
            let mut j = i;
            while j > 0 && bytes[j - 1] == b'\\' {
                backslashes += 1;
                j -= 1;
            }
            if backslashes % 2 == 0 {
                return Some(i);
            }
        }
        i += 1;
    }
    None
}

/// A whitespace-split token counts as a word if, after trimming
/// leading/trailing non-alphabetic characters (so `"state."`'s
/// trailing full stop, or a leading/trailing comma, doesn't disqualify
/// it), what remains is at least two characters and entirely
/// ASCII-alphabetic. A hyphen or digit *inside* the token (`toast-
/// end`, `z-50`) survives the trim and fails the all-alphabetic check,
/// which is what keeps CSS class lists and `data-*`/kebab-case tokens
/// from counting.
fn is_word_token(token: &str) -> bool {
    let core = token.trim_matches(|c: char| !c.is_ascii_alphabetic());
    core.len() >= 2 && core.chars().all(|c| c.is_ascii_alphabetic())
}

/// See the module doc for the calibration history and the survival
/// check against real content.
fn looks_like_prose(literal: &str) -> bool {
    if literal == "use strict" {
        // The ECMAScript strict-mode pragma: two genuine words, but
        // the one fixed phrase every file in scope repeats verbatim
        // in the same bare-statement position, never as copy.
        return false;
    }
    literal
        .split_whitespace()
        .filter(|t| is_word_token(t))
        .count()
        >= 2
}

#[derive(Debug)]
struct Violation {
    file: String,
    line: usize,
    text: String,
}

fn line_of(haystack: &str, byte_idx: usize) -> usize {
    1 + haystack[..byte_idx].matches('\n').count()
}

/// One left-to-right pass so a `'…"…"…'`-shaped literal (a
/// single-quoted string containing double quotes, e.g. `dm.js`'s
/// `'button[name="status"]'`) is read as the one string it is,
/// rather than two independent quote-type scans misreading the inner
/// `"` as its own literal's boundary.
fn scan_string_literals(stripped: &str, rel_path: &str, out: &mut Vec<Violation>) {
    let bytes = stripped.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'"' || b == b'\'' {
            let content_start = i + 1;
            match find_unescaped_quote_end(&stripped[content_start..], b) {
                Some(end_rel) => {
                    let literal = &stripped[content_start..content_start + end_rel];
                    if looks_like_prose(literal) {
                        out.push(Violation {
                            file: rel_path.to_string(),
                            line: line_of(stripped, i),
                            text: literal.to_string(),
                        });
                    }
                    i = content_start + end_rel + 1;
                    continue;
                }
                None => {
                    i += 1;
                    continue;
                }
            }
        }
        i += 1;
    }
}

fn all_static_js_files() -> Vec<(PathBuf, String)> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let static_dir = manifest_dir.join("..").join("..").join("static");
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir(&static_dir) else {
        panic!("read static dir {}", static_dir.display());
    };
    for entry in entries {
        let path = entry.expect("dir entry").path();
        if path.extension().is_some_and(|e| e == "js") {
            let filename = path
                .file_name()
                .expect("file name")
                .to_string_lossy()
                .to_string();
            out.push((path, filename));
        }
    }
    out
}

fn all_violations() -> Vec<Violation> {
    let files = all_static_js_files();
    let mut violations = Vec::new();
    for (path, filename) in &files {
        if is_allowlisted(filename) {
            continue;
        }
        let source =
            fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        let stripped = strip_line_comments(&source);
        scan_string_literals(&stripped, filename, &mut violations);
    }
    violations
}

#[test]
fn no_hardcoded_prose_in_static_js_outside_the_message_table() {
    let files = all_static_js_files();
    assert!(
        !files.is_empty(),
        "found no static/*.js files -- the workspace layout assumption \
         this scan depends on may have changed"
    );

    let violations = all_violations();
    assert!(
        violations.is_empty(),
        "user-visible prose found in static/*.js (route it through peisear_i18n::MessageKey \
         and a JSON island, `dm.js`'s pattern, or add a reasoned allowlist entry in \
         static_js_scan.rs):\n{}",
        violations
            .iter()
            .map(|v| format!("  {}:{} {:?}", v.file, v.line, v.text))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// `search.js` genuinely has string literals this guard would flag
/// if it weren't allowlisted -- proves the allowlist entry excludes a
/// real file with real matches, not a name that no longer needs it.
#[test]
fn search_js_allowlist_entry_still_matches_something() {
    let files = all_static_js_files();
    let (path, _) = files
        .iter()
        .find(|(_, name)| name == "search.js")
        .expect("search.js present under static/");
    let source = fs::read_to_string(path).expect("read search.js");
    let stripped = strip_line_comments(&source);
    let mut violations = Vec::new();
    scan_string_literals(&stripped, "search.js", &mut violations);
    assert!(
        !violations.is_empty(),
        "search.js no longer has anything this guard would flag -- if it was converted, \
         remove the allowlist entry and let this guard cover it"
    );
}
