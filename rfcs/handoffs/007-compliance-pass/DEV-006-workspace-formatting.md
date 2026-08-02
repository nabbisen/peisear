# DEV-006 — Bring the workspace to `cargo fmt` conformance

**Issued by**: Architect
**Date**: 2026-08-01
**Priority**: P1 — `NFR-MNT-007`, and it blocks every other handoff's exit
**Governing decision**: ISSUE-001 decision §4
**Depends on**: nothing
**Blocks**: **everything.** Lands first, alone.

---

## 1. Purpose

`cargo fmt --all -- --check` fails on `main` across 44 files. `NFR-MNT-007`
requires it to pass and is recorded as *"Implemented in CI"*; CI has never
passed. Make it true.

## 2. Background

This is not toolchain drift. Shapes like `Self::Throughput => { "…" }` are not
something `rustfmt` emits at any version — the code has **never been run
through `cargo fmt`**, contrary to the project rule requiring it after
implementation completes.

Tests and build are green; this is formatting debt only.

## 3. Change scope

Whatever `cargo fmt --all` touches. Expected: 44 files across all five library
crates and the binary.

**Nothing else.** No logic, no renames, no lint fixes, no reordering of items.

## 4. Required implementation

```bash
cargo fmt --all
```

That is the whole task.

Then verify:

```bash
cargo fmt --all -- --check    # must exit 0
```

**Do not hand-revise the output.** `NFR-MNT-007` states this explicitly:
formatting runs once and the result is not then edited. If a formatting result
looks wrong to you, report it — do not correct it.

**Do not add or change `rustfmt.toml`.** If you believe a setting is needed,
escalate. Changing formatting configuration silently changes the standard for
the whole project and is not an implementation decision.

## 5. Required verification

1. `cargo fmt --all -- --check` exits 0.
2. `cargo clippy --workspace --all-targets -- -D warnings` fails **exactly as
   before** — same 21 errors, same 8 files. Formatting must not change the lint
   surface. If the count moves, stop and report; it means something other than
   formatting changed.
3. The full per-crate test suite passes, unchanged:
   ```bash
   cargo test -p peisear-core --lib
   cargo test -p peisear-auth --lib
   cargo test -p peisear-storage --lib
   cargo test -p peisear-notify -- --test-threads=1
   for t in auth_boundary breadcrumb health_explainability issue_edit_url \
            optimistic_lock search smoke status_segment sub_issues \
            today_panel view_state; do
     cargo test -p peisear-web --test "$t" -- --test-threads=1
   done
   ```
   Same pass counts as before. A reformat that changes a test result means it
   was not a reformat.

## 6. Acceptance criteria

1. `cargo fmt --all -- --check` exits 0 on a clean tree.
2. Clippy's failure set is unchanged as a **multiset of (lint kind, file)
   pairs**. *(Amended 2026-08-01: this originally said "byte-for-byte
   unchanged", which is unsatisfiable — reformatting necessarily moves the line
   numbers and re-wraps the source snippets clippy quotes inline. The pair
   comparison is the right invariant.)*
3. Test pass counts unchanged.
4. The diff contains no semantic change — no identifier renamed, no item
   reordered, no code added or deleted.

## 7. Prohibited shortcuts

- Do not fix clippy findings here. That is DEV-007.
- Do not touch anything DEV-001..005 own beyond what `cargo fmt` itself changes.
- Do not introduce `rustfmt.toml` or `#[rustfmt::skip]` to reduce the diff.
- Do not split this across several commits by directory. One mechanical change,
  one commit — that is what makes it reviewable without reading it.

## 8. Required evidence

- `cargo fmt --all -- --check` output before (failing) and after (clean).
- File count and line count of the diff.
- Clippy output after, showing the **same** 21 errors.
- Full test output after.
- Explicit confirmation that the diff is whitespace and layout only.

## 9. Review

This is the one handoff where the reviewer should **not** read the diff line by
line. Review is: the tool was run, nothing else changed, the verification in §5
holds. Evidence carries the review, not inspection.

Review request into `.git-exclude/review-request/` per workflow §9.2 — short is
fine here.

**Escalate rather than deciding** if `cargo fmt` produces output you believe is
wrong, or if the clippy failure set changes.
