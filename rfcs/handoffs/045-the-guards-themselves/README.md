# Handoffs — the guards themselves

**0.52.0, authorized by the owner 2026-10-14.** Three rounds, one subject: **the checks this project
relies on, checked.** Every item here was found by a guard failing to be
what it claimed — not by a feature request.

`0.51.0` was a documentation release that absorbed three rounds of
non-documentation work, and said so twice in its own changelog. This
series is the correction: **guard infrastructure gets its own release**,
and `RFC 017` moves to `0.53.0`.

**The schedule this changed** is in one place now:
[`ROADMAP.md`'s **What ships next**](../../../ROADMAP.md#what-ships-next),
which carries the current numbers, the reason 0.52.0 is the guards
rather than `RFC 017`, and what the shift costs the rows behind it.
`RFC 018 §6`'s own release column is withdrawn in favour of it. The
reason in one line is `GRD-001`: a guard that `NFR-A11Y-007` depends on
is reporting clean on files it cannot read, and that should not queue
behind a release about installation.

---

## Rounds

| ID | Handoff | Subject | Release |
|---|---|---|---|
| GRD-001 | [the helper five scans share](./GRD-001-the-helper-five-scans-share.md) | `strip_line_comments` exists in **six** copies and **five** carry a naive `split("//")` that truncates any line holding `xmlns="http://…"` — five component files. `touch_target_scan` is one of the five, so its answer for those files is **unrelated to its question and clean either way** (`§10.36` in a guard `NFR-A11Y-007` depends on). The sixth copy, `alignment_scan`'s, is already correct. **Reverses `QA-002`'s duplication call for this helper**, which is recorded in a handoff and two source comments and in neither specification | 0.52.0 |
| GRD-002 | [the fifteen that are wholly tests](./GRD-002-the-fifteen-that-are-wholly-tests.md) | `NFR-MNT-005`'s kind 2, **settled by the owner 2026-10-13: move them.** Fifteen files under `crates/*/src/` are guards with no implementation beside them. Each splits into `<name>.rs` (helpers) + `<name>/tests.rs` (the `#[test]`s) — the owner's own `project-instructions-rust.md` shape, already used by the nine kind-1 files at `0.51.0`. **Zero build cost**; `§10.37` closes | 0.52.0 |
| GRD-003 | [the needle that cannot see what it bans](./GRD-003-the-needle-that-cannot-see-what-it-bans.md) | Rule 5 scans documents, RFCs **and published doc comments**, but matches the gitignored path itself — and a citation written as a bare review-file name never spells it. **29 such files are cited 64 times across 21 files** — 60 in doc comments, two in `requirements.md` itself — **and the rule reports zero.** `DOC-002` closed 19 of the path spelling while these sat in the same files. Clear them by extracting the substance, never by publishing the review; *then* widen the needle, to a shape that reads nothing under the gitignored path | 0.52.0 |
| REL-0.52.0 | [the release candidate](./REL-0.52.0-release-candidate.md) | The cut. **Nothing this release ships is different** — every changed line under `crates/` is a doc comment, a `#[cfg(test)]` module or a test file, so the browser gates are this round's falsification rather than a formality. `DEC-028` for both specifications, `DEC-007` **451**, and a changelog that leads with *nothing you can see changed* | 0.52.0 |

**The candidate closes the series.** Three rounds, then the cut.

**Order is deliberate.** `GRD-001` first: it is a live defect, and it
edits the helper half of five files that `GRD-002` then splits — one
edit each rather than two. `GRD-003` last, because it touches doc
comments across 21 files and should land when nothing else is moving
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
