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
//!
//! `SPEC-002` adds two more rules and a closed vocabulary, all read
//! from `requirements.md`'s own text already in [`Input`] -- Appendix A
//! is a section of that same document, not a separate population.
//!
//! (a) **Appendix A's own arithmetic**: each row's `Total` equals the
//!     sum of its own five status columns, and each of the two grand
//!     total rows equals the sum of its own member area rows. The check
//!     `§10`'s first row names directly -- "wrong in two compensating
//!     cells, so the total looked right" survives a total-only
//!     recomputation and does not survive reading every cell.
//! (b) **The table against the 168 real entries**: each area's `Total`
//!     equals the number of entries carrying that area's ID segment,
//!     and each status column equals the number of those entries whose
//!     own *governing* status -- the **last** `*Status*:` line in the
//!     entry's own body, not the first, per `FR-NAV-003`'s own amended-
//!     in-place shape -- maps to it. **The ID pattern is the 25
//!     segments actually in use** (`FR_AREAS`/`NFR_AREAS`), not a
//!     character class: `(FR|NFR)-[A-Z]+-[0-9]+` matches none of the
//!     eleven `NFR-A11Y-*` entries, since `A11Y` itself contains a
//!     digit, which is how the architect's own scoping measurement
//!     reached 58 of 69 non-functional entries and did not notice until
//!     the two counts disagreed (`SPEC-002` §2, Trap 1). This rule
//!     [`requirement_entries`] splits on the last `-` before the
//!     numeric id instead, and separately asserts the **population**
//!     it found (exactly the 25 known segments, exactly 168 entries) so
//!     a pattern that silently matches fewer entries fails loudly
//!     rather than passing quiet.
//!
//! An entry's own span runs from its `**ID — Title**` marker to the
//! next such marker **or the next Markdown heading, whichever comes
//! first** -- not only the next marker. `NFR-CMP-006` is the case this
//! guards: the last `NFR-` entry before `## 6. Data requirements`, a
//! marker-only span runs on through every `*Status*:`-labelled line in
//! that section's own prose and `§9.1`'s test inventory, crediting
//! `NFR-CMP-006` with nine statuses it does not have. Its real entry
//! has exactly one. [`requirement_entries`]'s own tests prove this
//! directly rather than trusting the fix by inspection.
//!
//! **Vocabulary**: [`status_column`] closes the leading status token to
//! the five Appendix A columns (Implemented, Partial, Specified,
//! Deferred, Divergent), with its own mapping and reasoning in its doc
//! comment -- `Met`/`Met at` → Implemented was raised in `SPEC-002` §6
//! as a proposal rather than a decision, and `SPEC-002`'s own review
//! found it already settled in the document's own words since 0.43.0;
//! the rest are small, directly-justified calls, each checked against
//! the entry's own text and, where the document states its own
//! convention (`NFR-A11Y-001`, as of `SPEC-003`'s own edits
//! `:7409`-`:7410`), matched to it rather than guessed.
//!
//! **Rule (b) is red on the real tree**, and is asserted as the known,
//! exact, bounded set rather than claimed clean -- see
//! [`tests::rule_b_s_known_pending_disagreements_are_exactly_these_and_no_others`]
//! for the population and why: nine areas where Appendix A's own cell
//! disagrees with what the entries currently say, none of them a
//! vocabulary gap (the vocabulary closure is itself green on the real
//! tree, see
//! [`tests::every_real_entry_s_governing_status_is_in_the_closed_vocabulary`]).
//! `SPEC-002` §6 is explicit that this is a finding to report, not a
//! bug in the rule to fix by editing the table.
//!
//! **One population this round leaves deliberately unchecked**: 24
//! `§10.N` numbers exist in both `requirements.md`'s older compact
//! register and the full `### 10.N` heading form (`SPEC-001` §1a); the
//! two use unrelated vocabularies (`Closed`/`Open`/`Closed in tree`
//! there, the five columns above here), so there is no cheap reuse of
//! [`status_column`] for them. Recorded, not built -- see
//! [`tests::the_24_dual_register_numbers_are_a_recorded_not_a_checked_population`].
//!
//! `SPEC-003` asked a different question of the same document: can a
//! scan tell a dated measurement (*"295 occurrences, measured
//! 2026-10-18"* -- true forever) from a present-tense claim (*"23
//! files"* -- true only while nothing changes)? **In isolation, no --
//! the two read identically.** `§10.34`/`REQ-003` already tried this
//! for a harder, adjacent question (does a requirement's *status*
//! still match the shipped code?) and the answer there was no, with
//! the evidence: four candidate text rules run against six historical
//! instances scored at best 4 of 6, one rule's zero false positives
//! were on zero opportunities, and two of the six needed a human
//! reading code against prose. That investigation is this module's own
//! precedent for not attempting a general version of the same thing
//! for counts.
//!
//! **Three shapes, not one rule:**
//!
//! - **A predicate already checked.** [`status_column`]'s own
//!   `NFR-MNT-005` is the model: *"live count zero, measured"* is
//!   enforced today by `test_harness_scan`, continuously, because it
//!   is a property ("no file violates this") rather than a count of
//!   named things. Nothing new needed here -- the shape already
//!   exists and already works.
//! - **One narrow, already-used annotation, surveyed but not wired
//!   up.** [`test_count_citations`] finds every `` `name` ... (N
//!   tests) `` citation beside an `*Acceptance*:` line -- eight of
//!   them, naming six distinct integration-test binaries. **Three were
//!   wrong** when checked against `cargo test --test <name> --
//!   --list` -- never a grep over `#[test]`, which agreed here but is
//!   not the trusted method: `view_state` read 5, is 9;
//!   `aggregate_privacy` read 12, is 13; `optimistic_lock` read 21, is
//!   22. Fixed directly in `requirements.md` as part of this round.
//!   **Not turned into a standing CI rule**: doing so would
//!   mean a test spawning `cargo test -- --list` as a child process,
//!   a pattern this codebase has never used (every existing scan
//!   reads text; `changelog_scan` is the one exception and it shells
//!   to `git`, not `cargo`), with real nested-invocation and CI-cost
//!   questions a three-citation fix does not need answered to ship.
//!   Proposed to the architect, not decided -- `SPEC-003`'s own
//!   completion report.
//! - **Everything else stays Shape C, and is written down as such
//!   rather than quietly left unmentioned.** The vast majority of
//!   `requirements.md`'s own numeric claims are already dated by this
//!   project's existing, informal, long-used convention -- an inline
//!   version marker (*"measured 0.40.0"*, *"Amended (0.52.0)"*), a
//!   defect-class `### 10.N` entry's own closure line, or Appendix C's
//!   per-release row -- which already does the job a new formal
//!   annotation would duplicate. `§9.2`'s own `*"This list is known to
//!   be badly incomplete. Audited 2026-08-26; not yet corrected,
//!   deliberately"*` is this project's own best existing example of
//!   the discipline Shape C asks for: date the snapshot, say it is
//!   one, move on. **No scan reads any of this**, and that is the
//!   correct, honest state for prose this project already knows how
//!   to date without a parser's help.

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
    out.extend(appendix_violations(&input.requirements));
    out
}

// =====================================================================
// `SPEC-002`: Appendix A summed from its own rows, and checked against
// the entries. Both read `requirements.md`'s own text, already in
// `Input` for rule 1 -- no new population to walk.
// =====================================================================

/// The area segments this project's own `FR-`/`NFR-` IDs use, derived
/// from the data (`SPEC-002` §2's own Trap 1) rather than assumed from
/// a character class. `(FR|NFR)-[A-Z]+-[0-9]+` matches **none** of the
/// eleven accessibility requirements, because `A11Y`'s own segment
/// contains a digit -- the architect's own first scoping measurement
/// hit this and silently examined 58 of 69 non-functional entries
/// before the two counts disagreed. [`requirement_entries`] never
/// matches on the segment's own character shape at all (it splits on
/// the literal `-` that always separates area from number, which
/// handles `A11Y` the same as any other segment); this list exists
/// only so [`violations`] can assert the *population* it found rather
/// than trust that finding = finding everything.
const FR_AREAS: &[&str] = &[
    "API", "AUTH", "CAL", "DM", "HLT", "ISS", "NAV", "NTF", "PER", "PLAN", "PROJ", "SCH", "SET",
    "SPR", "SUB", "TEAM",
];
const NFR_AREAS: &[&str] = &[
    "A11Y", "CMP", "CONC", "LANG", "MNT", "PERF", "PRIV", "REL", "SEC",
];

/// Appendix A's own area names, in the table's own order, mapped to
/// the ID segment(s) that back them. Two segments feed one row only
/// for `Direct manipulation` (`DM`) -- every other row is one segment,
/// one area name, read directly from the table rather than guessed
/// from a naming convention (`Sub-issues` is not `SUB-ISSUES`, etc.).
const AREA_NAMES: &[(&str, &[&str])] = &[
    ("Authentication", &["AUTH"]),
    ("Navigation", &["NAV"]),
    ("Projects", &["PROJ"]),
    ("Issues", &["ISS"]),
    ("Sub-issues", &["SUB"]),
    ("Teams", &["TEAM"]),
    ("Sprints", &["SPR"]),
    ("Sprint planning", &["PLAN"]),
    ("Health", &["HLT"]),
    ("Personal", &["PER"]),
    ("Notifications", &["NTF"]),
    ("Search", &["SCH"]),
    ("Calendar", &["CAL"]),
    ("Settings", &["SET"]),
    ("API", &["API"]),
    ("Direct manipulation", &["DM"]),
    ("Privacy", &["PRIV"]),
    ("Concurrency", &["CONC"]),
    ("Accessibility", &["A11Y"]),
    ("Language", &["LANG"]),
    ("Security", &["SEC"]),
    ("Performance", &["PERF"]),
    ("Maintainability", &["MNT"]),
    ("Release", &["REL"]),
    ("Compatibility", &["CMP"]),
];

/// One `FR-`/`NFR-` requirement entry: its ID, area segment, and the
/// leading token of its own *governing* status -- the **last**
/// `*Status*:` occurrence in its body, not the first. `FR-NAV-003` is
/// the case this matters for: its own body carries `*Status*: **Met
/// at 0.46.0**` after its first paragraph and `*Status*: Implemented.
/// *Priority*: P2.` after the correction narrative that follows --
/// taking the first would report a status the entry's own later text
/// says is no longer current. **Say what this costs**: an entry whose
/// amendment narrates a change in prose without a second `*Status*:`
/// line is read as never having changed, which is correct only if
/// every status change is written this way -- true of every entry
/// checked while building this rule, not proven true of all 168.
pub(crate) struct ReqEntry {
    pub id: String,
    pub area: String,
    pub status_token: Option<String>,
}

/// Every `**FR-AREA-NNN — Title**` / `**NFR-AREA-NNN — Title**` entry
/// in `text`, each entry's own span running from its marker line to
/// the next such marker **or** the next Markdown heading (`#` through
/// `####`), whichever comes first.
///
/// **The heading boundary is the fix for a recorded, recurring
/// mistake, not a defensive extra.** `NFR-CMP-006` (Licence) is the
/// last `NFR-` entry before `## 6. Data requirements`; a span that
/// only stops at the next `**ID**` marker runs on through every
/// `*Status*:`-labelled line in `§6.2`–`§6.5` and `§9.1`'s own test
/// inventory, crediting (or blaming) `NFR-CMP-006` with all of them.
/// Recognised on sight, by the architect's own account, "the fourth
/// measurement... to fail in this thread by the same family of
/// error" (`requirements.md:6401`) -- a mistake this rule is built not
/// to repeat, not a hypothetical it guards against.
pub(crate) fn requirement_entries(text: &str) -> Vec<ReqEntry> {
    let lines: Vec<&str> = text.lines().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        if let Some((kind, area, id)) = entry_marker(lines[i].trim()) {
            let mut body = String::new();
            let mut j = i + 1;
            while j < lines.len() {
                let t = lines[j].trim_start();
                if entry_marker(t.trim()).is_some() || is_markdown_heading(t) {
                    break;
                }
                body.push_str(lines[j]);
                body.push('\n');
                j += 1;
            }
            let _ = kind;
            out.push(ReqEntry {
                id,
                area,
                status_token: governing_status_token(&body),
            });
            i = j;
        } else {
            i += 1;
        }
    }
    out
}

fn is_markdown_heading(trimmed_line: &str) -> bool {
    let hashes = trimmed_line.chars().take_while(|c| *c == '#').count();
    (1..=4).contains(&hashes) && trimmed_line.as_bytes().get(hashes) == Some(&b' ')
}

/// If `line` (already trimmed) is `**(FR|NFR)-AREA-NNN — Title**`,
/// its `(kind, area, id)` -- `id` is the whole `KIND-AREA-NNN` text.
/// Splits on the *first* `-` after the kind prefix and the *first*
/// space after that, which is enough: no area segment in use today
/// contains a `-`, and `NNN` is a pure digit run, so there is exactly
/// one `-` in `AREA-NNN` regardless of whether `AREA` itself contains
/// a digit (`A11Y-002` splits into `A11Y` and `002` the same way
/// `AUTH-001` splits into `AUTH` and `001`).
fn entry_marker(line: &str) -> Option<(&'static str, String, String)> {
    let inner = line.strip_prefix("**")?;
    let (kind, rest) = if let Some(r) = inner.strip_prefix("FR-") {
        ("FR", r)
    } else {
        let r = inner.strip_prefix("NFR-")?;
        ("NFR", r)
    };
    let space = rest.find(' ')?;
    let (area_and_num, after) = (&rest[..space], &rest[space..]);
    if !after.trim_start().starts_with("— ") || !line.trim_end().ends_with("**") {
        return None;
    }
    let dash = area_and_num.rfind('-')?;
    let (area, num) = (&area_and_num[..dash], &area_and_num[dash + 1..]);
    if area.is_empty() || num.is_empty() || !num.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    Some((kind, area.to_string(), format!("{kind}-{area}-{num}")))
}

/// The leading token of the **last** `*Status*:` occurrence in
/// `body`, or `None` if it has none. Multi-word tokens this
/// vocabulary actually uses (`Met at`, `Not implemented`, `Not
/// defined`, `Partially implemented`) are matched before falling back
/// to a single word, and a leading `**` (every bold-wrapped status
/// value) is stripped first.
fn governing_status_token(body: &str) -> Option<String> {
    let marker = "*Status*:";
    let idx = body.rfind(marker)?;
    let after = body[idx + marker.len()..].trim_start();
    let after = after.strip_prefix("**").unwrap_or(after);
    const MULTI_WORD: &[&str] = &[
        "Met at",
        "Not implemented",
        "Not defined",
        "Partially implemented",
        "Expired at",
        "Measured and repaired",
    ];
    for token in MULTI_WORD {
        if after.starts_with(token) {
            return Some(token.to_string());
        }
    }
    let word: String = after
        .chars()
        .take_while(|c| c.is_ascii_alphabetic())
        .collect();
    if word.is_empty() { None } else { Some(word) }
}

/// Appendix A's own column this `status_token` belongs under, per the
/// mapping `SPEC-002` §3 asks this round to settle and write down.
/// **Decided here, not escalated**, except the one case named below:
///
/// - `Implemented`, `Expired at` → **Implemented**. A requirement that
///   carried its own end condition and satisfied it for its whole term
///   is "this was true for as long as it needed to be" -- the same
///   claim `Implemented` makes, reached by a different route.
/// - `Audited`, `Measured and repaired` → **Divergent**, not
///   Implemented, matching the document's own convention rather than
///   a guess: `NFR-A11Y-001`'s own body is `Audited (...) -- no
///   structural failure found, with the qualification below`, and
///   `requirements.md`'s own 0.43.0 note (`:7409`-`:7410`, as of
///   `SPEC-003`'s own edits; line citations in this document drift,
///   per its own README) names that
///   exact entry as one of Accessibility's own Divergent two, for the
///   residual gap its own text admits ("What it cannot establish").
///   `NFR-A11Y-005`'s `Measured and repaired` carries the same shape
///   in its own words -- "neither fact is guarded. Both drift the
///   moment someone adds a `bg-*` container" -- a verified-but-
///   unguarded state, which this document's own prior author already
///   treated as Divergent rather than Implemented.
/// - `Partial`, `Partially implemented` → **Partial**. The same
///   status, spelled two ways (`SPEC-002` §3 finding 1) -- the one
///   `Partially implemented` occurrence is normalised to `Partial` in
///   `requirements.md` itself, so this mapping exists for robustness,
///   not because the spelling still varies.
/// - `Specified` → **Specified**.
/// - `Deferred` → **Deferred**.
/// - `Not implemented`, `Not defined` → **Divergent**. Both describe
///   the requirement's own behaviour or value not matching what the
///   specification calls for yet, the same shape `Divergent` already
///   names for every other entry in that column -- a design decision,
///   not a specification one, since it adds no new column and changes
///   no entry's own words.
/// - `Met`, `Met at` → **Implemented**. `SPEC-002` §6 raised this as a
///   proposal rather than a decision; `SPEC-002`'s own review found
///   the question was already settled, in the document's own words,
///   since 0.43.0 -- `requirements.md:7409` as of `SPEC-003`'s own
///   edits: *"`Met` is counted as `Implemented`."* Confirmed, not
///   proposed, as of that review.
fn status_column(token: &str) -> Option<&'static str> {
    match token {
        "Implemented" | "Expired at" => Some("Implemented"),
        "Partial" | "Partially implemented" => Some("Partial"),
        "Specified" => Some("Specified"),
        "Deferred" => Some("Deferred"),
        "Not implemented" | "Not defined" | "Audited" | "Measured and repaired" => {
            Some("Divergent")
        }
        "Met" | "Met at" => Some("Implemented"),
        _ => None,
    }
}

/// One Appendix A row's own six numbers, in table order: `[Total,
/// Implemented, Partial, Specified, Deferred, Divergent]`.
pub(crate) struct AppendixRow {
    pub area: String,
    pub cells: [i64; 6],
}

/// Appendix A's own table, read as data. A cell is `—` (zero), a bare
/// count (`2 unimplemented`), or two counts together
/// (`1 (+1 unimplemented)`, `Health`'s own Divergent cell, the one row
/// that breaks a column's count into two named parts rather than one)
/// -- **every integer found in the cell, summed**, handles both
/// shapes without special-casing either: a bare count has one integer
/// to sum, and `1 (+1 ...)` sums to the `2` the row's own total
/// already requires. Checked against `Health`'s own row before
/// trusting it: `5+2+0+0+1=8`, not the `9` the row states, until the
/// parenthetical's own `1` is added too.
pub(crate) fn appendix_a_rows(text: &str) -> Vec<AppendixRow> {
    let Some(start) = text.find("## Appendix A") else {
        return Vec::new();
    };
    let rest = &text[start..];
    let end = rest[1..].find("\n## ").map(|p| p + 1).unwrap_or(rest.len());
    let mut out = Vec::new();
    for line in rest[..end].lines() {
        let line = line.trim();
        if !line.starts_with('|') {
            continue;
        }
        let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
        if cells.len() != 7 {
            continue;
        }
        let area = cells[0].trim_matches('*').trim().to_string();
        if area.is_empty() || area == "Area" || cells[1].trim_matches('*').trim() == "Total" {
            continue;
        }
        if cells[1..].iter().all(|c| {
            c.chars()
                .all(|ch| ch == '-' || ch == ':' || ch.is_whitespace())
        }) {
            continue;
        }
        let mut row = [0i64; 6];
        for (k, cell) in cells[1..7].iter().enumerate() {
            row[k] = sum_integers(cell);
        }
        out.push(AppendixRow { area, cells: row });
    }
    out
}

fn sum_integers(cell: &str) -> i64 {
    let mut sum = 0i64;
    let mut current = String::new();
    for c in cell.chars().chain(std::iter::once(' ')) {
        if c.is_ascii_digit() {
            current.push(c);
        } else if !current.is_empty() {
            sum += current.parse::<i64>().unwrap_or(0);
            current.clear();
        }
    }
    sum
}

/// Rule (a) and rule (b), both over `requirements.md`'s own text.
fn appendix_violations(requirements: &str) -> Vec<String> {
    let mut out = Vec::new();
    let rows = appendix_a_rows(requirements);
    let entries = requirement_entries(requirements);

    // Rule (a): each row's own Total equals the sum of its own five
    // status columns -- `§10`'s own first row, "wrong in two
    // compensating cells, so the total looked right", is exactly what
    // this catches: two wrong cells that cancel do not survive a
    // recomputation that reads every cell, only one that reads the
    // total alone.
    for row in &rows {
        let [total, implemented, partial, specified, deferred, divergent] = row.cells;
        let sum = implemented + partial + specified + deferred + divergent;
        if sum != total {
            out.push(format!(
                "rule a: Appendix A's {} row totals {total} but its own five columns sum to {sum}",
                row.area
            ));
        }
    }
    // The two grand-total rows additionally equal the sum of their
    // own constituent area rows -- found by splitting on the known
    // functional/non-functional area-name sets rather than by
    // position, since a reordered table must not silently pass this.
    let functional_names: std::collections::HashSet<&str> =
        AREA_NAMES[..16].iter().map(|(n, _)| *n).collect();
    check_grand_total(&rows, "Functional total", &functional_names, &mut out);
    let non_functional_names: std::collections::HashSet<&str> =
        AREA_NAMES[16..].iter().map(|(n, _)| *n).collect();
    check_grand_total(
        &rows,
        "Non-functional total",
        &non_functional_names,
        &mut out,
    );

    // Rule (b): the population examined, asserted rather than assumed
    // -- `SPEC-002` §2's own Trap 1. If a future area segment is not
    // in `FR_AREAS`/`NFR_AREAS`, its entries are still found (parsing
    // never gated on the known list) but their area never matches any
    // `AREA_NAMES` row, so they are reported here rather than
    // silently excluded from every area/status count below.
    let known_areas: std::collections::HashSet<&str> =
        FR_AREAS.iter().chain(NFR_AREAS.iter()).copied().collect();
    let examined_areas: std::collections::BTreeSet<&str> =
        entries.iter().map(|e| e.area.as_str()).collect();
    for area in &examined_areas {
        if !known_areas.contains(area) {
            out.push(format!(
                "rule b: entries use area segment {area}, not in this rule's own FR_AREAS/NFR_AREAS -- a new area, or a stale list"
            ));
        }
    }
    if examined_areas.len() != known_areas.len() {
        out.push(format!(
            "rule b: examined {} distinct area segments, expected {} -- population narrower or wider than this rule's own census",
            examined_areas.len(),
            known_areas.len()
        ));
    }

    // Rule (b): each area's Total in Appendix A equals the number of
    // real entries with that area.
    for (area_name, segments) in AREA_NAMES {
        let Some(row) = rows.iter().find(|r| r.area == *area_name) else {
            out.push(format!("rule b: Appendix A has no row named {area_name}"));
            continue;
        };
        let count = entries
            .iter()
            .filter(|e| segments.contains(&e.area.as_str()))
            .count() as i64;
        if count != row.cells[0] {
            out.push(format!(
                "rule b: {area_name} has {count} real entries, Appendix A's own Total says {}",
                row.cells[0]
            ));
        }
        // Each status column equals the number of entries in this
        // area whose own governing status maps to it.
        for (col_idx, col_name) in [
            "Implemented",
            "Partial",
            "Specified",
            "Deferred",
            "Divergent",
        ]
        .into_iter()
        .enumerate()
        {
            let mapped = entries
                .iter()
                .filter(|e| segments.contains(&e.area.as_str()))
                .filter(|e| e.status_token.as_deref().and_then(status_column) == Some(col_name))
                .count() as i64;
            if mapped != row.cells[col_idx + 1] {
                out.push(format!(
                    "rule b: {area_name}'s own entries mapped to {col_name} number {mapped}, Appendix A says {}",
                    row.cells[col_idx + 1]
                ));
            }
        }
    }

    // Vocabulary closure: every entry's own governing status must be
    // one this rule recognises -- an entry with none, or with a token
    // `status_column` does not map, is reported by name rather than
    // silently excluded from every count above.
    for e in &entries {
        match &e.status_token {
            None => out.push(format!("rule vocab: {} has no *Status*: line at all", e.id)),
            Some(t) if status_column(t).is_none() => out.push(format!(
                "rule vocab: {} has an unmapped status token {t:?}",
                e.id
            )),
            _ => {}
        }
    }

    out
}

fn check_grand_total(
    rows: &[AppendixRow],
    total_name: &str,
    member_names: &std::collections::HashSet<&str>,
    out: &mut Vec<String>,
) {
    let Some(total_row) = rows.iter().find(|r| r.area == total_name) else {
        out.push(format!("rule a: Appendix A has no {total_name} row"));
        return;
    };
    let mut sums = [0i64; 6];
    for row in rows {
        if member_names.contains(row.area.as_str()) {
            for (sum, cell) in sums.iter_mut().zip(row.cells.iter()) {
                *sum += cell;
            }
        }
    }
    if sums != total_row.cells {
        out.push(format!(
            "rule a: {total_name} reads {:?} but its own member rows sum to {:?}",
            total_row.cells, sums
        ));
    }
}

// =====================================================================
// `SPEC-003`: the one numeric-claim shape surveyed as a Shape B
// candidate. Not wired into `violations` -- see the module doc.
// =====================================================================

/// Every `` `name` ... (N tests) `` citation in `text`, in document
/// order: a backtick-quoted identifier followed, within a short span
/// that may cross one line break and may bold-wrap the digits, by `(N
/// tests)` or `(N test)`. Each tuple is `(name, n)`.
///
/// Deliberately narrow: this is the exact shape `requirements.md`
/// already uses for its own `*Acceptance*: \`crate\` (N tests)`
/// citations, not a general "count near an identifier" matcher. A
/// citation that drops the parenthetical count to continue a list --
/// *"`optimistic_lock` test crate (21 tests), `optimistic_lock_atomicity`
/// (14), `board_keyboard`, `confirmation`"* -- is not found by this
/// function; its own count was checked by hand for this round
/// (`cargo test --test optimistic_lock_atomicity -- --list`: 14,
/// matching) rather than taught to a parser for one occurrence.
pub(crate) fn test_count_citations(text: &str) -> Vec<(String, u32)> {
    let mut out = Vec::new();
    let bytes = text.as_bytes();
    let mut i = 0;
    while let Some(tick) = text[i..].find('`') {
        let name_start = i + tick + 1;
        let Some(name_end_rel) = text[name_start..].find('`') else {
            break;
        };
        let name_end = name_start + name_end_rel;
        let name = &text[name_start..name_end];
        if !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            let after = &text[name_end + 1..];
            let window_end = (name_end + 1 + 60).min(bytes.len());
            let mut we = window_end;
            while we < bytes.len() && !text.is_char_boundary(we) {
                we += 1;
            }
            let window = &text[name_end + 1..we];
            if let Some(paren) = window.find('(') {
                let rest = &window[paren + 1..];
                let digits: String = rest
                    .trim_start_matches('*')
                    .chars()
                    .take_while(|c| c.is_ascii_digit())
                    .collect();
                if !digits.is_empty() {
                    let after_digits = &rest.trim_start_matches('*')[digits.len()..];
                    let after_digits = after_digits.trim_start_matches('*').trim_start();
                    if after_digits.starts_with("test") && !window[..paren].contains('`') {
                        // Nothing else between the name and the count --
                        // not a different identifier's own citation.
                        if let Ok(n) = digits.parse() {
                            out.push((name.to_string(), n));
                        }
                    }
                }
            }
            let _ = after;
        }
        i = name_end + 1;
    }
    out
}

#[cfg(test)]
mod tests;
