# CON-001 — the backend nobody can select

**0.56.0, `RFC 018` P4, round 1 of 4.** Everything else in this
series is blocked on this round, so it is first and it is small.

P4's gate is *"both backends green on one suite"*, and the suite it
names is *"the same storage **and lock** tests"*. The lock tests are
integration tests inside `peisear-web`. **`peisear-web` cannot select
the PostgreSQL backend.** Neither can `peisear`. That is this round.

---

## §1 — measured, 2026-10-20, at `c98739a`; corrected 2026-10-20 after round 1

**Three crates, not two.** The first version of this section listed
`peisear` and `peisear-web` as the crates lacking a `[features]`
section and scoped the round to exactly those two. That census
enumerated the **direct** edges to `peisear-storage` and never asked
what else sat on the path. Round 1 traced it and found
`peisear-notify`, which both `peisear` and `peisear-web` depend on
unconditionally, and which depends on `peisear-storage`
unconditionally in turn. **It stopped rather than improvising, which
was right** — see `§5`'s stop condition, which it invoked verbatim.

The corrected population — and this one *is* the whole of it, every
workspace member checked:

| Crate | `[features]` at `c98739a` | `peisear-storage` edge |
|---|---|---|
| `peisear-storage` | `default = ["sqlite"]`, `sqlite`, `postgres` | — |
| `peisear-web` | **none** | `{ workspace = true }` — defaults on |
| `peisear` | **none** | `{ workspace = true }` — defaults on |
| `peisear-notify` | **none** | `{ workspace = true }` — defaults on |

No other workspace crate names `peisear-storage` at all.

`dialect.rs:89` makes the two features a `compile_error!` when both
are on, so Cargo's feature unification is what bites: *any*
unconditional edge anywhere on the path merges `sqlite` into an
explicitly requested `postgres` and produces the one combination the
crate refuses. Both CLI forms fail, as quoted:

```
$ cargo check -p peisear-web --features peisear-storage/postgres
error: peisear-storage: `sqlite` and `postgres` are alternatives, not additions.
$ cargo check -p peisear-web --no-default-features --features peisear-storage/postgres
error: peisear-storage: `sqlite` and `postgres` are alternatives, not additions.
```

The second is the instructive one: `--no-default-features` applies
to `peisear-web`, which has no features to turn off, so it does
nothing to any edge underneath.

**There is no command line that builds `peisear-web` or `peisear`
against PostgreSQL.** Not a flag we forgot to document — one that
cannot be written.

**`docs/operations/scaling.md` already says this, correctly**
(lines 110–117). Nothing published is misleading, and this round is
not a correction of a claim. It is the prerequisite that paragraph
describes as missing.

## §2 — the change

Each of the three crates gets a `[features]` section mirroring
`peisear-storage`'s pair — `default = ["sqlite"]`, `sqlite`,
`postgres` — forwarding to **every backend-carrying crate it depends
on directly**: `peisear-notify` to `peisear-storage`; `peisear-web`
to both; `peisear` to all three.

**The `default-features = false` goes on the three
`[workspace.dependencies]` entries, not on the member edges:**

```toml
[workspace.dependencies]
peisear-storage = { version = "0", path = "crates/peisear-storage", default-features = false }
peisear-notify  = { version = "0", path = "crates/peisear-notify",  default-features = false }
peisear-web     = { version = "0", path = "crates/peisear-web",     default-features = false }
```

Members stay on `{ workspace = true }` throughout. Cargo refuses
`default-features = false` as a member-level override of an
inherited entry (*"cannot override workspace's
`default-features`"*), so the alternative is a full `{ version,
path, default-features = false }` in three manifests — which works,
but duplicates the path three times and opts those edges out of
future workspace changes.

**The real reason is the failure mode, not the tidiness.** A crate
added later that depends on `peisear-storage` and forgets to forward
features would, under the member-edge shape, silently pull `sqlite`
and break the PostgreSQL build with *"alternatives, not additions"*
— an error naming neither the new crate nor the cause, which is
exactly what cost round 1 its `cargo tree -i` hunt. Under the
workspace shape it gets *"no storage backend selected. Enable
exactly one…"*: loud, local, true.

Each crate keeps its own `default = ["sqlite"]`, so an external
consumer of `peisear-web` or `peisear-notify` still gets SQLite by
default.

**Measured on this shape** in a detached worktree at `da30573`:
`cargo build -p peisear --no-default-features --features postgres`
finishes; **no `sqlite` on the `peisear-storage`, `peisear-web` or
`peisear-notify` nodes**; the default resolved-feature diff against
`da30573` is six lines, all three crates showing `default,sqlite`
where they previously showed nothing, with no third-party crate
moving. **The gates are still the round's to run.**

**Corrected 2026-10-20**: this paragraph first said *"the postgres
resolution contains no `sqlite`"*, and round 2 repeated it. That is
false of the resolution as a whole — `sqlx` carries `sqlite`,
`sqlx-sqlite` and `libsqlite3-sys` with `bundled` in a `postgres`
build, and the built binary contains 387 distinct `sqlite3_*`
symbols. The scoped claim above is what was measured.
`peisear-storage`'s `sqlite = []` is an empty feature: it gates the
`cfg` and not the driver, since `sqlx`'s own `sqlite` is
unconditional at `Cargo.toml:51`. Pre-existing and documented in
`peisear-storage/Cargo.toml`'s own comment; **made reachable** by
this round, because before it no PostgreSQL binary could be built at
all. Not this round's to fix — `ROADMAP.md`'s **P5** carries it and
`CON-004` opens the register entry.

## §3 — the passthrough is test infrastructure, not an install path

A passthrough that is documented as a way to run peisear becomes an
install path, and `NFR-REL-011` then requires CI to walk it. **That
walk would certify something false.** Measured at `c98739a`:

| | Against SQLite | Against PostgreSQL after this series |
|---|---|---|
| `peisear-storage` unit tests | 31 | 48 |
| `peisear-web` integration tests | **335** | **37** (`CON-003`’s lock tests) |

**298 integration tests would never have run against PostgreSQL**,
and the storage modules they are the *only* coverage for are the
ones where being wrong is worst:

- `personal_metrics.rs`, `user_burnout.rs`, `project_health.rs`,
  `metrics_snapshots.rs`, `user_metrics_snapshots.rs`, `issues.rs`,
  `sprints.rs` — **zero unit tests, every one of them.**
- `sprints.rs:1148` computes `NFR-PRIV-007`'s contributor floor with
  `COUNT(DISTINCT i.assignee_id)`; `project_health.rs:202` does the
  same for active assignees; `issues.rs:897`'s `project_workload`
  is the sole source for `NFR-PRIV-001`'s (**P0**) capacity
  visibility.
- The tests that prove those two privacy controls hold —
  `aggregate_privacy` (13) and `workload_privacy` (5) — are
  integration tests, and they stay SQLite-only after this series.

**This is not a claim that the translations are wrong.** `PG-004`
did this part carefully: the date primitives have equivalence tests,
and `capture_record`'s `INSERT OR REPLACE` → `ON CONFLICT DO UPDATE`
rewrite has `recapturing_a_sprint_record_still_bumps_captured_at`
proving the `captured_at` semantics it changes. The claim is the
project's own standing one: **a check that was never run is not
evidence.** Two P0 privacy controls are enforced by SQL that was
rewritten for a second backend, and nothing we would ship verifies
them on it.

So this round **adds the feature and documents no install path.**
`NFR-REL-011` is not engaged; there is no second archive set and no
second container tag.

**Silence is not the alternative, though.** `peisear` and
`peisear-web` are published crates, so a `[features]` section is
public API and docs.rs lists it whether we mention it or not. An
unexplained feature is worse for a reader than either answer. The
status paragraph is written in `CON-004` with the rest of the
documentation; this round does not touch `scaling.md`.

**Take the one real benefit the walk would have given.** Booting the
server against PostgreSQL proves more than any unit suite does, and
it does not require documenting anything. Add it to the job that
already has the service container: build with the passthrough, start
the binary against PostgreSQL with `JWT_SECRET` set, hit the health
endpoint, stop. **A CI job, not an install path.**

## §4 — gates

- `cargo build -p peisear --no-default-features --features postgres`
  succeeds. **State the command you ran, not that it worked.**
- The **default** build is unchanged: `cargo build --workspace` and
  `cargo tree -p peisear -f "{p} {f}"` both show the same feature set
  as at `c98739a`. A passthrough that quietly changes the default
  build is the one failure mode here that reaches every user.
- **The boot smoke green**: the binary built with the passthrough,
  started against the CI PostgreSQL service, health endpoint 200.
  Quote the job, not a summary of it.
- `DEC-007` — unchanged. This round adds no `cargo test` test; the
  boot smoke is a CI step, not a test in the per-crate block. If
  that turns out to be the wrong home for it, say so.
- `publish-dry-run`. A `[features]` section changes what `cargo
  package` resolves; `0.53.0`'s leg 3 is why that gate exists.
- `fmt`, `clippy`, `rustdoc`, `mdbook`.

## §5 — escalate rather than deciding

- **If the mirrored shape does not work** — anything that makes
  `default-features = false` on these entries reach further than the
  backend choice. Round 1 invoked this correctly and §1/§2 are
  rewritten because of it; invoke it again if a fourth crate or a
  fourth shape appears.
- **If the default build's resolved features move at all.** Stop.
- **If the boot smoke fails.** That is a finding about the backend,
  and a valuable one — it would be the first time peisear itself ran
  on PostgreSQL. Report it; do not fix it inside this round.
- **If you find yourself writing a line telling someone how to run
  peisear on PostgreSQL.** That is §3's whole subject. Stop.

## §6 — exit condition

`peisear`, `peisear-web` and `peisear-notify` each carry the feature
pair, each forwarding to every backend-carrying crate it depends on
directly; `default-features = false` on the three
`[workspace.dependencies]` entries, with every member still on
`{ workspace = true }`;
`cargo build -p peisear --no-default-features --features postgres`
green with its command quoted; the default build's resolved feature
set shown unchanged against `c98739a`; the boot smoke running in the
job that already has the service container; `DEC-007` unchanged;
**no install path documented and `scaling.md` untouched** — the
status paragraph is `CON-004`'s.
