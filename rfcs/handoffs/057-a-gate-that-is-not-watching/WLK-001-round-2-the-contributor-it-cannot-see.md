# WLK-001 round 2 — the contributor it cannot see

**Round 1 made the gate run. It does not run for the contributors the
gate exists for.**

`env-example-walk.yml` now fires on a bare `push:` with a path filter and
**no `pull_request:` trigger**. `.github/CONTRIBUTING.md:235-237` — a file
in that very path filter — tells a contributor:

> 1. **Fork the repository** and create a topic branch from `main`.
> 3. **Push to your fork and open a pull request** against `main`.

**A `push` event for a fork's branch occurs in the fork, not here.** So in
this repository the walk never fires for the contribution flow this
project documents. The one guard whose subject is *the contributor path*
is blind to the outside contributor.

**This is `TTY-002`'s and `CFG-002` round 2's shape**: the subject
achieved, a property of the project's own standard not met. Round 1 is
accepted and not reverted; this round completes it.

## §0 — the second half, which is this repository's own convention

`test.yml` — 48 jobs, the most load-bearing workflow here — uses:

```yaml
on:
  push:
    branches: [main]
  pull_request:
  workflow_dispatch:
```

**`push` restricted to `main`, branch work covered by `pull_request`.**
Round 1 departed from that and said it was departing, which was honest,
but named only the upside (a contributor's own branch gets the signal) and
not the cost: **no fork coverage at all**, and in-repo work firing the walk
**twice** for one change — once on the branch push, once on the squash to
`main`. At roughly **7 minutes** a run (measured: 7m00s and 7m22s), on
files as frequently touched as `main.rs` and `config.rs`, that second
firing is pure duplication.

## §1 — what this round is

Bring the trigger to the arrangement `test.yml` already uses, keeping the
same `paths:` list on both events and keeping `workflow_dispatch` exactly
as it is:

```yaml
on:
  push:
    branches: [main]
    paths: [ … the six, unchanged … ]
  pull_request:
    paths: [ … the same six … ]
  workflow_dispatch:
    inputs: { ref: … unchanged … }
```

**Do not change the path list.** Round 1 verified it, and its one addition
beyond the handoff — `crates/peisear/src/main.rs`, because
`build_env_filter` lives there — was correct and was proven by the plant
that broke exactly that function.

## §2 — what you can prove here, and what you cannot

**Three things are measurable and all three are required:**

1. **A `pull_request` touching a filtered path fires the walk.** Open a
   PR from a branch in this repository; confirm the run's own event reads
   `pull_request`, not `push`.
2. **A push to a non-`main` branch no longer fires it.** Push a filtered
   path to a branch with no PR; confirm no run appears. **This is the
   half that will be tempting to skip** — it is the change's whole cost
   saving, and a `branches:` filter that does not actually narrow is
   exactly `§10.36`'s class.
3. **A push to `main` still fires it**, which the squash of this round
   will demonstrate by itself if a filtered path is in the diff. The
   workflow file is in its own filter, so it will be.

**One thing is not measurable here and must not be reported as if it
were**: a **fork** PR. No fork of this repository is available to push
from, and simulating one is not worth the round. The fork case rests on
documented GitHub behaviour — `push` fires in the repository that was
pushed to; `pull_request` fires in the base repository for a fork PR.
**State it as reasoning from documented behaviour, not as a measurement**,
and say which of the two it is. That distinction is this series' whole
subject.

## §3 — the trap

**`github.sha` means something different on `pull_request`.** Round 1's
`ref: ${{ inputs.ref || github.sha }}` resolves, on a `pull_request`
event, to the **merge commit**, not the PR head. That is very likely what
you want — it is what will land — but **verify it by reading the run's own
checkout step**, not by reasoning about it. If the checkout fails or walks
something unexpected, say what and stop; a `ref` expression that silently
walks the wrong commit is a gate reporting on code nobody is merging.

**Second trap**: do not lose the dispatch path. It is the only path this
job had for its whole life before yesterday, and round 1 proved it still
worked **by dispatching it**. Do the same.

## §4 — gates

`cargo fmt --check`; `clippy --workspace --all-targets -- -D warnings`;
`mdbook build docs` only if a document changes; `DEC-007` from the runner
— **zero delta expected**, stated. CI's own conclusion quoted, plus every
`Env Example Walk` run id this round produces **with its event**.

## §5 — escalate rather than deciding

- If `pull_request` + `paths` turns out not to filter the way `push` +
  `paths` does, **report the observed behaviour** rather than widening the
  filter until something fires.
- If keeping `push: branches: [main]` loses a signal you think matters,
  name it — the trade was made on this repository's own convention and
  `test.yml`'s precedent, and evidence beats precedent.

## §6 — exit conditions

1. `pull_request` with the same six paths; `push` narrowed to `main`;
   `workflow_dispatch` unchanged.
2. A `pull_request`-triggered run, its event confirmed from the run page.
3. A filtered push to a non-`main` branch producing **no** run, confirmed
   by absence rather than assumed.
4. The dispatch path shown still working **by dispatching it**.
5. The fork case stated as documented behaviour, labelled as such.
6. `§4`'s gates after the last edit, every run id with its event, CI
   quoted.
