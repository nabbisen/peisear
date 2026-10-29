# SUP-001 — the feature that gates the `cfg` and not the driver

**0.59.0, first of four work rounds.** `RFC 021` §4, `§10.48`.
Authorized 2026-10-29. **Read `§10.48` first** — it carries the
measurement and this handoff does not repeat it.

## §1 — what is wrong

`peisear-storage`'s `sqlite = []` is an empty Cargo feature. It drives
`#[cfg(feature = "sqlite")]` correctly throughout the crate and gates
no *dependency*, because `sqlx`'s own `sqlite` feature is
unconditional in `[workspace.dependencies]` rather than forwarded from
either backend feature. Measured at `f0cd461` when `§10.48` was
opened: a `--no-default-features --features postgres` build of
`peisear` still links the bundled SQLite C library — **387 distinct
`sqlite3_*` symbols in the binary** — and needs a C toolchain to
produce a build that will never open a SQLite file.

**Re-measure it at the current tip before changing anything.** That
figure is from 0.56.0 and a count that decides something is measured
twice; if it has moved, say so and use the new one.

## §2 — why this round is first

It is the only round that can change what a binary links, so it is
the one most likely to break a build, and `SUP-002` runs the whole
suite against PostgreSQL afterwards — which is the real test of this
change and cannot be run before it exists.

It is also the condition that makes *which backend am I running* a
question with one answer. `RFC 021` §3 asks for CI to exercise the
configuration on the same population as the default; a binary that
carries both drivers makes "exercised under `--features postgres`"
mean less than it looks.

## §3 — what to build

Forward the driver from the feature that names it:
`sqlite = ["sqlx/sqlite"]`, `postgres = ["sqlx/postgres"]`, with
`sqlx`'s own unconditional `sqlite` removed from
`[workspace.dependencies]`.

**Expect this to surface call sites that compiled only because the
driver was always present.** That is the finding, not an obstacle —
each one is a place where the `cfg` and the dependency disagreed.
Report them rather than papering over them with a wider feature.

**Both single-backend builds must work, and so must the default.**
`--no-default-features --features sqlite`,
`--no-default-features --features postgres`, and the plain default
build. State what the default is after this round and whether it
changed; if it did, that is an escalation, not a decision for this
round — `cargo install peisear` must keep working exactly as
documented.

## §4 — the proof

**Symbol counts, both directions, at the same commit.** A
`--features postgres` build must carry **zero** `sqlite3_*` symbols,
and a `--features sqlite` build must still carry them. Quote both
numbers; a single number proves only that something changed.

**And a C toolchain is no longer needed for the PostgreSQL build** —
if that is now true, show how it was established; if it is not, say
why, because it is half the reason `§10.48` names this a defect.

## §5 — gates

`DEC-007`'s per-crate block in `.github/CONTRIBUTING.md`, lines
96–115, re-extracted fresh. **Count with the runner.** Then, after
the round's last edit: `fmt`; `clippy --workspace --all-targets`;
`clippy` under each single-backend feature set for all four crates
that carry them; `cargo doc --workspace --no-deps` with
`CONTRIBUTING.md`'s own documented flag; `mdbook build docs`;
`publish --workspace --dry-run`. Quote result lines verbatim.

**`Cargo.lock` will change.** Say what left the tree and what did
not, since removing a driver from a default build is the kind of
change a lock file records and a summary hides.

## §6 — escalate rather than deciding

- If the default feature set would have to change to make this work.
- If forwarding the driver breaks a crate that is not
  `peisear-storage` in a way that needs a design call about where a
  `cfg` belongs.
- If `sqlx` turns out not to allow one backend without the other at
  the versions pinned here.

## §7 — exit condition

1. `sqlite` and `postgres` each gate their own `sqlx` driver;
   `[workspace.dependencies]` no longer forces `sqlite`.
2. Three builds proven: default, `--features sqlite`,
   `--features postgres`.
3. Symbol counts quoted both ways, zero `sqlite3_*` under
   `--features postgres`.
4. `§10.48`'s own 387-symbol figure re-measured at the tip, and
   whether it moved stated either way.
5. Every call site the change surfaced reported, with what was done.
6. The gates in §5, re-run after the last edit, quoted.
