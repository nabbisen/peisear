# PERF-002 — `PRAGMA optimize`, so each install's own data chooses its plans

**Issued by**: Architect
**Date**: 2026-09-25
**Target release**: 0.40.0
**Governing RFC**: none.
**Source**: `PERF-001`'s measurement. Ruled in
`.git-exclude/reviewed/PERF-001-review.md` §3. **Depends on**: nothing.

---

## 1. The decision, and the reason — which is not the 7×

`PERF-001` measured: statistics move two of six reads — WIP-violators 16.0 →
2.2 ms, search ~1.3–1.5× — and leave the issue list, health basis, backlog and
inbox unchanged. **14 ms off a page load would not buy this change.**

**The reason is that we cannot predict install shapes.** Your own caveat
decides it: the fixture has 30 users, so `idx_issues_assignee` is unselective;
on an install with hundreds the no-statistics plan for that query would be the
good one. *Nothing* bets the built-in guesses suit every install. *`ANALYZE` at
migration time* freezes one snapshot taken when the database is nearly empty —
the worst possible moment to sample. **`PRAGMA optimize` is the only option
that lets each install's own data decide.**

## 2. What to do

**Run `PRAGMA optimize` on connection close.** It is a no-op until there is
something to do; `PERF-001` measured it choosing nine tables and taking 33 ms
once on the linked 3.46.0.

**Three things `PERF-001` did not measure and this handoff must settle:**

- **On close versus periodic.** Close keeps it off the request path, which is
  why it is the default here. **If sqlx's pool gives no usable close hook**,
  say so and bring what it does give rather than putting it on the request
  path uninvited.
- **What it does on a warm database** — one that already has `sqlite_stat1`.
  It should be cheap and mostly a no-op; **measure it**, because "runs on every
  connection close" is only acceptable if that is true.
- **The pool has eight connections.** `PRAGMA optimize` is per-connection.
  Measure what eight closes cost in a burst, not one.

**Not in scope**: an `ANALYZE` migration, changing pool settings, or any index.

## 3. Verification

- **The two movers improve on a fresh install once it has been used** —
  WIP-violators and search, on `PERF-001`'s fixture, through the pool, with the
  linked SQLite named. **Report before and after as `PERF-001` did.**
- **The cost of the thing itself**: a warm close, and eight at once.
- **Nothing regresses**: the other four reads unchanged in plan and time.
- **A fresh database is not slowed at startup** — `optimize` must not turn a
  first request into a 33 ms one. Say where it runs and prove it is not there.
- `DEC-007`: report the new count, last recorded **346**. A test that asserts
  `sqlite_stat1` appears after use is worth one.
- Three consecutive workspace runs; `fmt`; `clippy`. The overflow gate is
  untouched.

## 4. The register entry I owe, so you do not chase it

`PERF-001` found that **without statistics, which plan ships depends on which
SQLite is linked** — 3.46.0 uses the partial index for the issue list, 3.53.4
does not — so a `libsqlite3-sys` bump can change plans with no change in this
repository. **That is mine to record**, and it is part of why this handoff
exists: statistics make the planner depend on the data rather than on the
library's guesses. Do not write it up.

## 5. Escalate rather than deciding

- **If there is no close hook** (§2).
- **If a warm `optimize` is not cheap.** That changes the answer, and *nothing,
  recorded* becomes the better option — bring the number.
- **If it makes any plan worse**, on either SQLite version.
- **If eight simultaneous closes are measurable** on a request the user waits
  for.

## 6. Exit condition

A used install has statistics without anyone running anything; the two movers
are faster and nothing else changed; the cost of `optimize` itself is measured
warm and in a burst of eight; no migration was added.
