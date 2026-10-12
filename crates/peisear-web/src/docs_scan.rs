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
//!    intended.
//! 2. `installation.md`'s MSRV claims, against `Cargo.toml`'s `rust-version`.
//! 3. `deployment.md`'s *What to ship* manifest against `static/`'s actual
//!    contents, exhaustively in both directions: nothing the manifest
//!    lists is missing from `static/`, and nothing in `static/` is
//!    missing from the manifest.
//! 4. Documented environment-variable defaults, against `config.rs`'s
//!    `unwrap_or_else`/`unwrap_or` fallbacks.
//! 5. Committed citations of `.git-exclude/`, a gitignored path. Its
//!    source of truth is `git`'s own tracked-file list, not a text
//!    fixture this module's [`Input`] can hold cheaply, so its functions
//!    and tests sit apart from the other four, at the end of this file.
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
    pub cargo_toml: &'a str,
    pub installation: &'a str,
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

    // Rule 2, live: installation.md's MSRV claims match Cargo.toml.
    let msrv = rust_version_major_minor(input.cargo_toml);
    match bold_or_later(input.installation) {
        Some(bold) if bold != msrv => out.push(format!(
            "rule 2: installation.md's \"{bold} or later\" does not match Cargo.toml's rust-version {msrv}"
        )),
        Some(_) => {}
        None => out
            .push("rule 2: installation.md has no \"X.Y or later\" MSRV sentence to check".into()),
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
    cargo_toml: String,
    installation: String,
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
        cargo_toml: fs::read_to_string(root.join("Cargo.toml")).expect("read Cargo.toml"),
        installation: fs::read_to_string(root.join("docs/getting-started/installation.md"))
            .expect("read installation.md"),
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
        cargo_toml: &r.cargo_toml,
        installation: &r.installation,
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
fn rule_2_catches_a_stale_msrv_sentence() {
    let mut r = real();
    r.installation = r.installation.replace("1.88 or later", "1.85 or later");
    let v = only(run(&r), "rule 2:");
    assert!(v.iter().any(|m| m.contains("1.85")), "got {v:?}");
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

// ---------- rule 5: committed citations of a gitignored path ----------
//
// `git` is the source of truth for "every committed file" -- a filesystem
// walk would have to reimplement `.gitignore` pattern matching to answer
// the same question, which `git ls-files` already does correctly. This is
// the one function in this module that shells out; every other scan in
// this crate walks the filesystem directly (see `changelog_scan`'s
// `other_markdown`), because none of them needs git's own tracked/ignored
// distinction -- this rule's whole subject *is* that distinction.

/// This module's own path, repo-root-relative -- excluded by
/// [`tracked_files`] (`DOCS-005` review, finding F-1). The rule explains
/// itself in its own doc comment and test expectations, which
/// necessarily contain the string `.git-exclude` -- without this
/// exclusion, the rule could never reach zero, because landing the file
/// that reports the count raises the count. `DOCS-006` is left to decide
/// what *else* may legitimately cite the path (a handoff discussing this
/// problem does, more than once); this exclusion only stops the rule
/// banning its own existence.
const SELF_PATH: &str = "crates/peisear-web/src/docs_scan.rs";

/// `repo_root()` has no `.git` of its own -- which `git -C <dir>
/// ls-files` does not require to answer *something*: git walks upward
/// and happily resolves against an *enclosing* repository if `dir` is a
/// subdirectory of one. That is exactly what a `git archive` extraction
/// under `.git-exclude/tmp/` triggers during release-candidate
/// verification: `.git-exclude/` is itself gitignored and nested inside
/// this very checkout, so `git -C <extracted-tree> ls-files` silently
/// answers against *this* repository and correctly reports nothing
/// tracked under a gitignored path -- which reads as "zero citations"
/// rather than "no git information is available here." Checked via
/// `.git`'s own presence (a file in a worktree, a directory in an
/// ordinary checkout; `.exists()` covers both) rather than shelling out
/// again, since the question is only "does this directory have its own
/// git metadata," not anything git itself needs to answer.
fn in_own_git_repo() -> bool {
    repo_root().join(".git").exists()
}

/// Every path `git` considers tracked, unfiltered -- the raw list
/// [`tracked_files`] filters `SELF_PATH` out of. Kept separate so a test
/// can assert against the *un*filtered list too (`DOCS-008` §3): a test
/// that only checks `SELF_PATH` is absent from the filtered output
/// passes vacuously if `SELF_PATH` is stale (the file renamed or moved),
/// since an absent path is trivially "not found" either way.
///
/// `None` when [`in_own_git_repo`] is false -- callers must not read an
/// empty `Vec` as "git ran and found nothing," because that is exactly
/// the silent-wrong-answer shape this function exists to avoid.
fn git_tracked_paths() -> Option<Vec<String>> {
    if !in_own_git_repo() {
        return None;
    }
    let root = repo_root();
    let output = std::process::Command::new("git")
        .arg("-C")
        .arg(&root)
        .arg("ls-files")
        .arg("-z")
        .output()
        .expect("git ls-files");
    Some(
        String::from_utf8_lossy(&output.stdout)
            .split('\0')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect(),
    )
}

fn tracked_files() -> Option<Vec<(String, String)>> {
    let root = repo_root();
    let mut out = Vec::new();
    for rel in git_tracked_paths()? {
        if rel == SELF_PATH {
            continue;
        }
        if let Ok(text) = fs::read_to_string(root.join(&rel)) {
            out.push((rel, text));
        }
    }
    Some(out)
}

/// Occurrences, not distinct paths -- matching `git grep -o '.git-exclude'`.
fn git_exclude_citations(files: &[(String, String)]) -> usize {
    files
        .iter()
        .map(|(_, text)| text.matches(".git-exclude").count())
        .sum()
}

/// Always runs, never fails: surfaces the count so a reader of `cargo test
/// -p peisear-web --lib -- --nocapture` sees it without the live suite
/// going red over a gap `DOCS-006` (0.50.0) is what's expected to close.
/// Reports plainly, rather than a false `0`, when run somewhere with no
/// git information for this repository (`REL-0.49.0`'s own extracted-tree
/// verification found this the hard way: an absent count silently read
/// as a zero count).
#[test]
fn git_exclude_citation_count_is_reported() {
    let Some(files) = tracked_files() else {
        println!(
            "docs_scan rule 5: no git information available for this repository here \
             (no `.git` at {:?} -- e.g. a `git archive` extraction) -- count not measured",
            repo_root()
        );
        return;
    };
    let count = git_exclude_citations(&files);
    println!(
        "docs_scan rule 5: {count} citations to `.git-exclude` in tracked files, \
         excluding this module's own ({SELF_PATH}) -- 130 at last measurement \
         (2026-10-11, DOCS-005 review's own count, after subtracting the 9 this \
         file necessarily carries), taken as a count of occurrences, not distinct \
         paths, which is why it moves release to release; DOCS-006 (0.50.0) is \
         expected to remove them"
    );
}

#[test]
fn a_planted_git_exclude_citation_is_counted() {
    let files = vec![(
        "x.md".to_string(),
        "see .git-exclude/foo and .git-exclude/bar".to_string(),
    )];
    assert_eq!(git_exclude_citations(&files), 2);
}

/// `DOCS-005` review, finding F-1: `tracked_files()` used to count this
/// module's own file, which cites `.git-exclude` nine times in its doc
/// comments and test expectations -- so the rule below could never reach
/// zero, even after `DOCS-006` closes the real gap.
///
/// `DOCS-008` review, finding F-3: the first version of this test only
/// asserted `SELF_PATH` was *absent* from `tracked_files()`'s filtered
/// output -- which also holds, vacuously, if `SELF_PATH` is stale (the
/// file renamed or moved), since the filtered list would never contain
/// the old path either way. The one failure mode this test exists to
/// catch is the one that shape can't see. Fixed to a positive assertion
/// first: `SELF_PATH` must be present in git's own *unfiltered* tracked
/// list, which fails loudly, naming the path, the moment a rename makes
/// `SELF_PATH` stale -- then the filtered list must exclude it.
///
/// Skips, rather than fails, when [`in_own_git_repo`] is false: this
/// test's own subject is git's tracked/untracked distinction, and
/// nothing can be proved about it without git information for this
/// repository to check against. Found during `REL-0.49.0`'s own
/// extracted-tree verification -- exactly the context this guard exists
/// for, not a hypothetical one.
#[test]
fn the_scan_does_not_count_its_own_file() {
    let Some(tracked) = git_tracked_paths() else {
        println!(
            "docs_scan: no git information available for this repository here -- \
             skipping (cannot prove the exclusion without it)"
        );
        return;
    };
    assert!(
        tracked.iter().any(|p| p == SELF_PATH),
        "{SELF_PATH} is not a tracked file -- if docs_scan.rs was renamed \
         or moved, update SELF_PATH to match, or this test cannot prove \
         the exclusion it exists to prove"
    );
    let files = tracked_files().expect("git_tracked_paths() just succeeded above");
    assert!(
        !files.iter().any(|(path, _)| path == SELF_PATH),
        "tracked_files() must exclude {SELF_PATH}, or rule 5 can never pass"
    );
}

/// Deferred to `DOCS-006` (0.50.0), the same way rule 5's own handoff text
/// proposes for itself (§1): "write it, and report its current count
/// rather than landing it failing... write it as a `#[ignore]`d test with
/// the count in its name." The count is in the ignore reason rather than
/// the test name because it is a measured, moving number, not a stable
/// identifier -- a name that goes stale the moment the count changes would
/// be its own small drift.
#[test]
#[ignore = "DOCS-006 (0.50.0) removes committed citations of `.git-exclude/`; \
            130 at last measurement (2026-10-11, this module's own excluded) -- \
            see docs_scan's rule 5 and this module's own doc comment"]
fn no_committed_file_cites_git_exclude() {
    let files = tracked_files()
        .expect("run this inside the repository's own git working tree, not an extracted one");
    let count = git_exclude_citations(&files);
    assert_eq!(
        count, 0,
        "{count} committed citations to .git-exclude/ remain"
    );
}
