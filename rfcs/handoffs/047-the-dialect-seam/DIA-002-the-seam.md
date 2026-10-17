# DIA-002 — the seam

**Target**: 0.54.0, after `DIA-001`. `RFC 018` **P2**. Gate, from the
RFC: *"every existing test green on SQLite; the seam compiles with the
flag absent."*

**No PostgreSQL code.** Not a query, not a migration, not a connection
string. This round builds the place a backend would go and proves the
place is empty.

---

## §1 — what the seam has to cross, measured 2026-10-17

| fact | figure |
|---|---|
| `pub type Pool = sqlx::SqlitePool` | **one** alias, `peisear-storage/src/lib.rs:35` |
| runtime queries under `peisear-storage/src/` | **142** |
| compile-time `sqlx::query!` macros | **0** |

**The zero is why this is feasible at all** — the RFC's own point.
Every query is a runtime string, so none of them is bound to SQLite by
a macro's compile-time checking. **Re-measure all three**; mine are
from the day this was scoped and the RFC's said 137.

## §2 — what to build

- **A `postgres` feature** on `peisear-storage`, off by default, and
  whatever the workspace needs to pass it through.
- **A dialect module** that owns what differs: the `Pool` type, the
  placeholder style, and the error-code translation `DIA-001` just
  built. **What belongs in it is this round's real question** — a
  module that abstracts too little forces the backend round to reopen
  it; one that abstracts too much invents differences that do not
  exist.
- **`compile_error!` guards** so an impossible combination fails at
  build time with a sentence, not at runtime with a type error — both
  features on, or a feature on with no backend selected, whichever the
  design makes reachable.

**The flag absent is the shipping configuration.** Everything that
exists today keeps working, unchanged, with no feature enabled; that
is the gate.

## §3 — the trap this round is most likely to fall into

**Abstracting from one example.** With no PostgreSQL code to check
against, every "this will differ" is a guess, and a guess written into
a trait is harder to remove than one left out.

**Prefer leaving a difference unabstracted** and letting P3 widen the
seam when it has a second implementation to compare. **Say in the
report which differences you chose not to abstract and why** — that
list is the handoff to P3 and is worth more than the module.

## §4 — what must not happen

- **No `#[cfg(feature = "postgres")]` scattered through handlers.**
  The seam is in `peisear-storage`; if a `cfg` reaches `peisear-web`,
  the seam is in the wrong place — report it rather than spreading it.
- **No second `Cargo.lock` story.** The feature is off by default and
  the lockfile must not move for the default build. Check it.
- **No PostgreSQL in CI yet.** P4 is the conformance suite; a job that
  starts a database now is scope from a later phase.

## §5 — specification work (`DEC-028`)

- **`NFR-CMP-003`** says the database is a single file. It is **not**
  restated here — the RFC puts that at P4, *"restated to say for the
  SQLite backend"*. **Leave it and say you did**; a requirement
  restated two phases early is a claim about code that does not exist.
- **`NFR-MNT-001`** (one crate, one concern) is the entry the seam
  serves. Check whether its text already covers a backend boundary.

## §6 — gates

- `DEC-007` from a fresh block, **with the feature absent** — the
  shipping configuration. **Then run it again with the feature on**
  and report both; a seam that only compiles one way is half a seam.
- `fmt`, `clippy` both profiles **and with the feature enabled**,
  rustdoc after the last edit, `mdbook`.
- `publish-dry-run` — a new feature changes what `cargo publish`
  verifies. **Confirm it stayed green**; `FIV-007` is why that job
  exists and why this line is here.
- The three browser gates: unaffected in theory, which is the reason
  to run them.

## §7 — escalate rather than deciding

- **If the seam cannot be built without touching `peisear-web`.**
  That is a design finding about crate boundaries, not a licence.
- **If `compile_error!` cannot express a combination** you think
  should be impossible. Bring the combination.
- **If abstracting the placeholder style turns out to need the
  queries rewritten.** 142 call sites is a different round; stop.

## §8 — exit condition

A `postgres` feature that is off by default; every existing test green
with it absent **and** the test suite run with it present, both
figures reported; a dialect module whose contents are justified one by
one; **the list of differences deliberately left unabstracted**, which
is P3's real inheritance; `compile_error!` guards shown to fire by
building a forbidden combination; no `cfg` outside `peisear-storage`;
`Cargo.lock` unmoved for the default build; `publish-dry-run` green;
all gates re-run after the last edit.
