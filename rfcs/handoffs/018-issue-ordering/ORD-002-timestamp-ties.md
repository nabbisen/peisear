# ORD-002 — a tie on a one-second timestamp picks the wrong row

**Issued by**: Architect
**Date**: 2026-09-24
**Target release**: 0.38.0
**Governing RFC**: none — defect fix.
**Source**: the dev team asked, in `PLAN-003`'s review request §1, whether the
sprint-plan backlog should take `ORD-001`'s `rowid` tiebreak. Enumerating the
answer found eleven sites and, in three of them, a wrong value rather than a
wrong order. Confirmed in `.git-exclude/reviewed/PLAN-003-review.md`.
**Depends on**: nothing. `ORD-001` and `PLAN-003` have shipped.

---

## 1. The defect

`created_at` and `updated_at` are `CURRENT_TIMESTAMP`, which SQLite stores at
**one-second resolution**. Any ordering on them ties for rows written in the
same second, and **SQLite returns ties in scan order — oldest first**. A `DESC`
ordering therefore reads *backwards* within a tie.

`ORD-001` hit this on the issue list and answered it locally with
`created_at DESC, rowid DESC`. **It is not local.** Every recency ordering in
`peisear-storage`:

| | count | sites |
|---|---|---|
| already correct | 2 | `issues.rs:98`, `:121` (`ORD-001`) |
| **a tie picks a row** | **3** | `user_capacities.rs:121`, `:151`; `issues.rs:857` |
| a tie reverses a list | 7 | `projects.rs:52`, `:69`; `sprints.rs:102`, `:827`; `search.rs:103`, `:163`; `notifications.rs:126` |
| correct by accident | 2 | `issues.rs:146`, `user_capacities.rs:253` — `ASC`, and scan order happens to be `ASC` |

## 2. The three that return a wrong value — do these first

All three resolve **a user's current capacity** with
`ORDER BY created_at DESC LIMIT 1`. Saving a capacity **inserts** a row
(`user_capacities.rs:312`; update and delete are separate paths), so a user may
hold several overlapping rows and the newest is meant to win.

Measured — two rows for one user sharing a second, the later holding 99:

```
ORDER BY created_at DESC LIMIT 1          ->  id=old  points=10
ORDER BY created_at DESC, rowid DESC ...  ->  id=new  points=99
```

**The tie returns the superseded row**, and that number feeds workload, WIP and
the health indicators.

> **Correction, 2026-09-24, from this handoff's own review.** This paragraph
> read *"Two saves inside one second — what a double-click produces — silently
> keep the first value while the page reports success."* **That is wrong about
> the mechanism.** The dev team measured the path through the app: `insert`
> calls `overlaps_existing` first and the handler renders a visible conflict
> message, so a *sequential* second save is refused, not stored. The reachable
> form is a **race between two concurrent requests** — check-then-insert is not
> in a transaction — which stores several overlapping rows, and the tie then
> returns a superseded one. The read-side fix below is unchanged and still
> right. The race itself is `CAP-001`.

This is reachable, it is a wrong number rather than a wrong order, and it is
why this handoff exists rather than a one-line addition to `PLAN-003`.

## 3. The rule

**Every ordering on a timestamp column ends with an explicit tiebreak on the
same table's `rowid`, in the same direction as the timestamp term.**

One rule, no per-site judgement, and it reads the same in all twelve places.
`DESC` orderings take `rowid DESC`; the two `ASC` orderings take `rowid ASC`,
which makes their present correctness **explicit rather than accidental** — they
rely today on scan order, which nothing documents and nothing tests.

- **Use the query's own alias** where there is one (`p.rowid`, `i.rowid`,
  `uc.rowid`), not a bare `rowid`, so a later join cannot make it ambiguous.
- **Not a `created_at` format change.** Millisecond timestamps would make ties
  rarer everywhere and look like the root fix, but they do not *eliminate* ties,
  they touch every table's defaults and `0017`'s trigger authority, and any code
  parsing the stored format becomes a migration risk. A tiebreak is exact, local
  and reversible; this is not the place to spend that risk.
- **What `rowid` guarantees, and what it does not**, is already written on
  `list_in_project` (`ORD-001`, commit `6586b56`): creation order in the
  ordinary case, stable and deterministic always, and **not** creation order
  unconditionally, because SQLite reuses the rowid of a deleted highest row.
  Every alternative tiebreak is arbitrary in *all* cases, so this is never
  worse. Do not repeat that paragraph eleven times — reference it.

## 4. What to do

- The three `LIMIT 1` picks in §2 first, as their own commit if you prefer.
- Then the seven `DESC` lists, then the two `ASC` lists.
- `sprints.rs:827` is `PLAN-003`'s backlog: `p.name ASC, i.created_at DESC,
  i.rowid DESC`. The Rust severity sort stays stable and untouched.
- **A short comment at one site only**, pointing at `list_in_project`'s
  paragraph. Eleven copies of the reasoning is the maintenance cost this
  project avoids elsewhere.

## 5. Verification

- **A burst test for each of the three picks**: two capacity rows written
  inside one second, the later value asserted to win. **Report the before
  result as well as the after** — these fail on the current code, and that
  failure is the evidence.
- **One burst test for the inbox** (`notifications.rs:126`), where several
  notifications from a single action in one second is the realistic case, and
  one for the backlog. The other five lists are the same mechanism; a test each
  is not proportionate, but say so rather than leaving it unstated.
- Assert by **relative position**, not `body.contains`.
- The two `ASC` sites: assert the order is unchanged — this makes existing
  behaviour explicit and must not move.
- `sort=priority`, the backlog's grouping and filters, sub-issue order: all
  unchanged.
- `DEC-007`: report the new count, last recorded **281**. New test *files* mean
  editing `.github/CONTRIBUTING.md`'s block in the same change.
- Three consecutive workspace runs; `fmt`; `clippy`. The overflow gate is
  untouched.

## 6. Escalate rather than deciding

- **If any site cannot take a `rowid` tiebreak** — a `GROUP BY`, a `DISTINCT`,
  a compound `SELECT` or a view would each be a reason, and a reason to stop
  rather than work around.
- **If a tie turns out to decide something beyond these eleven** — a
  `LIMIT 1` elsewhere, or any "the latest one wins" resolved in Rust rather
  than SQL. §1's table is from a search of `ORDER BY` on timestamp columns in
  one crate; it is not a proof that no other code picks by recency.
- **If the query plan changes** on any site.
- **If a user-visible string changes.** None should.

## 7. Exit condition

Every timestamp ordering in `peisear-storage` carries an explicit `rowid`
tiebreak in the matching direction; a capacity saved twice in one second
resolves to the second value, with a test that failed before the change;
the two `ASC` sites are pinned and unchanged.
