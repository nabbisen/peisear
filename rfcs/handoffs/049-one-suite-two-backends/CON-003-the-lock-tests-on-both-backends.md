# CON-003 — the lock tests on both backends

**0.56.0, `RFC 018` P4. Unblocked — this is the next round.**

Three rounds stood in front of it and all three are closed.
`CON-001` gave `peisear-web` a way to select the backend at all;
`CON-005` replaced the fourteen `BEGIN IMMEDIATE` call sites with a
seam whose four mechanisms are each proven necessary; `CON-006`
widened every integer column to `BIGINT`, without which no issue
could be read on PostgreSQL. **This is the first round in the series
that can actually run.**

P4's gate names *"the same storage **and lock** tests against both
backends."* `CON-002` does the storage half inside
`peisear-storage`. The lock half is not there — it is three
integration-test files in `peisear-web`, and they are the half that
actually exercises `NFR-CONC-001`/`-002` through a running router.

---

## §1 — the population, measured 2026-10-20 at `c98739a`

| File | Tests |
|---|---|
| `crates/peisear-web/tests/optimistic_lock.rs` | 22 |
| `crates/peisear-web/tests/optimistic_lock_atomicity.rs` | **14** |
| `crates/peisear-web/tests/board_card_lock_value.rs` | 1 |
| **total** | **37** |

**Corrected 2026-10-21, by round 1.** This table first said 3 and 26.
Most of `optimistic_lock_atomicity.rs`'s tests are declared
`#[tokio::test(flavor = "multi_thread", worker_threads = 4)]`, and the
architect counted them with a literal search for the bare
`#[tokio::test]`, which matches none of them. **Count tests with the
runner, not with a grep over source text**: `cargo test -p peisear-web
--test optimistic_lock_atomicity` prints `14 passed`.

The same error ran through the whole series' arithmetic: the
integration-suite total was published as **309** and is **335**
(`capacity_atomicity` 1→4, `optimistic_lock_atomicity` 3→14,
`race_guards` 7→18, `sprint_record` 10→11). `ROADMAP.md`, `CON-001`
§3 and this series' README are corrected. **The install-path decision
those figures supported is unchanged and slightly strengthened** —
298 of 335 unrun, not 283 of 309.

**These three files, and no others.** P4's words are *"storage and
lock tests"*, and the other 30 files in `crates/peisear-web/tests/`
are not lock tests. Widening to all 33 is a different release, taken
deliberately when someone chooses it — **not reached by drift
because this round was already in the directory.** If the mechanism
built here happens to make the other 30 cheap, say so in the
completion package as a finding; do not run them.

## §2 — the blocker, and the design question inside it

`tests/common/server.rs:77`:

```rust
let url = format!("sqlite://{}/test.db", temp_dir.path().display());
```

Every `TestApp::spawn` gets a **fresh, private, fully-migrated
database** — a new tempdir per test. That isolation is free on
SQLite and is why these tests can assert on absolute state.

On PostgreSQL there is one server and 26 tests. **Reproducing the
isolation is the whole round.** The two shapes worth considering:

1. **A database per test.** Connect as the service user, `CREATE
   DATABASE peisear_it_<unique>`, migrate it, drop it at the end.
   Closest to what SQLite gives, and the isolation is unconditional.
   Cost: 37 creates and 37 migration runs of 23 migrations.
2. **A schema per test.** Cheaper, but migrations and the schema's
   eleven triggers would have to be schema-scoped, and `search_path`
   is per-connection — across a pool that is a property you have to
   keep proving rather than one you have.

**`CON-002` has already shipped a fourth shape, and this round
owns retiring it.** `optimistic_lock_guards::migrated_pool`'s
PostgreSQL body deletes three roots by fixed id (`'t'`, `'p'`,
`'u'`) before every test, which reproduces what a fresh SQLite file
gave for free — but only because CI passes `-- --test-threads=1`.
Run those five tests in parallel and one deletes the rows another is
asserting on. It is correct today and correct for the reason this
round is told not to rely on. **Whatever isolation mechanism this
round builds, apply it there too and delete the delete-by-fixed-id
reset.** One mechanism for the crate, not two.

**A third shape exists and is a trap**: one shared database,
truncated between tests, leaning on the `-- --test-threads=1` that
CI already passes to every integration file. It would pass today.
It makes correctness depend on a flag in a YAML file rather than on
the fixture, so the day someone drops the flag the failure is a
confusing cross-test one, not a clear one. **If you choose it
anyway, the isolation must be enforced in the fixture and not only
in the invocation** — and bring the reasoning before writing it.

**The architect's prior is (1)**, on the grounds that the SQLite
fixture's guarantee is *unconditional isolation* and a conformance
suite whose two halves guarantee different things is not one suite.
It is a prior, not a decision: measure the cost and say if it is
wrong.

## §3 — what must stay true

- **The SQLite path does not move.** Same 37 tests, same assertions,
  same tempdir fixture, same timings within noise. A round that
  finds itself rewriting what a lock test asserts has left its
  scope — the subject here is *where the database is*, not *what is
  true of it*.
- **The assertions are shared, not forked.** Two copies of a lock
  test that drift apart is worse than one that runs once. If some
  assertion genuinely cannot hold on both, that is an escalation
  (§5), not a `#[cfg]`.
- `0021`'s millisecond lock token is the dangerous one here, as it
  was in `PG-002`. `PG-004` already proved `stale_guard` and the
  token round-trip at the storage layer; this round proves the same
  thing through the router, which is a different claim and is the
  one `NFR-CONC-001`/`-002` are written about.

## §3a — two small things this round carries

Neither is this round's subject; both belong to whoever next touches
these files, and that is this round.

**1. The conformance predicate is narrower than its own invariant.**
`CON-006` added
`dialect::postgres_tests::no_column_in_this_schemas_own_tables_is_a_four_byte_integer`,
which tests `data_type = 'integer'`. **`smallint` would pass it and
fail identically** — `int2` decoded into `i64` is the same
`ColumnDecode` error. The invariant was *every integer column is
8-byte*; the predicate was specified narrower than that, which was
the architect's wording, not the round's. One word:
`data_type IN ('integer', 'smallint')`, with the reason in the
test's own comment. Bound: those two are the only integer types
narrower than `bigint`; `numeric`/`decimal` would also fail an `i64`
decode but are not integer columns and are out of scope here.

**2. `migrations-postgres/`'s in-place expiry — already in the tree;
this item was written in error.** `docs/development/migrations.md`
was *created* by `CON-006`'s own commit `214d8db` and its `## The
expiry` section already states the condition. The architect asserted
an absence without running the search that would have established
it, in a handoff written against the very commit that added the file.
Nothing is owed; the item stands only as a record of the mistake.

**Superseded text:** `CON-006` amended eight shipped migration files in place,
correctly — the append-only rule's measured reason is a recorded
checksum on a database that cannot exist while no install path is
documented. **That expiry currently lives only in a review.** State
it in `docs/development/`'s migration guidance: `migrations-postgres/`
becomes append-only on the same terms as `migrations/` the day
`ROADMAP.md`'s P5 documents an install path, and `CON-006` is not a
precedent for `migrations/`. If this round does not touch
`docs/development/`, hand it to `CON-004`; if neither does,
`REL-0.56.0` must.

## §3b — round 1's §4 question, answered: fix it inline

**Fix `project_health::wip_violators` in this round.** Not a new
round, and the reason is a measurement, not a preference.

```
$ grep -rn 'HAVING' crates/peisear-storage/src/*.rs
project_health.rs:236:            HAVING current_wip > effective_limit
```

**One `HAVING` in the entire crate, and it is this one.** The thing
that justified spinning `CON-005` and `CON-006` out was scale —
fourteen call sites, then a schema-wide pattern — and it is absent
here. A one-line mechanical fix that unblocks this round's own exit
condition does not earn a round of its own; leaving the round open
over it would cost the series more than the scope costs.

The fix is the known one: **repeat the expressions in `HAVING`**
rather than their `SELECT`-list aliases. PostgreSQL resolves
output-column names for `ORDER BY` and `GROUP BY` but not for `WHERE`
or `HAVING`, which is why `GROUP BY u.id, effective_limit` works in
the same query and the `HAVING` does not.

**It needs a test, because `project_health` has none.** The module has
zero unit tests, so nothing but `board_card_lock_value.rs` would
catch a regression — and that file is in this round's population by
accident of what it renders, not by design. Either assert the query
directly in `postgres_tests`, or state in the round that the board
page is the only thing standing behind it. **Say which.**

**Do not claim the class is closed.** `HAVING` is bounded at one
occurrence; *"a `SELECT`-list alias used where PostgreSQL forbids
it"* is not, because the same mistake in a `WHERE` clause cannot be
found by grepping for a keyword. **This round renders three of the
project's thirty-three integration files against PostgreSQL**, and
the remaining thirty are where any sibling defect would still be
hiding. That closure belongs to `ROADMAP.md`'s P5, and the round
should say so rather than imply `project_health` was the last one.

**One observation for the record, not a fix.** The same query's
`FROM ( … )` subquery carries no alias, which SQLite allows and
older PostgreSQL majors reject. `postgres:17` accepted it — the
error that surfaced was the `HAVING` one, not a missing-alias one —
so whether this query works on earlier majors is **unknown and not
measured here.** Note it; it becomes a real question the day a
supported PostgreSQL version range is declared, which no release
does yet.

## §4 — gates

- All 37 green on **both** backends, with both commands quoted and
  both test counts shown — `board_card_lock_value.rs` included, which
  means §3b's fix landing in this round.
- **`project_health::wip_violators` covered by something**, named
  explicitly: either its own assertion in `postgres_tests` or a
  stated reliance on the board page.
- The SQLite run unchanged: 26 tests, same names.
- `test.yml` runs the PostgreSQL half in the job that already has
  the service container. **Not by name** — `CON-002`'s `§10.46` is
  the reason, and adding three new name-filtered lines in the round
  after closing four of them would be its own kind of defect.
- `DEC-007` reconciled against whatever the delta is, stated.
- `fmt`, `clippy` both ways, `rustdoc`, `mdbook`, `publish-dry-run`.
- **`test.yml` edited means `test.yml`'s gate run.**

## §5 — escalate rather than deciding

- **If a lock assertion cannot hold on both backends.** Stop. That
  is `NFR-CONC-001`/`-002` and a requirement question, and it is
  exactly the finding P4 exists to surface.
- **If isolation cannot be made unconditional** at acceptable cost.
  Bring the measurement and the alternative; do not quietly adopt
  the shared-database shape.
- **If the SQLite run moves at all** — count, names or timings
  outside noise.
- **If the mechanism tempts you into the other 30 files.** Report
  it; do not take it.
- **If `CON-005`'s transaction seam turns out not to be enough** for
  a lock test to hold through the router. That is the same class of
  finding `CON-002` escalated and goes the same way.

## §6 — exit condition

All 37 lock tests running against both backends from one set of
assertions; `optimistic_lock_guards`'s delete-by-fixed-id reset
replaced by this round's own isolation mechanism; isolation on PostgreSQL enforced by the fixture rather
than by a test-runner flag, with the chosen shape justified against
§2's three; the SQLite run unchanged in count, names and timing;
the PostgreSQL half wired into CI without a name filter; the other
30 integration files untouched and any finding about them reported
rather than acted on; `DEC-007` reconciled; all gates re-run after
the last edit.
