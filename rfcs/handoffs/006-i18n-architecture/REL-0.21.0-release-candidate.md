# REL-0.21.0 — Prepare the 0.21.0 release candidate

**Issued by**: Architect
**Date**: 2026-08-11
**Priority**: release-blocking
**Governing RFC**: [006](../../done/006-i18n-architecture.md)
**Depends on**: **I18N-007's two corrections landed** (`I18N-007-review.md` §3).
Nothing to package before then.

---

## 1. Purpose

Cut the minor release carrying RFC 006 — the i18n architecture and the
vocabulary guard.

**Do not tag. Do not publish.** The owner approves first (workflow Phase 8).
Producing an artefact is not releasing it.

## 2. Do the corrections first, and prove them

`I18N-007-review.md` §3 has two false negatives in `prose_scan.rs`, both
demonstrated by planting rather than by reading:

1. A brace-less `#[cfg(test)]` item blinds the scan for the rest of the file.
2. A text node between two `{expr}` children is missed.

Land both, then **prove each one the way §3 disproved it** — plant the exact
shape from the review, watch the scan fail, remove the plant, watch it pass.
Both transcripts go in the evidence directory. A guard whose corrections were
verified only by "the suite is still green" has not been verified at all: the
suite was green while both holes were open.

If §3.2's proposed fix reintroduces any of the sixteen false positives you
catalogued, **report that instead of widening the allowlist**, and this release
waits. The allowlist standard does not relax for a release.

## 3. Change scope

- `Cargo.toml` — the workspace version, one line
- `CHANGELOG.md` — a new `[0.21.0]` section
- `prose_scan.rs` — the two §2 corrections
- A release tarball, produced but not published

Nothing else under `crates/*/src`. If a conversion looks wrong while you are
here, report it — a release candidate is not where copy changes.

## 4. Item 1 — version bump

`Cargo.toml:24` — `version = "0.20.1"` → `"0.21.0"`. All member crates inherit
via `version.workspace = true`; the `[workspace.dependencies]` entries are
`version = "0"` and need nothing.

**Minor, not patch.** `peisear-i18n` is a new crate and `StorageError`'s
variants changed type. The workspace is not published, so this is not a
semver obligation to anyone outside the repo — it is an accurate description of
the change, which is the point of the number.

## 5. Item 2 — the changelog, and the part that matters

Add a dated `[0.21.0]` section and open a fresh `[Unreleased]` above it.

Write it for someone deciding whether to upgrade. The temptation with this
release is to describe it as "internationalisation", which would be wrong twice
over: `NFR-LANG-005` keeps additional locales deferred, and one locale ships.
What actually changed is that **§1.7's vocabulary constraint became checkable**,
and the checking found real defects — a `Score N / 100` badge, a `Concern`
severity mapped to danger colouring, `"Failed to update status"`, a
`"conflict: "` prefix leaking to users, and a severity ceiling that held
everywhere except the sentence directly under the badge.

Say what the guard covers and what it does not. The queue README's "What the
guard can and cannot do" is the honest version: copy but not interpolated data,
vocabulary but not tone, and nothing it cannot see as a literal — now including
`prose_scan.rs`'s named blind spot and its nine allowlisted `confirm()` dialogs.

**No claim that every string is covered.** The scan test and its allowlist are
the check; point at them. `I18N-007`'s review §2 records what happened the last
two times that claim was made in prose.

## 6. Item 3 — final gate run, cold cache

`cargo clean` first, then the full gate set per `DEC-007` — per-crate tests,
each `peisear-web` integration target individually, fmt, workspace clippy.
Capture the output. Warm-cache logs have hidden a stale build in this project
before.

Expected: 80 integration tests, 6 `peisear-web` lib unit tests, `peisear-i18n`
6 + 4. If any count differs from that, stop and report — a count that moved
during a version bump means something else moved too.

## 7. Item 4 — the tarball

Produce the release artefact by the 0.19.1 bundle's procedure, except its
version-bump step, which is wrong and stays wrong until someone fixes it
(`§10.4`).

**Verify by extraction, not from the build command**: extract into an empty
directory, confirm the tree builds and the suite passes there, and confirm no
`.git-exclude/` content is present. That last check is not a formality — this
repository keeps its entire internal review record under that path.

## 8. Item 5 — release-candidate information

Report, in the review request: the version, the artefact's name and size, the
gate results, the two correction transcripts from §2, and the changelog section
as written.

## 9. Acceptance

1. Both §2 corrections landed, each proven by a planted failure.
2. Version bumped; changelog section written and accurate about scope.
3. Cold-cache gates all pass, counts as in §6.
4. Tarball produced, verified by extraction, `.git-exclude/` absent.
5. Nothing tagged, nothing published.

## 10. Prohibited

Do not tag. Do not publish. Do not `cargo publish` — registry publication is an
open decision (`publish = false`) and not yours or mine to make. No rewording of
shipped copy. No allowlist growth. Do not weaken any guard to make a gate pass;
if a gate fails, the release waits.

## 11. Required review-request format

Workflow §9.2, one directory under `.git-exclude/review-request/`, with the two
correction transcripts as first-class evidence rather than a line in a log.
