//! `CSS-002` (unit two, `059`, `§10.52`'s own unfinished half) —
//! `CSS-001` fixed the stylesheet and both of its documented causes
//! (`style/tailwindcss/tailwind.config.js`'s content glob, this crate's
//! own build-output path) but left the relationship between a class a
//! component writes and a rule the shipped stylesheet serves entirely
//! unchecked: `cargo build` runs nothing in `style/tailwindcss/`
//! (`DEC-051`, correctly — a self-hoster needs no Node), so nothing
//! compares what `crates/peisear-web/src/components/**/*.rs` asks for
//! against what `crates/peisear-web/static/{tailwind.css,daisyui.min.css,
//! app.css}` actually serves. `static/*.js` has three guards
//! (`static_js_scan`, `undo_toast_attachment_scan`, three browser gates);
//! the CSS had a `style/tailwindcss/README.md` sentence that was true,
//! correct and useless for three weeks (`ASSET-001` → `FIV-007` →
//! `CSS-001`).
//!
//! **The rule, one sentence**: every class token a component writes
//! must have a rule in one of the three stylesheets the binary serves,
//! or be named in [`JS_HOOK_ALLOWLIST`].
//!
//! ## Where the tokens come from — bounded deliberately
//!
//! - Files: `crates/peisear-web/src/components/**/*.rs` only.
//! - Literals: `class="…"`, `grow("…")` and `grow_text("…")` —
//!   including through an `if`/`else` whose whole value is the call's
//!   sole argument ([`enclosing_call_text`]), the shape every affected
//!   call site in this crate actually uses (`issues.rs`'s
//!   `board_classes`/`list_classes`/two `cls` bindings, `calendar.rs`'s
//!   view-selector `class`).
//! - A token containing `{` or `}` is skipped, not flagged — it is
//!   interpolated and this scan cannot know its value.
//! - Rust line-continuations (`\` at the end of a line inside a
//!   string) are joined, per Rust's own string-literal escape rule,
//!   before splitting on whitespace ([`join_line_continuations`]).
//!   **Load-bearing**: skipping this reads the backslash itself as a
//!   class token and misses every token on the continued line — real
//!   call sites in this tree span a line exactly this way
//!   (`breadcrumb.rs`, `calendar.rs`, `me.rs`, `teams.rs`,
//!   `notification_preferences.rs`).
//!
//! ## Where the rules come from
//!
//! `crates/peisear-web/static/tailwind.css`, `daisyui.min.css` and
//! `app.css`, read from `CARGO_MANIFEST_DIR` — the same resolution
//! `static_js_scan`/`dm_fallback_boundary_scan` already use, and the
//! whole reason they survived `FIV-007` while `tailwind.config.js` and
//! this crate's own build-output path did not (`CSS-001`). **Never
//! resolved from the repo root.**
//!
//! A token is present if its escaped selector (`.md\:flex`, `.w-1\/2`,
//! `.text-\[11px\]`) is followed by a selector or declaration boundary
//! — `{ , : > + ~ [ ) .` or whitespace — so `.btn` does not match
//! `.btn-ghost` ([`token_has_rule`]).
//!
//! ## The allowlist
//!
//! [`JS_HOOK_ALLOWLIST`]: two selectors `static/*.js` queries directly
//! and neither intended to carry style — `issue-card` (`board.js`'s
//! drag-handle lookup) and `js-status-form` (`dm.js`'s
//! submit-on-select wiring). **An allowlist entry that nothing queries
//! is itself a defect**:
//! [`tests::every_allowlist_entry_is_queried_by_some_static_js`]
//! asserts each name is still the target of a live `querySelectorAll`
//! call in some `static/*.js` file.
//!
//! ## Bounds — what this scan does not see
//!
//! 1. **Classes built by `format!`.** `format!("badge-{}", x)` never
//!    matches the three literal shapes above — the scan finds no
//!    literal at all for that call, and so cannot check it.
//! 2. **`class:foo=` bindings** (Leptos's own toggle-class attribute
//!    syntax) — not one of the three shapes, not scanned.
//! 3. **Classes returned as strings from other crates.** `peisear_core`
//!    returns DaisyUI badge classes as plain `&'static str`
//!    (`Priority::badge_class`, `WorkloadState::badge_class`,
//!    `DisplayHealthState::badge_class`) — exactly why
//!    `tailwind.config.js`'s own content glob covers `crates/**/*.rs`,
//!    not `crates/peisear-web/**` alone. This scan's bound is
//!    narrower than the real build's.
//! 4. **Classes written in `static/*.js`.** This scan reads `.rs`
//!    files only.
//! 5. **`grow(CONST)` / `grow_text(CONST)` call sites.** A bare
//!    identifier argument carries no string literal for this scan to
//!    find — `layout.rs`'s own `grow_text(MENU_FOCUS_RING)` sites
//!    contribute nothing to this scan's population, which is a named
//!    limit, not an oversight:
//!    [`tests::a_const_argument_contributes_no_tokens`] pins it.
//!
//! A guard that implies more reach than it has is `§10.36`'s own class
//! of defect, and this project has paid for it before
//! (`touch_target_scan`'s own module doc, `TT-001`).
//!
//! ## What this round does not do
//!
//! No new CI job — `test.yml`'s existing `peisear-web --lib` block
//! runs this. No regeneration of the stylesheet, and no edit to
//! `style/tailwindcss/` — `CSS-001` left those correct. No widening to
//! `handlers/**` or to the other crates — a wider bound is a separate,
//! separately-measured decision.

use std::fs;
use std::path::{Path, PathBuf};

use crate::scan_support::strip_line_comments;

/// Class selectors `static/*.js` queries directly and that carry no
/// style of their own — each entry names the file and call that
/// queries it, so a stale entry has something to point back at.
const JS_HOOK_ALLOWLIST: &[(&str, &str)] = &[
    (
        "issue-card",
        "static/board.js's `.issue-card` drag-handle lookup",
    ),
    (
        "js-status-form",
        "static/dm.js's `.js-status-form` submit-on-select wiring",
    ),
];

fn is_allowlisted(token: &str) -> bool {
    JS_HOOK_ALLOWLIST.iter().any(|(name, _)| *name == token)
}

/// Every `.rs` file under `dir`, recursively — the same walk
/// `contrast_scan`/`touch_target_scan` use, duplicated rather than
/// shared (`scan_support`'s own doc comment: a straight-line helper
/// may be duplicated safely).
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

fn components_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("components")
}

fn static_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("static")
}

/// The three stylesheets the binary serves, concatenated — the same
/// `CARGO_MANIFEST_DIR` resolution `static_js_scan`/
/// `dm_fallback_boundary_scan` already use. Panics (not `Result`) on a
/// missing file, the established shape for a scan whose own
/// population must never silently read as zero (`§10.36`).
fn stylesheet_text() -> String {
    let dir = static_dir();
    let mut out = String::new();
    for name in ["tailwind.css", "daisyui.min.css", "app.css"] {
        let path = dir.join(name);
        let content =
            fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        out.push_str(&content);
        out.push('\n');
    }
    out
}

/// Every `"..."` string-literal's content span in `source` —
/// `(quote_pos, content_start, content_end)`. Identical in shape to
/// `touch_target_scan::quoted_string_spans`: handles `\"` escapes,
/// does not attempt raw strings or any other Rust string form.
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

/// The text, trimmed, enclosing the nearest `(` that syntactically
/// contains the string literal whose opening quote sits at
/// `quote_pos` — e.g. for `grow(if cond { "a" } else { "b" })`, both
/// `"a"`'s and `"b"`'s calls resolve to a text ending in `grow`. Walks
/// backward tracking bracket depth: `)`/`}`/`]` open a skip region,
/// `(`/`{`/`[` closes one (transparent at depth 0, since an
/// `if`/`else` block's own boundary says nothing about what encloses
/// it) — the same shape `touch_target_scan::is_grow_call_argument`
/// already proved for this exact case.
fn enclosing_call_text(source: &str, quote_pos: usize) -> Option<&str> {
    let bytes = source.as_bytes();
    let mut depth: i32 = 0;
    let mut i = quote_pos;
    while i > 0 {
        i -= 1;
        match bytes[i] {
            b')' | b'}' | b']' => depth += 1,
            b'(' => {
                if depth == 0 {
                    return Some(source[..i].trim_end());
                }
                depth -= 1;
            }
            b'{' | b'[' if depth > 0 => depth -= 1,
            _ => {}
        }
    }
    None
}

/// True if the string literal whose opening quote sits at `quote_pos`
/// is one of this scan's three token sources: a `class=` attribute
/// value, or the (possibly `if`/`else`-wrapped) sole argument of a
/// `grow(...)` or `grow_text(...)` call.
fn is_token_source(source: &str, quote_pos: usize) -> bool {
    if source[..quote_pos].trim_end().ends_with("class=") {
        return true;
    }
    match enclosing_call_text(source, quote_pos) {
        Some(before) => before.ends_with("grow") || before.ends_with("grow_text"),
        None => false,
    }
}

/// Rust string-literal line-continuation semantics: a `\` immediately
/// followed by a newline (optionally preceded by `\r`) removes the
/// backslash, the line ending, and every whitespace character that
/// follows up to the next non-whitespace one. Must run before
/// splitting on whitespace — skipping it reads the backslash itself as
/// a class token and misses every token on the continued line.
fn join_line_continuations(content: &str) -> String {
    let bytes = content.as_bytes();
    let mut out = String::with_capacity(content.len());
    let mut copy_start = 0;
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\\' {
            let mut j = i + 1;
            if j < bytes.len() && bytes[j] == b'\r' {
                j += 1;
            }
            if j < bytes.len() && bytes[j] == b'\n' {
                j += 1;
                while j < bytes.len() && (bytes[j] as char).is_whitespace() {
                    j += 1;
                }
                out.push_str(&content[copy_start..i]);
                copy_start = j;
                i = j;
                continue;
            }
        }
        i += 1;
    }
    out.push_str(&content[copy_start..]);
    out
}

/// Every whitespace-separated token this scan will check from one
/// token-source literal's content — line continuations joined first,
/// then any token carrying `{` or `}` dropped (interpolated, value
/// unknowable to this scan).
fn tokens_from_content(content: &str) -> Vec<String> {
    join_line_continuations(content)
        .split_whitespace()
        .filter(|t| !t.contains('{') && !t.contains('}'))
        .map(str::to_string)
        .collect()
}

/// CSS's own selector-escaping rule for a class token, empirically
/// matched against this project's shipped, pinned stylesheets
/// (`.md\:flex`, `.mt-0\.5`, `.max-w-\[12rem\]`,
/// `.focus-visible\:\!outline`, `.bg-primary\/10`): every byte that is
/// not an ASCII letter, digit, `-` or `_` is preceded by a backslash.
fn escape_css_token(token: &str) -> String {
    let mut out = String::with_capacity(token.len() * 2);
    for c in token.chars() {
        if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
            out.push(c);
        } else {
            out.push('\\');
            out.push(c);
        }
    }
    out
}

/// True if `token`'s escaped selector form appears in `stylesheet`,
/// followed by a selector or declaration boundary (`{ , : > + ~ [ ) .`
/// or whitespace) — so `.btn` does not match inside `.btn-ghost`.
/// Scans forward past boundary-failing occurrences rather than
/// stopping at the first substring hit.
fn token_has_rule(token: &str, stylesheet: &str) -> bool {
    let needle = format!(".{}", escape_css_token(token));
    let bytes = stylesheet.as_bytes();
    let mut search_from = 0;
    while let Some(rel) = stylesheet[search_from..].find(needle.as_str()) {
        let abs = search_from + rel;
        let after = abs + needle.len();
        let boundary_ok = match bytes.get(after) {
            None => true,
            Some(&b) => {
                matches!(
                    b,
                    b'{' | b',' | b':' | b'>' | b'+' | b'~' | b'[' | b')' | b'.'
                ) || (b as char).is_whitespace()
            }
        };
        if boundary_ok {
            return true;
        }
        search_from = abs + 1;
    }
    false
}

/// One class token examined by this scan and found with no rule in
/// any of the three stylesheets, nor in [`JS_HOOK_ALLOWLIST`].
#[derive(Debug, Clone)]
struct Violation {
    file: String,
    token: String,
}

/// This scan's full result: how many **distinct** class tokens it
/// checked against the stylesheets (`population`), how many distinct
/// tokens were recognised as JS hooks instead (`allowlisted`), and the
/// unexplained ones (empty on a satisfying tree). Deduplicated,
/// because the same token is written at dozens of call sites and
/// checking it once per site measures call-site count, not the
/// population this rule is actually about. `population` is returned
/// explicitly, not implied by an empty `violations` list — a scan
/// that silently examined zero files must not read as passing
/// (`§10.36`).
struct ScanResult {
    population: usize,
    allowlisted: usize,
    violations: Vec<Violation>,
}

fn run_scan() -> ScanResult {
    let stylesheet = stylesheet_text();
    let dir = components_dir();
    let mut files = Vec::new();
    collect_rs_files(&dir, &mut files);

    // token -> the first file it was found in, for the violation
    // message; the token itself is what's deduplicated.
    let mut seen: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();

    for path in &files {
        let raw =
            fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        let stripped = strip_line_comments(&raw);
        let display = path.display().to_string();
        for (quote_pos, content_start, content_end) in quoted_string_spans(&stripped) {
            if !is_token_source(&stripped, quote_pos) {
                continue;
            }
            for token in tokens_from_content(&stripped[content_start..content_end]) {
                seen.entry(token).or_insert_with(|| display.clone());
            }
        }
    }

    let mut population = 0;
    let mut allowlisted = 0;
    let mut violations = Vec::new();
    for (token, file) in &seen {
        if is_allowlisted(token) {
            allowlisted += 1;
            continue;
        }
        population += 1;
        if !token_has_rule(token, &stylesheet) {
            violations.push(Violation {
                file: file.clone(),
                token: token.clone(),
            });
        }
    }

    ScanResult {
        population,
        allowlisted,
        violations,
    }
}

/// True if `haystack` contains `.{name}` as a whole selector token —
/// not merely as a substring. `.js-status-form` is itself a valid
/// prefix of `.js-status-form-renamed` (`-` is a legal class-name
/// character), so a bare `.contains` would stay green after exactly
/// the rename this allowlist assertion exists to catch; caught
/// planting that rename during this round, the same shape
/// `token_has_rule`'s own boundary check exists for.
fn contains_whole_selector(haystack: &str, name: &str) -> bool {
    let needle = format!(".{name}");
    let mut search_from = 0;
    while let Some(rel) = haystack[search_from..].find(needle.as_str()) {
        let abs = search_from + rel;
        let after = abs + needle.len();
        let boundary_ok = match haystack.as_bytes().get(after) {
            None => true,
            Some(&b) => !(b.is_ascii_alphanumeric() || b == b'-' || b == b'_'),
        };
        if boundary_ok {
            return true;
        }
        search_from = abs + 1;
    }
    false
}

fn static_js_files() -> Vec<PathBuf> {
    let dir = static_dir();
    let Ok(entries) = fs::read_dir(&dir) else {
        return Vec::new();
    };
    entries
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "js"))
        .collect()
}

#[cfg(test)]
mod tests;
