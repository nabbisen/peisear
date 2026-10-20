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
| CON-001 | [the backend nobody can select](./CON-001-the-backend-nobody-can-select.md) | The feature passthrough. **Measured: no command line builds `peisear-web` or `peisear` against PostgreSQL** — both CLI forms hit `dialect.rs:89`'s `compile_error!`. Everything else here is blocked on it. **Carries one question for the owner** (§3) | — | **Next** |
| CON-002 | [the suite that runs whole](./CON-002-the-suite-that-runs-whole.md) | **48 tests compile under `postgres`; CI runs 31 by name.** And a `--lib` name filter that matches nothing exits 0 — measured — so those lines cannot tell *green* from *examined nothing*. `§10.46` | CON-001 (partly) | Not started |
| CON-003 | [the lock tests on both backends](./CON-003-the-lock-tests-on-both-backends.md) | P4's gate says *storage **and lock*** tests. The 26 lock tests are in `peisear-web`, and `TestApp::spawn` hardcodes a `sqlite://` tempdir. **Isolation across 26 tests on one server is the round** | CON-001 | Not started |
| CON-004 | [the operations story](./CON-004-the-operations-story.md) | `backup.md` is wholly SQLite and never says so; **`JWT_SECRET` is mandatory on PostgreSQL and no document says it**; `NFR-CMP-003` restated for the SQLite backend | — (written last by choice) | Not started |
| REL-0.56.0 | not written | The cut, `DEC-028`, `DEC-056`. Written when the four rounds close | all four | Not written |

**Status lives in the table above, not in a message.**

---

## One thing needs the owner's authorization before `CON-001` acts

`ROADMAP.md`'s 0.56.0 row is *"one suite against both backends in
CI, and the operations story"*. **The feature passthrough is not in
it** — `scaling.md` calls it *"not yet scheduled"*.

Scoping found it is not an addition for convenience. It is a
**prerequisite discovered by measurement**: P4's gate names the lock
tests, the lock tests live in `peisear-web`, and `peisear-web`
cannot select the backend. The gate is unreachable without it. That
is the same shape as `PG-004` in the last series — a gap in the
phase table found by measuring rather than by reading it.

The passthrough itself is therefore proposed as in-scope. What needs
an answer is **what it means for the release**, because a documented
install path pulls in `NFR-REL-011`'s CI walk:

1. **Document `cargo install peisear --no-default-features --features
   postgres` and walk it in CI** — recommended. The service
   container already exists for the storage job.
2. **Ship it undocumented** — no new install path, `NFR-REL-011`
   untouched.

**Document-and-not-walk is not an option**; that is `§10.41`, which
this project has already paid for once.

`CON-001` §3 carries this in full and tells the round to wait.
**Neither option adds a second archive set or a second container
tag.** Five release targets do not become ten here.

---

## What this series is not

**Not the other 30 integration files.** P4's words are *"storage and
lock tests"*. `CON-003` takes the 26 lock tests in three files. If
the mechanism makes the rest cheap, that is a finding to report, not
a round to start.

**Not a data migration.** There is still no way to carry an existing
SQLite installation into PostgreSQL, `peisear migrate` does not
exist, and nothing here changes that. `scaling.md` says so already
and should keep saying it.

**Not a change to SQLite's path.** Same default, same tests, same
timings. Every round in this series has that as an explicit gate,
and `CON-001` is the one where getting it wrong would reach every
user at once — a passthrough that quietly moves the default build's
resolved features is this series' worst outcome.

**Not open to a fifth round.** Anything else found goes to the
architect and waits, with the standing exception: a defect that
**blocks the release** is not a scope question.

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
