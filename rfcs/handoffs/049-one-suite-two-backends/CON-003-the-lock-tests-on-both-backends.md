# CON-003 — the lock tests on both backends

**0.56.0, `RFC 018` P4.** Blocked on `CON-001` (these tests live in
`peisear-web`, which could not select the backend until that round
landed) and now on **`CON-005`**: `CON-002` found that all fourteen
guarded-write call sites open `pool.begin_with("BEGIN IMMEDIATE")`,
which is a syntax error on PostgreSQL. Every one of the 26 lock
tests below exercises a guarded write. **This round cannot start
until `CON-005` closes.**

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
| `crates/peisear-web/tests/optimistic_lock_atomicity.rs` | 3 |
| `crates/peisear-web/tests/board_card_lock_value.rs` | 1 |
| **total** | **26** |

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
   Cost: 26 creates and 26 migration runs of 23 migrations.
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

- **The SQLite path does not move.** Same 26 tests, same assertions,
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

## §4 — gates

- All 26 green on **both** backends, with both commands quoted and
  both test counts shown.
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

All 26 lock tests running against both backends from one set of
assertions; `optimistic_lock_guards`'s delete-by-fixed-id reset
replaced by this round's own isolation mechanism; isolation on PostgreSQL enforced by the fixture rather
than by a test-runner flag, with the chosen shape justified against
§2's three; the SQLite run unchanged in count, names and timing;
the PostgreSQL half wired into CI without a name filter; the other
30 integration files untouched and any finding about them reported
rather than acted on; `DEC-007` reconciled; all gates re-run after
the last edit.
