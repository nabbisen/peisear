# Handoffs — read-then-write

**Not RFC-governed.** Guards that read a condition on one connection and write
on another, with no transaction across the two, so two concurrent requests both
pass. `CAP-001` (0.38.0) fixed the first instance and established the pattern;
the dev team's survey in its review request found four more.

**Targeted at 0.39.0, not 0.38.0** — all are pre-existing, none is a regression
from that release, and 0.38.0 already carries five changes. Scheduling, not
severity.

**Both open as of 2026-09-24.** 0.38.0 is tagged and published and both
specifications are amended to it (`DEC-028`). They can be taken in either
order; `RACE-001` carries the more consequential defect.

| ID | Link | What | Release |
|---|---|---|---|
| RACE-001 | [RACE-001](./RACE-001-guards-that-read-then-write.md) | Three sites taking `CAP-001`'s `BEGIN IMMEDIATE` fix. The worst is the **last-admin guard**: `role_for` → `admin_count` → write, all on the pool, so two admins pressing *Leave team* at once leave a team with **zero admins**, which no remaining member can undo. Also `sprints::start` (two active sprints, a state `ORD-002`'s deterministic pick now hides) and `close_at`'s read-modify-write. | 0.39.0 |
| RACE-002 | [RACE-002](./RACE-002-the-optimistic-lock-is-not-atomic.md) | `check_optimistic_lock` is a pure comparison over a value the handler read earlier, so two saves carrying the same valid stamp both pass and the earlier writer is **silently lost** — the outcome the lock exists to prevent, on every locking surface. Different fix from `RACE-001`: fold the comparison into the write's `WHERE` and read zero affected rows as the conflict. | 0.39.0 |
| RACE-003 | [RACE-003](./RACE-003-two-small-guards-and-a-window-that-stays-open.md) | The small remainder of the survey. `plan_add` races `complete`, so an issue can join a finished sprint — the one sibling case `RACE-002`'s stamp does not close, because the plan routes carry no lock. `teams::add_member` gets the **right state and the wrong words**: the primary key refuses the duplicate, but a raw database error surfaces where `UserAlreadyTeamMemberMessage` is owed. And the actor-authority window is **ruled not a defect** and written down once, so the fourth survey stops rediscovering it. | 0.39.0 |

