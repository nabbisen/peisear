# PG-001 — the connection

**Target**: 0.55.0, first. `RFC 018` P3's *"pool options"*.

**Nothing else in this series can be tested without it.** `DIA-002`
left `pool::connect` and `pool::migrate` as `unimplemented!()` stubs
behind the `postgres` feature; until they open a real connection, a
ported migration cannot be applied and a ported trigger cannot be
fired.

---

## §1 — what is there

`crates/peisear-storage/src/pool.rs`: two `#[cfg(all(feature =
"postgres", not(feature = "sqlite")))]` functions that panic, with
doc comments saying P3 replaces them. The SQLite side above them is
the shape to match — read it before writing the PostgreSQL one.

**`DIA-002` deliberately left connection options unabstracted**, and
gave the reason: *"`SqliteConnectOptions`/`SqliteJournalMode` have no
PostgreSQL equivalent; a real server decides the Postgres tuning, not
a guess here."* **That is now your decision to make against a real
server.**

## §2 — what to build

- **`connect`**: `PgPoolOptions`, with max connections, acquire
  timeout and whatever else you choose. **Each setting gets a reason**
  — the SQLite side's own options carry theirs, and a number without
  one is the thing `§10.43` is about.
- **`migrate`**: applies the PostgreSQL migration set. `PG-002`
  creates that set; **until it exists, this function has nothing to
  run** — so land `connect` first and say plainly that `migrate` is
  still a stub if `PG-002` has not landed.
- **A PostgreSQL service in CI.** `RFC 018` §5 puts the twice-run
  suite at P4, but **nothing in this release can be proved without a
  database**, so the service arrives here. One job, `postgres`
  feature only; the SQLite suite is untouched and still the default.

## §3 — what must not happen

- **No `sqlite` behaviour changed.** The default build, its tests and
  its timings are not this round's subject. If a shared helper needs
  moving, say so rather than changing SQLite's path to suit
  PostgreSQL's.
- **No queries ported.** That is `PG-004` and it is 295 placeholders.
- **No trigger work.** `PG-003`.

## §4 — gates

- `DEC-007` from a fresh block, **feature absent** — unchanged, and
  say so.
- The suite with `--no-default-features --features postgres`:
  **report the figure honestly.** It will still be mostly failures
  until `PG-002` and `PG-004`; what matters is that the failures move
  from `unimplemented!()` to something real.
- `fmt`, `clippy` both profiles **and with the feature**, `rustdoc`,
  `mdbook`, **`cargo publish --workspace --dry-run`**.
- **The CI PostgreSQL job green**, and the connection proved by
  something — a `SELECT 1` is enough at this stage; say what you used.

## §5 — escalate rather than deciding

- **If `PgPoolOptions`' defaults look right and you cannot justify
  changing them.** Say that; an unchanged default with a reason beats
  a tuned number without one.
- **If the CI service cannot be added without touching the SQLite
  job.** Report the coupling.

## §6 — exit condition

`connect` opening a real pool with every option reasoned; `migrate`
implemented or honestly still a stub pending `PG-002`; a PostgreSQL
service in CI with one green job proving a real connection; the
SQLite default untouched, shown by an unchanged `DEC-007`; the
postgres-feature suite's figure reported as it is, not as it should
be; all gates re-run after the last edit.
