# RFC 0017: The first five minutes — from nothing installed to a board on screen

**Status**: **Proposed** (2026-10-11) — the two channel decisions are already
taken (owner, 2026-10-11: **prebuilt binaries *and* a container image**); what
remains for review is §4's security change and §5's default bind
**Target**: **0.51.0** as its own theme. 0.50.0 is already carrying
`DOCS-006`, `RFC 0016`'s README repositioning, the sprint-plan fixture and
the alignment defect
**Raised by**: the owner, 2026-10-11 — *"the current installation on
documentation was too poor for unskilled users. We should make it much easier
to start up."*
**Related requirements**: `NFR-CMP-002` (self-hostable), `NFR-CMP-003`
(single-file database), `NFR-SEC-004` (no secrets in the repository or the
release artefact), `NFR-REL-002` (archive naming), `FR-AUTH-002` (sessions)
**Last updated**: 2026-10-11

## Summary

**There is nothing to install.** Every release page carries **zero assets**
(measured: 0.46.0, 0.47.0, 0.48.0 — `gh release view --json assets`), there is
**no `Dockerfile`** anywhere in the tree, and `.github/workflows/` holds two
files, `docs.yml` and `test.yml`. The only ways to run peisear are
`cargo install peisear` and `git clone` — **both of which begin by installing
a Rust compiler and waiting for a build.**

`docs/getting-started/installation.md` is **48 lines, and all of them are
about installing `rustc`.** It never installs peisear. It hands off to
Configuration, which is about `.env`, and then to First run, which builds.

**The owner is right, and the gap is not documentation.** A page cannot
document a download that does not exist.

This RFC proposes the five things that stand between *nothing installed* and
*a board on screen*, and removes each one.

## Background — the five barriers, counted

For someone who does not have Rust and does not know what an environment
variable is:

| # | Barrier | Where it comes from |
|---|---|---|
| 1 | Install a compiler | `installation.md`, all 48 lines |
| 2 | Wait for a release build | `cargo install peisear` |
| 3 | Understand `.env` and environment variables | `configuration.md` |
| 4 | Generate a secret with `openssl rand -base64 48` | `README.md`'s Quick Start |
| 5 | Know that `0.0.0.0:3000` means `http://localhost:3000` | the startup log line |

**Barriers 1 and 2 are removed by having something to download. Barriers 3
and 4 are removed by the program needing no configuration. Barrier 5 is one
line of output.**

## Proposal 1 — prebuilt binaries on every release (owner-approved)

Attach a per-platform archive to each GitHub Release, built in CI on the tag:
linux `x86_64` and `aarch64`, macOS `aarch64` and `x86_64`, Windows
`x86_64`. Naming and layout per `NFR-REL-002`: version-suffixed, files at the
archive root, no intermediate directory.

**This also closes a finding we already have.** `DOCS-003` found the Quick
Start's first command opening *a tarball no release provides*. **The
instruction was right and the artefact was missing** — that is this proposal,
stated from the other end.

**Maintenance, stated honestly** (the owner's standing concern is continuous
cost, not initial cost): a cross-compilation matrix is the part most likely to
break, and it breaks at the worst moment — on a tag. **Mitigation: the asset
workflow runs *after* the Release exists and never gates it.** A failed
binary build leaves a Release with no assets, exactly today's state, and is
fixed by re-running one job rather than by re-cutting a release.

## Proposal 2 — a container image (owner-approved)

`ghcr.io/nabbisen/peisear`, built from a `Dockerfile` in the repository,
published on the same tag. One volume for `/data`, one port.

```
docker run -p 3000:3000 -v ./data:/data ghcr.io/nabbisen/peisear
```

**This is the only path with no prerequisite at all** beyond a container
runtime, and it is one build rather than five.

## Proposal 3 — the installation page is for installing peisear

Today's page becomes **two** pages:

- **Install** — download for your platform, or `docker run`, or
  `cargo install peisear` for people who already have Rust. Each path ends
  the same way: *open `http://localhost:3000`*, with the first-run
  screenshots beside it.
- **Build from source** — the current 48 lines, moved, where contributors
  will look for them. Nothing is lost; it stops being the first thing a user
  meets.

## Proposal 4 — first run needs no configuration, and is safer than today

**This is the part that needs review, because it is a security change and it
is not optional.**

`crates/peisear-web/src/config.rs:21-27`: when `JWT_SECRET` is unset, peisear
falls back to the literal **`"dev-only-insecure-change-me"`** and logs a
warning. That constant is **published** — it is in a public repository and in
every crates.io copy of the source.

**So any instance started without `JWT_SECRET` signs its session cookies with
a key the whole world knows.** Session forgery against such an instance is
trivial. The only thing standing between a user and that state today is their
willingness to run the second line of a three-line quick start, and the only
notice is one `warn!` in a log a newcomer will not read.

**Making startup easier makes this worse, not better.** A one-command start
that skips configuration is a one-command start into that state. **The two
changes have to land together.**

**Proposed**: on first run, if `JWT_SECRET` is unset, **generate 32 random
bytes, persist them beside the database** (`data/jwt_secret`, mode `0600`),
and reuse the file on later runs. The environment variable still wins where
it is set. **The published constant is deleted.** If the file cannot be
written, **fail with a clear message** naming the path — never fall back to a
constant.

Consequences to document rather than discover:
- **The backup unit becomes the directory, not the file.** `NFR-CMP-003` says
  backup is a file copy; it becomes `cp -r data/`. The database and the key
  that validates its sessions belong together.
- **Sessions survive restarts**, which they do not reliably do today for
  anyone who sets the variable ad hoc.
- **Existing users are unaffected** where `JWT_SECRET` is set, and strictly
  better off where it is not.

## Proposal 5 — two lines of output a beginner can act on

`main.rs:55` logs `listening addr=0.0.0.0:3000` through `tracing`. It is
correct, and it asks the reader to translate. **Print, once, on a successful
bind, in plain words: the URL to open** — and, when the bind is not
loopback, **that the instance is reachable from the network**.

## §5 — one open decision: the default bind address

`BIND_ADDR` defaults to **`0.0.0.0:3000`** — every interface. Combined with
proposal 4's *current* state, a first run on a laptop publishes an instance
to the local network with a publicly-known signing key. Proposal 4 removes
half of that; the other half is a deliberate default.

**Recommendation: default to `127.0.0.1:3000`, and have the container image
set `BIND_ADDR=0.0.0.0:3000` itself**, where binding every interface is
correct and expected. Safe by default; the one context that needs the open
bind sets it explicitly and is the context where it is safe.

**It is a behaviour change** for anyone relying on the default to reach their
instance from another machine, so it is a changelog entry with a named
remedy, not a silent correction. **`docs/security/hardening.md` was corrected
at `61e7366` to say the default is `0.0.0.0`** — that correction stays true
until this lands, and must be revised with it.

## Alternatives considered

- **Documentation only.** Rejected by the owner's own decision, and it cannot
  work: the first step would remain "install a compiler".
- **Container only.** Rejected: it contradicts the project's own "one static
  binary, `systemctl restart peisear`" story and asks a newcomer to know
  Docker.
- **An installer script** (`curl … | sh`). Not proposed. It is a third thing
  to maintain, and asking a beginner to pipe a URL into a shell is a habit
  worth not teaching.
- **Keeping the published fallback secret with a louder warning.** Rejected:
  a warning is not a control. The failure it guards is silent and total.

## What follows this, recorded not scheduled

**Migration, import first** (owner, 2026-10-11). Once a newcomer can start in
one command, the next thing they have is work somewhere else. Two items, in
the owner's stated order:

1. **Import from other trackers** — GitHub Issues, Jira, Redmine, CSV.
2. **Instance migration and upgrade** — moving an instance between machines
   and versions, and eventually SQLite → PostgreSQL, which the requirements
   already carry as a Phase 3 deferred item.

**Both belong in `requirements.md` §11** (deferred and future requirements)
as rows, which is a `DEC-028` amendment and therefore **0.50.0's round, not
0.49.0's** — the candidate is in flight and does not get new scope.

## Exit condition, if accepted

A release page with assets for five targets; a container image on the same
tag; an Install page whose first instruction is a download; a first run that
needs no `.env` and no `openssl`; a key that is generated rather than
published; and a line of output naming the URL to open.
