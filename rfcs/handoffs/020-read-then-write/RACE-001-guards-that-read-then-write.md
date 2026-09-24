# RACE-001 — three guards that check on one connection and write on another

**Issued by**: Architect
**Date**: 2026-09-24
**Target release**: 0.39.0 — **not 0.38.0.** All three are pre-existing, none is
a regression, and 0.38.0 already carries five changes. Scheduling, not severity.
**Governing RFC**: none — defect fix.
**Source**: the dev team's survey in `CAP-001`'s review request §5, after
`CAP-001` fixed the fourth instance. Rulings in
`.git-exclude/reviewed/CAP-001-review.md` §3.
**Depends on**: `CAP-001`, which established the pattern. Not on `RACE-002`,
which needs a different fix.

---

## 1. The shape

`CAP-001` was one instance of: **read a guard condition on the pool, decide,
then write on the pool, with no transaction across the two.** Two concurrent
requests both read the pre-change state, both decide they may proceed, and both
write. Three more sites have it.

## 2. The three, in priority order

**2.1 — The last-admin guard. Do this one first.**

`handlers/teams.rs:296-306` (demote) and `:336-345` (remove) both run
`role_for` → `admin_count` → a separate `update_role` / `remove_member`, all on
`&state.db`. **Read and confirmed, not inferred.** Two concurrent demotions of
a team's last two admins each see `admins == 2` and both proceed.

**The result is a team with zero admins, and no remaining member can undo it**
— adding an admin requires being one. The reachable path is ordinary rather
than exotic: two admins both pressing *Leave team* while a team is wound down,
since `is_self_removal` goes through the same guard at `:336`.

This is the only one of the three whose outcome a user cannot recover from, and
it should be its own commit even if the others travel with it.

**2.2 — `sprints::start`.** Reads the sprint, calls `active_for_team`, refuses
if another is active, then `UPDATE … SET status='active'` on the pool. Two
concurrent starts on different sprints of one team both pass, and the team has
two active sprints — a state the function's own doc says cannot exist, and
which no schema constraint prevents.

Note what `ORD-002` changed here: `active_for_team` is
`ORDER BY started_at DESC, rowid DESC LIMIT 1`, so the pick is now
deterministic. **A deterministic pick over an impossible state hides it** —
the product will consistently show one of the two active sprints and never
signal that the other exists. That makes this worth fixing rather than less so.

**2.3 — `user_capacities::close_at`.** A read-modify-write around the
now-atomic `update`: it calls `find`, then `update` with the row's
previously-read `points` and `note`. `CAP-001`'s transaction covers the
`update` but not the `find`, so a concurrent edit to `points` between them is
overwritten with the stale value.

## 3. The fix

**`CAP-001`'s pattern, unchanged**: `pool.begin_with("BEGIN IMMEDIATE")`, the
guard read *through the transaction*, the write on the same transaction,
commit. The reasoning for `IMMEDIATE` over a deferred `BEGIN` is in
`user_capacities.rs`'s module comment — **reference it, do not restate it.**

- **2.1 spans the web layer**, which is the one structural difference. The
  guard lives in `handlers/teams.rs` and the writes in `storage::teams`. Do not
  open a transaction in a handler: **move the guard into a storage function**
  that owns check-and-write as one operation, and have the handler call that
  and render the existing error on `Conflict`. Two functions — one for demote,
  one for remove — or one with the role as a parameter, whichever reads better;
  say which you chose.
- **Keep every existing message.** `LastAdminDemotionError` and
  `LastAdminRemovalError` are what users see and the redirect shape stays.
- **2.3 is the smallest**: extend the existing transaction backwards to include
  the `find`.

## 4. Verification

- **A concurrent test per site, failing before and passing after, with both
  runs reported and a stated failure rate**, as `CAP-001` did — N simultaneous
  requests released by a barrier, and the before-run must fail *reliably*, not
  occasionally. Say what N is and how many of how many runs failed.
- **2.1**: a team with exactly two admins, both demoted at once → one succeeds,
  one is refused, **one admin remains**. The same for two simultaneous
  self-removals, which is the realistic path.
- **2.2**: two sprints of one team started at once → one active.
- **2.3**: a concurrent `points` edit during `close_at` is not overwritten.
- **Sequential behaviour and every message unchanged** — name the tests that
  show it, and if none exists, pin it as `CAP-001` did.
- Non-conflicting concurrent operations still all succeed: demoting two
  *different* members of a five-admin team, starting sprints in two different
  teams.
- **Report the write-lock cost** in the shape `CAP-001` §2 used if it is
  measurable here. Team membership changes are rarer than capacity saves, so I
  expect this to be noise; measure rather than assume.
- `DEC-007`: report the new count. New test files mean the `CONTRIBUTING.md`
  block **and** the matching `test.yml` job — **both authorised**, and they
  travel together.
- Three consecutive workspace runs; `fmt`; `clippy`. The overflow gate is
  untouched.

## 5. Escalate rather than deciding

- **If `BEGIN IMMEDIATE` around the team guard measurably delays unrelated
  writes.** One database-wide write lock now covers a little more.
- **If moving the guard into storage changes any user-visible string or the
  redirect behaviour.** It should not.
- **If a fifth instance appears.** The survey that found these four was a
  script plus reading; it is not a proof.
- **If `check_optimistic_lock` gets in the way** — that is `RACE-002` and a
  different fix. Do not fold it in here.

## 6. Exit condition

Concurrent last-admin demotions and removals leave exactly one admin;
concurrent starts leave one active sprint; `close_at` cannot overwrite a
concurrent edit; each demonstrated by a test that failed reliably before.
