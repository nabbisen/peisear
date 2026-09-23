# PLAN-003 — the sprint-plan backlog sorts priority alphabetically, so `high` comes last

**Issued by**: Architect
**Date**: 2026-09-24
**Target release**: 0.38.0
**Governing RFC**: none — defect fix.
**Source**: found by the dev team while checking `020`'s D-5 design review
against the code; confirmed and widened in
`.git-exclude/reviewed/D5-design-review-code-check-review.md`.
**Depends on**: nothing. **Nothing depends on it** — in particular it is *not*
D-5's first step, and it must not wait on D-5's owner decision.

---

## 1. The defect

`sprints::backlog_for_team` (`crates/peisear-storage/src/sprints.rs:820`)
orders the sprint-plan backlog:

```sql
ORDER BY p.name ASC, i.priority DESC, i.created_at DESC
```

`priority` is **TEXT** (`low` | `medium` | `high` | `urgent`), so `DESC` is
alphabetical, not by severity. SQLite's own descending order of those four
values is:

```
urgent   medium   low   high
```

**`high` sorts last — below `low`.** Measured against SQLite directly, not
inferred from the collation rules.

The rows reach the page in that order unchanged: `handlers/sprints.rs:719`
puts `backlog` straight into the view struct, and the only sort in the handler
is line 710, which sorts the assignee filter's options. Nothing re-sorts.

This has been the behaviour since `PLAN-001`.

**Why it matters more than its size suggests.** The sprint plan is the one
screen whose entire purpose is choosing what a team does next, and its backlog
column presents the second-most-severe band at the bottom. A person scanning
from the top sees urgent, then medium, then low, and has to reach the end for
high. Everything about the page's layout says the top is what matters.

**What is not affected**: the priority *filter* on the same query
(`AND (?3 IS NULL OR i.priority = ?3)`) is an equality test and is correct.
The issue list's `sort=priority` is correct — see §2.

## 2. Scope — checked unbounded, not from where it was found

A defect found in one query is a claim about one query until the whole
workspace is searched. It was:

- **`grep "ORDER BY"` across all of `crates/peisear-storage/src` returns
  exactly one priority ordering in the product, and it is this one.**
- No SQL anywhere compares `priority` with an inequality, `MIN`, `MAX` or
  `BETWEEN`.
- Every other priority ordering is **Rust with an explicit rank**:
  `handlers/issues.rs:167-175`, `Urgent => 0, High => 1, Medium => 2,
  Low => 3`, a stable `sort_by_key` so the storage-default order survives ties.

So the product already has the correct pattern, written down and commented.
**This one query is the outlier**, and it became one by trusting `TEXT DESC`
to mean severity. That is what makes this a defect fix rather than a design
question.

## 3. The design decision

**Severity order is a property of `Priority`, and after this change it is
written down exactly once.**

Today it is written once (in `issues.rs`) and *assumed* once (in SQL). Adding a
SQL `CASE` expression to this query would fix the symptom and leave the order
expressed twice, in two languages, with nothing keeping them equal — a fifth
priority value would then be a two-file change with one silent failure mode.
That is the kind of continuing maintenance cost worth spending a small amount
now to avoid.

**3.1 — `Priority::severity_rank()` in `peisear-core`**, beside `as_str` and
`parse`: the single expression of severity order, `Urgent` most severe. Its
doc comment says it exists because the column is TEXT and SQL therefore cannot
order by severity.

**3.2 — `handlers/issues.rs:167` uses it**, replacing the inline `match`. This
is a refactor with **no behaviour change**, and the existing tests over
`sort=priority` are what confirm that. Keep the comment about the stable sort;
it is still the reason the code reads the way it does.

**3.3 — `backlog_for_team` drops `i.priority DESC` from the SQL and sorts
after the fetch**, inside `backlog_for_team` itself — not in the handler. The
function's contract is "the backlog in display order", and keeping the sort
next to the query means the next reader sees both at once instead of finding
half the ordering a crate away.

```
ORDER BY p.name ASC, i.created_at DESC   -- severity is not orderable here; see below
```

then a **stable** sort by `(project_name, severity_rank)`. Stable is
load-bearing: it is what preserves `created_at DESC` within a
(project, priority) group, so only the priority term's behaviour changes.

Leave a short comment at the `ORDER BY` saying why priority is absent from it.
A reader who sees an `ORDER BY` without the field the page is grouped by will
otherwise assume it is a bug and put it back.

**Not considered**: an ordinal column, or a stored severity integer. It would
let SQL order correctly everywhere forever, and it costs a migration, a change
to both insert paths and a second representation of priority in the schema —
for one query. If a second query ever needs to order by severity in SQL, that
is the moment to reconsider, and this handoff is the place to say so.

## 4. What to do

- `Priority::severity_rank()` in `peisear-core`, doc-commented per §3.1.
- `handlers/issues.rs` uses it; confirm `sort=priority` is unchanged.
- `backlog_for_team`: SQL term removed, stable sort added, comment at the
  `ORDER BY`.
- **A test that pins the sequence.** `tests/sprint_plan.rs` asserts membership
  today, not order — that is why this survived since `PLAN-001`. Create
  backlog issues at all four priorities and assert the rendered order is
  urgent, high, medium, low. Assert **positions relative to each other**, not
  `body.contains` — an unscoped containment assertion passes on any order,
  which is exactly how `PLAN-002` round 2's first attempt failed review.
- If more than one project is in play, pin that `p.name ASC` still groups
  first and severity orders within a project.

## 5. Verification

- The new sequence test fails on the current code and passes after. **Run it
  against the unfixed tree first and report both results** — a test that has
  never been seen to fail is not yet evidence.
- `sort=priority` on the issue list unchanged, before and after.
- The backlog's priority *filter* still returns only the chosen band.
- `DEC-007`: report the new count. The last recorded is **272**. No new test
  *file* is expected — `sprint_plan.rs` exists; if one is added, edit
  `.github/CONTRIBUTING.md`'s command block in the same change.
- Three consecutive workspace runs; `fmt`; `clippy`.
- The overflow gate is untouched by this and does not need a run.

## 6. Escalate rather than deciding

- **If the stable sort does not preserve `created_at DESC` within a group** —
  it should, but measure rather than assume.
- **If removing `i.priority DESC` changes the plan produced for a large
  backlog** in a way that matters. The comment above the function says the
  join is a correctness concern, not a performance one; if that turns out to
  be optimistic, say so rather than working around it.
- **If `severity_rank` turns out to want a third caller** you did not expect.
  That is useful to know and may change §3's "not considered" paragraph.
- **If a user-visible string changes** anywhere in this. It should not.

## 7. Exit condition

The sprint-plan backlog reads urgent, high, medium, low within each project;
a test fails without the fix and passes with it; severity order appears exactly
once in the codebase; the issue list is unchanged.
