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

---

# PG-001 round 2 — neither backend says the URL is for the other one

Round 1 is accepted (see the review). `connect` is real, every option
is reasoned, the CI service is green, and leaving `PgPoolOptions`'
defaults alone with a stated reason was the right call. **One thing
`connect` now owns and does not do.**

## §7 — the most likely mistake a user will make, and what they get

This release makes a second backend real, so **choosing the wrong
`DATABASE_URL` stops being impossible and becomes the first thing
anyone switching will get wrong.** Measured at the review, both
directions:

| build | `DATABASE_URL` | what the user gets |
|---|---|---|
| default (`sqlite`) | `postgres://localhost/peisear` | `Error: error returned from database: (code: 14) unable to open database file` |
| `--features postgres` | `sqlite://data/app.db` | **a 30-second hang**, then `Database(PoolTimedOut)` |

**Neither message names the real problem.** The SQLite one is a file
error about a URL that names no file, printed *after* a startup line
that logs `database=postgres://localhost/peisear` as if it were fine.
The PostgreSQL one makes the reader wait half a minute to learn
nothing — `sqlite://data/app.db` parses as a plausible host, so sqlx
dials it and gives up on the acquire timeout.

**Round 1's own report already saw half of this** and read it as a
good sign: *"connect moved from 'cannot be reached' to 'reached, and
correctly fails against the wrong kind of URL.'"* It is reached, and
it does fail — but *"correctly"* is doing work there that a user
would not recognise.

## §8 — what to build

**Each `connect` checks the scheme first and says which build it is.**
One comparison before any option parsing, in both branches:

- the `sqlite` branch rejects a URL whose scheme is not `sqlite`;
- the `postgres` branch rejects one whose scheme is not `postgres` or
  `postgresql` — **both spellings; libpq accepts either and so does
  `PgConnectOptions`**, so rejecting `postgresql://` would be a new
  defect.

**The message names three things**: which backend this build is, what
the URL's scheme says, and that a different build is needed —
`DIA-002`'s own `compile_error!` text is the register to match, since
it already tells a reader what to do next rather than only what is
wrong.

**Fail before connecting, not after.** The point is the 30 seconds as
much as the wording.

## §9 — the bound, so this does not grow

**Scheme only.** Not the host, not the database name, not whether the
server exists — those are what a connection attempt is for, and a
`connect` that pre-validates them would be a second, worse parser.

**One `StorageError` variant, or an existing one** — your call, with
the reason. A new variant that only this check raises is defensible;
so is reusing `Bootstrap`. Say which and why.

## §10 — gates

- `DEC-007`, feature absent — **this moves** by the new tests.
  Account for the delta exactly.
- **Both rejections tested, both spellings of `postgresql` accepted**,
  and a correct URL still connecting on each branch — the last one
  matters, because a scheme check that rejects everything also passes
  a test that only checks rejection.
- The CI PostgreSQL job still green.
- `fmt`, `clippy` both ways, `rustdoc`, `mdbook`, `publish-dry-run`.

## §11 — exit condition

Both `connect` branches rejecting the other backend's scheme before
attempting a connection; `postgres://` and `postgresql://` both
accepted by the PostgreSQL branch; each message naming the build, the
scheme found, and what to do; the check bounded to the scheme with
that bound written down; a correct URL still connecting on both
branches, shown by test; `DEC-007`'s delta accounted for; all gates
re-run after the last edit.

---

# PG-001 round 3 — the check rejects a path that always worked

Round 2 is accepted (see the review). The wrong-backend case is
closed, both spellings of `postgresql` are accepted, and turning a
390-second suite into a 0.00-second one is a real result. **The check
is one case too wide.**

## §12 — `url_scheme` has no notion of "no scheme"

```rust
fn url_scheme(url: &str) -> &str {
    url.split(':').next().unwrap_or(url)
}
```

**A URL with no colon returns the whole string**, which is never
`"sqlite"`, so the `sqlite` branch rejects it. Measured at the review
against this round's own build:

| `DATABASE_URL` | result |
|---|---|
| `sqlite://:memory:` | starts |
| `sqlite::memory:` | starts |
| **`data/app.db`** | **`expected a sqlite:// URL`** |

**`sqlx` accepts a bare path.** From `sqlx-sqlite-0.8.6`'s own
`FromStr`:

```rust
url = url.trim_start_matches("sqlite://").trim_start_matches("sqlite:");
```

It *trims the scheme if present* and treats the rest as a path. So
`DATABASE_URL=data/app.db` has worked in every release of this
project, and stops working here.

**The Windows edge is the one that makes it concrete.** This project
ships an `x86_64-pc-windows-msvc` archive since 0.53.0, and
`C:\peisear\app.db` — the most natural thing a Windows operator
types — has scheme `"C"` under this function. `sqlx` would accept it.

**`§10.45`'s neighbour**: 0.53.0 shipped installable artefacts, so
upgrade breakage is a real cost again (`project_no_production_use_yet`
expired). A `DATABASE_URL` that worked at 0.54.0 and fails at 0.55.0
is a breaking change, and this one is undocumented-but-working rather
than documented — which makes it *more* likely to be in someone's
systemd unit, not less.

## §13 — the rule each branch actually needs

**They are not symmetric, and that is correct**:

- **`sqlite`**: reject only when the URL *names another backend*. A
  string with no `://` is a path — accept it, which is what `sqlx`
  does. `postgres://…` is rejected; `data/app.db` and
  `C:\peisear\app.db` are not.
- **`postgres`**: keep requiring `postgres`/`postgresql`. A bare path
  genuinely is not a PostgreSQL URL, so rejecting it there is right
  and the message is the useful one.

**Say the asymmetry out loud in the code**, with the `sqlx` quote
above as the reason — otherwise the next reader "fixes" the
inconsistency and reintroduces this.

**Do not build a URL parser.** `§9`'s bound stands: scheme only, and
now *the presence of a scheme* as well.

## §14 — gates

- `DEC-007` — **moves again**; account for it.
- **The cases this round is about, all tested**: `data/app.db`,
  `C:\peisear\app.db`, `sqlite://…`, `sqlite::memory:` all accepted on
  the `sqlite` branch; `postgres://…` still rejected there; a bare
  path still rejected on the `postgres` branch. **The accept cases
  matter most** — round 2's tests only asserted rejection, which is
  how a check that is too wide passes its own suite.
- `fmt`, `clippy` both ways, `rustdoc`, `mdbook`, `publish-dry-run`,
  CI PostgreSQL job.

## §15 — exit condition

A bare path and a Windows path accepted on the `sqlite` branch and
rejected on the `postgres` one; `postgres://` still rejected on
`sqlite`; the asymmetry stated in the code with `sqlx`'s own parsing
as the reason; **accept cases tested, not only rejections**;
`DEC-007`'s delta accounted for; all gates re-run.

---

# PG-001 — closed at round 3, with one correction to `§12`

Round 3 is accepted (see the review) and **`PG-001` is closed.** Two
things belong in the record rather than in a fourth round.

## §16 — `§12`'s own claim was imprecise, and the architect wrote it

`§12` said *"`DATABASE_URL=data/app.db` has worked in every release of
this project."* **Conditionally true, stated unconditionally.**
Measured at round 3's review:

| `DATABASE_URL` | parent directory | result |
|---|---|---|
| `data/app.db` | `data/` exists | **starts** |
| `data/app.db` | `data/` missing | `unable to open database file` |
| `app.db` | cwd, always exists | **starts** |

`connect`'s own `create_dir_all` is guarded by
`url.strip_prefix("sqlite://")` (`pool.rs:84`), so **a schemeless path
has never had its parent created** — long before `PG-001` existed.

**The finding itself stands**: round 2 rejected the bare path outright,
whatever its parent, and round 3 fixed that. But *"has worked in every
release"* overstated it, and this project's own most-repeated defect
is a claim measured in one condition and written as a general one.
**Corrected here rather than quietly**, because the overstatement is
in a published handoff.

## §17 — the residual, recorded and deliberately not scoped

`sqlite://data/app.db` creates `data/` if it is missing; `data/app.db`
does not. **That asymmetry predates this entire series** and is not
round 3's to fix.

It is also arguably correct — auto-creating directories for a bare
relative path is more surprising than refusing to — so this is a
question, not a defect, and it does not get a round. **If a future
round touches `connect`'s directory handling, decide it then**; if
nothing does, it stays as it is, documented here.
