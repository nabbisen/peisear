# REL-0.55.0 — release candidate

**Target**: 0.55.0, *a second backend that runs*. `RFC 018` P3 plus
the queries, four rounds, all closed 2026-10-20.

**What is different about this release**: 0.54.0's novel risk was a
migration existing users had to apply. **0.55.0's is the opposite** —
the SQLite set has not changed at all, and what ships instead is a
second backend with real code behind a feature that was empty last
release. **Nothing an existing operator runs should move; everything
new is reachable only by choosing it.** Both halves need proving.

---

## §0 — already done; reconcile, do not redo

| round | what it left for the cut |
|---|---|
| `PG-001` | `connect`, pool options, a PostgreSQL service in CI, URL-scheme rejection both ways |
| `PG-002` | `migrations-postgres/`, the dialect-aware embed, the filename drift guard |
| `PG-003` | eleven triggers, `ERRCODE`, the shared validation list |
| `PG-004` | `dialect::sql`, six divergent constructs, `0023`'s `rowid` surrogate |

`DEC-007`: **511 → 514** across this release's own rounds (509 at the
start of `PG-003`). Measure it fresh.

## §1 — the ordinary cut

- **Version**: `0.54.0` → `0.55.0`.
- `CHANGELOG.md`: `## [Unreleased]` → `## [0.55.0] — <the date you
  cut it>`.
- **`ROADMAP.md`'s 0.55.0 row leaves, and the "Shipped" line lands,
  when the release is *cut* — not when this candidate is written.**
  This is now an **exit condition**, not a warning: 0.53.0's row said
  shipped while leg 3 was failing, and 0.54.0's candidate wrote the
  line before the legs ran and was saved only by their succeeding.
  **Leave both to the architect**, who runs the legs and knows when
  they finished.

## §2 — the claim to falsify: nothing an existing user runs has moved

`git diff 0.54.0..HEAD -- crates/peisear-storage/migrations/` is
**empty**, and the `postgres` feature is off by default. So an
existing SQLite operator upgrading should see **no migration, no new
behaviour, nothing**.

**That is a claim, so walk it**, the same shape 0.54.0's §2 used and
for the opposite reason:

1. `cargo install peisear --version 0.54.0` into a scratch root; run
   it from an empty directory; register; confirm `/projects`.
2. Run the candidate against **that same database**.
3. **Assert the migration count did not change** — read
   `_sqlx_migrations` directly, before and after. Still **22**.
4. The original session's cookie still authenticates.

**A migration appearing here would be the defect**, and it is exactly
what a stray file in `migrations/` would cause. Report both counts
verbatim.

## §3 — the feature that now has code behind it

`peisear-storage` published a `postgres` feature at 0.54.0 with
`unimplemented!()` behind it. **At 0.55.0 it has a real backend.** The
crate root already says so accurately — `PG-004` rewrote it — so this
round's job is to **check it still does** and fix what did not keep up:

- **`lib.rs:21`'s own next paragraph is stale**: *"The current
  implementation is backed by SQLite via `sqlx` … trait abstraction
  becomes useful once a second backend (PostgreSQL, per the roadmap)
  **is in flight**; until then it is speculative infrastructure."*
  A second backend is no longer *per the roadmap* — it is in the
  crate, described two paragraphs above. **The sentence contradicts
  its own neighbour.**
- **`289` appears in two published doc comments** (`lib.rs:10`,
  `dialect.rs:67`), both hedged *"roughly"*, when `PG-004` measured
  **284** at close and said why the five went. **`dialect.rs` is the
  file that already carries "the two should never be quoted
  interchangeably"** — about a different pair of numbers, for exactly
  this reason.

**Do not soften the "not yet proven" half.** The crate root says the
conformance suite does not exist and that some test helpers still
hardcode `sqlite://`. That is true and it is the sentence most likely
to be tidied away by someone improving the prose at release time.

## §4 — `DEC-028`: both specifications, five passes

- **Pass 1** amends from the code: anything claiming PostgreSQL has
  no implementation.
- **Pass 5 greps for the instruction, not the document** — `0.53.0`
  caveated one of three files carrying one sentence.
- **Appendix A recomputed from its own rows**, never by the delta.
- **`NFR-CMP-003` is still not restated.** `RFC 018` puts it at P4 and
  this release is not P4. Say you left it and why.
- **`§11`/Appendix C's 0.55.0 row** names what shipped **and what it
  does not prove**: no conformance suite, no two-backend run of the full
  storage and lock tests, `NFR-CMP-003` untouched.

## §5 — the gate table, re-measured against the candidate commit

- `DEC-007` from a fresh block, three runs, **feature absent**.
- `fmt`; `clippy` **both profiles and both feature configurations**;
  `rustdoc` **under both features**, after the last edit; `mdbook`.
- **`cargo publish --workspace --dry-run`** — and
  `cargo package -p peisear-storage --list` showing **both** migration
  directories, `0023` included.
- **The three browser gates.**
- **The CI PostgreSQL job**, read by job rather than by run.

## §6 — what this release does not prove, stated not implied

- **No conformance suite.** `RFC 018` P4 runs the storage and lock
  tests against both backends; this release runs a targeted set.
- **The postgres-feature `--lib` suite still fails outside that set**,
  because test helpers hardcode a `sqlite://` pool — **not because a
  production query is wrong.** That distinction belongs in the
  changelog, not only in a doc comment.
- **`NFR-CMP-003`** still describes a single-file database without
  qualification.

## §7 — escalate rather than deciding

- **If §2's walk shows any migration applied.** Stop.
- **If a gate moves and you cannot account for the delta.**
- **If the crate root's "not yet proven" half needs weakening** to
  stay accurate — that would mean something shipped that this
  candidate did not expect, and it is a stop.

## §8 — exit condition

Version bumped; changelog dated at the cut and naming what is not
proven; **`ROADMAP.md`'s row and shipped line left to the architect**;
§2's walk performed with both migration counts reported verbatim and
**22 before and after**; §3's two stale items fixed and the crate
root's unproven half intact; both specifications amended under
`DEC-028` with Appendix A recomputed from its rows and `NFR-CMP-003`
deliberately untouched; every §5 gate re-measured; and a
recommendation to the architect — **not a tag**.
