# PERF-001 — the query planner never has statistics

**Issued by**: Architect
**Date**: 2026-09-25
**Target release**: 0.40.0
**Governing RFC**: none.
**Source**: found by the dev team while measuring `ORD-002`'s plan changes —
*"the app runs neither `ANALYZE` nor `PRAGMA optimize` (grep of the workspace:
none), so the no-statistics plan is the one that ships."*
**Depends on**: nothing. **After `REQ-001`.**

---

## 1. What is true today

**SQLite chooses every plan in this product without statistics.** Nothing runs
`ANALYZE`; nothing runs `PRAGMA optimize`. The planner works from the schema
and its built-in guesses.

That has been fine and is not obviously a defect — but it is a **property
nobody chose**, and it has already changed how two measurements read:
`ORD-002` found an index flip that appears only without statistics, and
`ORD-003` a temp b-tree that disappeared. Both were measured in the
no-statistics plan **because that is what ships**, which is correct and also
means nobody has ever seen the other one.

## 2. What to do — measure first, and the measurement may be the deliverable

**2.1 — Find out whether it matters.** On a database with realistic volume
(the 60,000-issue shape `ORD-002` used), compare the plans and the wall-clock
of the product's heaviest reads **before and after `ANALYZE`**: the issue list,
the board, the sprint plan's backlog, the project health basis, search, and the
inbox.

**If nothing meaningful moves, that is the answer**, and the deliverable is the
measurement plus one recorded sentence. **Do not add a `PRAGMA` because it is
conventional.**

**2.2 — If something does move**, the options in order of what this product
would accept:

- **`PRAGMA optimize` on connection close or at intervals** — the modern
  recommendation, cheap, and it does nothing until it has something to do.
- **`ANALYZE` once at migration time** — simple, and immediately stale.
- **Nothing, recorded.** A deliberate no is a fine outcome and better than a
  `PRAGMA` nobody can justify.

**Bring the measurement and your reading; do not pick one.** Which of these
ships is a decision about a running system's behaviour and it is mine.

## 3. Verification

- The comparison in §2.1, with the SQLite version stated — CLI or linked, and
  **which one you used**, as `ORD-002` and `ORD-003` both did.
- **Plans before and after, side by side**, for each query named.
- If anything is added: `DEC-007` reported, three consecutive workspace runs,
  `fmt`, `clippy`. If nothing is added, say the count is unchanged at **346**.
- **No migration** unless §2.2 chooses `ANALYZE`, in which case say so before
  writing it.

## 4. Escalate rather than deciding

- **If `ANALYZE` changes a plan for the worse** on any query. That is more
  interesting than an improvement and it settles the question by itself.
- **If the realistic-volume fixture is expensive to build**, say so — an
  approximate answer from a smaller shape, labelled as approximate, beats a day
  spent on a fixture.
- **If a plan turns out to depend on statistics in a way that makes an earlier
  measurement wrong** — `ORD-002`'s or `ORD-003`'s. Say which.

## 5. Exit condition

A measurement that answers whether this product's plans are worse without
statistics, and either a change I have approved or a recorded decision that
none is wanted.
