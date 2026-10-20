use super::*;

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
    // Derived from the live workspace version, not a hardcoded string --
    // `REG-001`'s own sibling test in this file was written against
    // `0.40.0` and broke silently the moment `0.50.0`'s series rollover
    // moved that version out of `CHANGELOG.md` and into an archive
    // (`§10.36`'s own shape, found running this exact test at that
    // release). This duplicates the *current* release's own section into
    // an archive -- the realistic defect shape at any point in time.
    let mut r = real();
    let (maj, min, pat) = r.version;
    let heading_prefix = format!("## [{maj}.{min}.{pat}]");
    let older = r.archives[0].1.clone();
    let dup: String = r
        .changelog
        .lines()
        .skip_while(|l| !l.starts_with(&heading_prefix))
        .take(2)
        .map(|l| format!("{l}\n"))
        .collect();
    r.archives[0].1 = format!("{older}\n{dup}");
    let v = run(&r);
    assert!(
        only(v, "rule 1")
            .iter()
            .any(|m| m.contains(&format!("{maj}.{min}.{pat}"))),
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
    // link to a section that is still in CHANGELOG.md -- the live workspace
    // version's own heading, derived rather than hardcoded, for the same
    // reason as the sibling test above: `0400--2026-09-25` was that section
    // once and stopped being the moment 0.40.0 moved to an archive.
    let (maj, min, pat) = r.version;
    let heading_prefix = format!("## [{maj}.{min}.{pat}]");
    let live_slug = r
        .changelog
        .lines()
        .find(|l| l.starts_with(&heading_prefix))
        .map(|l| slug(l.trim_start_matches('#').trim()))
        .expect("the live version has a section in CHANGELOG.md");
    let mut r = real();
    r.docs.push((
        "docs/x.md".into(),
        format!("[a](../changelog/0.30-0.39.md#0390--2026-09-25) [b](../CHANGELOG.md#{live_slug})"),
    ));
    assert!(only(run(&r), "rule 5").is_empty());
}

#[test]
fn the_released_version_must_have_a_dated_section() {
    // Derived from the live workspace version rather than a hardcoded string,
    // so this test does not go stale at the next release the way its first
    // version did: it was written against 0.40.0's own dated heading and
    // broke silently the moment 0.41.0's bump made 0.40.0 no longer "the
    // released version" this rule checks.
    let mut r = real();
    let (maj, min, pat) = r.version;
    let heading_prefix = format!("## [{maj}.{min}.{pat}] —");
    let dated_heading = r
        .changelog
        .lines()
        .find(|l| l.starts_with(&heading_prefix))
        .unwrap_or_else(|| {
            panic!(
                "the live CHANGELOG.md has no section for the workspace version {maj}.{min}.{pat}"
            )
        })
        .to_string();
    let undated_heading = format!("## [{maj}.{min}.{pat}] — unreleased");
    r.changelog = r.changelog.replacen(&dated_heading, &undated_heading, 1);
    let v = only(run(&r), "rule 6");
    assert!(v.iter().any(|m| m.contains("has no date")), "got {v:?}");

    // A dated NEXT-version section passes whatever heading it opens with: the
    // arrangement is what this module checks, not the prose.
    let next = (maj, min, pat + 1);
    let mut r = real();
    r.version = next;
    r.changelog = r.changelog.replacen(
        &format!("## [{maj}.{min}.{pat}]"),
        &format!(
            "## [{}.{}.{}] — 2099-01-01\n\n### Added\n\n- something\n\n## [{maj}.{min}.{pat}]",
            next.0, next.1, next.2
        ),
        1,
    );
    assert!(only(run(&r), "rule 6").is_empty());

    // A version bump with no section.
    let mut r = real();
    r.version = next;
    assert!(
        only(run(&r), "rule 6")
            .iter()
            .any(|m| m.contains("no section"))
    );
}

/// `REL-0.55.0` round 2, `§10`: the exact defect this round found, planted
/// rather than only described. `b002284` removed `## [0.54.0] — 2026-10-18`
/// but left that section's own body in place, now unlabelled under the
/// following heading — reproduced here by removing only the heading line,
/// the same shape, not a stand-in for it.
#[test]
fn a_release_tag_with_no_heading_in_either_file_is_found() {
    let mut r = real();
    let before = r.changelog.clone();
    r.changelog = r.changelog.replacen("## [0.54.0] — 2026-10-18\n\n", "", 1);
    assert_ne!(r.changelog, before, "0.54.0's heading must exist to remove");
    let v = only(run(&r), "rule 7");
    assert!(
        v.iter().any(|m| m.contains("0.54.0")),
        "a released version missing its section must be reported, got {v:?}"
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
