# STORE-002 — the backstop, and the cost of durability

**Two small subjects, both storage.** `RFC 018` P0b, and the half of
`STORE-001` its own demonstration could not reach.

**Review**: `.git-exclude/reviewed/STORE-001-review.md` (§2, §3).
**Target**: 0.50.0, second round.

---

## §1 — the five guards `STORE-001` never reached

`STORE-001` demonstrated the **Rust** check (`check_optimistic_lock`, 14 call
sites across `issues.rs`, `projects.rs`, `sprints.rs`, `settings.rs`). Three
entities carry a **second guard, in SQL**:

```
AND (?N IS NULL OR julianday(updated_at) = julianday(?N))
```

`projects.rs:162`, `projects.rs:210`, `sprints.rs:204`, `sprints.rs:240`,
`user_capacities.rs:514`. **Five sites, three entities.**

**A handler test cannot reach them** — the Rust check refuses a stale token
first, so the request never gets to the storage call. **These are
storage-level tests**, calling the update functions directly.

**What to assert, per guard**: a stale **millisecond** token is refused
(no row updated, or the crate's own conflict signal — follow what each
function already returns), and the **current** token succeeds. Both halves:
a guard that refuses everything passes the first assertion and is useless.

**And it must be seen to fail.** `0021` is already landed, so write the test,
then **plant**: restore the pre-`0021` trigger for one of the three tables in
a scratch copy, confirm the test for that entity fails, restore
byte-identical. **Plant by file copy — never `git checkout` or `git stash`
on the working tree** — and rebuild whatever binary the test actually reads
before trusting a run.

**I measured these guards in isolation and they hold at millisecond
precision** (review §2's table). **That is not the same as demonstrating
them, which is the point of this round** — and if your demonstration
disagrees with my measurement, yours is the one that counts.

## §2 — `RFC 018` P0b: what `synchronous = FULL` costs

`pool.rs:29` sets `SqliteSynchronous::Normal`. With WAL that survives an
application crash but **can lose recently committed transactions on a power
cut or an OS crash**. `FULL` removes that, at a write cost **nobody here has
measured**.

**Measure it, then decide it on the number.** Not a benchmark suite — a
figure honest enough to choose by:

- A write-heavy path this product actually runs. The fixture builders in
  `browser-checks/` already create a registration, a team, a sprint, a
  project and several issues through the real forms; that sequence timed
  under both settings is a fair, boring measurement.
- **Report both numbers and the ratio**, and say what hardware produced
  them — a figure without a machine behind it is not reusable.

**The decision is yours to recommend and mine to confirm.** My prior, stated
so you can argue against it: **`FULL` is probably right for this product** —
5–30 users on one node, writes that are human-paced, and a durability
guarantee that is part of why someone self-hosts. **If the measured cost
says otherwise, say so and show the number.**

**Whichever way it goes, the setting gets a comment naming the trade** and
the measurement date. An unexplained pragma is the thing that drifts.

## §3 — gates

- **`DEC-007` will move** — §1's tests, and §2's if you leave any behind.
  **Say the number and name each addition.**
- **The three browser gates** if `synchronous` changes: it alters write
  timing, and `undo-mousedown-trap` and `drag-outcome-gate` both measure
  behaviour after a write.
- `fmt`, `clippy -D warnings`, `rustdoc-links` (seven crates, eight
  private-link warnings, zero broken).
- **No new migration**, so the candidate's database check stays at **21 with
  one applied** — `STORE-001`'s change, unchanged by this round.

## §4 — escalate rather than deciding

- **If any of the five guards does not refuse a stale millisecond token.**
  That is a live defect and it stops this round.
- **If the plant cannot be made to fail** for the entity you choose — then
  the test is not testing the guard.
- **If `FULL`'s measured cost is large enough to be a real trade-off**
  rather than a rounding error. Bring the number; do not choose for me.

## §5 — exit condition

Five guards demonstrated against a stale millisecond token, with one of them
planted and seen to fail; a measured write cost for `NORMAL` and `FULL` with
the machine named; the setting chosen, commented with its trade and its
measurement date; `DEC-007` stated with each addition named.
