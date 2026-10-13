//! I18N-007 §3 — the completion claim replaced by a test.
//!
//! Four handoffs each declared their surface complete and each was
//! followed by another find (`I18N-006-review.md` §4). This walks
//! every `.rs` file under `src/` at test time and fails on hardcoded
//! user-visible prose, so "every string is in the table" stops
//! depending on anyone's thoroughness.
//!
//! **`QA-010` §4 — widened from `src/components/**` and
//! `src/handlers/**` to all of `src/`.** Those two were the directories
//! actually found to carry copy, but the list itself was never
//! checked against anything: `components.rs`, `handlers.rs`,
//! `error.rs`, `app.rs`, `extractors.rs`, `jobs.rs`, `state.rs` and
//! `config.rs` sat unscanned, and planting
//! `r#"<span aria-label="This sprint has no planned work">"#` into
//! `components.rs` — one level up from the directory this scan used to
//! walk — passed clean (`evidence/section4-plant-components-rs-level-up.log`
//! in the `QA-010` review request). **Measured before choosing**:
//! widened to all of `src/` first and re-ran against the real tree —
//! **zero new hits**, even across the fourteen additional files
//! (`evidence/section4-widen-to-all-src-hit-count.log`). Chosen over
//! the alternative — keep the two-directory scope and add a second,
//! smaller guard asserting nothing outside it contains a
//! [`SCOPED_ATTRS`] literal — because a scan that simply covers every
//! file is easier to explain and maintain than a scan plus a
//! meta-scan of the scan's own scope, and at zero hit cost there is
//! nothing the narrower version was buying. Also closes the gap for
//! good: any future top-level module under `src/` is covered
//! automatically, with no one needing to remember to widen an
//! allowlist again.
//!
//! Scope, deliberately narrow on *what counts as prose* even though it
//! is no longer narrow on *which files it reads* (`I18N-007` §3's own
//! calibration standard — "if the test needs more than a handful of
//! allowlist entries, the heuristic is wrong, not the codebase"):
//!
//! - A literal `="…"` value on one of [`SCOPED_ATTRS`] — the
//!   attributes actually found to carry copy in this crate.
//!   `class`/`href`/`name`/`value`/`role`/the SVG presentation
//!   attributes and friends are excluded by construction: they are
//!   simply not in that list, not filtered out of a broader one.
//! - A bare quoted text node directly between two tag delimiters
//!   inside a `view!` body — `>"…"<`, or a line that is nothing but
//!   `"…"` with a tag-close on the line before and a tag-open on the
//!   line after.
//!
//! **Known blind spot, evaluated and not closed**: copy assembled by
//! a `format!()`/`match` arm into an intermediate `String` binding
//! before it ever reaches template markup (the exact shape
//! `render_trend_chip` and `/me`'s load/throughput chips had before
//! `I18N-007` converted them) is invisible to this scan — the
//! literal never appears in an attribute or text-node position. A
//! blanket "any `format!` literal with two-plus letters outside
//! `{}`" filter was tried against this tree and rejected: past the
//! two categories this test already allowlists, it additionally
//! flagged twelve non-copy sites (seven `"badge badge-sm {}"`-style
//! CSS class constructions, five `"channel__{k}__{chan}"`-style form
//! field name constructions) — exactly the "more than a handful"
//! signal `I18N-007` §3 says means the heuristic is wrong. Recorded
//! here rather than force-fit into the allowlist.

use std::fs;
use std::path::{Path, PathBuf};

use crate::scan_support::strip_line_comments;

/// Attribute names this scan treats as copy-bearing. `onsubmit` is
/// here because of the nine `confirm('…')` dialogs, not because
/// `onsubmit` is copy-bearing in general — see [`ALLOWLIST`].
const SCOPED_ATTRS: [&str; 4] = ["aria-label", "title", "placeholder", "onsubmit"];

#[derive(Debug)]
struct Violation {
    file: String,
    line: usize,
    kind: String,
    text: String,
}

struct AllowEntry {
    file: &'static str,
    snippet: &'static str,
    /// Read by humans reviewing this file, not by the scan itself --
    /// `every_allowlist_entry_still_matches_something` is what keeps
    /// entries honest at test time.
    #[allow(dead_code)]
    reason: &'static str,
}

/// Every entry needs a reason that is a decision, not "not converted
/// yet" (`I18N-007` §3).
const ALLOWLIST: &[AllowEntry] = &[
    AllowEntry {
        file: "components/teams.rs",
        snippet: "Detach this project from the team?",
        reason: "onsubmit confirm() dialog -- reversible action, RFC 010 open question 1 settled: keeps its confirm() dialog unchanged (CONF-001 §1's out-of-scope five)",
    },
    AllowEntry {
        file: "components/teams.rs",
        snippet: "Leave this team?",
        reason: "onsubmit confirm() dialog -- same disposition as the teams.rs entry above",
    },
    AllowEntry {
        file: "components/teams.rs",
        snippet: "Remove this member from the team?",
        reason: "onsubmit confirm() dialog -- same disposition as the teams.rs entry above",
    },
    AllowEntry {
        file: "components/settings.rs",
        snippet: "Remove this capacity row?",
        reason: "onsubmit confirm() dialog -- same disposition as the teams.rs entry above",
    },
    AllowEntry {
        file: "components/notification_preferences.rs",
        snippet: "Silence all notification kinds?",
        reason: "onsubmit confirm() dialog -- reversible since INBOX-001's resume banner (0.24.0); CONF-001 §1 leaves it unchanged, same disposition as the teams.rs entry above",
    },
];

fn is_allowlisted(v: &Violation) -> bool {
    ALLOWLIST
        .iter()
        .any(|a| v.file.ends_with(a.file) && v.text.contains(a.snippet))
}

/// Best-effort removal of `#[cfg(test)]`-attributed item bodies so
/// test-only code never trips the scan. Brace-counts without
/// skipping over string literals, so a `{`/`}` embedded in a string
/// inside a `#[cfg(test)]` block could throw the count off -- no
/// file in scope has one today; this is forward cover, not a
/// guarantee.
///
/// `I18N-007-review.md` §3.1: a brace-less item (`use`/`const`/
/// `static`/`type`, terminated by `;` rather than a `{...}` body)
/// used to blind the scan for the *entire rest of the file* --
/// `after.find('{')` would walk past the marker's own item and
/// match some unrelated brace much further down, swallowing every
/// real attribute/text-node literal in between. Demonstrated in
/// review by planting `#[cfg(test)] use std::fmt as _unused_fmt;`
/// above an already-caught literal; the scan passed, silently. Fixed
/// by checking whether `;` or `{` comes first after the marker: `;`
/// first means a brace-less item, and the item ends there.
fn strip_cfg_test_blocks(src: &str) -> String {
    const MARKER: &str = "#[cfg(test)]";
    let mut result = String::with_capacity(src.len());
    let mut rest = src;
    while let Some(idx) = rest.find(MARKER) {
        result.push_str(&rest[..idx]);
        let after = &rest[idx..];
        let brace_pos = after.find('{');
        let semi_pos = after.find(';');
        let end = match (brace_pos, semi_pos) {
            (Some(b), Some(s)) if s < b => s + 1,
            (Some(b), _) => {
                let bytes = after.as_bytes();
                let mut depth = 0usize;
                let mut i = b;
                let mut end = after.len();
                while i < bytes.len() {
                    match bytes[i] {
                        b'{' => depth += 1,
                        b'}' => {
                            depth -= 1;
                            if depth == 0 {
                                end = i + 1;
                                break;
                            }
                        }
                        _ => {}
                    }
                    i += 1;
                }
                end
            }
            (None, Some(s)) => s + 1,
            (None, None) => MARKER.len(),
        };
        rest = &after[end..];
    }
    result.push_str(rest);
    result
}

/// Byte offset (relative to `s`) of the first `"` not preceded by an
/// odd number of backslashes.
fn find_unescaped_quote_end(s: &str) -> Option<usize> {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'"' {
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

/// True if, after discarding anything inside `{...}`, at least two
/// consecutive ASCII letters remain -- the proxy for "contains a
/// word" used throughout this scan.
fn has_alpha_word_outside_braces(s: &str) -> bool {
    let mut outside = String::with_capacity(s.len());
    let mut depth = 0u32;
    for c in s.chars() {
        match c {
            '{' => depth += 1,
            '}' => depth = depth.saturating_sub(1),
            _ if depth == 0 => outside.push(c),
            _ => {}
        }
    }
    let mut run = 0;
    for c in outside.chars() {
        if c.is_ascii_alphabetic() {
            run += 1;
            if run >= 2 {
                return true;
            }
        } else {
            run = 0;
        }
    }
    false
}

fn looks_like_percent_pattern(s: &str) -> bool {
    s.contains('%')
}

/// A line that is nothing but one balanced `{expr}` block -- e.g.
/// `{t(MessageKey::Something)}` on its own line. `I18N-007-review.md`
/// §3.2: a standalone text node sitting between two such expression
/// children (fragment composition -- a literal connector between two
/// converted fragments) has neither a tag-close before it nor a
/// tag-open after it, so the original tag-only boundary check missed
/// it. Verified this addition does not reintroduce either
/// false-positive family the tag-boundary check was built to
/// exclude: the six CSS-class match arms (`if is_board {` / `} else
/// {`) neither start with `{` nor end with `}` as a whole trimmed
/// line, and `root.rs`'s lone `}` closing `health()` is one
/// character, not a `{...}` pair.
fn looks_like_complete_expression(l: &str) -> bool {
    l.len() >= 2 && l.starts_with('{') && l.ends_with('}')
}

fn line_of(haystack: &str, byte_idx: usize) -> usize {
    1 + haystack[..byte_idx].matches('\n').count()
}

fn scan_attrs(stripped: &str, rel_path: &str, out: &mut Vec<Violation>) {
    for attr in SCOPED_ATTRS {
        let pattern = format!("{attr}=\"");
        let mut search_from = 0;
        while let Some(rel_idx) = stripped[search_from..].find(&pattern) {
            let idx = search_from + rel_idx;
            let boundary_ok = idx == 0
                || !matches!(
                    stripped[..idx].chars().next_back(),
                    Some(c) if c.is_alphanumeric() || c == '-' || c == '_'
                );
            let content_start = idx + pattern.len();
            if boundary_ok
                && let Some(end_rel) = find_unescaped_quote_end(&stripped[content_start..])
            {
                let literal = &stripped[content_start..content_start + end_rel];
                if has_alpha_word_outside_braces(literal) && !looks_like_percent_pattern(literal) {
                    out.push(Violation {
                        file: rel_path.to_string(),
                        line: line_of(stripped, idx),
                        kind: format!("attr:{attr}"),
                        text: literal.to_string(),
                    });
                }
            }
            search_from = content_start;
        }
    }
}

fn scan_inline_text_nodes(stripped: &str, rel_path: &str, out: &mut Vec<Violation>) {
    let mut search_from = 0;
    while let Some(rel_idx) = stripped[search_from..].find(">\"") {
        let idx = search_from + rel_idx;
        let content_start = idx + 2;
        if let Some(end_rel) = find_unescaped_quote_end(&stripped[content_start..]) {
            let literal = &stripped[content_start..content_start + end_rel];
            let after = &stripped[content_start + end_rel + 1..];
            if after.starts_with('<') && has_alpha_word_outside_braces(literal) {
                out.push(Violation {
                    file: rel_path.to_string(),
                    line: line_of(stripped, idx),
                    kind: "text-node-inline".to_string(),
                    text: literal.to_string(),
                });
            }
            search_from = content_start + end_rel + 1;
        } else {
            search_from = content_start;
        }
    }
}

fn scan_standalone_text_nodes(stripped: &str, rel_path: &str, out: &mut Vec<Violation>) {
    let lines: Vec<&str> = stripped.split('\n').collect();
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.len() < 2 || !trimmed.starts_with('"') || !trimmed.ends_with('"') {
            continue;
        }
        let inner = &trimmed[1..trimmed.len() - 1];
        if inner.contains('"') {
            continue; // not a single simple literal
        }
        if !has_alpha_word_outside_braces(inner) || looks_like_percent_pattern(inner) {
            continue;
        }
        // A genuine view! child either has a tag closing just before
        // it and opening just after (`"Committed"` between
        // `<span>`/`</span>`), or sits between two complete `{expr}`
        // template expressions (fragment composition -- see
        // `looks_like_complete_expression`'s doc comment). Either
        // shape distinguishes it from a standalone format!()/
        // match-arm literal, whose neighbours are ordinary Rust
        // syntax instead.
        let prev = lines[..i]
            .iter()
            .rev()
            .map(|l| l.trim())
            .find(|l| !l.is_empty());
        let next = lines[i + 1..]
            .iter()
            .map(|l| l.trim())
            .find(|l| !l.is_empty());
        let prev_ok = prev.is_some_and(|l| l.ends_with('>') || looks_like_complete_expression(l));
        let next_ok = next.is_some_and(|l| l.starts_with('<') || looks_like_complete_expression(l));
        if prev_ok && next_ok {
            out.push(Violation {
                file: rel_path.to_string(),
                line: i + 1,
                kind: "text-node-line".to_string(),
                text: inner.to_string(),
            });
        }
    }
}

fn scan_file(path: &Path, rel_path: &str) -> Vec<Violation> {
    let content = fs::read_to_string(path).unwrap_or_default();
    let without_comments = strip_line_comments(&content);
    let stripped = strip_cfg_test_blocks(&without_comments);
    let mut out = Vec::new();
    scan_attrs(&stripped, rel_path, &mut out);
    scan_inline_text_nodes(&stripped, rel_path, &mut out);
    scan_standalone_text_nodes(&stripped, rel_path, &mut out);
    out
}

fn collect_rs_files(dir: &Path, root: &Path, out: &mut Vec<(PathBuf, String)>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries {
        let entry = entry.expect("dir entry");
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, root, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            let rel = path
                .strip_prefix(root)
                .expect("path under root")
                .to_string_lossy()
                .replace('\\', "/");
            out.push((path, rel));
        }
    }
}

fn all_violations() -> Vec<Violation> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let src_dir = manifest_dir.join("src");
    let mut files = Vec::new();
    collect_rs_files(&src_dir, &src_dir, &mut files);

    let mut violations = Vec::new();
    for (path, rel) in &files {
        violations.extend(scan_file(path, rel));
    }
    violations
}

#[cfg(test)]
mod tests;
