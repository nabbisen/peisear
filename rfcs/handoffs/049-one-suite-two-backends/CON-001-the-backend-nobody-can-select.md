# CON-001 — the backend nobody can select

**0.56.0, `RFC 018` P4, round 1 of 4.** Everything else in this
series is blocked on this round, so it is first and it is small.

P4's gate is *"both backends green on one suite"*, and the suite it
names is *"the same storage **and lock** tests"*. The lock tests are
integration tests inside `peisear-web`. **`peisear-web` cannot select
the PostgreSQL backend.** Neither can `peisear`. That is this round.

---

## §1 — measured, 2026-10-20, at `c98739a`

`peisear-storage` publishes the feature pair. The two crates above it
do not, and their dependency edge carries `default-features` on:

| Crate | `[features]` section | `peisear-storage` edge |
|---|---|---|
| `peisear-storage` | `default = ["sqlite"]`, `sqlite`, `postgres` | — |
| `peisear-web` | **none** | `{ workspace = true }` — defaults on |
| `peisear` | **none** | `{ workspace = true }` — defaults on |

`dialect.rs:89` makes the two features a `compile_error!` when both
are on. So the defaults-on edge is not merely unhelpful, it is
fatal — enabling `postgres` from above adds it to `sqlite` rather
than replacing it. Both CLI forms were run and both fail:

```
$ cargo check -p peisear-web --features peisear-storage/postgres
error: peisear-storage: `sqlite` and `postgres` are alternatives, not additions.
$ cargo check -p peisear-web --no-default-features --features peisear-storage/postgres
error: peisear-storage: `sqlite` and `postgres` are alternatives, not additions.
```

The second is the interesting one: `--no-default-features` applies to
`peisear-web`, which has no features to turn off, so it does nothing
to the edge underneath.

**There is no command line that builds `peisear-web` or `peisear`
against PostgreSQL.** Not a flag we forgot to document — one that
cannot be written.

**`docs/operations/scaling.md` already says this, correctly**
(lines 110–117: *"any way to reach the feature from the published
`peisear` binary at all"*, and that selecting `postgres` means
building from a modified `Cargo.toml`). Nothing published is
misleading, and **this round is not a correction of a claim.** It is
the prerequisite that paragraph describes as missing.

## §2 — the change

A `[features]` section on `peisear-web` and on `peisear`, each
mirroring the pair below it, and `default-features = false` on the
`peisear-storage` edge so that selecting one really does deselect the
other. `peisear`'s `postgres` must also carry `peisear-web/postgres`,
or the facade and the library disagree about the backend.

**Mirror the pair; do not invent a third shape.** `peisear-storage`
already decided that the two are alternatives and that `sqlite` is
the default. This round propagates that decision; it does not revisit
it. If propagating it turns out to need a different shape — say,
because `default-features = false` on a workspace-table edge reaches
further than expected — **bring that, do not improvise it.**

## §3 — the passthrough is test infrastructure, not an install path

A passthrough that is documented as a way to run peisear becomes an
install path, and `NFR-REL-011` then requires CI to walk it. **That
walk would certify something false.** Measured at `c98739a`:

| | Against SQLite | Against PostgreSQL after this series |
|---|---|---|
| `peisear-storage` unit tests | 31 | 48 |
| `peisear-web` integration tests | **309** | **26** (`CON-003`'s lock tests) |

**283 integration tests would never have run against PostgreSQL**,
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
  `default-features = false` on these edges reach further than the
  backend choice.
- **If the default build's resolved features move at all.** Stop.
- **If the boot smoke fails.** That is a finding about the backend,
  and a valuable one — it would be the first time peisear itself ran
  on PostgreSQL. Report it; do not fix it inside this round.
- **If you find yourself writing a line telling someone how to run
  peisear on PostgreSQL.** That is §3's whole subject. Stop.

## §6 — exit condition

`peisear-web` and `peisear` each carry the feature pair;
`default-features = false` on both `peisear-storage` edges;
`cargo build -p peisear --no-default-features --features postgres`
green with its command quoted; the default build's resolved feature
set shown unchanged against `c98739a`; the boot smoke running in the
job that already has the service container; `DEC-007` unchanged;
**no install path documented and `scaling.md` untouched** — the
status paragraph is `CON-004`'s.
