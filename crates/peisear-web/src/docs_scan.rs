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
//! 2. MSRV claims, against `Cargo.toml`'s `rust-version` --
//!    `build-from-source.md` (`installation.md` until `FIV-004` split
//!    it from the Install page the MSRV sentence has nothing to do
//!    with) and, since `DOC-004`, `.github/CONTRIBUTING.md`, which
//!    cites it but carried its own copy of the number and had drifted
//!    independently.
//! 3. `deployment.md`'s *What to ship* manifest against `static/`'s actual
//!    contents, exhaustively in both directions: nothing the manifest
//!    lists is missing from `static/`, and nothing in `static/` is
//!    missing from the manifest.
//! 4. Documented environment-variable defaults, against `config.rs`'s
//!    `unwrap_or_else`/`unwrap_or` fallbacks.
//! 5. No document under `docs/`, no RFC, and no published doc comment
//!    cites the private area -- not only its gitignored path spelled out,
//!    but a citation written as the bare review-file name (`` `QA-002-
//!    review.md` §3 ``), the form this project actually uses and the one
//!    the original rule never saw (`GRD-003`) -- four named exceptions.
//!    Its population (hundreds of files under `docs/`, `rfcs/`, and
//!    `crates/*/src/`) is too large for this module's [`Input`] to hold
//!    cheaply the way rules 1-4's few fixed documents can, so its
//!    functions and tests sit apart from the other four, at the end of
//!    this file.
//! 7. `.env.example`'s own uncommented `KEY=value` lines (`DIA-003`,
//!    0.54.0), against each key's own source -- `config.rs` for most,
//!    `main.rs`'s `tracing_subscriber::EnvFilter` fallback for
//!    `RUST_LOG`, `peisear-notify`'s `SmtpConfig` for `SMTP_PORT`. No
//!    tool here may read `.env*` directly; this rule's own test for
//!    the real file is the only reader it has, and the live assertion
//!    is deliberately plain -- see [`env_example_entries`]'s own doc
//!    comment for why a key with no fixed default (`JWT_SECRET`) is
//!    not the same case as one this rule simply does not recognise.
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
//! is an execution record, not documentation. `DOC-002` (`RFC 019`,
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
    for e in fs::read_dir(
        repo_root()
            .join("crates")
            .join("peisear-web")
            .join("static"),
    )
    .expect("crates/peisear-web/static/ exists")
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

// ---------- rule 7: .env.example vs each key's own default ----------

/// `RUST_LOG`'s literal fallback in `main.rs` -- a different shape from
/// [`literal_string_default`]'s `env::var("X").unwrap_or_else(...)`,
/// since `tracing_subscriber::EnvFilter::try_from_default_env()` reads
/// `RUST_LOG` itself; there is no `env::var("RUST_LOG")` call to anchor
/// on, only the fallback that runs when that read comes back empty.
fn rust_log_default(main_rs: &str) -> Option<String> {
    let marker = "try_from_default_env()";
    let idx = main_rs.find(marker)?;
    let rest = &main_rs[idx + marker.len()..];
    let window = &rest[..rest.len().min(120)];
    let start = window.find('"')? + 1;
    let after_start = &window[start..];
    let end = after_start.find('"')?;
    Some(after_start[..end].to_string())
}

/// `SMTP_PORT`'s literal numeric fallback in `peisear-notify`'s
/// `SmtpConfig::from_env` -- `.unwrap_or(465)`, not a quoted string, so
/// [`literal_string_default`] does not fit this one either.
fn smtp_port_default(notify_config_rs: &str) -> Option<u16> {
    let marker = "env::var(\"SMTP_PORT\")";
    let idx = notify_config_rs.find(marker)?;
    let rest = &notify_config_rs[idx + marker.len()..];
    let window = &rest[..rest.len().min(200)];
    let marker2 = ".unwrap_or(";
    let idx2 = window.find(marker2)?;
    let after = &window[idx2 + marker2.len()..];
    let end = after.find(')')?;
    after[..end].trim().parse().ok()
}

/// Every uncommented `KEY=value` line in `text` -- `.env.example`'s own
/// shape, and the only one this rule anticipates (`DIA-003` §3): a
/// `#`-prefixed line, after trimming, is documentation and out of
/// scope; everything else is read as `key=value` with no quoting, no
/// multi-line values, and no interpolation, because none of those
/// exist in this file today and this is not a general `.env` parser.
///
/// **Why `JWT_SECRET` is not simply "unknown" or "no default to
/// check."** Every other key either has a literal default this rule
/// can compare against, or has no default *by design* (`SMTP_HOST`
/// and its required siblings have none at all; `SMTP_TLS_MODE`
/// derives from `SMTP_PORT` rather than naming one) -- for those, an
/// uncommented value is not overriding anything, so there is nothing
/// for this rule to compare and it passes. `JWT_SECRET` is neither: it
/// has no literal default either, but its *correct* state when unset
/// is "generate one," and that is exactly the behaviour an uncommented
/// value silently replaces. The violation is not a mismatched value --
/// it is the line being uncommented at all, which is what the original
/// defect (`§10.40`, 0.53.0) actually was.
fn env_example_entries(text: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some((key, value)) = trimmed.split_once('=') else {
            continue;
        };
        out.push((key.trim().to_string(), value.trim().to_string()));
    }
    out
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
    pub build_from_source: &'a str,
    pub contributing: &'a str,
    pub static_files: Vec<String>,
    pub deployment: &'a str,
    pub configuration: &'a str,
    pub config_rs: &'a str,
    /// Does this repo-root-relative path exist? (rule 6 only.)
    pub exists: &'a dyn Fn(&str) -> bool,
    pub env_example: &'a str,
    pub main_rs: &'a str,
    pub notify_config_rs: &'a str,
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

    // Rule 2, live: MSRV claims match Cargo.toml -- build-from-source.md
    // (installation.md until FIV-004 split the page; the MSRV sentence
    // moved with it, since it's a build-time requirement, not something
    // the Install page's three paths need) and, since DOC-004,
    // CONTRIBUTING.md, which cites it but carries its own copy of the
    // number and drifted independently (`DOC-004` found it at 1.85
    // while Cargo.toml had moved to 1.88). Reworded to the same
    // `**X.Y or later**` shape so one parser serves both documents
    // rather than adding a second pattern.
    let msrv = rust_version_major_minor(input.cargo_toml);
    for (doc_name, doc) in [
        ("build-from-source.md", input.build_from_source),
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
        match verify_block_version(input.build_from_source, tool) {
            Some(v) if v != msrv => out.push(format!(
                "rule 2: build-from-source.md's Verify block shows {tool} {v}.x, Cargo.toml says {msrv}"
            )),
            Some(_) => {}
            None => out.push(format!(
                "rule 2: build-from-source.md's Verify block has no `# {tool} X.Y.x` line to check"
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

    // Rule 4, live: documented defaults match config.rs. Each lookup
    // against config.rs's own literal text is asserted present before
    // it is trusted -- `WIZ-002`'s own review: `None` here used to
    // make the `if let` below simply not match, so a refactor that
    // moved the literal (a shared constant, a different call shape)
    // left this rule reporting zero violations while reading nothing,
    // `§10.36`'s own class.
    for var in ["DATABASE_URL", "BIND_ADDR"] {
        let Some(actual) = literal_string_default(input.config_rs, var) else {
            out.push(format!(
                "rule 4: no literal default found in config.rs for {var} -- has the call shape changed?"
            ));
            continue;
        };
        if let Some(documented) = table_default(input.configuration, var)
            && actual != documented
        {
            out.push(format!(
                "rule 4: configuration.md documents {var}'s default as `{documented}`, config.rs's is `{actual}`"
            ));
        }
    }
    match literal_bool_default(input.config_rs, "COOKIE_SECURE") {
        None => out.push(
            "rule 4: no literal default found in config.rs for COOKIE_SECURE -- has the call shape changed?"
                .into(),
        ),
        Some(actual) => {
            if let Some(documented) = table_default(input.configuration, "COOKIE_SECURE") {
                let actual_str = if actual { "1" } else { "0" };
                if actual_str != documented {
                    out.push(format!(
                        "rule 4: configuration.md documents COOKIE_SECURE's default as `{documented}`, config.rs's is `{actual_str}`"
                    ));
                }
            }
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

    // Rule 7: every uncommented .env.example key matches its own
    // source's default -- see env_example_entries's own doc comment
    // for JWT_SECRET's special case and why the SMTP siblings listed
    // alongside it are not treated the same way.
    for (key, value) in env_example_entries(input.env_example) {
        match key.as_str() {
            "DATABASE_URL" | "BIND_ADDR" => match literal_string_default(input.config_rs, &key) {
                Some(default) if default != value => out.push(format!(
                    "rule 7: .env.example sets {key}={value}, config.rs's default is {default}"
                )),
                Some(_) => {}
                None => out.push(format!(
                    "rule 7: .env.example sets {key}, but config.rs has no literal default for it to check against"
                )),
            },
            "COOKIE_SECURE" => match literal_bool_default(input.config_rs, "COOKIE_SECURE") {
                Some(default) => {
                    let default_str = if default { "1" } else { "0" };
                    if default_str != value {
                        out.push(format!(
                            "rule 7: .env.example sets COOKIE_SECURE={value}, config.rs's default is {default_str}"
                        ));
                    }
                }
                None => out.push(
                    "rule 7: .env.example sets COOKIE_SECURE, but config.rs has no literal default for it to check against"
                        .into(),
                ),
            },
            "RUST_LOG" => match rust_log_default(input.main_rs) {
                Some(default) if default != value => out.push(format!(
                    "rule 7: .env.example sets RUST_LOG={value}, main.rs's default is {default}"
                )),
                Some(_) => {}
                None => out.push(
                    "rule 7: .env.example sets RUST_LOG, but main.rs has no literal default for it to check against"
                        .into(),
                ),
            },
            "SMTP_PORT" => match smtp_port_default(input.notify_config_rs) {
                Some(default) if default.to_string() != value => out.push(format!(
                    "rule 7: .env.example sets SMTP_PORT={value}, peisear-notify's default is {default}"
                )),
                Some(_) => {}
                None => out.push(
                    "rule 7: .env.example sets SMTP_PORT, but peisear-notify has no literal default for it to check against"
                        .into(),
                ),
            },
            "JWT_SECRET" => out.push(
                "rule 7: .env.example sets JWT_SECRET (uncommented) -- it has no default to \
                 match (unset generates one on first run), so an uncommented value here is \
                 itself the violation, not a mismatch against one"
                    .to_string(),
            ),
            "SMTP_HOST" | "SMTP_USER" | "SMTP_PASSWORD" | "SMTP_FROM_ADDRESS"
            | "SMTP_FROM_NAME" | "SMTP_TLS_MODE" => {
                // Known to peisear-notify's SmtpConfig::from_env, and
                // deliberately not checked: the first four are
                // required with no fallback at all, FROM_NAME is
                // optional with none, and TLS_MODE derives from
                // SMTP_PORT rather than naming its own literal -- none
                // of the six has a fixed default this rule could
                // compare a value against.
            }
            other => out.push(format!(
                "rule 7: .env.example sets {other}, which no known source recognises"
            )),
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
    build_from_source: String,
    contributing: String,
    static_files: Vec<String>,
    deployment: String,
    configuration: String,
    config_rs: String,
    env_example: String,
    main_rs: String,
    notify_config_rs: String,
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
        build_from_source: fs::read_to_string(
            root.join("docs/getting-started/build-from-source.md"),
        )
        .expect("read build-from-source.md"),
        contributing: fs::read_to_string(root.join(".github/CONTRIBUTING.md"))
            .expect("read CONTRIBUTING.md"),
        static_files: static_files(),
        deployment: fs::read_to_string(root.join("docs/operations/deployment.md"))
            .expect("read deployment.md"),
        configuration: fs::read_to_string(root.join("docs/getting-started/configuration.md"))
            .expect("read configuration.md"),
        config_rs: fs::read_to_string(root.join("crates/peisear-web/src/config.rs"))
            .expect("read config.rs"),
        // `.env.example` is public, committed, and part of the
        // documented start path -- not a `.env*` runtime config file.
        // No tool outside a compiled test may read it (DIA-003 §1);
        // this `fs::read_to_string` runs inside `cargo test`, the one
        // reader this rule can have.
        env_example: fs::read_to_string(root.join(".env.example")).expect("read .env.example"),
        main_rs: fs::read_to_string(root.join("crates/peisear/src/main.rs")).expect("read main.rs"),
        notify_config_rs: fs::read_to_string(root.join("crates/peisear-notify/src/config.rs"))
            .expect("read peisear-notify's config.rs"),
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
        build_from_source: &r.build_from_source,
        contributing: &r.contributing,
        static_files: r.static_files.clone(),
        deployment: &r.deployment,
        configuration: &r.configuration,
        config_rs: &r.config_rs,
        exists: &exists,
        env_example: &r.env_example,
        main_rs: &r.main_rs,
        notify_config_rs: &r.notify_config_rs,
    })
}

fn only(v: Vec<String>, rule: &str) -> Vec<String> {
    v.into_iter().filter(|m| m.starts_with(rule)).collect()
}

// ---------- rule 5: no document, RFC, or published doc comment cites a
// gitignored path (`DOC-002`, `RFC 019`) ----------
//
// Narrowed from "no committed file cites the path," which could never
// pass: that version banned `docs/specification/README.md`'s own
// sentence defining the convention, and it banned every handoff, which
// is an execution record, not documentation. The rule that matches the
// defect `RFC 019` actually found: no document under `docs/`, no RFC,
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
/// `RFC 019` itself, which proposed this very rule and quotes the
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

/// Every `///` or `//!` line across `crates/*/src/`. A `//`
/// implementation comment never reaches a published crate's
/// documentation and is out of this rule's scope (`DOC-002` §5).
///
/// **A deliberate over-approximation of what rustdoc, and so docs.rs,
/// actually publishes, not a measurement of it** (`GRD-003 §3`): this
/// walks every `.rs` file under `crates/*/src/`, including the
/// `#[cfg(test)]`-gated scan modules and private submodules rustdoc
/// never renders. An over-approximation on a *ban* costs nothing --
/// there is no legitimate reason to write a citation of
/// `.git-exclude/` inside a test-only module either -- so this is not
/// narrowed to the published subset. Narrowing it would add attribute
/// parsing to a rule whose whole design is a filesystem walk, to
/// permit something nobody wants to write.
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

/// True if `c` can appear inside the identifier token this needle looks
/// for -- a letter, a digit, `.` (`REL-0.51.0`'s own dots) or `-`
/// (`DEV-001-004`'s own multi-hyphen shape).
fn is_review_id_token_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '.' || c == '-'
}

/// True if `prefix` -- the token immediately before a `-review`/
/// `-review.md` suffix -- has the shape this project's own private-area
/// naming uses: an uppercase ID (`[A-Z][A-Z0-9]*`) followed by `-` and a
/// digit. `TT-003`, `I18N-005c`, `QA-001`, `REL-0.51.0` and
/// `DEV-001-004` all qualify; an ordinary hyphenated prose word
/// (`mid-review`, `design-review`) does not, because its first segment
/// is lowercase -- `§10.x`'s own residual note: this is what keeps the
/// needle from needing an exception list for prose that merely contains
/// the word "review".
fn is_review_id_shape(prefix: &str) -> bool {
    let Some(dash) = prefix.find('-') else {
        return false;
    };
    let id = &prefix[..dash];
    let first_upper = id.chars().next().is_some_and(|c| c.is_ascii_uppercase());
    let id_shape = first_upper
        && !id.is_empty()
        && id
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit());
    if !id_shape {
        return false;
    }
    prefix[dash + 1..]
        .chars()
        .next()
        .is_some_and(|c| c.is_ascii_digit())
}

/// The private area's own naming suffixes this needle catches, each
/// tried with and without a trailing `.md` (`GRD-003 §4`).
///
/// **No census of that directory belongs here.** It is gitignored, it
/// grows every time a review is written, and no reader of this file can
/// check a number taken from it -- so a count copied in rots silently
/// and in the one direction that matters, by overstating coverage. What
/// this list is measured against is the *shape* below, not a tally: a
/// suffix is in this set because the naming convention uses it, and
/// [`is_review_id_shape`] decides the rest.
const REVIEW_FILE_SUFFIXES: &[&str] = &["review", "recommendation", "decisions", "decision"];

/// Every bare review-file citation in `text`: a token ending in one of
/// [`REVIEW_FILE_SUFFIXES`] (with or without a trailing `.md`), bounded
/// on the left by a non-identifier character (or the start of `text`)
/// and on the right by a non-alphanumeric character (or the end of
/// `text`) -- so `-reviewed`/`-reviewer`/`-reviewing` never match --
/// whose prefix has [`is_review_id_shape`]. This is the needle
/// `GRD-003` widens rule 5 with: a citation written as the bare file
/// name (`` `QA-002-review.md` §3 ``) spells no path at all, so the
/// literal `.git-exclude` match in [`git_exclude_citations`] has never
/// seen one.
///
/// **Stated residual, not discovered later**: a private file whose name
/// does not reach a digit immediately after its first hyphen is not
/// caught. That is a compound prefix (`DOCS-AUDIT-...`, `DM-TEST-...`,
/// `FR-DM-...`, `CHANGELOG-POLICY-...`) or a lowercase word in that
/// position, and as of 2026-10-14 it is **nine** of the files in the
/// private area, not the four the first measurement reported -- which
/// counted the files whose *suffix* this list matches and read that as
/// the needle's coverage. Five more end in a listed suffix and still
/// fail [`is_review_id_shape`]. Not catchable without an exception
/// list, which the rule's own design forbids growing. None is cited in
/// rule 5's population today -- checked, and demonstrated by planting
/// four of them in a doc comment and watching this rule still report
/// clean. If one ever is cited, this is where it goes uncaught.
fn review_file_citations(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for suffix in REVIEW_FILE_SUFFIXES {
        let needle = format!("-{suffix}");
        let mut search_from = 0;
        while let Some(rel) = text[search_from..].find(needle.as_str()) {
            let dash_pos = search_from + rel;
            let after_suffix = dash_pos + needle.len();
            let end = if text[after_suffix..].starts_with(".md") {
                after_suffix + 3
            } else {
                after_suffix
            };
            let right_boundary_ok = text[end..]
                .chars()
                .next()
                .is_none_or(|c| !c.is_ascii_alphanumeric());
            if right_boundary_ok {
                let mut start = dash_pos;
                while start > 0 {
                    let c = text[..start].chars().next_back().expect("non-empty");
                    if is_review_id_token_char(c) {
                        start -= c.len_utf8();
                    } else {
                        break;
                    }
                }
                let prefix = &text[start..dash_pos];
                if is_review_id_shape(prefix) {
                    out.push(text[start..end].to_string());
                }
            }
            search_from = after_suffix;
        }
    }
    out
}

/// Occurrences, not distinct IDs -- the same shape
/// [`git_exclude_citations`] reports in. Added to that count, this is
/// rule 5's own enforced total (`no_document_rfc_or_published_doc_
/// comment_cites_a_private_file`, below).
fn review_file_citation_count(files: &[(String, String)]) -> usize {
    files
        .iter()
        .map(|(_, text)| review_file_citations(text).len())
        .sum()
}

#[cfg(test)]
mod tests;
