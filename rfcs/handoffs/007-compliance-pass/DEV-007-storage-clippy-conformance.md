# DEV-007 — Clear the clippy debt in `peisear-storage`

**Issued by**: Architect
**Date**: 2026-08-01
**Priority**: P1 — `NFR-MNT-007`
**Governing decision**: ISSUE-001 decision §4
**Depends on**: **DEV-006 must land first.** Otherwise your diff is buried in a
44-file reformat.

---

## 1. Purpose

`cargo clippy --workspace --all-targets -- -D warnings` fails with 21 errors,
all in `peisear-storage`. `NFR-MNT-007` requires it to pass.

## 2. Why this is separate from DEV-006

DEV-006 is mechanical. This is **design work**. Clearing these lints changes
function signatures in the storage crate:

| Lint | Count | What clearing it means |
|---|---|---|
| `type_complexity` | 14 | Introduce **named types** for complex tuple returns |
| `too_many_arguments` | 4 | Introduce **parameter structs** |
| `ptr_arg` | 1 | `&Vec<T>` → `&[T]` — a public signature change |
| `redundant_pattern_matching` | 1–2 | Local, mechanical |

Affected files: `issues.rs`, `metrics_snapshots.rs`, `notifications.rs`,
`sprints.rs`, `teams.rs`, `user_burnout.rs`, `user_capacities.rs`,
`user_metrics_snapshots.rs`.

## 3. Change scope

`crates/peisear-storage/src/**` only, plus call sites in other crates that the
signature changes force. List those call sites explicitly in the review request
— they are the part a reviewer must actually check.

## 4. Non-change scope

- **No behaviour change.** Same queries, same results, same error mapping.
- **No schema or migration change.**
- `translate_trigger_error` and its `RAISE`-message matching stay exactly as
  they are (`DEC-011`). If a lint appears to require touching it, escalate.
- Do not touch the crates DEV-001..005 are editing beyond forced call-site
  updates. None of them touch `peisear-storage`, so this should be rare.
- Do not "fix" lints by `#[allow(...)]` — see §6.

## 5. Required implementation

Clear all 21 errors so `cargo clippy --workspace --all-targets -- -D warnings`
exits 0.

### Ambition limit — read before the guidance below

**Clear the lints conservatively. Do not take the opportunity to redesign the
storage crate.**

This lands in a release whose entire purpose is *correcting* defects. A
regression introduced here would undo the release's own point. The smallest
correct change that makes the gate green is the right change, even where a more
elegant shape is visible.

Fourteen `type_complexity` hits in one crate probably *does* indicate a real
problem with the crate's shape. That is a legitimate finding — **report it, do
not fix it here.** If you see it, say so in the review request and I will open
an RFC for a later slot. A storage-shape refactor deserves its own design
review, not a slot inside a correction release.

The bar for this handoff is: gate green, behaviour identical, diff as small as
it can be while still engaging with each finding rather than suppressing it.

### Design guidance, since this is the part with judgment in it

1. **Named types over tuples.** A returned `(String, i64, Option<DateTime>, …)`
   becomes a struct with named fields. Name it for what it *is* in the domain,
   not for its shape — `SprintProgress`, not `SprintTuple`.
2. **Keep row types private.** `*Row` types stay internal to `-storage`;
   domain types cross the boundary (`DEC-001`, external design §5). A new named
   type that crosses the crate boundary is a domain type and must be named and
   placed accordingly. If you find yourself exporting something that looks like
   a database row, stop and escalate.
3. **Parameter structs** for `too_many_arguments`: group by meaning, not by
   position. Seven parameters that are really "a filter" plus "a range" should
   become two arguments, not one bag of seven fields.
4. `&Vec<T>` → `&[T]` at every call site.

## 6. On `#[allow]`

Suppression is permitted **only** where the lint is genuinely wrong for the
case, and then it must carry a comment stating why, adjacent to the attribute.

A blanket `#![allow(clippy::type_complexity)]` at crate level, or suppressions
added to make the gate pass without engaging with the finding, will be rejected
on review. Fourteen `type_complexity` hits in one crate is a signal about the
crate's shape, not a signal that the lint is miscalibrated.

If you conclude a lint genuinely should be suppressed in more than two places,
escalate instead — that is a design conversation, not an implementation choice.

## 7. Required tests

No new tests. The existing suite is the safety net, and it must pass unchanged
— identical pass counts, no test edited to accommodate a signature change
unless the signature change forced it, in which case call it out.

`peisear-storage`'s two lib tests (`FR-SCH-004`, LIKE-escaping) must still pass.

## 8. Acceptance criteria

1. `cargo clippy --workspace --all-targets -- -D warnings` exits 0.
2. `cargo fmt --all -- --check` still exits 0 (run `cargo fmt` after your edits,
   once, per `NFR-MNT-007`).
3. Full test suite passes with unchanged counts.
4. No behaviour change; no schema change.
5. Every `#[allow]` added, if any, carries an adjacent justification.

## 9. Required evidence

- Changed-file list, **with the forced call-site changes separated** from the
  storage-internal ones.
- Clippy output before (21 errors) and after (clean).
- Full test output.
- For each new named type or parameter struct: one line on what it represents.
  If you cannot say what it represents in one line, the grouping is probably
  wrong.

## 10. Review focus to request

Two things:

1. The named types and parameter groupings — whether they express domain
   concepts or merely satisfy the lint. That is the difference between clearing
   debt and relocating it.
2. **Anything you had to leave alone** because of the ambition limit in §5. If
   the crate's shape is the real problem, your review request is where that
   gets recorded for a future RFC. Say it plainly; it will not be treated as
   scope you failed to cover.

**Escalate rather than deciding** if a lint appears to require a behaviour
change, if a new type would need to expose a row shape across the crate
boundary, or if suppression looks necessary in more than two places.
