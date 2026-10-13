//! `GRD-001` — the one `strip_line_comments` the scan modules share.
//!
//! Six copies of this helper existed: five identical and naive
//! (`line.split("//").next()`, which cuts a line at the first literal
//! `//` even inside a real string literal) and one, `alignment_scan`'s
//! own, written independently in `0.51.0` and already correct — it
//! tracks quote state and backslash escapes and cuts only at a `//`
//! outside a string. Five component files carry
//! `xmlns="http://www.w3.org/2000/svg"`; every naive copy truncated
//! those lines mid-literal, leaving an unterminated quote that
//! misdirects every subsequent `"..."` span's pairing for the rest of
//! the file. This is `§10.36` in a guard `NFR-A11Y-007` depends on:
//! with quote-pairing corrupted, `touch_target_scan`'s answer for
//! those five files was not wrong, it was unrelated to the question,
//! and it reported clean either way.
//!
//! **This round's decision, `QA-002`'s duplication call reversed for
//! this one helper**: a straight-line helper may be duplicated safely
//! — `collect_rs_files`, `extract_hidden_field` read one thing and
//! return it, and a second copy is either right or obviously wrong.
//! This one carries state across a line and has edge cases, so six
//! copies were six chances to be wrong differently, and five shared
//! the one chance that was. A defect found in one copy does not reach
//! the others, and no single copy tells you how many siblings are
//! wrong. See `requirements.md`'s maintainability section for the
//! rule stated where it governs; `collect_rs_files` and
//! `extract_hidden_field` stay duplicated, this is not a blanket
//! "share every helper" reversal.
//!
//! Lifted verbatim from `alignment_scan.rs:81` — the reviewed,
//! correct body, not rewritten while moving it.

/// Strips `//`-style line comments, tracking quote state so a `//`
/// inside a real string literal (`xmlns="http://…"`) is not mistaken
/// for a comment start. Backslash-escaped quotes do not toggle
/// in-string state. Line-based: a `/* */` block comment is not
/// stripped, only `//` to end of line.
pub(crate) fn strip_line_comments(source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    for line in source.lines() {
        let bytes = line.as_bytes();
        let mut in_str = false;
        let mut i = 0;
        let mut cut = line.len();
        while i < bytes.len() {
            match bytes[i] {
                b'"' => in_str = !in_str,
                b'\\' if in_str && i + 1 < bytes.len() => i += 1,
                b'/' if !in_str && i + 1 < bytes.len() && bytes[i + 1] == b'/' => {
                    cut = i;
                    break;
                }
                _ => {}
            }
            i += 1;
        }
        out.push_str(&line[..cut]);
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests;
