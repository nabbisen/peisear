# CON-005 — the transaction seam

**0.56.0, `RFC 018` P4, inserted after `CON-002` and before
`CON-003`.** The series said it was not open to a fifth round. It
carried one exception — *"a defect that blocks the release is not a
scope question"* — and this is that: **`CON-003` cannot run at all
until this is answered.**

`CON-002` found it, escalated it rather than fixing it, and was
right to.

---

## §1 — what was found

`user_capacities::insert`, reached by a guard test running against a
real PostgreSQL server for the first time:

```
Database(PgDatabaseError { code: "42601",
  message: "syntax error at or near \"IMMEDIATE\"" })
```

`pool.begin_with("BEGIN IMMEDIATE")`. **Fourteen production call
sites**, all unconditional — re-counted independently for this
handoff:

| File | Sites | Lines |
|---|---|---|
| `user_capacities.rs` | 3 | 344, 428, 570 |
| `issues.rs` | 4 | 413, 553, 665, 740 |
| `sprints.rs` | 5 | 292, 353, 504, 572, 637 |
| `teams.rs` | 2 | 397, 437 |

**These are the guarded writes.** Capacity save, the four guarded
issue writes, sprint lifecycle, team membership. So on PostgreSQL
today **no guarded write can execute at all** — not a degraded
guarantee, a syntax error.

**It compiles.** `Pool::begin_with` is sqlx's own method and exists
for every driver; the string is only parsed by the server. That is
why `CON-001`'s build, `clippy` under both features, and every
earlier gate passed while this sat there. **A literal that is valid
for one backend and syntactically invalid for the other, in a
position nothing type-checks, is exactly what the `dialect` module
exists to prevent** — and there are fourteen of them outside it.

## §2 — what was claimed, and the correction owed

`0.55.0` shipped two statements that this finding bears on, both
still in the tree or in the published record:

- `crates/peisear-storage/src/lib.rs`, crate-root docs: *"as of
  `PG-004`, every production query runs against it too"*, and that
  the remaining suite failures are *"not because a production query
  is wrong."*
- `CHANGELOG.md`'s `[0.55.0]` section: *"Every production query
  runs"*, and the same *"not because a production query is wrong"*
  explanation for the rest of the suite failing.

**Read narrowly, "every production query runs" survives** — a
transaction-control statement is not a query, and `PG-004`'s
placeholder and date-function work was real and holds. **The
explanatory clause does not.** It asserted a cause for *all*
remaining failures, measured on the subset whose cause was a
`sqlite://` helper. There was a second cause, in production code,
and it was not examined.

That is `§10.42`'s family — a finding about the cases checked, read
as a verdict on all of them — and **the architect accepted
`PG-004`'s claim in review without testing its bound**, which is
where it should have been caught. Open it as **`§10.47`**, citing
`§10.42` as the parent class, and say plainly in it that the review
missed the bound rather than only that the round overstated it.

**What to correct, and what to leave:**

- `lib.rs`'s crate-root docs: corrected now. It is in the tree and
  it is wrong.
- `CHANGELOG.md`'s `[0.55.0]` section: **left exactly as it is.**
  Shipped history is not rewritten; `0.53.0`'s own precedent.
- `CHANGELOG.md`'s `[0.56.0]` section states the correction plainly
  when it is written, naming what `0.55.0` said and what is true.
- `docs/operations/scaling.md`: `CON-004` §4 is writing the status
  paragraph. **It now has a much harder fact to state**, and it must
  state it: guarded writes do not work on PostgreSQL. Tell `CON-004`
  rather than editing it here.

## §3 — the design direction, and what is not decided

**The seam belongs in `dialect`.** Fourteen literals asserting a
backend's transaction syntax, outside the one module whose whole
purpose is localising exactly that, is the defect independently of
which statement PostgreSQL ends up needing. Give it one home beside
`dialect::sql` and `dialect::stale_guard`, and the call sites ask
for the guarantee rather than spelling a backend's syntax.

**The mechanism is not decided here, and must not be chosen
globally.** `BEGIN IMMEDIATE` takes SQLite's write lock *before* the
check, so a check-then-write cannot interleave with a competing
writer. PostgreSQL's READ COMMITTED does not give that: two
transactions can both read, both see no conflict, and both write.
So something is needed — but what, per site, depends on what the
site actually guards.

**Measured on one site, to show the classification is not
rhetorical**: `user_capacities::insert` guards `overlaps_existing`,
a **range-overlap** predicate. No unique constraint can express it
on SQLite, so the cleanest possible answer — turn the check-and-write
into one atomic statement and need no lock on either backend — **is
not available there.** The architect's first instinct was exactly
that, and reading the site refuted it. Expect the same per site:
**classify all fourteen by what they guard before choosing
anything.**

Candidates, to be evaluated rather than adopted:

- **`pg_advisory_xact_lock(key)`** — the closest analogue. Serialises
  prospective writers on a logical key, covers conflicts with rows
  that do not exist yet, releases at transaction end. Needs a key
  derivation that is correct, not merely unique.
- **`SERIALIZABLE` plus retry** — gives the guarantee by isolation
  rather than by locking, and **changes error handling**: a
  serialization failure is a new condition the layers above must
  answer for. If this is the answer anywhere, it is an
  `NFR-CONC-*` question → see §5.
- **`SELECT … FOR UPDATE`** — insufficient wherever the conflicting
  row does not exist yet, which is at least the capacity case.

**Bring the classification and the chosen mechanism before
implementing it across fourteen sites.** Three times in this project
the architect has proposed a change that measurement refuted; a
fourteen-site edit chosen from reasoning alone is the same bet at
higher stakes.

## §3a — round 1's classification, as accepted and corrected

Round 1 classified all fourteen and recommended a mechanism per
group. **Accepted, with one site moved.** Round 2 implements this
table, not §3's open question.

| Group | Sites | Mechanism |
|---|---|---|
| **A** — one row, already existing, nothing to guard but itself | **6**: `issues::update_guarded` (413), `update_status_guarded` (553), `update_schedule_guarded` (665), `delete_guarded` (740), `sprints::add_issue_if_status` (572), `remove_issue_if_status` (637) | `SELECT … FOR UPDATE` on that row, first statement after a plain `begin()` |
| **B** — an aggregate over siblings that all exist, scoped by a shared key | **4**: `sprints::start_guarded` (292), `reopen_guarded` (504), `teams::update_role` (397), `remove_member` (437) | `SELECT … FOR UPDATE` over the scoped set (`team_id`) |
| **C** — a predicate against a row that may not exist, over a possibly empty set | **3**: `user_capacities::insert` (344), `update_guarded` (428), `close_at_guarded` (570) | `pg_advisory_xact_lock(classid, objid)`, two-key form with a reserved `classid`, keyed on `user_id` |
| **D** — one row plus the set it aggregates over | **1**: `sprints::complete_guarded` (353) | §3b |

**The SQLite branch of every one is `BEGIN IMMEDIATE`, byte for
byte, untouched.**

`SERIALIZABLE` is declined for all fourteen, on round 1's own
reasoning: every guarantee here is reachable with a blocking lock
that reproduces `BEGIN IMMEDIATE`'s shape and is then answered by the
stamp/status/conflict check already written, so adopting it would buy
nothing and surface a new error class.

**`SET LOCAL lock_timeout = '5s'` on the `postgres` branch is
required**, not optional, and lives in the seam rather than at any
call site. It matches SQLite's already-shipped `busy_timeout` rather
than inventing a number; round 1 checked the thing that mattered and
found `StorageError` has no busy/timeout variant on either backend,
so a SQLite busy timeout and PostgreSQL's `55P03` both already land
on `StorageError::Database`. **Nothing becomes newly visible to a
user, so this is not an `NFR-CONC-*` question.**

## §3b — `sprints::complete_guarded` is not a Group A site

Group A's mechanism would **weaken a guarantee the SQLite path has
and this module's own doc comment states.** `complete_guarded` holds
its transaction across four reads of state a sprint-row lock does not
cover:

1. the sprint row — locked
2. stamp + status check, then `UPDATE sprints SET status='completed'`
3. `live_totals` — `SUM(i.effort)`, `SUM(CASE WHEN i.status='done')`,
   `COUNT(*)` over `sprint_issues si JOIN issues i`
4. `burndown_live` — a second read of the same issue rows
5. `live_contributor_basis` — `COUNT(DISTINCT i.assignee_id)`, same join
6. write `sprint_records` and the burndown series

**3, 4 and 5 are three separate statements**, so under READ COMMITTED
each takes its own snapshot. `issues::update_status_guarded` touches
`issues` only — it never reads or locks `sprints` (`issues.rs:553`–
`600`) — so under Group A's mechanism it can commit **between steps 3
and 5**. The persisted record then mixes two states:
`completed_count` from before the change, `contributor_count` from
after. **`contributor_count` is `NFR-PRIV-007`'s suppression basis**,
so the tear lands on a **P0** privacy control, not on a wrong total.

The module already says what the transaction is for: *"a crash or **a
competing writer** cannot leave a `completed` sprint without its
record."* `BEGIN IMMEDIATE` excludes that writer; a sprint-row lock
does not.

**The candidate — evaluate it, it is not prescribed**: Group B's
shape at a different scope, `SELECT … FOR UPDATE` over `issues`
joined through `sprint_issues WHERE sprint_id = ?`, so a concurrent
guarded issue write on any member issue serialises against the
capture. Group C's empty-set objection does not apply: a sprint with
no issues has trivially stable aggregates, and the only way to add
one is `add_issue_if_status`, a Group A site that takes the
sprint-row lock.

**Deterministic lock order is required.** This is the first site to
lock a *set*, and two captures over overlapping issue sets acquiring
rows in different scan orders is the one deadlock shape these
mechanisms introduce. `lock_timeout` is the backstop, not the answer.

## §4 — gates

- **The guarantee, proven, not the syntax.** For each mechanism
  adopted: a test that **fails with the mechanism removed** and
  passes with it, on PostgreSQL. A check never seen to fail is not
  evidence, and "it no longer syntax-errors" is not the guarantee
  `BEGIN IMMEDIATE` was there for.
- **For `complete_guarded`, that test is the torn read specifically**
  — a concurrent guarded issue write landing between the aggregate
  statements, shown to corrupt the captured record without the
  mechanism and not with it. Not merely that the transaction opens.
- **`lock_timeout` seen to apply**: a blocked transaction failing
  rather than hanging, demonstrated once.
- The five `optimistic_lock_guards` tests green on **both** backends
  with **no `#[ignore]` left** — `CON-002`'s ignore exists to be
  deleted by this round.
- `DEC-007` — the SQLite side must not move. 515 before; state the
  delta and its cause if this round adds tests, which it should.
- `cargo test -p peisear-storage --no-default-features --features
  postgres --lib` unfiltered: **46 run, 46 pass, 0 ignored.**
- `grep -c 'begin_with' crates/peisear-storage/src` outside the
  seam: **0**. State the command and its output.
- `fmt`, `clippy` both ways, `rustdoc`, `mdbook`,
  `publish-dry-run`, and `test.yml`'s own gate if it is touched.

## §5 — escalate rather than deciding

- **If any site's answer changes what an error means to a user** —
  a retry, a new surfaced failure, a changed status code. That is
  `NFR-CONC-*` and the owner's, not the round's and not the
  architect's.
- **If the fourteen do not classify into a small number of shapes.**
  If each site needs its own reasoning, say so; that is a finding
  about the write paths, not a failure of this round.
- **If SQLite's own behaviour would change at all.** Stop. The
  SQLite path losing nothing is a condition of `RFC 018`'s
  acceptance at every phase.
- **If the guarantee cannot be given on PostgreSQL for some site**,
  bring it. A known gap stated plainly is acceptable; a site that
  silently stops being guarded is not.

## §6 — exit condition

All fourteen sites implemented per §3a/§3b, the classification
having been reported and reviewed before the broad edit (round 1); one home in
`dialect` for the transaction seam and no `begin_with` literal
outside it; for every mechanism adopted, a test seen to fail
without it; the five guard tests green on both backends with
`CON-002`'s `#[ignore]` deleted; the unfiltered `postgres` suite at
46 passed, 0 ignored; `DEC-007`'s SQLite side unmoved except for
tests this round adds, stated; `lock_timeout` in the seam and seen to apply;
deterministic lock order wherever a set is locked; `CON-004` told what `scaling.md` must
now say; all gates re-run after the last edit.
