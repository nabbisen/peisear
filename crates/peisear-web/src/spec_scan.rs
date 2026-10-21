//! The first mechanical reader `requirements.md` has ever had (`§10.42`,
//! `SPEC-001`). Two rules, following `docs_scan`'s own shape: a pure
//! [`violations`] function over an [`Input`], so each rule is plantable
//! without touching a real file.
//!
//! 1. **Every `§10.N` citation in a maintained document resolves to a
//!    real `### 10.N` heading in `requirements.md`.** "Maintained"
//!    means `docs/`, excluding the frozen `docs/specification/history/`
//!    snapshots -- the same exclusion `docs_scan`'s own rule 5 makes,
//!    for the same reason (a closed historical document is not rewritten
//!    to satisfy a guard written after it). **`rfcs/` is out of this
//!    rule's scope entirely**, not merely excluded from one directory
//!    within it: a closed handoff records what was believed when it was
//!    written (`[0.55.0]`'s own changelog section is the same
//!    principle, `0.53.0`'s precedent), and a dangling citation inside
//!    one is not a live defect to fix, only a historical fact to count.
//!    `SPEC-001` §3's own exit condition is that count, not a passing
//!    assertion over `rfcs/` -- see
//!    [`tests::rfcs_citations_to_the_four_numbers_are_counted_not_enforced`].
//! 2. **No four-digit, zero-leading `RFC 0` + three more digits
//!    reference outside `docs/specification/history/`.** This
//!    project's own live citation form pads its own small number to
//!    three digits, not four with a leading zero; the four-digit form
//!    is `docs/specification/history/`'s own, frozen, citation style
//!    for the pre-`v2.1` specification documents it preserves. Same
//!    population as rule 1's own documents,
//!    plus every published doc comment (`///`/`//!`) across
//!    `crates/*/src/` -- `docs_scan` rule 5's own reason for including
//!    doc comments in its population applies here too: a public, shipped
//!    comment is part of what a reader of this project's own published
//!    crates sees, the same specification surface in a different file
//!    format.
//!
//! **Both walk their own population**, not `docs_scan`'s: the two rules'
//! scope differs just enough from rule 5's own (`rfcs/` entirely absent
//! here, present with its own exclusions there) that sharing the walker
//! would need its own parameter for the difference -- the module doc's
//! own precedent (`scan_support`'s commentary on `collect_rs_files`/
//! `extract_hidden_field`) is that a straight-line directory walk is
//! safe to duplicate; only the shared, stateful, easy-to-get-wrong
//! parser (`strip_line_comments`) was worth consolidating.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Every `### 10.N` heading's own `N`, read from `requirements.md`'s own
/// text. A line must start (after trimming) with `### 10.` followed by
/// one or more ASCII digits -- the maximal digit run is the number, so
/// `### 10.47 ...` reads `47`, not `4` followed by stray `7`.
pub(crate) fn heading_numbers(requirements: &str) -> BTreeSet<u32> {
    let mut out = BTreeSet::new();
    for line in requirements.lines() {
        let line = line.trim_start();
        if let Some(rest) = line.strip_prefix("### 10.") {
            let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
            if let Ok(n) = digits.parse() {
                out.insert(n);
            }
        }
    }
    out
}

/// Every `§10.N` citation's own `N` in `text`, in the order found,
/// duplicates included -- counting occurrences is part of what
/// `SPEC-001` §1's own census reports.
pub(crate) fn section_citations(text: &str) -> Vec<u32> {
    let mut out = Vec::new();
    let bytes = text.as_bytes();
    let marker = "§10.";
    let mut start = 0;
    while let Some(rel) = text[start..].find(marker) {
        let digit_start = start + rel + marker.len();
        let mut end = digit_start;
        while end < bytes.len() && bytes[end].is_ascii_digit() {
            end += 1;
        }
        if end > digit_start
            && let Ok(n) = text[digit_start..end].parse()
        {
            out.push(n);
        }
        start = digit_start.max(start + rel + marker.len());
        if end > digit_start {
            start = end;
        }
    }
    out
}

/// Every four-digit `RFC 0NNN` reference in `text` -- `RFC 0` followed
/// by exactly three more ASCII digits (four digits total, the first of
/// them a literal `0`), with nothing else digit-shaped on either side.
///
/// **The leading zero is the whole signal, not the digit count alone.**
/// This project's own citations are small numbers (`rfcs/accepted/`'s
/// own files run `014`–`020` today): the historical style pads them to
/// four digits with a leading zero, the live style to three without one
/// -- both are this project's own number, spelled two ways. A real IETF
/// RFC is a different thing this project also cites correctly
/// (`RFC 2119`, `RFC 5322`, `RFC 7538`, …) and is never small enough to
/// carry a leading zero. Matching "exactly four digits" alone, without
/// the leading-zero anchor, was tried first and wrongly flagged every
/// one of those real external citations -- found by running rule 2
/// against the real tree before trusting it, not by inspection.
pub(crate) fn four_digit_rfc_citations(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let bytes = text.as_bytes();
    let marker = "RFC 0";
    let mut start = 0;
    while let Some(rel) = text[start..].find(marker) {
        let digit_start = start + rel + marker.len();
        let mut end = digit_start;
        while end < bytes.len() && bytes[end].is_ascii_digit() {
            end += 1;
        }
        // Three more digits after the leading `0` -- four in total --
        // and no further digit immediately after, so a longer run
        // (five digits or more) is not mistaken for this one.
        if end - digit_start == 3 {
            out.push(format!("0{}", &text[digit_start..end]));
        }
        start = if end > digit_start {
            end
        } else {
            start + rel + marker.len()
        };
    }
    out
}

/// True if `rel` (repo-root-relative, forward slashes) is in either
/// rule's own population: a markdown file under `docs/`, excluding the
/// frozen `docs/specification/history/` snapshots. `rfcs/` is never in
/// scope here -- see this module's own doc comment.
fn in_scope(rel: &str) -> bool {
    rel.ends_with(".md")
        && rel.starts_with("docs/")
        && !rel.starts_with("docs/specification/history/")
}

/// Every in-scope markdown file, as `(repo-root-relative path, text)`.
fn maintained_docs() -> Vec<(String, String)> {
    let root = repo_root();
    let mut out = Vec::new();
    walk_markdown(&root, &root.join("docs"), &mut out);
    out.sort();
    out
}

fn walk_markdown(root: &Path, dir: &Path, out: &mut Vec<(String, String)>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for e in entries.flatten() {
        let p = e.path();
        let rel = p
            .strip_prefix(root)
            .expect("under root")
            .to_string_lossy()
            .replace('\\', "/");
        if p.is_dir() {
            if rel == "docs/specification/history" {
                continue;
            }
            walk_markdown(root, &p, out);
        } else if in_scope(&rel)
            && let Ok(text) = fs::read_to_string(&p)
        {
            out.push((rel, text));
        }
    }
}

/// Every `///` or `//!` line across `crates/*/src/` -- rule 2's own
/// doc-comment population, the same reason `docs_scan` rule 5 includes
/// them: a `//` implementation comment never reaches a published
/// crate's documentation. One file's lines are joined with `\n` so
/// [`four_digit_rfc_citations`] can scan the whole file as one string,
/// the same shape [`maintained_docs`]'s entries already have.
fn published_doc_comments() -> Vec<(String, String)> {
    let root = repo_root();
    let mut out = Vec::new();
    let Ok(crates) = fs::read_dir(root.join("crates")) else {
        return out;
    };
    for c in crates.flatten() {
        walk_doc_comments(&root, &c.path().join("src"), &mut out);
    }
    out.sort();
    out
}

fn walk_doc_comments(root: &Path, dir: &Path, out: &mut Vec<(String, String)>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            walk_doc_comments(root, &p, out);
            continue;
        }
        let Some(ext) = p.extension() else { continue };
        if ext != "rs" {
            continue;
        }
        let Ok(text) = fs::read_to_string(&p) else {
            continue;
        };
        let doc_lines: String = text
            .lines()
            .filter(|l| {
                let t = l.trim_start();
                t.starts_with("///") || t.starts_with("//!")
            })
            .collect::<Vec<_>>()
            .join("\n");
        if doc_lines.is_empty() {
            continue;
        }
        let rel = p
            .strip_prefix(root)
            .expect("under root")
            .to_string_lossy()
            .replace('\\', "/");
        out.push((rel, doc_lines));
    }
}

/// The population both rules read, pre-walked so [`violations`] stays a
/// pure function over owned strings -- plantable with a synthetic
/// `Input` in a unit test, never by writing a temp file.
pub(crate) struct Input {
    /// Every in-scope `docs/` markdown file, `(path, text)`.
    pub maintained_docs: Vec<(String, String)>,
    /// Every published doc-comment file, `(path, joined doc lines)`.
    pub doc_comments: Vec<(String, String)>,
    /// `requirements.md`'s own text, for rule 1's heading set -- named
    /// separately rather than found by path inside `maintained_docs`,
    /// so a planted `Input` never has to fake a path match to be valid.
    pub requirements: String,
}

/// Every rule breach, each a sentence saying which rule, which file,
/// and which number.
pub(crate) fn violations(input: &Input) -> Vec<String> {
    let mut out = Vec::new();
    let headings = heading_numbers(&input.requirements);

    for (path, text) in &input.maintained_docs {
        for n in section_citations(text) {
            if !headings.contains(&n) {
                out.push(format!(
                    "rule 1: {path} cites §10.{n}, which has no ### 10.{n} heading in requirements.md"
                ));
            }
        }
        for rfc in four_digit_rfc_citations(text) {
            out.push(format!(
                "rule 2: {path} cites the four-digit form RFC {rfc}, outside docs/specification/history/"
            ));
        }
    }
    for (path, text) in &input.doc_comments {
        for rfc in four_digit_rfc_citations(text) {
            out.push(format!(
                "rule 2: {path} cites the four-digit form RFC {rfc} in a published doc comment"
            ));
        }
    }
    out
}

#[cfg(test)]
mod tests;
