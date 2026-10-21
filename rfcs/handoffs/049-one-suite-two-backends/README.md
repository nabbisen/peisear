# Handoffs — one suite, two backends

**0.56.0.** `RFC 018` **P4**, the last phase. One subject: **the
claim that both backends work, made by something that would notice
if they did not** — plus the operations story that a second backend
implies and `NFR-CMP-003`'s restatement, which is P4's own closing
clause.

0.55.0 shipped a backend that runs. This series is about whether
anyone can tell.

---

## Rounds

| ID | Handoff | Subject | Blocked on | State |
|---|---|---|---|---|
| CON-001 | [the backend nobody can select](./CON-001-the-backend-nobody-can-select.md) | The feature passthrough. **Measured: no command line builds `peisear-web` or `peisear` against PostgreSQL** — both CLI forms hit `dialect.rs:89`'s `compile_error!`. Everything else here is blocked on it. Adds the feature and a **CI boot smoke**; documents no install path (§3) | — | **Closed** — 2 rounds. Round 1 stopped correctly (§1’s census was two crates; the answer is three); round 2 implemented the corrected shape, and CI booted `peisear` against a real PostgreSQL server for the first time. One claim of the architect’s was retracted in review — the postgres build *does* carry the SQLite driver; recorded on `ROADMAP.md`’s P5 and in `CON-004` §5 |
| CON-002 | [the suite that runs whole](./CON-002-the-suite-that-runs-whole.md) | **48 tests compile under `postgres`; CI ran 31 by name.** And a `--lib` name filter that matches nothing exits 0 — measured — so those lines cannot tell *green* from *examined nothing*. `§10.46` | CON-001 | **Closed** — 1 round. Unfiltered suite in CI, four name-filtered lines deleted, 46 of 48 running with the two SQLite-specific exceptions named and gated. **Escalated a production defect that reshapes the series** — see `CON-005` |
| CON-005 | [the transaction seam](./CON-005-the-transaction-seam.md) | **Fourteen production call sites open `pool.begin_with("BEGIN IMMEDIATE")`** — a syntax error on PostgreSQL, so no guarded write runs there at all. It compiles, which is why every earlier gate passed. The seam belongs in `dialect`; the mechanism is per-site and not decided | CON-002 | **Closed** — 3 rounds. All fourteen sites, the seam in one home, and **all four mechanisms proven necessary by a test seen to fail without them**. Round 3’s own first attempt passed while reverted; it investigated rather than taking the green, and reported it |
| CON-006 | [the integer that is four bytes](./CON-006-the-integer-that-is-four-bytes.md) | `migrations-postgres` declares `INTEGER` (4-byte) where Rust decodes `i64`. **`issues.effort` is one of them, so no issue can be read on PostgreSQL at all.** Fix the invariant — every integer column `BIGINT` — and assert it against the live schema | CON-005 | **Closed** — 1 round. Every integer column `BIGINT`, a no-allow-list predicate over `information_schema` seen red with all 28 offenders named and green after, and the `SUM(int8)→numeric` second-order defect its own widening caused found by CI and fixed in the same round |
| CON-003 | [the lock tests on both backends](./CON-003-the-lock-tests-on-both-backends.md) | P4's gate says *storage **and lock*** tests. The 26 lock tests are in `peisear-web`, and `TestApp::spawn` hardcodes a `sqlite://` tempdir. **Isolation across 26 tests on one server is the round** | — (all three closed) | **Next** |
| CON-004 | [the operations story](./CON-004-the-operations-story.md) | `backup.md` is wholly SQLite and never says so; **`JWT_SECRET` is mandatory on PostgreSQL and no document says it**; `NFR-CMP-003` restated for the SQLite backend | — (written last by choice) | Not started |
| REL-0.56.0 | not written | The cut, `DEC-028`, `DEC-056`. Written when the six rounds close | all six | Not written |

**Status lives in the table above, not in a message.**

---

## The passthrough is in scope, and it is test infrastructure

`ROADMAP.md`'s 0.56.0 row is *"one suite against both backends in
CI, and the operations story"*. **The feature passthrough is not in
it** — `scaling.md` calls it *"not yet scheduled"*.

Scoping found it is not an addition for convenience. It is a
**prerequisite discovered by measurement**: P4's gate names the lock
tests, the lock tests live in `peisear-web`, and `peisear-web`
cannot select the backend. The gate is unreachable without it — the
same shape as `PG-004` in the last series, a gap in the phase table
found by measuring rather than by reading it.

**What it does not become is an install path.** Measured at
`c98739a`:

| | SQLite | PostgreSQL, after this series |
|---|---|---|
| `peisear-storage` unit tests | 31 | 48 |
| `peisear-web` integration tests | **309** | **26** |

The 283 that would not run are the only coverage that exists for
`personal_metrics.rs`, `user_burnout.rs`, `project_health.rs`,
`metrics_snapshots.rs`, `user_metrics_snapshots.rs`, `issues.rs` and
`sprints.rs` — **every one of which has zero unit tests.** Two of
them carry **P0** privacy controls enforced in SQL that `PG-004`
rewrote: `NFR-PRIV-007`'s contributor floor
(`sprints.rs:1148`, `COUNT(DISTINCT i.assignee_id)`) and
`NFR-PRIV-001`'s capacity visibility (`issues.rs:897`,
`project_workload`). The tests that prove them —
`aggregate_privacy`, `workload_privacy` — stay SQLite-only.

Nothing here says the translations are wrong; `PG-004` was careful
and its own equivalence tests show it. It says what this project
already holds: **a check that was never run is not evidence.**
Documenting an install path would engage `NFR-REL-011`, and the walk
it requires would certify a configuration in which a P0 privacy
control is unverified.

So: **the feature ships, no install path is documented, no second
archive set, no second container tag.** `CON-004` §4 writes the
status honestly in `scaling.md` — the feature is public API on
docs.rs whether we mention it or not, and an unexplained feature is
worse for a reader than either answer. The one real benefit a walk
would have had is taken without the claim: `CON-001` adds a **CI
boot smoke** — build with the passthrough, start against the service
container, hit the health endpoint — which is more than any unit
suite proves and documents nothing.

What would earn a documented, walked install path is the full
integration suite on both backends. That is a later release and is
**not scheduled**.

## What this series is not

**Not the other 30 integration files.** P4's words are *"storage and
lock tests"*. `CON-003` takes the 26 lock tests in three files. If
the mechanism makes the rest cheap, that is a finding to report, not
a round to start.

**Not a supported PostgreSQL deployment.** The section above is not
a caveat on the release, it is the release's own claim: the backend
becomes buildable and better tested, and it does not become
something an operator is invited to run.

**Not a data migration.** There is still no way to carry an existing
SQLite installation into PostgreSQL, `peisear migrate` does not
exist, and nothing here changes that. `scaling.md` says so already
and should keep saying it.

**Not a change to SQLite's path.** Same default, same tests, same
timings. Every round in this series has that as an explicit gate,
and `CON-001` is the one where getting it wrong would reach every
user at once — a passthrough that quietly moves the default build's
resolved features is this series' worst outcome.

**It was not open to a fifth round, and `CON-005` is one.** The
standing exception applied exactly as written: `CON-002` found that
no guarded write executes on PostgreSQL, which blocks `CON-003`'s own
gate, so it was never a scope question. Everything *else* found still
goes to the architect and waits.

---

## What the last series left for whoever reads this first

- **A filter is not a gate.** `CON-002` §2 measured it: `cargo test
  --lib <name-that-matches-nothing>` exits **0**. `cargo test --test
  <file-that-does-not-exist>` exits **101**. The first shape is in
  CI four times; the second is in CI about thirty-five times and is
  fine. Know which one you are writing.
- **Measure the population before carrying a figure.** `PG-004`
  reported *11 pass / 18 fail* under `postgres`; the compiled set is
  **48**. Both can be true of different selections, which is exactly
  why one must not be quoted for the other.
- **`test.yml` is a gate's own source.** Editing it means running
  it, not reading its other lines.
- **`publish-dry-run` is a CI job** (`FIV-007`). A `[features]`
  section changes what `cargo package` resolves — `0.53.0`'s leg 3
  is the standing example of finding that out at publish time.
- **`.env.example` is behind a standing deny rule** and the
  architect has not read it while scoping this. Nothing in these
  handoffs describes its contents.
