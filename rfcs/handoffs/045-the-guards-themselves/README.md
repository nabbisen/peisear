# Handoffs — the guards themselves

**0.52.0, proposed.** Three rounds, one subject: **the checks this project
relies on, checked.** Every item here was found by a guard failing to be
what it claimed — not by a feature request.

`0.51.0` was a documentation release that absorbed three rounds of
non-documentation work, and said so twice in its own changelog. This
series is the correction: **guard infrastructure gets its own release**,
and `RFC 0017` moves to `0.53.0`.

**Schedule change, for the owner's authorisation.** `RFC 0018` records
*"0.51.0 is documentation accuracy and 0.52.0 is `RFC 0017`"*. This
series proposes `0.52.0` = the guards, `0.53.0` = `RFC 0017` (the first
five minutes), `0.54.0`–`0.56.0` = `RFC 0018` P1–P4. The reason is
`GRD-001`: a guard that `NFR-A11Y-007` depends on is reporting clean on
files it cannot read, and that should not queue behind a release about
installation.

---

## Rounds

| ID | Handoff | Subject | Release |
|---|---|---|---|
| GRD-001 | [the helper five scans share](./GRD-001-the-helper-five-scans-share.md) | `strip_line_comments` exists in **six** copies and **five** carry a naive `split("//")` that truncates any line holding `xmlns="http://…"` — five component files. `touch_target_scan` is one of the five, so its answer for those files is **unrelated to its question and clean either way** (`§10.36` in a guard `NFR-A11Y-007` depends on). The sixth copy, `alignment_scan`'s, is already correct. **Reverses `QA-002`'s duplication call for this helper**, which is recorded in a handoff and two source comments and in neither specification | 0.52.0 |
| GRD-002 | [the fifteen that are wholly tests](./GRD-002-the-fifteen-that-are-wholly-tests.md) | `NFR-MNT-005`'s kind 2, **settled by the owner 2026-10-13: move them.** Fifteen files under `crates/*/src/` are guards with no implementation beside them. Each splits into `<name>.rs` (helpers) + `<name>/tests.rs` (the `#[test]`s) — the owner's own `project-instructions-rust.md` shape, already used by the nine kind-1 files at `0.51.0`. **Zero build cost**; `§10.37` closes | 0.52.0 |
| GRD-003 | the needle that cannot see what it bans | Rule 5 scans documents, RFCs **and published doc comments**, but matches the gitignored path itself — and a citation written as a bare review-file name never spells it. **31 such files are cited 60 times across 25 files in `crates/`, 27 of them in doc comments that render on docs.rs.** Widen the needle, then close the sixty by extracting the substance, never by publishing the review | 0.52.0 |

**Order is deliberate.** `GRD-001` first: it is a live defect, and it
edits the helper half of five files that `GRD-002` then splits — one
edit each rather than two. `GRD-003` last, because it touches doc
comments across 25 files and should land when nothing else is moving
them.

---

## What this series is not

**Not a tidy-up.** Each round's exit condition requires showing what the
guard reported **before and after**. A refactor that leaves every scan
reporting exactly what it reported before has not been demonstrated to
fix anything — that is `§10.36` read forwards instead of backwards.

**Not a place to fix what the fixed guards find.** If `GRD-001`'s
corrected `touch_target_scan` reports real violations in the five
component files it could not read, those are a finding to **report, with
the count**, not to repair in the same round. The guard and the thing it
guards are separate work.
