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

## §3 — the release question this opens, and who answers it

A passthrough that exists and is not documented is a feature only a
reader of `Cargo.toml` can find. A passthrough that **is** documented
becomes an install path, and `NFR-REL-011` requires every documented
install path to be exercised by CI. That requirement exists because
of `§10.41` — a documented install path the published artefact could
not satisfy — which this project has already paid for once.

So the honest options are two, and they are not symmetric:

1. **Document `cargo install peisear --no-default-features --features
   postgres` and walk it in CI** — install, start it against the CI
   PostgreSQL service, hit the health endpoint. One new path, one new
   walk, both cheap because the service container already exists for
   the storage job.
2. **Ship the passthrough undocumented**, so no install path is added
   and `NFR-REL-011` is untouched.

**Not an option: document it and do not walk it.** That is `§10.41`
exactly.

**The architect recommends (1)** and has put it to the owner with
this series. **Do not act on it until the series README's State
column says it is authorized.** If it is not, do (2) and say so in
`scaling.md` in place of the *"once that passthrough exists"*
paragraph.

**Out of scope either way**: a second set of per-platform release
archives, and a second container tag. Five targets do not become ten
in the release that first makes the backend selectable.

## §4 — gates

- `cargo build -p peisear --no-default-features --features postgres`
  succeeds. **State the command you ran, not that it worked.**
- The **default** build is unchanged: `cargo build --workspace` and
  `cargo tree -p peisear -f "{p} {f}"` both show the same feature set
  as at `c98739a`. A passthrough that quietly changes the default
  build is the one failure mode here that reaches every user.
- `DEC-007` — unchanged. This round adds no test and removes none.
- `publish-dry-run`. A `[features]` section changes what `cargo
  package` resolves; `0.53.0`'s leg 3 is why that gate exists.
- `fmt`, `clippy`, `rustdoc`, `mdbook`.

## §5 — escalate rather than deciding

- **If the mirrored shape does not work** — anything that makes
  `default-features = false` on these edges reach further than the
  backend choice.
- **If the default build's resolved features move at all.** Stop.
- **The §3 question is the owner's, relayed through the architect.**
  Do not pick an option.

## §6 — exit condition

`peisear-web` and `peisear` each carry the feature pair;
`default-features = false` on both `peisear-storage` edges;
`cargo build -p peisear --no-default-features --features postgres`
green with its command quoted; the default build's resolved feature
set shown unchanged against `c98739a`; `DEC-007` unchanged; the §3
question answered by the owner before any documentation changes, and
`scaling.md` left alone until it is.
