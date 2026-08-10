# REL-0.20.1 — Prepare the 0.20.1 release candidate

**Issued by**: Architect
**Date**: 2026-08-03
**Priority**: release-blocking
**Governing decision**: [`DEC-046`](../../../.git-exclude/tasks/architect/003-approved-decisions-2026-07-31.md)
**Depends on**: **I18N-004 accepted.** Nothing to package before then.

---

## 1. Purpose

Cut a patch release carrying the `ISSUE-006` fix.

**Do not tag. Do not publish.** The owner approves first (workflow Phase 8).
Producing an artefact is not releasing it.

## 2. Why 0.20.1 exists at all

0.20.0's changelog tells users the `Watch` severity ceiling is enforced by a
compile-time guarantee. It is enforced for badges and glyphs and **not** for the
summary sentence directly beneath them, which still named the unclamped state in
prose.

That claim is public and incomplete, on the product's defining commitment. The
owner approved a patch rather than a quiet correction in 0.21.0 because the
whole argument for the release-cycle exit criteria was that overclaiming is how
this project got here.

**A filing note.** This corrects work RFC 007 delivered, but was found during
RFC 006's execution, so both handoffs sit here. RFC 007 stays in `done/` — it
did ship — with a status note recording the follow-up. Neither reopening a
completed RFC nor inventing a new one for a two-defect fix would represent the
history more honestly than this does.

## 3. Change scope

- `Cargo.toml` — the workspace version, one line
- `CHANGELOG.md` — a new `[0.20.1]` section, and a correction note on `[0.20.0]`
- A release tarball, produced but not published

**No code.** If you find yourself editing anything under `crates/*/src`, stop
and report — the fix is I18N-004's and should already be merged and reviewed.

## 4. Item 1 — version bump: one line

`Cargo.toml:23` — `version = "0.20.0"` → `"0.20.1"`.

That is the whole change. All six member crates inherit via
`version.workspace = true`; the five `[workspace.dependencies]` entries are
`version = "0"` and need nothing. The 0.19.1 handoff bundle's release procedure
still says otherwise and is still wrong.

**Patch, not minor.** These are defect corrections: a requirement violation and
two template/value mismatches. No feature, no schema change, no API change.

## 5. Item 2 — the changelog, and the part that matters

Add a dated `[0.20.1]` section and open a fresh `[Unreleased]` above it. Then:

### 5.1 Correct the 0.20.0 entry — without rewriting it

The 0.20.0 entry (`CHANGELOG.md:76-87`) describes the ceiling as enforced by
`DisplayHealthState`. **Leave that text intact.** It is a released entry and a
historical record; silently editing shipped changelog text to be less wrong is
its own kind of dishonesty.

Instead **append a clearly-marked correction note** to that entry, stating that
the clamp covered badge and glyph rendering and not the summary sentence, and
pointing to `[0.20.1]`.

A reader who arrives at the 0.20.0 entry must not leave it believing something
untrue. A reader who wants to know what was actually claimed at the time must
still be able to see it.

### 5.2 The 0.20.1 entry

Record what was wrong, not only what changed (`NFR-MNT-009`), and be specific
about why it survived: the clamp was attached to a type used by badges rather
than to every path that renders a state, and the guarding test matched a
capitalised substring, so it passed against a page rendering the word in
lowercase prose.

Also record the two sentence fixes, and — if it reproduced — the WipCompliance
one. If it did not reproduce, say that instead.

### 5.3 Check it against §1.7

The changelog is user-visible and ships in the tarball. Run the same check
DEV-009 did: no prohibited vocabulary in the new section.

Note the *use* versus *mention* problem DEV-009 hit — describing a removed
string without reproducing it. That ambiguity in §1.7 is on my list to
amend and is not yours to resolve; do what DEV-009 did and flag it.

## 6. Item 3 — final gate run, cold cache

`cargo clean` first, then the full set. The web test list has not changed since
0.20.0; `peisear-i18n` is new since then and has its own job.

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p peisear-core --lib
cargo test -p peisear-auth --lib
cargo test -p peisear-storage --lib
cargo test -p peisear-i18n
cargo test -p peisear-notify  -- --test-threads=1
for t in auth_boundary board_keyboard breadcrumb health_explainability \
         issue_edit_url optimistic_lock search smoke status_segment \
         sub_issues today_panel view_state workload_privacy; do
  cargo test -p peisear-web --test "$t" -- --test-threads=1
done
```

Confirm every test crate still has a CI job.

## 7. Item 4 — the tarball

`peisear-0.20.1.tar.gz`, built with **`git archive` at the release commit** —
adopted as the procedure after DEV-009, because an exclude list only excludes
what it names, while `git archive` excludes everything untracked by
construction.

**Verify by extraction, not from the build command**: extract into an empty
directory, confirm files sit at the root with no intermediate directory, confirm
`target/`, `data/`, `.git/`, `.git-exclude/` are absent, and confirm the file
count matches `git ls-tree -r --name-only <commit> | wc -l`.

Record the SHA-256 alongside it.

## 8. Item 5 — release-candidate information

| Field | Notes |
|---|---|
| Version | 0.20.1 |
| Source commit | the exact SHA the tarball was built from |
| Included changes | the ceiling clamp, the two sentence fixes, the label consolidation, the ceiling test |
| Excluded changes | §10.11 assignee/workload defect; the eight `peisear-web` lint suppressions; `workspace-layout.md`'s stale tree — all unchanged from 0.20.0 |
| Executed tests / results | §6 |
| Supported environments | unchanged from 0.20.0 — MSRV 1.88.0, pinned 1.97.1 |
| Migration considerations | none; confirm `migrations/` still ends at `0015` |
| Rollback | forward-fix only; no schema change |

## 9. Acceptance criteria

1. `Cargo.toml` declares `0.20.1`; workspace builds.
2. `CHANGELOG.md` has a dated `[0.20.1]`, a fresh `[Unreleased]`, and a
   correction note on `[0.20.0]` that leaves the original text intact.
3. All gates green from a cold cache, with logs.
4. Tarball correctly named and **verified by extraction**.
5. The §8 packet is complete.
6. No code changed.

## 10. Prohibited

- **Do not tag. Do not publish.**
- Do not fix anything noticed along the way — report it. A release candidate is
  a snapshot of reviewed work.
- **Do not edit the 0.20.0 changelog text itself.** Append a correction; do not
  revise history to look better.
- Do not make the 0.20.1 entry vaguer than the defect was.

## 11. Required review-request format

Per workflow §9.2, into `.git-exclude/review-request/`. The §8 packet can be the
body.

**Escalate rather than deciding** if any gate fails, if the extraction check
surprises you, or if the §1.7 check flags changelog text you cannot reword
without losing accuracy.
