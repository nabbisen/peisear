# Scaling

Practical advice on when peisear's defaults stop being enough and
what to do about it. peisear is intentionally small; "scaling"
here mostly means "moving from SQLite to PostgreSQL" and
"thinking about what the background task does under load",
because there isn't much else to scale.

## Where the defaults sit

The shipping configuration is targeted at:

- A team of up to ~50 active users
- A few hundred active projects, each with up to a few hundred
  issues
- A request rate measured in single-digit RPS

Within that envelope, peisear runs on small VMs (1 vCPU, 1 GB
RAM, 10 GB disk) without worrying about resource shape. The
SQLite database file grows in the low-MB-per-year range (see
[data-retention.md](data-retention.md)) and the background task
costs are sub-second per tick.

If you're comfortably inside that envelope, **stop reading**. The
right scaling work is the work you don't do.

## When SQLite is enough

SQLite handles concurrent reads well and serialises writes
behind the WAL. peisear's write rate is low (one INSERT per
issue mutation; ~tens of mutations per active user per day) and
the read rate is bounded by HTTP requests, which the same
process handles directly anyway. For most installations,
SQLite never breaks a sweat.

The signs that SQLite is keeping up:

- `database is locked` errors are rare (a few per day at most)
- p95 request latency is below ~50 ms
- The database file size is in the MB-to-low-GB range

The signs that SQLite is *starting* to strain:

- Several `database is locked` per hour, especially during the
  background tick
- p95 request latency creeping into hundreds of ms
- File size approaching tens of GB

If you're at the strain edge, the easy levers come first.

### Easy lever 1: WAL mode

peisear opens SQLite in WAL mode by default, which is the right
mode for concurrent readers + a single writer. If for some reason
your installation isn't in WAL mode (older databases imported
from elsewhere), switching helps.

### Easy lever 2: Tune `busy_timeout`

The pool sets a sensible busy timeout (5s) so transient lock
contention waits instead of failing. Increasing this is rarely
useful — beyond a few seconds, a "wait" becomes
"indistinguishable from hung" — but checking that you have
*some* timeout is worth doing.

### Easy lever 3: Tune the snapshot tick

If the snapshot tick is causing lock contention with regular
requests, you can:

- Increase `SNAPSHOT_INTERVAL` (currently 6 hours) — see
  [background-jobs.md](background-jobs.md)
- Run the tick at off-peak times by aligning the initial pass
  (this requires code changes, not config — open an issue if you
  need it)

Both of these are blunt instruments. The actual fix for sustained
contention is the next section.

## When SQLite isn't enough

The headline signs:

- Multiple peisear instances would be useful (HA deployment,
  blue/green upgrades), but they can't share a SQLite file
  reliably
- The database file is over ~50 GB and queries are getting slow
- You need point-in-time recovery, replication, or a managed
  backup story that SQLite can't satisfy

**This is the PostgreSQL line, and as of 0.56.0 it is selectable at
build time on `peisear` and `peisear-web`, not only on
`peisear-storage`.** Both crates publish a `postgres` Cargo feature
(off by default, mutually exclusive with `sqlite`) with a real
connection, a ported migration set, the schema's eleven triggers and
every production query — the port the previous paragraph warned was
larger than the type boundary suggests: the eleven triggers and five
validation codes they carry are re-implemented, not merely compiled
against, and the date arithmetic (`julianday`, `datetime('now', …)`
modifiers) is rewritten per backend where PostgreSQL has no identical
spelling. See
[ROADMAP.md](https://github.com/nabbisen/peisear/blob/main/ROADMAP.md#postgresql-backend)
for what shipped in which release and why the reason was stability
and security, not scale.

**Running peisear on PostgreSQL is a supported configuration as of
`0.59.0`** (`RFC 021`), reached by `cargo install` with the feature
named — see [Install](../getting-started/install.md)'s own "Running
against PostgreSQL instead." The distinction that mattered before this
release was a property, not a count, and the property is now measured
rather than assumed: all 33 of `peisear-web`'s integration-test files
run against a real PostgreSQL server in CI, unfiltered (`SUP-002`),
including the two **P0** privacy controls (`NFR-PRIV-007`'s
contributor floor, `NFR-PRIV-001`'s capacity visibility) that
PostgreSQL's own queries enforce in different SQL than SQLite's — the
exact gap that, before `SUP-002`, three lock-test files and the
storage suite alone could not have proven either way.

**The guarantee every guarded write depends on is now proven on both
backends.** Earlier in 0.56.0, no guarded write executed on PostgreSQL
at all — fourteen production call sites opened SQLite's own `BEGIN
IMMEDIATE`, a syntax error there. `dialect`'s own transaction seam
replaced all fourteen with a per-site mechanism (a row lock, a
sibling-set lock, or an advisory lock, chosen by what each site
actually guards), and all four mechanisms are proven necessary by a
test seen to fail without them, on a real server. A reader building
`--features postgres` today gets a backend that can take a write, not
one that silently accepts and loses the guarantee that write depended
on.

**`JWT_SECRET` must be set explicitly on PostgreSQL, or peisear
refuses to start.** The generated-signing-key mechanism
([`hardening.md`](../security/hardening.md)) writes its file beside
the SQLite database file; a PostgreSQL `DATABASE_URL` has no
filesystem path to put one beside, so startup exits naming exactly
that — *"`JWT_SECRET` is not set, and `DATABASE_URL` (…) has no
filesystem path to persist a generated signing key beside. Set
`JWT_SECRET` explicitly."* This is not a bug to work around —
inventing a filesystem home for a signing key nobody asked for would
be worse — but it is a fact worth knowing before the server exits on
it: generate a real secret
(`openssl rand -base64 48` or any CSPRNG you trust) and set it
alongside `DATABASE_URL`.

**What this still does not give you**: a way to carry an *existing*
SQLite installation's data across — the feature is a second backend
to start fresh with, not a converter, and `peisear migrate` does not
exist; and **a second set of release artefacts**. The archives in
[Install](../getting-started/install.md)'s Option A and the container
image in Option B are unchanged — the same five per-platform
archives, the same single container tag, both SQLite only.
`cargo install` with the feature named (Option C's own PostgreSQL
note) is the one supported path, not a fourth artefact shape; a
second archive set or container tag is its own, separate proposal,
not implied by this one.

If you need those, that's the most useful operator feedback we can
get; an issue describing your scale and timeline is welcome.

## What the background task does under load

The snapshot loop's cost grows linearly with `(active projects +
active users)`. There are no quadratic terms; no per-issue work
in the inner loop. So the question of "how big can the team get
before the tick gets slow?" has a generous answer.

Concretely, the tick's per-row work is:

- **Project pass**: one `SELECT` with grouped `SUM(...)` over the
  project's issues + one `INSERT`. Few ms each.
- **User pass**: one `SELECT` with similar grouping over the
  user's issues + one `INSERT`. Few ms each.

A team with 1000 projects and 200 active users does ~1200 rows of
work per tick. At a few ms each, that's a few seconds of
sustained activity every 6 hours. Not nothing, but not load that
warrants engineering against.

If you're at scales where the tick measurably slows the
application during its execution window:

- This is the most likely first signal that you've outgrown the
  single-process / single-database model
- Splitting the data into multiple peisear instances by
  organisational unit is the simplest answer; future Team /
  organisation features will make this natural
- Eventually a per-project-shard SQLite or a single PostgreSQL
  becomes the architecture

## What about HTTP load?

axum + tokio handles thousands of in-flight requests on modest
hardware. peisear's per-request cost is dominated by the
database query (most pages render with one or two queries) and
the Leptos render. We don't ship a load-balancer; the deployment
expectation is that a reverse proxy in front (nginx, Caddy,
Traefik) handles TLS, optionally HTTP/2, and request distribution
if you do scale to multiple peisear processes.

For multiple peisear processes against the same data: that needs
the PostgreSQL story above, since SQLite locking will fight you.

## The "just one binary" promise

peisear's deployment story is one binary, one config file (or
env vars), one database file. The reason it's a *promise*, not
an *accident*, is that we want operators to be able to run it
without becoming experts in a stack. Most of the scaling
recommendations above start with "you've outgrown that promise",
and the moment you're shopping for managed PostgreSQL, the
promise has changed.

That's fine. The boundary is well-defined. Inside the envelope,
peisear is a single binary plus a SQLite file. Outside the
envelope, you're in standard relational-database operations
territory and there is plenty of literature for that.

## See also

- [background-jobs.md](background-jobs.md) — costs and
  observability of the periodic tasks
- [data-retention.md](data-retention.md) — growth rate of the
  append-only tables
- [observability.md](observability.md) — what to monitor as load
  increases
- [ROADMAP.md](https://github.com/nabbisen/peisear/blob/main/ROADMAP.md#postgresql-backend) — the
  PostgreSQL story's status
