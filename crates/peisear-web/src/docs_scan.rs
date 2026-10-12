//! A scan for what a scan can check (`DOCS-005`).
//!
//! `REQ-003` bounds this module: no rule here tries to judge prose, because
//! none that were tried reached the bar. What *is* mechanisable is "does the
//! number in this document match the number in that file," and that is the
//! whole design -- five rules, each comparing one document against a source
//! of truth read at runtime:
//!
//! 1. The crate set a document names, against `crates/*/` -- all three
//!    documents that enumerate the crate set (`README.md`,
//!    `workspace-layout.md`, `crate-boundaries.md`), as this rule always
//!    intended. **Rule 1b**, since `DOC-004`: the crate set's own
//!    *count*, in the four fixed phrase shapes the documents use for it
//!    -- `docs/README.md` included, which rule 1 itself still does not
//!    read (it has no crate to be missing; it only ever claims a count).
//! 2. MSRV claims, against `Cargo.toml`'s `rust-version` -- `installation.md`
//!    and, since `DOC-004`, `.github/CONTRIBUTING.md`, which cites
//!    `installation.md` but carried its own copy of the number and had
//!    drifted independently.
//! 3. `deployment.md`'s *What to ship* manifest against `static/`'s actual
//!    contents, exhaustively in both directions: nothing the manifest
//!    lists is missing from `static/`, and nothing in `static/` is
//!    missing from the manifest.
//! 4. Documented environment-variable defaults, against `config.rs`'s
//!    `unwrap_or_else`/`unwrap_or` fallbacks.
//! 5. No document under `docs/`, no RFC, and no published doc comment
//!    cites `.git-exclude/`, a gitignored path -- four named exceptions.
//!    Its population (hundreds of files under `docs/`, `rfcs/`, and
//!    `crates/*/src/`) is too large for this module's [`Input`] to hold
//!    cheaply the way rules 1-4's few fixed documents can, so its
//!    functions and tests sit apart from the other four, at the end of
//!    this file.
//!
//! **Rules 1 and 3 were not always this wide.** `DOCS-005` landed both as
//! half-rules, deliberately: `README.md`'s crate count and the manifest's
//! one already-true direction, with the other half asserted only in a
//! named test that documented the gap rather than landing the suite red
//! -- the trap `TT-007`'s own ordering note warns about, hitting a rule
//! against a tree a *later* handoff is what fixes. `DOC-001` (`0.51.0`)
//! is that later handoff: once `docs/architecture/` named `peisear-i18n`
//! and `deployment.md` named all eight files in `static/`, both halves
//! became assertable, the two gap-documenting tests were deleted, and
//! the rules above are what they always intended to be.
//!
//! **Rule 1b exists because rule 1 checks presence, never cardinality**
//! (`DOC-001`'s own review, finding F-3). `DOC-004` measured the claims
//! instead of assuming one rule would cover them: three documents, four
//! claims, two different denominators (the full crate set, and the
//! implementation crates alone, facade excluded) -- and `crate-boundaries.md`
//! also says "five of the six", "four of the other five" and "existing
//! five", subset counts a rule keyed on *any* number word near "crate"
//! would flag as violations. So rule 1b matches the four fixed phrase
//! shapes the real claims use, the same way rule 2 matches `**X.Y or
//! later**` rather than any number near "rust" -- declining the generic
//! version rather than shipping a rule with an exception list longer
//! than itself.
//!
//! **Rule 5 was not always this narrow.** It started as "no *committed*
//! file cites the path," which could never pass -- that version banned
//! `docs/specification/README.md`'s own sentence defining what
//! `.git-exclude/` is, and it banned every implementation handoff, which
//! is an execution record, not documentation. `DOC-002` (`RFC 0019`,
//! `0.51.0`) found the rule the defect was actually about and this is
//! it: three documentation surfaces, four named exceptions, no `git`
//! shell-out, enforced rather than reported.
//!
//! **Rule 4's own bound**: the security error this rule is named for
//! (`hardening.md` once claimed `127.0.0.1:3000` was the default bind) was
//! a *prose* claim, not a table cell -- `docs/getting-started/
//! configuration.md`'s structured table is what this rule checks, because
//! a table cell is a number-to-number comparison and a sentence like "the
//! default-safe choice" is exactly the prose judgement `REQ-003` already
//! found unmechanisable. A regex keyed on the word "default" near a
//! `BIND_ADDR=` literal was tried and rejected here for the same reason
//! `REQ-003`'s own four candidates were: `hardening.md`'s current text
//! *quotes* the old wrong sentence in its own correction note, and a rule
//! that cannot tell a live claim from a quoted-for-correction one would
//! flag the fix itself. `hardening.md`'s prose stays pass 5's territory.

use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

// ---------- rule 1: the crate set ----------

fn crate_dirs() -> Vec<String> {
    let mut out = Vec::new();
    for e in fs::read_dir(repo_root().join("crates"))
        .expect("crates/ exists")
        .flatten()
    {
        if e.path().is_dir() {
            out.push(e.file_name().to_string_lossy().to_string());
        }
    }
    out.sort();
    out
}

/// `peisear-i18n` -> `i18n`; `peisear` (the facade) -> `peisear`.
fn short_crate_name(dir: &str) -> &str {
    dir.strip_prefix("peisear-").unwrap_or(dir)
}

/// True if `doc` names `dir`, in either convention this project actually
/// uses: the full crate name as a plain substring (the architecture docs'
/// style, inside an ASCII tree or a backtick span), or the short name in
/// its own backtick span (`README.md`'s friendlier style).
fn mentions_crate(doc: &str, dir: &str) -> bool {
    doc.contains(dir) || doc.contains(&format!("`{}`", short_crate_name(dir)))
}

// ---------- rule 1b: the crate set's own cardinality (`DOC-004`) ----------
//
// Rule 1 asks "is each crate named"; it never asked "does the document's
// own claimed *count* match." `crate-boundaries.md` also says "five of
// the six", "four of the other five" and "existing five" -- subset
// counts, not claims about the crate set's own size -- so this does not
// match any number word near the word "crate": it matches the four
// fixed phrase shapes the documents actually use for the claim this
// rule is about, the same way rule 2 matches `**X.Y or later**` rather
// than any number near "rust".

/// A cardinal-number word, "one" through "ten" -- this workspace's crate
/// count is nowhere near needing more.
fn word_to_number(word: &str) -> Option<usize> {
    Some(match word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "ten" => 10,
        _ => return None,
    })
}

/// The number word immediately before `marker` in `text`.
fn number_before(text: &str, marker: &str) -> Option<usize> {
    let idx = text.find(marker)?;
    let word = text[..idx].trim_end().rsplit(char::is_whitespace).next()?;
    word_to_number(word)
}

/// `workspace-layout.md`: "peisear is a Cargo workspace containing
/// {N} published crates".
fn published_crate_count(workspace_layout: &str) -> Option<usize> {
    number_before(workspace_layout, " published crates")
}

/// `docs/README.md`'s index line: "why there are {N} crates and what
/// belongs in each".
fn readme_index_crate_count(docs_readme: &str) -> Option<usize> {
    number_before(docs_readme, " crates and what belongs in each")
}

/// `crate-boundaries.md`'s intro: "peisear is split into {N}
/// implementation crates plus a thin facade".
fn implementation_crate_count_intro(crate_boundaries: &str) -> Option<usize> {
    number_before(
        crate_boundaries,
        " implementation crates plus a thin facade",
    )
}

/// `crate-boundaries.md`'s own "## The {N} implementation crates"
/// heading -- matched as a whole line, not a substring, so the subset
/// sentence at `crate-boundaries.md:115` ("five of the **six**
/// implementation crates") can never be mistaken for it: that line is
/// prose, not a `##` heading.
fn implementation_crate_count_heading(crate_boundaries: &str) -> Option<usize> {
    crate_boundaries.lines().find_map(|l| {
        let rest = l.strip_prefix("## The ")?;
        word_to_number(rest.strip_suffix(" implementation crates")?)
    })
}

// ---------- rule 2: the MSRV ----------

fn rust_version_major_minor(cargo_toml: &str) -> String {
    let after = &cargo_toml[cargo_toml
        .find("[workspace.package]")
        .expect("[workspace.package]")..];
    let line = after
        .lines()
        .find(|l| l.trim_start().starts_with("rust-version"))
        .expect("rust-version line");
    let full = line.split('"').nth(1).expect("quoted rust-version");
    let mut parts = full.split('.');
    format!(
        "{}.{}",
        parts.next().unwrap_or(""),
        parts.next().unwrap_or("")
    )
}

/// The `X.Y` out of a `**X.Y or later**` sentence.
fn bold_or_later(installation: &str) -> Option<String> {
    let marker = " or later**";
    let idx = installation.find(marker)?;
    let before = &installation[..idx];
    let start = before.rfind("**")? + 2;
    Some(before[start..].trim().to_string())
}

/// The `X.Y` out of a `# <tool> X.Y.x` Verify-block comment line.
fn verify_block_version(installation: &str, tool: &str) -> Option<String> {
    let marker = format!("# {tool} ");
    let idx = installation.find(&marker)?;
    let rest = &installation[idx + marker.len()..];
    let end = rest.find(".x")?;
    Some(rest[..end].to_string())
}

// ---------- rule 3: the deployment manifest vs static/ ----------

fn static_files() -> Vec<String> {
    let mut out = Vec::new();
    for e in fs::read_dir(repo_root().join("static"))
        .expect("static/ exists")
        .flatten()
    {
        if e.path().is_file() {
            out.push(e.file_name().to_string_lossy().to_string());
        }
    }
    out.sort();
    out
}

/// The filenames nested under `static/` in `deployment.md`'s *What to
/// ship* tree -- the lines indented a level deeper than `static/` itself.
fn manifest_static_files(deployment: &str) -> Vec<String> {
    let Some(after) = deployment.find("## What to ship").map(|i| &deployment[i..]) else {
        return Vec::new();
    };
    let Some(start) = after.find("```") else {
        return Vec::new();
    };
    let rest = &after[start + 3..];
    let Some(end) = rest.find("```") else {
        return Vec::new();
    };
    let block = &rest[..end];
    let mut out = Vec::new();
    for line in block.lines() {
        if !line.starts_with("    ") {
            continue;
        }
        let trimmed = line.trim_start();
        let Some(ns) = trimmed.find(|c: char| c.is_ascii_alphanumeric()) else {
            continue;
        };
        let tail = &trimmed[ns..];
        let name = tail
            .split_whitespace()
            .next()
            .unwrap_or("")
            .trim_end_matches('/');
        if !name.is_empty() {
            out.push(name.to_string());
        }
    }
    out
}

// ---------- rule 4: documented defaults vs config.rs ----------

/// The literal string inside `env::var("<var>").unwrap_or_else(|_| "...".to_string())`.
/// Scoped to a short window after the marker so a multi-line closure whose
/// first string literal is something else (`JWT_SECRET`'s warning text, not
/// its fallback value) is deliberately not matched -- callers must not rely
/// on this for a variable whose closure body isn't `"literal".to_string()`.
fn literal_string_default(config_rs: &str, var: &str) -> Option<String> {
    let marker = format!("env::var(\"{var}\")");
    let idx = config_rs.find(&marker)?;
    let rest = &config_rs[idx + marker.len()..];
    let window = &rest[..rest.len().min(120)];
    let start = window.find('"')? + 1;
    let after_start = &window[start..];
    let end = after_start.find('"')?;
    Some(after_start[..end].to_string())
}

/// The boolean inside `env::var("<var>")....unwrap_or(<bool>)`.
fn literal_bool_default(config_rs: &str, var: &str) -> Option<bool> {
    let marker = format!("env::var(\"{var}\")");
    let idx = config_rs.find(&marker)?;
    let rest = &config_rs[idx + marker.len()..];
    let window = &rest[..rest.len().min(200)];
    if window.contains(".unwrap_or(true)") {
        Some(true)
    } else if window.contains(".unwrap_or(false)") {
        Some(false)
    } else {
        None
    }
}

// ---------- rule 6: README.md's absolute image URLs ----------

const RAW_URL_PREFIX: &str = "https://github.com/nabbisen/peisear/raw/main/";

/// Every path named by a `.../raw/main/<path>` URL in `text`, in the
/// order they appear. Mechanical text splitting, same shape as rule 3's
/// manifest parse -- no network call, ever (`DOCS-008` §7's own
/// condition: "if rule 6 cannot be written without a network call, then
/// it is not rule 6 and should not be built"; it doesn't need one, since
/// the question is only "does this path exist in the tree", not "does
/// the URL resolve").
fn raw_url_paths(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = text;
    while let Some(idx) = rest.find(RAW_URL_PREFIX) {
        let after = &rest[idx + RAW_URL_PREFIX.len()..];
        let end = after
            .find(|c: char| c == ')' || c == '"' || c == '\'' || c.is_whitespace())
            .unwrap_or(after.len());
        out.push(after[..end].to_string());
        rest = &after[end..];
    }
    out
}

/// The *Default* cell of `configuration.md`'s reference table row for `var`.
fn table_default(configuration: &str, var: &str) -> Option<String> {
    let marker = format!("`{var}`");
    let line = configuration
        .lines()
        .find(|l| l.trim_start().starts_with('|') && l.contains(&marker))?;
    let cells: Vec<&str> = line.split('|').collect();
    let cell = cells.get(2)?.trim();
    Some(cell.trim_matches('`').to_string())
}

// ---------- the four rules held in one Input, as a pure function ----------

pub(crate) struct Input<'a> {
    pub crate_dirs: Vec<String>,
    pub readme: &'a str,
    pub workspace_layout: &'a str,
    pub crate_boundaries: &'a str,
    pub docs_readme: &'a str,
    pub cargo_toml: &'a str,
    pub installation: &'a str,
    pub contributing: &'a str,
    pub static_files: Vec<String>,
    pub deployment: &'a str,
    pub configuration: &'a str,
    pub config_rs: &'a str,
    /// Does this repo-root-relative path exist? (rule 6 only.)
    pub exists: &'a dyn Fn(&str) -> bool,
}

/// Every rule breach, each a sentence saying which rule and where.
pub(crate) fn violations(input: &Input) -> Vec<String> {
    let mut out = Vec::new();

    // Rule 1: every crate is named in each of the three documents that
    // enumerate the crate set.
    for (doc_name, doc) in [
        ("README.md", input.readme),
        ("workspace-layout.md", input.workspace_layout),
        ("crate-boundaries.md", input.crate_boundaries),
    ] {
        for dir in &input.crate_dirs {
            if !mentions_crate(doc, dir) {
                out.push(format!(
                    "rule 1: {doc_name} does not name crate {dir} (as `{}` or `{dir}`)",
                    short_crate_name(dir)
                ));
            }
        }
    }

    // Rule 1b: the crate set's own cardinality, in the four fixed phrase
    // shapes the documents actually use -- `DOC-004`.
    let total_crates = input.crate_dirs.len();
    let implementation_crates = input
        .crate_dirs
        .iter()
        .filter(|d| d.starts_with("peisear-"))
        .count();
    match published_crate_count(input.workspace_layout) {
        Some(n) if n != total_crates => out.push(format!(
            "rule 1b: workspace-layout.md claims {n} published crates, crates/ has {total_crates}"
        )),
        Some(_) => {}
        None => out.push(
            "rule 1b: workspace-layout.md has no \"N published crates\" sentence to check".into(),
        ),
    }
    match readme_index_crate_count(input.docs_readme) {
        Some(n) if n != total_crates => out.push(format!(
            "rule 1b: docs/README.md claims {n} crates, crates/ has {total_crates}"
        )),
        Some(_) => {}
        None => out.push(
            "rule 1b: docs/README.md has no \"N crates and what belongs in each\" sentence to check"
                .into(),
        ),
    }
    match implementation_crate_count_intro(input.crate_boundaries) {
        Some(n) if n != implementation_crates => out.push(format!(
            "rule 1b: crate-boundaries.md's intro claims {n} implementation crates, crates/ has {implementation_crates} (excluding the facade)"
        )),
        Some(_) => {}
        None => out.push(
            "rule 1b: crate-boundaries.md has no \"N implementation crates plus a thin facade\" sentence to check"
                .into(),
        ),
    }
    match implementation_crate_count_heading(input.crate_boundaries) {
        Some(n) if n != implementation_crates => out.push(format!(
            "rule 1b: crate-boundaries.md's \"## The {n} implementation crates\" heading does not match crates/'s {implementation_crates} (excluding the facade)"
        )),
        Some(_) => {}
        None => out.push(
            "rule 1b: crate-boundaries.md has no \"## The N implementation crates\" heading to check"
                .into(),
        ),
    }

    // Rule 2, live: MSRV claims match Cargo.toml -- installation.md and,
    // since DOC-004, CONTRIBUTING.md, which cites installation.md but
    // carries its own copy of the number and drifted independently
    // (`DOC-004` found it at 1.85 while Cargo.toml had moved to 1.88).
    // Reworded to the same `**X.Y or later**` shape so one parser
    // serves both documents rather than adding a second pattern.
    let msrv = rust_version_major_minor(input.cargo_toml);
    for (doc_name, doc) in [
        ("installation.md", input.installation),
        ("CONTRIBUTING.md", input.contributing),
    ] {
        match bold_or_later(doc) {
            Some(bold) if bold != msrv => out.push(format!(
                "rule 2: {doc_name}'s \"{bold} or later\" does not match Cargo.toml's rust-version {msrv}"
            )),
            Some(_) => {}
            None => out.push(format!(
                "rule 2: {doc_name} has no \"X.Y or later\" MSRV sentence to check"
            )),
        }
    }
    for tool in ["rustc", "cargo"] {
        match verify_block_version(input.installation, tool) {
            Some(v) if v != msrv => out.push(format!(
                "rule 2: installation.md's Verify block shows {tool} {v}.x, Cargo.toml says {msrv}"
            )),
            Some(_) => {}
            None => out.push(format!(
                "rule 2: installation.md's Verify block has no `# {tool} X.Y.x` line to check"
            )),
        }
    }

    // Rule 3: deployment.md's manifest and static/ agree, in both
    // directions -- every file the manifest lists exists in static/, and
    // every file in static/ is listed in the manifest.
    let manifest = manifest_static_files(input.deployment);
    for name in &manifest {
        if !input.static_files.contains(name) {
            out.push(format!(
                "rule 3: deployment.md's manifest lists {name}, which is not in static/"
            ));
        }
    }
    for name in &input.static_files {
        if !manifest.contains(name) {
            out.push(format!(
                "rule 3: static/{name} is not listed in deployment.md's manifest"
            ));
        }
    }

    // Rule 4, live: documented defaults match config.rs.
    for var in ["DATABASE_URL", "BIND_ADDR"] {
        if let (Some(actual), Some(documented)) = (
            literal_string_default(input.config_rs, var),
            table_default(input.configuration, var),
        ) && actual != documented
        {
            out.push(format!(
                "rule 4: configuration.md documents {var}'s default as `{documented}`, config.rs's is `{actual}`"
            ));
        }
    }
    if let (Some(actual), Some(documented)) = (
        literal_bool_default(input.config_rs, "COOKIE_SECURE"),
        table_default(input.configuration, "COOKIE_SECURE"),
    ) {
        let actual_str = if actual { "1" } else { "0" };
        if actual_str != documented {
            out.push(format!(
                "rule 4: configuration.md documents COOKIE_SECURE's default as `{documented}`, config.rs's is `{actual_str}`"
            ));
        }
    }

    // Rule 6, live: every README.md raw-URL image names a file that exists.
    for path in raw_url_paths(input.readme) {
        if !(input.exists)(&path) {
            out.push(format!(
                "rule 6: README.md's raw URL names {path}, which does not exist"
            ));
        }
    }

    out
}

struct Real {
    crate_dirs: Vec<String>,
    readme: String,
    workspace_layout: String,
    crate_boundaries: String,
    docs_readme: String,
    cargo_toml: String,
    installation: String,
    contributing: String,
    static_files: Vec<String>,
    deployment: String,
    configuration: String,
    config_rs: String,
}

fn real() -> Real {
    let root = repo_root();
    Real {
        crate_dirs: crate_dirs(),
        readme: fs::read_to_string(root.join("README.md")).expect("read README.md"),
        workspace_layout: fs::read_to_string(root.join("docs/architecture/workspace-layout.md"))
            .expect("read workspace-layout.md"),
        crate_boundaries: fs::read_to_string(root.join("docs/architecture/crate-boundaries.md"))
            .expect("read crate-boundaries.md"),
        docs_readme: fs::read_to_string(root.join("docs/README.md")).expect("read docs/README.md"),
        cargo_toml: fs::read_to_string(root.join("Cargo.toml")).expect("read Cargo.toml"),
        installation: fs::read_to_string(root.join("docs/getting-started/installation.md"))
            .expect("read installation.md"),
        contributing: fs::read_to_string(root.join(".github/CONTRIBUTING.md"))
            .expect("read CONTRIBUTING.md"),
        static_files: static_files(),
        deployment: fs::read_to_string(root.join("docs/operations/deployment.md"))
            .expect("read deployment.md"),
        configuration: fs::read_to_string(root.join("docs/getting-started/configuration.md"))
            .expect("read configuration.md"),
        config_rs: fs::read_to_string(root.join("crates/peisear-web/src/config.rs"))
            .expect("read config.rs"),
    }
}

fn run(r: &Real) -> Vec<String> {
    let root = repo_root();
    let exists = move |p: &str| root.join(p).exists();
    violations(&Input {
        crate_dirs: r.crate_dirs.clone(),
        readme: &r.readme,
        workspace_layout: &r.workspace_layout,
        crate_boundaries: &r.crate_boundaries,
        docs_readme: &r.docs_readme,
        cargo_toml: &r.cargo_toml,
        installation: &r.installation,
        contributing: &r.contributing,
        static_files: r.static_files.clone(),
        deployment: &r.deployment,
        configuration: &r.configuration,
        config_rs: &r.config_rs,
        exists: &exists,
    })
}

fn only(v: Vec<String>, rule: &str) -> Vec<String> {
    v.into_iter().filter(|m| m.starts_with(rule)).collect()
}

#[test]
fn the_repository_satisfies_the_live_rules() {
    let r = real();
    let v = run(&r);
    assert!(v.is_empty(), "docs scan violations:\n{}", v.join("\n"));
}

#[test]
fn rule_1_catches_a_crate_missing_from_the_readme() {
    let mut r = real();
    r.readme = r.readme.replace("`i18n`, ", "");
    let v = only(run(&r), "rule 1:");
    assert!(v.iter().any(|m| m.contains("peisear-i18n")), "got {v:?}");
}

#[test]
fn rule_1_catches_a_crate_missing_from_workspace_layout() {
    let mut r = real();
    r.workspace_layout = r.workspace_layout.replace("peisear-i18n", "");
    let v = only(run(&r), "rule 1:");
    assert!(
        v.iter()
            .any(|m| m.contains("workspace-layout.md") && m.contains("peisear-i18n")),
        "got {v:?}"
    );
}

#[test]
fn rule_1_catches_a_crate_missing_from_crate_boundaries() {
    let mut r = real();
    r.crate_boundaries = r.crate_boundaries.replace("peisear-i18n", "");
    let v = only(run(&r), "rule 1:");
    assert!(
        v.iter()
            .any(|m| m.contains("crate-boundaries.md") && m.contains("peisear-i18n")),
        "got {v:?}"
    );
}

#[test]
fn rule_1b_catches_a_stale_count_in_workspace_layout() {
    let mut r = real();
    r.workspace_layout =
        r.workspace_layout
            .replacen("seven published crates", "six published crates", 1);
    let v = only(run(&r), "rule 1b:");
    assert!(
        v.iter()
            .any(|m| m.contains("workspace-layout.md") && m.contains("6 published crates")),
        "got {v:?}"
    );
}

#[test]
fn rule_1b_catches_a_stale_count_in_docs_readme() {
    let mut r = real();
    r.docs_readme = r.docs_readme.replacen(
        "seven crates and what belongs",
        "six crates and what belongs",
        1,
    );
    let v = only(run(&r), "rule 1b:");
    assert!(
        v.iter()
            .any(|m| m.contains("docs/README.md") && m.contains("6 crates")),
        "got {v:?}"
    );
}

#[test]
fn rule_1b_catches_a_stale_count_in_crate_boundaries_intro() {
    let mut r = real();
    r.crate_boundaries = r.crate_boundaries.replacen(
        "six implementation crates plus a thin facade",
        "five implementation crates plus a thin facade",
        1,
    );
    let v = only(run(&r), "rule 1b:");
    assert!(
        v.iter()
            .any(|m| m.contains("intro") && m.contains("5 implementation crates")),
        "got {v:?}"
    );
}

#[test]
fn rule_1b_catches_a_stale_count_in_crate_boundaries_heading() {
    let mut r = real();
    r.crate_boundaries = r.crate_boundaries.replacen(
        "## The six implementation crates",
        "## The five implementation crates",
        1,
    );
    let v = only(run(&r), "rule 1b:");
    assert!(
        v.iter()
            .any(|m| m.contains("heading") && m.contains("5 implementation crates")),
        "got {v:?}"
    );
}

#[test]
fn rule_2_catches_a_stale_msrv_sentence() {
    let mut r = real();
    r.installation = r.installation.replace("1.88 or later", "1.85 or later");
    let v = only(run(&r), "rule 2:");
    assert!(v.iter().any(|m| m.contains("1.85")), "got {v:?}");
}

#[test]
fn rule_2_catches_a_stale_msrv_in_contributing() {
    let mut r = real();
    r.contributing = r.contributing.replace("1.88 or later", "1.85 or later");
    let v = only(run(&r), "rule 2:");
    assert!(
        v.iter()
            .any(|m| m.contains("CONTRIBUTING.md") && m.contains("1.85")),
        "got {v:?}"
    );
}

#[test]
fn rule_2_catches_a_stale_verify_block() {
    let mut r = real();
    r.installation = r.installation.replace("# rustc 1.88.x", "# rustc 1.85.x");
    let v = only(run(&r), "rule 2:");
    assert!(v.iter().any(|m| m.contains("rustc 1.85.x")), "got {v:?}");
}

#[test]
fn rule_3_catches_a_manifest_entry_that_does_not_exist() {
    let mut r = real();
    r.deployment = r.deployment.replace("app.css", "app-renamed.css");
    let v = only(run(&r), "rule 3:");
    assert!(v.iter().any(|m| m.contains("app-renamed.css")), "got {v:?}");
}

#[test]
fn rule_3_catches_a_static_file_missing_from_the_manifest() {
    let mut r = real();
    r.deployment = r.deployment.replace("    ├── calendar.js\n", "");
    let v = only(run(&r), "rule 3:");
    assert!(
        v.iter()
            .any(|m| m.contains("static/calendar.js") && m.contains("not listed")),
        "got {v:?}"
    );
}

#[test]
fn rule_4_catches_a_documented_default_that_does_not_match_config_rs() {
    let mut r = real();
    r.configuration = r
        .configuration
        .replace("`0.0.0.0:3000`", "`127.0.0.1:3000`");
    let v = only(run(&r), "rule 4:");
    assert!(v.iter().any(|m| m.contains("BIND_ADDR")), "got {v:?}");
}

#[test]
fn rule_4_catches_a_documented_boolean_default_that_does_not_match_config_rs() {
    let mut r = real();
    let needle = table_default(&r.configuration, "COOKIE_SECURE").expect("COOKIE_SECURE row");
    r.configuration = r.configuration.replacen(&format!("`{needle}`"), "`1`", 1);
    let v = only(run(&r), "rule 4:");
    assert!(v.iter().any(|m| m.contains("COOKIE_SECURE")), "got {v:?}");
}

#[test]
fn rule_6_catches_a_raw_url_naming_a_file_that_does_not_exist() {
    let mut r = real();
    r.readme = format!(
        "{}\n![missing](https://github.com/nabbisen/peisear/raw/main/docs/src/assets/screenshots/does-not-exist.png)\n",
        r.readme
    );
    let v = only(run(&r), "rule 6:");
    assert!(
        v.iter().any(|m| m.contains("does-not-exist.png")),
        "got {v:?}"
    );
}

#[test]
fn the_helpers_do_what_the_rules_assume() {
    assert_eq!(short_crate_name("peisear-i18n"), "i18n");
    assert_eq!(short_crate_name("peisear"), "peisear");
    assert!(mentions_crate("the `peisear-core` crate", "peisear-core"));
    assert!(mentions_crate("Seven crates: `core`, ...", "peisear-core"));
    assert!(!mentions_crate("Six crates: `auth`, ...", "peisear-i18n"));
    assert_eq!(
        bold_or_later("needs `rustc` **1.88 or later**."),
        Some("1.88".into())
    );
    assert_eq!(word_to_number("seven"), Some(7));
    assert_eq!(word_to_number("eleven"), None);
    assert_eq!(
        number_before(
            "peisear is split into six implementation crates plus a thin facade",
            " implementation crates plus a thin facade"
        ),
        Some(6)
    );
    // The subset sentences rule 1b must not match: no fixed phrase shape
    // appears in either, so none of the extractors find anything to flag.
    assert_eq!(
        implementation_crate_count_intro("Depends on five of the six implementation crates"),
        None
    );
    assert_eq!(
        implementation_crate_count_heading("four of the other five crates depend on it"),
        None
    );
    assert_eq!(
        implementation_crate_count_heading("## The six implementation crates"),
        Some(6)
    );
    assert_eq!(
        published_crate_count("containing seven published crates plus static assets"),
        Some(7)
    );
    assert_eq!(
        readme_index_crate_count("why there are\n  seven crates and what belongs in each"),
        Some(7)
    );
    assert_eq!(
        verify_block_version("# rustc 1.88.x ...   (or newer)", "rustc"),
        Some("1.88".into())
    );
    assert_eq!(
        literal_string_default(
            "env::var(\"DATABASE_URL\").unwrap_or_else(|_| \"sqlite://data/app.db\".to_string());",
            "DATABASE_URL"
        ),
        Some("sqlite://data/app.db".into())
    );
    assert_eq!(
        literal_bool_default(
            "env::var(\"COOKIE_SECURE\").map(|v| v==\"1\").unwrap_or(false)",
            "COOKIE_SECURE"
        ),
        Some(false)
    );
    assert_eq!(
        table_default("| `BIND_ADDR` | `0.0.0.0:3000` | notes |", "BIND_ADDR"),
        Some("0.0.0.0:3000".into())
    );
    assert_eq!(
        manifest_static_files(
            "## What to ship\n\n```\n/var/lib/peisear/\n├── peisear\n└── static/\n    ├── app.css\n    └── board.js\n```\n"
        ),
        vec!["app.css".to_string(), "board.js".to_string()]
    );
    assert_eq!(
        raw_url_paths(
            "![a](https://github.com/nabbisen/peisear/raw/main/docs/a.png) and [b](https://github.com/nabbisen/peisear/raw/main/docs/b.png \"title\")"
        ),
        vec!["docs/a.png".to_string(), "docs/b.png".to_string()]
    );
    // `DOCS-008` review, F-1: rule 6 passes vacuously if `RAW_URL_PREFIX`
    // ever stops matching anything in README.md (a renamed repository, a
    // host change) -- the plant test can't catch that, since it builds
    // its own URL from the same literal. This is the guard: the real
    // README.md must still contain at least one URL the prefix matches.
    assert!(
        !raw_url_paths(&real().readme).is_empty(),
        "README.md has no raw_url_paths() match -- rule 6 would pass by matching nothing"
    );
}

// ---------- rule 5: no document, RFC, or published doc comment cites a
// gitignored path (`DOC-002`, `RFC 0019`) ----------
//
// Narrowed from "no committed file cites the path," which could never
// pass: that version banned `docs/specification/README.md`'s own
// sentence defining the convention, and it banned every handoff, which
// is an execution record, not documentation. The rule that matches the
// defect `RFC 0019` actually found: no document under `docs/`, no RFC,
// and no published doc comment cites `.git-exclude/`.
//
// A pure filesystem walk, not a git shell-out. The old version needed
// `git`'s own tracked/ignored distinction because it scanned *every*
// tracked file, including source and config, and had to tell a
// genuinely untracked scratch file from one `.gitignore` merely
// excludes. This version scans three fixed, already-known locations
// (`docs/`, `rfcs/` minus `rfcs/handoffs/`, and `///`/`//!` lines under
// `crates/*/src/`), so there is nothing left for "no `.git` here" to
// mean -- the extracted-tree failure mode `REL-0.49.0` found the hard
// way does not exist for this rule any more.

/// This module's own path, repo-root-relative. The rule explains
/// itself in its own doc comments and test expectations, which
/// necessarily contain the string `.git-exclude` -- without this
/// exclusion the rule could never reach zero, because landing the file
/// that enforces the count raises it.
const SELF_PATH: &str = "crates/peisear-web/src/docs_scan.rs";

/// The other three named exceptions, each for its own reason: both
/// *define* the convention they are quoted for (one documents what
/// `.git-exclude/` is, in a table about what is normative; the other is
/// `RFC 0019` itself, which proposed this very rule and quotes the
/// paths it governs -- the same self-reference as `SELF_PATH`, one
/// directory over). If this list needs a fifth entry, that is `§10.36`'s
/// own shape: a rule whose exception list keeps growing is a rule whose
/// subject is narrower than its name.
const GIT_EXCLUDE_RULE_EXCEPTIONS: &[&str] = &[
    "docs/specification/README.md",
    "rfcs/accepted/019-what-is-in-the-repository.md",
];

/// True if `rel` (repo-root-relative, forward slashes) is in rule 5's
/// population: a markdown file under `docs/` or `rfcs/`, excluding
/// `rfcs/handoffs/` (execution records, not documentation), the frozen
/// `docs/specification/history/` snapshots, and the four named
/// exceptions above.
fn in_rule_5_scope(rel: &str) -> bool {
    if rel == SELF_PATH || GIT_EXCLUDE_RULE_EXCEPTIONS.contains(&rel) {
        return false;
    }
    if rel.starts_with("docs/specification/history/") || rel.starts_with("rfcs/handoffs/") {
        return false;
    }
    rel.ends_with(".md") && (rel.starts_with("docs/") || rel.starts_with("rfcs/"))
}

/// Every markdown file under `docs/` and `rfcs/` that [`in_rule_5_scope`]
/// admits, as `(repo-root-relative path, text)`.
fn docs_and_rfcs_files() -> Vec<(String, String)> {
    let root = repo_root();
    let mut out = Vec::new();
    walk_rule_5_markdown(&root, &root.join("docs"), &mut out);
    walk_rule_5_markdown(&root, &root.join("rfcs"), &mut out);
    out.sort();
    out
}

fn walk_rule_5_markdown(root: &Path, dir: &Path, out: &mut Vec<(String, String)>) {
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
            if rel == "rfcs/handoffs" || rel == "docs/specification/history" {
                continue;
            }
            walk_rule_5_markdown(root, &p, out);
        } else if in_rule_5_scope(&rel)
            && let Ok(text) = fs::read_to_string(&p)
        {
            out.push((rel, text));
        }
    }
}

/// Every `///` or `//!` line across `crates/*/src/` -- what rustdoc, and
/// so docs.rs, actually reads. A `//` implementation comment never
/// reaches a published crate's documentation and is out of this rule's
/// scope (`DOC-002` §5).
fn published_doc_comment_lines() -> Vec<(String, String)> {
    let root = repo_root();
    let mut out = Vec::new();
    let Ok(crates) = fs::read_dir(root.join("crates")) else {
        return out;
    };
    for c in crates.flatten() {
        walk_rule_5_doc_comments(&root, &c.path().join("src"), &mut out);
    }
    out.sort();
    out
}

fn walk_rule_5_doc_comments(root: &Path, dir: &Path, out: &mut Vec<(String, String)>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            walk_rule_5_doc_comments(root, &p, out);
            continue;
        }
        if p.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }
        let rel = p
            .strip_prefix(root)
            .expect("under root")
            .to_string_lossy()
            .replace('\\', "/");
        if rel == SELF_PATH {
            continue;
        }
        let Ok(text) = fs::read_to_string(&p) else {
            continue;
        };
        let doc_lines: Vec<&str> = text
            .lines()
            .filter(|l| {
                let t = l.trim_start();
                t.starts_with("///") || t.starts_with("//!")
            })
            .collect();
        if !doc_lines.is_empty() {
            out.push((rel, doc_lines.join("\n")));
        }
    }
}

/// Occurrences, not distinct paths.
fn git_exclude_citations(files: &[(String, String)]) -> usize {
    files
        .iter()
        .map(|(_, text)| text.matches(".git-exclude").count())
        .sum()
}

#[test]
fn a_planted_git_exclude_citation_is_counted() {
    let files = vec![(
        "x.md".to_string(),
        "see .git-exclude/foo and .git-exclude/bar".to_string(),
    )];
    assert_eq!(git_exclude_citations(&files), 2);
}

/// `DOC-002` §7: both halves of the scope boundary, proven on the
/// literal paths the rule reasons about -- a document in scope, a
/// handoff that must not be, and all four named exceptions.
#[test]
fn rule_5_scope_catches_a_document_and_excludes_a_handoff() {
    assert!(in_rule_5_scope("docs/architecture/workspace-layout.md"));
    assert!(in_rule_5_scope("rfcs/done/999-example.md"));
    assert!(
        !in_rule_5_scope("rfcs/handoffs/999-example/README.md"),
        "a handoff is an execution record, not documentation"
    );
    assert!(!in_rule_5_scope(SELF_PATH));
    assert!(!in_rule_5_scope("docs/specification/README.md"));
    assert!(!in_rule_5_scope(
        "docs/specification/history/peisear-0.20.0-requirements-en.md"
    ));
    assert!(!in_rule_5_scope(
        "rfcs/accepted/019-what-is-in-the-repository.md"
    ));
}

/// Both halves again, this time against the real tree rather than
/// literal strings -- `rule_5_scope_catches_a_document_and_excludes_a_
/// handoff` proves the predicate; this proves the *walker* applies it
/// correctly to what is actually on disk, which a predicate test alone
/// cannot: a directory-traversal bug would still pass that one.
#[test]
fn rule_5_population_includes_docs_and_rfcs_but_excludes_handoffs() {
    let files = docs_and_rfcs_files();
    assert!(
        files.iter().any(|(p, _)| p.starts_with("docs/")),
        "expected at least one real docs/ file in rule 5's population"
    );
    assert!(
        files
            .iter()
            .any(|(p, _)| p.starts_with("rfcs/") && !p.starts_with("rfcs/handoffs/")),
        "expected at least one real rfcs/ file (not a handoff) in rule 5's population"
    );
    assert!(
        !files.iter().any(|(p, _)| p.starts_with("rfcs/handoffs/")),
        "rfcs/handoffs/ is an execution record, not documentation, and must not appear"
    );
    assert!(
        !files
            .iter()
            .any(|(p, _)| p == "docs/specification/README.md"),
        "the definition row's own document must be excepted"
    );
}

/// Live, not `#[ignore]`d: `DOC-002` closed the real gap (19 citations,
/// 15 of them resolved to a rewrite or a cut, four kept as the
/// exceptions above), so the rule that could never pass before now can,
/// and is enforced rather than merely reported.
#[test]
fn no_document_rfc_or_published_doc_comment_cites_git_exclude() {
    let mut files = docs_and_rfcs_files();
    files.extend(published_doc_comment_lines());
    let count = git_exclude_citations(&files);
    assert_eq!(
        count, 0,
        "{count} citations to .git-exclude/ remain outside the named exceptions"
    );
}
