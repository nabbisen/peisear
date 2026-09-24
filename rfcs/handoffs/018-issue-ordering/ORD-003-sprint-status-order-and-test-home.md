# ORD-003 — the sprint's issues read Done first, and the ordering tests need a home

**Issued by**: Architect
**Date**: 2026-09-24
**Target release**: 0.38.0
**Governing RFC**: none — defect fix.
**Source**: the dev team found the ordering while implementing `ORD-002` and
correctly left it alone as a different defect; `.git-exclude/reviewed/ORD-002-review.md`
§4 and §5. **Depends on**: nothing.

---

## 1. The defect — the third instance of one shape

`sprints::issues_in_sprint` (`sprints.rs:362`) orders:

```sql
ORDER BY i.status ASC, si.assigned_at ASC, si.rowid ASC
```

`status` is **TEXT**, so `ASC` is alphabetical: `done, in_progress, open`.
**A sprint's issues read Done first and Open last**, on both surfaces that
render them — sprint detail (`handlers/sprints.rs:199`) and the sprint plan's
sprint column (`:673`). Neither re-sorts; the only `sort_by` in that file is on
assignee names.

This is the **third** instance of *SQL ordering a TEXT column as though its
alphabetical order carried meaning*, after `ORD-001` (`status`, the issue list)
and `PLAN-003` (`priority`, the backlog). Three is a shape rather than a
coincidence, and it now has an entry in `§10` — written by me, not part of this
handoff.

## 2. The design decision — and it differs from `ORD-001` on purpose

**Group by status in lifecycle order: Open, In progress, Done.** Not by
removing the status term, which is what `ORD-001` did for the project issue
list.

The difference is that the project list **has a board beside it** doing status
grouping properly, so a second grouping rule there was redundant. A sprint has
no board. "What is left in this sprint" is the question the page exists to
answer, and grouping open work above finished work answers it. Removing the
term would interleave done and open issues in assignment order, which is worse
than either alternative.

**`IssueStatus::lifecycle_rank(self) -> u8`** in `peisear-core`, beside
`as_str` / `parse` / `all`, `Open` 0: the same shape as
`Priority::severity_rank` from `PLAN-003`, for the same reason, with a doc
comment saying so and saying it is **not** a derived `Ord` and not the enum's
declaration order by accident. One pattern used twice beats two patterns.

**Then the same split `PLAN-003` used**: the status term leaves the SQL, which
keeps `si.assigned_at ASC, si.rowid ASC`, and a **stable** Rust `sort_by_key`
on `lifecycle_rank` runs inside `issues_in_sprint`, not in either handler.
Stable is load-bearing — it is what preserves assignment order within a status.
Put a SQL comment at the `ORDER BY` saying the status term is deliberately
absent, as `backlog_for_team` now has.

`IssueStatus::all()` already returns lifecycle order and the board builds its
columns from it. **Do not make `lifecycle_rank` a lookup into `all()`** — an
index search to answer a constant is slower and less clear than a `match`.
Mirror `severity_rank` exactly.

## 3. The ordering tests need their own file

Seven of `ORD-002`'s tests live in `view_state.rs` because that is where
`ORD-001`'s tie test landed, and it is not the home for capacity or inbox
ordering. The dev team asked rather than creating a file whose `DEC-007` block
edit I reserve.

- **Create `crates/peisear-web/tests/ordering.rs`** and move the ordering tests
  into it — `ORD-001`'s tie test, `ORD-002`'s seven, and this handoff's new
  ones. Leave in `view_state.rs` only what is about view state, including
  `explicit_sorts_are_unchanged`, which is about the sort control.
- **Edit `.github/CONTRIBUTING.md`'s `DEC-007` command block in the same
  change — authorised.** A move must not change the count: report it before and
  after and they must match except for the tests this handoff adds.
- Moving a test is where assertions get quietly weakened. **Move them
  byte-identical** apart from imports, and say so.

## 4. Verification

- **A test that fails before and passes after**, both reported: a sprint with
  issues in all three statuses renders Open, In progress, Done on **both**
  surfaces — sprint detail and the sprint plan's sprint column. By relative
  offset, not `body.contains`.
- **Assert Done is no longer first**, named, so the fix cannot silently revert.
- **Assignment order is preserved within a status** — two issues assigned at
  different times inside one status band, asserted in assignment order. Plant
  it: reverse the SQL `assigned_at` term and confirm exactly that test fails.
- `ORD-002`'s `si.rowid ASC` tiebreak stays and is not disturbed.
- `burndown` and the sprint summary are unchanged — they consume
  `issues_in_sprint`'s rows; confirm no total moves.
- `DEC-007`: report the count before and after the move separately from the
  count after the new tests. Last recorded **289**.
- Three consecutive workspace runs; `fmt`; `clippy`. The overflow gate is
  untouched — no markup or class changes.

## 5. Escalate rather than deciding

- **If anything else consumes `issues_in_sprint` order-sensitively** beyond the
  two handlers and `burndown`.
- **If a fourth `ORDER BY` on a TEXT column exists.** Two were found by
  accident and one by enumeration; a deliberate sweep of every `ORDER BY` on a
  non-numeric, non-timestamp column would settle it. Report what you find; do
  not fix it here.
- **If the test move changes the count** in any way this handoff does not
  predict.
- **If a user-visible string changes.** None should.

## 6. Exit condition

A sprint's issues read Open, In progress, Done on both surfaces, with
assignment order preserved inside each; `lifecycle_rank` is the single
expression of status order; the ordering tests live in `ordering.rs` and the
`DEC-007` block names it.
