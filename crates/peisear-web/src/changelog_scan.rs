//! The changelog policy, checked (`docs/development/changelog-and-releases.md`).
//!
//! `CHANGELOG.md` is the one place release notes live, and it stays a readable
//! size by holding **the current series only**: older series are *moved*, never
//! copied, into one file each under `changelog/`. That arrangement is easy to
//! break silently -- a version left in two files, an archive nothing links to, a
//! document linking to a section that has since moved -- so the rules are read
//! from the files rather than trusted:
//!
//! 1. **Each version appears in exactly one file** across `CHANGELOG.md` and
//!    `changelog/*.md`.
//! 2. **`CHANGELOG.md` holds only the current series** -- the series of the
//!    workspace version. Before 1.0.0 a series is ten minor versions
//!    (0.1-0.9, 0.10-0.19, ...); from 1.0.0 it is one major version.
//! 3. **Each archive file holds one whole series**, the one its name says, and
//!    no `[Unreleased]`.
//! 4. **Every archive is linked from `CHANGELOG.md`, and every relative link in
//!    `CHANGELOG.md` resolves.**
//! 5. **No markdown file links to a section that has moved**: a link into
//!    `CHANGELOG.md` or `changelog/*.md` with a `#fragment` must name a section
//!    that is in that file.
//! 6. **The released version has a dated section**, and **from `0.41.0` every
//!    dated section opens with `### Highlights`** (before any other `###`
//!    heading). Earlier sections predate the rule and are not rewritten.
//!
//! **What this does not check**, so nobody reads more into a green run: that an
//! external URL (including the release-notes link in a tag message) resolves --
//! nothing here goes to the network; that a tag's message carries the link --
//! that is the release procedure, done by hand and written in the policy; that
//! the Highlights are *good*; and links written in anything but inline markdown
//! form `[text](target)` (reference-style definitions are not read).
//!
//! The rules are a pure function of text ([`violations`]) so each can be
//! **planted** by editing the real files in memory: a rule never seen to fail
//! is not evidence, and every one below has a test that breaks it.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Component, Path, PathBuf};

type Version = (u32, u32, u32);

/// The first version whose section must open with `### Highlights`.
const HIGHLIGHTS_FROM: Version = (0, 41, 0);

fn parse_version(s: &str) -> Option<Version> {
    let mut parts = s.split('.');
    let v = (
        parts.next()?.parse().ok()?,
        parts.next()?.parse().ok()?,
        parts.next()?.parse().ok()?,
    );
    parts.next().is_none().then_some(v)
}

/// The series a version belongs to: `(major, 0)` from 1.0.0; before it,
/// `(0, minor / 10)`.
fn series_of(v: Version) -> (u32, u32) {
    if v.0 == 0 { (0, v.1 / 10) } else { (v.0, 0) }
}

/// The archive file stem for a series (`changelog/<stem>.md`).
fn series_stem(s: (u32, u32)) -> String {
    match s {
        (0, 0) => "0.1-0.9".to_string(),
        (0, d) => format!("0.{}0-0.{}9", d, d),
        (major, _) => format!("{major}.x"),
    }
}

/// GitHub's heading anchor: lowercase, keep alphanumerics, spaces, hyphens and
/// underscores, spaces become hyphens.
fn slug(heading: &str) -> String {
    heading
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
        .map(|c| if c == ' ' { '-' } else { c })
        .collect()
}

#[derive(Debug)]
struct Section {
    /// `None` for `[Unreleased]`.
    version: Option<Version>,
    label: String,
    dated: bool,
    first_h3: Option<String>,
    slug: String,
}

/// Every `## [..]` section of a changelog file, outside code fences.
fn sections(text: &str) -> Vec<Section> {
    let mut out: Vec<Section> = Vec::new();
    let mut fenced = false;
    for line in text.lines() {
        if line.trim_start().starts_with("```") {
            fenced = !fenced;
            continue;
        }
        if fenced {
            continue;
        }
        if let Some(rest) = line.strip_prefix("## [") {
            let Some(close) = rest.find(']') else {
                continue;
            };
            let label = rest[..close].to_string();
            let tail = &rest[close + 1..];
            out.push(Section {
                version: parse_version(&label),
                dated: has_iso_date_after_dash(tail),
                first_h3: None,
                slug: slug(line.trim_start_matches('#').trim()),
                label,
            });
        } else if line.starts_with("## ") {
            // Some other level-2 heading ("Earlier series"): ends the section.
            out.push(Section {
                version: None,
                label: String::new(),
                dated: false,
                first_h3: None,
                slug: slug(line.trim_start_matches('#').trim()),
            });
        } else if line.starts_with("### ")
            && let Some(last) = out.last_mut()
            && last.first_h3.is_none()
        {
            last.first_h3 = Some(line.trim().to_string());
        }
    }
    out.retain(|s| !s.label.is_empty() || s.version.is_some());
    out
}

/// ` — 2026-09-25` after the closing bracket.
fn has_iso_date_after_dash(tail: &str) -> bool {
    let t = tail.trim_start();
    let t = t
        .strip_prefix('—')
        .or_else(|| t.strip_prefix('-'))
        .unwrap_or(t)
        .trim_start();
    let b: Vec<char> = t.chars().take(10).collect();
    b.len() == 10
        && b.iter().enumerate().all(|(i, c)| match i {
            4 | 7 => *c == '-',
            _ => c.is_ascii_digit(),
        })
}

/// Inline markdown link targets `](target)`, outside code fences.
fn link_targets(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut fenced = false;
    for line in text.lines() {
        if line.trim_start().starts_with("```") {
            fenced = !fenced;
            continue;
        }
        if fenced {
            continue;
        }
        let mut rest = line;
        while let Some(at) = rest.find("](") {
            let after = &rest[at + 2..];
            let Some(end) = after.find(')') else { break };
            out.push(after[..end].trim().to_string());
            rest = &after[end + 1..];
        }
    }
    out
}

/// Resolve `target` (a relative link with an optional `#fragment`) against the
/// directory of `from`, to a repo-root-relative path and the fragment.
fn resolve(from: &str, target: &str) -> Option<(String, Option<String>)> {
    if target.is_empty()
        || target.starts_with('#')
        || target.contains("://")
        || target.starts_with("mailto:")
    {
        return None;
    }
    let (path, frag) = match target.split_once('#') {
        Some((p, f)) => (p, Some(f.to_string())),
        None => (target, None),
    };
    let mut buf = PathBuf::from(from);
    buf.pop();
    for c in Path::new(path).components() {
        match c {
            Component::ParentDir => {
                buf.pop();
            }
            Component::Normal(n) => buf.push(n),
            _ => {}
        }
    }
    Some((buf.to_string_lossy().replace('\\', "/"), frag))
}

pub(crate) struct Input<'a> {
    /// The text of `CHANGELOG.md`.
    pub changelog: &'a str,
    /// `(file name, text)` for each `changelog/*.md`.
    pub archives: Vec<(String, String)>,
    /// The workspace package version.
    pub workspace_version: Version,
    /// Does this repo-root-relative path exist?
    pub exists: &'a dyn Fn(&str) -> bool,
    /// `(repo-relative path, text)` of every other markdown file.
    pub docs: Vec<(String, String)>,
}

/// Every rule breach, each a sentence saying which rule and where.
pub(crate) fn violations(input: &Input) -> Vec<String> {
    let mut out = Vec::new();
    let current = series_of(input.workspace_version);
    let main_sections = sections(input.changelog);

    // Rule 1: each version in exactly one file.
    let mut homes: BTreeMap<Version, Vec<String>> = BTreeMap::new();
    for s in &main_sections {
        if let Some(v) = s.version {
            homes.entry(v).or_default().push("CHANGELOG.md".into());
        }
    }
    for (name, text) in &input.archives {
        for s in sections(text) {
            if let Some(v) = s.version {
                homes
                    .entry(v)
                    .or_default()
                    .push(format!("changelog/{name}"));
            }
        }
    }
    for (v, files) in &homes {
        if files.len() != 1 {
            out.push(format!(
                "rule 1: version {}.{}.{} appears {} times ({})",
                v.0,
                v.1,
                v.2,
                files.len(),
                files.join(", ")
            ));
        }
    }

    // Rule 2: CHANGELOG.md holds only the current series.
    for s in &main_sections {
        if let Some(v) = s.version
            && series_of(v) != current
        {
            out.push(format!(
                "rule 2: CHANGELOG.md holds {}.{}.{}, which is not in the current series ({}); it belongs in changelog/{}.md",
                v.0,
                v.1,
                v.2,
                series_stem(current),
                series_stem(series_of(v))
            ));
        }
    }

    // Rule 3: each archive is one whole series, the one its name says.
    for (name, text) in &input.archives {
        let stem = name.strip_suffix(".md").unwrap_or(name);
        for s in sections(text) {
            match s.version {
                None if s.label == "Unreleased" => out.push(format!(
                    "rule 3: changelog/{name} has an [Unreleased] section"
                )),
                Some(v) if series_stem(series_of(v)) != stem => out.push(format!(
                    "rule 3: changelog/{name} holds {}.{}.{}, which belongs to {}",
                    v.0,
                    v.1,
                    v.2,
                    series_stem(series_of(v))
                )),
                _ => {}
            }
        }
    }

    // Rule 4: every archive linked from CHANGELOG.md; every relative link resolves.
    let targets = link_targets(input.changelog);
    let resolved: Vec<(String, Option<String>)> = targets
        .iter()
        .filter_map(|t| resolve("CHANGELOG.md", t))
        .collect();
    for (name, _) in &input.archives {
        let want = format!("changelog/{name}");
        if !resolved.iter().any(|(p, _)| *p == want) {
            out.push(format!(
                "rule 4: CHANGELOG.md does not link changelog/{name}"
            ));
        }
    }
    for (path, _) in &resolved {
        if !(input.exists)(path) {
            out.push(format!(
                "rule 4: CHANGELOG.md links {path}, which does not exist"
            ));
        }
    }

    // Rule 5: no link into a changelog file names a section that is not in it.
    let mut slugs: BTreeMap<String, Vec<String>> = BTreeMap::new();
    slugs.insert(
        "CHANGELOG.md".into(),
        main_sections.iter().map(|s| s.slug.clone()).collect(),
    );
    for (name, text) in &input.archives {
        slugs.insert(
            format!("changelog/{name}"),
            sections(text).into_iter().map(|s| s.slug).collect(),
        );
    }
    let mut all_docs: Vec<(String, &str)> = input
        .docs
        .iter()
        .map(|(p, t)| (p.clone(), t.as_str()))
        .collect();
    all_docs.push(("CHANGELOG.md".into(), input.changelog));
    for (name, text) in &input.archives {
        all_docs.push((format!("changelog/{name}"), text.as_str()));
    }
    for (from, text) in all_docs {
        for target in link_targets(text) {
            let Some((path, Some(frag))) = resolve(&from, &target) else {
                continue;
            };
            if let Some(known) = slugs.get(&path)
                && !known.contains(&frag)
            {
                out.push(format!(
                    "rule 5: {from} links {path}#{frag}, but no such section is in that file (moved?)"
                ));
            }
        }
    }

    // Rule 6: the released version is dated; from HIGHLIGHTS_FROM, sections open with Highlights.
    match main_sections
        .iter()
        .find(|s| s.version == Some(input.workspace_version))
    {
        None => out.push(format!(
            "rule 6: no section for the workspace version {}.{}.{} in CHANGELOG.md",
            input.workspace_version.0, input.workspace_version.1, input.workspace_version.2
        )),
        Some(s) if !s.dated => out.push(format!("rule 6: the section for {} has no date", s.label)),
        Some(_) => {}
    }
    for s in &main_sections {
        if let Some(v) = s.version
            && v >= HIGHLIGHTS_FROM
            && s.dated
            && s.first_h3.as_deref() != Some("### Highlights")
        {
            out.push(format!(
                "rule 6: the section for {} must open with `### Highlights` (its first ### is {:?})",
                s.label, s.first_h3
            ));
        }
    }
    out
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn workspace_version() -> Version {
    let toml = fs::read_to_string(repo_root().join("Cargo.toml")).expect("read Cargo.toml");
    let after = &toml[toml
        .find("[workspace.package]")
        .expect("[workspace.package] in Cargo.toml")..];
    let line = after
        .lines()
        .find(|l| l.trim_start().starts_with("version"))
        .expect("workspace version line");
    let v = line.split('"').nth(1).expect("quoted version");
    parse_version(v).expect("semantic version")
}

/// Every markdown file in the repo other than the changelog files, skipping
/// build output, git internals and the untracked working area.
fn other_markdown() -> Vec<(String, String)> {
    fn walk(root: &Path, dir: &Path, out: &mut Vec<(String, String)>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for e in entries.flatten() {
            let p = e.path();
            let name = e.file_name().to_string_lossy().to_string();
            if p.is_dir() {
                if matches!(
                    name.as_str(),
                    "target" | ".git" | ".git-exclude" | "node_modules"
                ) {
                    continue;
                }
                walk(root, &p, out);
            } else if name.ends_with(".md") {
                let rel = p
                    .strip_prefix(root)
                    .expect("under root")
                    .to_string_lossy()
                    .replace('\\', "/");
                if rel == "CHANGELOG.md" || rel.starts_with("changelog/") {
                    continue;
                }
                if let Ok(text) = fs::read_to_string(&p) {
                    out.push((rel, text));
                }
            }
        }
    }
    let root = repo_root();
    let mut out = Vec::new();
    walk(&root, &root, &mut out);
    out.sort();
    out
}

struct Real {
    changelog: String,
    archives: Vec<(String, String)>,
    docs: Vec<(String, String)>,
    version: Version,
}

fn real() -> Real {
    let root = repo_root();
    let changelog = fs::read_to_string(root.join("CHANGELOG.md")).expect("read CHANGELOG.md");
    let mut archives = Vec::new();
    for e in fs::read_dir(root.join("changelog"))
        .expect("changelog/ exists")
        .flatten()
    {
        let name = e.file_name().to_string_lossy().to_string();
        if name.ends_with(".md") {
            archives.push((name, fs::read_to_string(e.path()).expect("read archive")));
        }
    }
    archives.sort();
    Real {
        changelog,
        archives,
        docs: other_markdown(),
        version: workspace_version(),
    }
}

fn run(r: &Real) -> Vec<String> {
    let root = repo_root();
    let exists = move |p: &str| root.join(p).exists();
    violations(&Input {
        changelog: &r.changelog,
        archives: r.archives.clone(),
        workspace_version: r.version,
        exists: &exists,
        docs: r.docs.clone(),
    })
}

fn only(v: Vec<String>, rule: &str) -> Vec<String> {
    v.into_iter().filter(|m| m.starts_with(rule)).collect()
}

#[test]
fn the_repository_satisfies_the_changelog_policy() {
    let r = real();
    assert!(
        !r.archives.is_empty(),
        "changelog/ must hold the older series, or rules 1 to 5 prove nothing"
    );
    let v = run(&r);
    assert!(
        v.is_empty(),
        "changelog policy violations:\n{}",
        v.join("\n")
    );
}

#[test]
fn a_version_in_two_files_is_found() {
    let mut r = real();
    let older = r.archives[0].1.clone();
    let dup: String = r
        .changelog
        .lines()
        .skip_while(|l| !l.starts_with("## [0.40.0]"))
        .take(2)
        .map(|l| format!("{l}\n"))
        .collect();
    r.archives[0].1 = format!("{older}\n{dup}");
    let v = run(&r);
    assert!(
        only(v, "rule 1").iter().any(|m| m.contains("0.40.0")),
        "a duplicated version must be reported"
    );
}

#[test]
fn an_old_section_left_in_the_changelog_is_found() {
    let mut r = real();
    r.changelog = r.changelog.replacen(
        "## Earlier series",
        "## [0.39.5] — 2026-01-01\n\n## Earlier series",
        1,
    );
    let v = only(run(&r), "rule 2");
    assert!(v.iter().any(|m| m.contains("0.39.5")), "got {v:?}");
}

#[test]
fn an_archive_holding_the_wrong_series_is_found() {
    let mut r = real();
    let i = r
        .archives
        .iter()
        .position(|(n, _)| n == "0.20-0.29.md")
        .expect("the 0.20-0.29 archive");
    r.archives[i].1.push_str("\n## [0.31.0] — 2026-01-01\n");
    let v = run(&r);
    assert!(
        only(v.clone(), "rule 3")
            .iter()
            .any(|m| m.contains("0.31.0")),
        "got {v:?}"
    );
}

#[test]
fn an_unlinked_archive_and_a_dead_link_are_found() {
    let mut r = real();
    r.changelog = r
        .changelog
        .replace("(changelog/0.30-0.39.md)", "(changelog/0.3-0.39.md)");
    let v = only(run(&r), "rule 4");
    assert!(
        v.iter()
            .any(|m| m.contains("does not link changelog/0.30-0.39.md")),
        "an unlinked archive must be reported: {v:?}"
    );
    assert!(
        v.iter()
            .any(|m| m.contains("changelog/0.3-0.39.md") && m.contains("does not exist")),
        "a dead link must be reported: {v:?}"
    );
}

#[test]
fn a_link_to_a_moved_section_is_found() {
    let mut r = real();
    r.docs.push((
        "docs/x.md".into(),
        "See [the 0.39.0 notes](../CHANGELOG.md#0390--2026-09-25).".into(),
    ));
    let v = only(run(&r), "rule 5");
    assert!(
        v.iter()
            .any(|m| m.contains("docs/x.md") && m.contains("0390--2026-09-25")),
        "got {v:?}"
    );

    // The same link into the archive that now holds it is fine, and so is a
    // link to a section that is still in CHANGELOG.md.
    let mut r = real();
    r.docs.push((
        "docs/x.md".into(),
        "[a](../changelog/0.30-0.39.md#0390--2026-09-25) [b](../CHANGELOG.md#0400--2026-09-25)"
            .into(),
    ));
    assert!(only(run(&r), "rule 5").is_empty());
}

#[test]
fn the_released_version_must_be_dated_and_new_sections_open_with_highlights() {
    let mut r = real();
    r.changelog = r
        .changelog
        .replace("## [0.40.0] — 2026-09-25", "## [0.40.0] — unreleased");
    let v = only(run(&r), "rule 6");
    assert!(v.iter().any(|m| m.contains("has no date")), "got {v:?}");

    // A 0.41.0 release without Highlights.
    let mut r = real();
    r.version = (0, 41, 0);
    r.changelog = r.changelog.replacen(
        "## [0.40.0]",
        "## [0.41.0] — 2026-10-01\n\n### Added\n\n- something\n\n## [0.40.0]",
        1,
    );
    let v = only(run(&r), "rule 6");
    assert!(
        v.iter()
            .any(|m| m.contains("0.41.0") && m.contains("Highlights")),
        "got {v:?}"
    );

    // And with them, it passes.
    let mut r = real();
    r.version = (0, 41, 0);
    r.changelog = r.changelog.replacen(
        "## [0.40.0]",
        "## [0.41.0] — 2026-10-01\n\n### Highlights\n\n- something\n\n### Added\n\n- something\n\n## [0.40.0]",
        1,
    );
    assert!(only(run(&r), "rule 6").is_empty());

    // A version bump with no section.
    let mut r = real();
    r.version = (0, 41, 0);
    assert!(
        only(run(&r), "rule 6")
            .iter()
            .any(|m| m.contains("no section"))
    );
}

#[test]
fn the_helpers_do_what_the_rules_assume() {
    assert_eq!(slug("[0.40.0] — 2026-09-25"), "0400--2026-09-25");
    assert_eq!(series_stem(series_of((0, 9, 0))), "0.1-0.9");
    assert_eq!(series_stem(series_of((0, 10, 1))), "0.10-0.19");
    assert_eq!(series_stem(series_of((0, 39, 0))), "0.30-0.39");
    assert_eq!(series_stem(series_of((0, 40, 0))), "0.40-0.49");
    assert_eq!(series_stem(series_of((2, 3, 4))), "2.x");
    assert_eq!(
        resolve("docs/README.md", "../CHANGELOG.md#a"),
        Some(("CHANGELOG.md".into(), Some("a".into())))
    );
    assert_eq!(resolve("CHANGELOG.md", "https://x/y"), None);
    assert!(has_iso_date_after_dash(" — 2026-09-25"));
    assert!(!has_iso_date_after_dash(" — initial release"));
    // A heading inside a code fence is not a section.
    assert!(sections("```\n## [1.0.0] — 2026-01-01\n```\n").is_empty());
}
