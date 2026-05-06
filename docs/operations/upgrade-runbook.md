# Upgrade runbook

How to upgrade a running peisear installation from one version to
the next, with a clear-eyed account of what can and cannot be
rolled back.

## How peisear upgrades work

peisear is a single binary plus a directory of static assets. An
upgrade is, in the happy case, three steps:

1. Stop the running process.
2. Replace the binary (and `static/` if the new release ships
   updated CSS / JS).
3. Start the new process.

On startup, the new binary runs its embedded migrations against
the existing database. `sqlx::migrate!()` applies any migration
files that are newer than the highest one recorded in the
`_sqlx_migrations` table. This is automatic, idempotent, and
fast; for the migrations we ship, it's typically sub-second on a
small database.

There is no upgrade tool. There is no separate migration command.
The binary that serves requests is the same binary that owns the
schema.

## What you need to think about

### 1. Migrations are forward-only

We do not write `down.sql` reverse migrations. This is a
deliberate stance, not an oversight: most peisear migrations are
either trivial column additions (which are reversible by hand if
needed) or they add new tables. The few that do refactor existing
data are clearly noted in CHANGELOG.

The operational consequence is: **once you start a new version
against a database, you can't downgrade by simply running the old
binary again**. If you need to go back, you need to restore from
a backup taken before the upgrade.

This is why backup-before-upgrade is non-negotiable. See
[backup.md](backup.md).

### 2. Binary compatibility is not the same as schema compatibility

A new version reading an old database is fine (forward migrations
make it the new schema). An old version reading a new database
will at best refuse to start (it won't recognise tables added
since); at worst it will write rows that don't satisfy schema
constraints we added in the upgrade. Don't run mixed versions
against the same file.

### 3. The background snapshot job catches up on first tick

After an upgrade, the snapshot loop's first tick happens within
seconds of process start (the `capture_all` and `capture_all_users`
calls before the sleep). One pre-existing snapshot row per active
project / user lands on disk almost immediately. This is
intentional — see [background-jobs.md](background-jobs.md) — and
means trend chips stay populated across upgrades.

If your upgrade adds a new snapshot table (as 0.9.0 added
`metrics_snapshots` and 0.10.0 added `user_metrics_snapshots`),
the first tick is also when those tables receive their first
rows. Trend chips for the new dimension show `Unavailable` until
enough ticks accumulate (7-14 days for the project trend; ~2 days
for the burnout overload streak).

## Pre-upgrade checklist

Run these before the upgrade window starts:

1. **Read the CHANGELOG entry** for the version you're upgrading to
   (and any versions you skipped). Migrations and design changes
   are noted in the *Changed* section; deferred items in the
   *Deferred* section.
2. **Take a backup.** `sqlite3 data/app.db ".backup data/pre-upgrade.db"`
   is the canonical command. See [backup.md](backup.md) for the
   complete dance, including how to restore.
3. **Inspect the migrations that will run.** They're in
   `crates/peisear-storage/migrations/` of the source tree, in
   order. Each starts with a long comment explaining the schema
   reasoning.
4. **Quiet the database.** Stop other writers (any external
   tooling that hits the SQLite file). The upgrade binary will
   migrate atomically, but a concurrent writer increases the
   surface area for unrelated breakage.

## Dry-run a migration

The most reliable way to know how an upgrade will go is to do it
on a copy first:

```bash
# Take a full copy of the production database
sqlite3 data/app.db ".backup /tmp/peisear-dryrun.db"

# Run the new binary against the copy. Use a different bind addr
# so it doesn't collide with the live process.
DATABASE_URL=sqlite:///tmp/peisear-dryrun.db \
  BIND_ADDR=127.0.0.1:9999 \
  JWT_SECRET=throwaway-for-dryrun \
  ./peisear-new-version
```

Watch the startup logs. Migrations run during pool init; you
should see no errors before the listening line:

```
INFO peisear: starting peisear database=sqlite:///tmp/peisear-dryrun.db addr=127.0.0.1:9999
INFO peisear_web::jobs: snapshot_loop started
INFO peisear: listening addr=127.0.0.1:9999
```

If you see migration errors (most likely a `CHECK` constraint
failure on existing data), that's your signal to investigate
before touching production. Stop the process, inspect the failing
constraint, and either patch the data or coordinate with the
peisear maintainers.

A good dry-run also includes a **read smoke test**: hit a few
pages (`/projects`, `/projects/{id}`, `/me`) and confirm they
render. If they do, the upgrade is almost certainly safe.

## Production upgrade

The minimal, defensible sequence:

```bash
# 1. Backup
sqlite3 data/app.db ".backup data/app.pre-X.Y.Z.db"

# 2. Stop the service
systemctl stop peisear   # or however you run it

# 3. Replace the binary and assets
cp /path/to/new/peisear /usr/local/bin/peisear
cp -r /path/to/new/static /var/lib/peisear/static

# 4. Start the service
systemctl start peisear

# 5. Watch the logs for migration completion + the listening line
journalctl -u peisear -f
```

The downtime window is the time from step 2 to the listening line
in step 5. For a small database, this is a few seconds.

## Rollback

There is no `peisear --rollback`. Rollback is restore-from-backup:

```bash
# 1. Stop the new (broken) process
systemctl stop peisear

# 2. Restore the pre-upgrade database
cp data/app.pre-X.Y.Z.db data/app.db

# 3. Replace the binary with the old version
cp /path/to/old/peisear /usr/local/bin/peisear

# 4. Start the service
systemctl start peisear
```

You lose any writes that happened on the new version between the
upgrade and the rollback. There's no way around this with the
forward-only migration story, which is why dry-running matters.

## Skipping versions

You can skip versions on the upgrade path. The migrations are
ordered, idempotent, and self-contained, so going from 0.5.0
straight to 0.10.0 applies the intermediate migrations in the
right order on first start.

The one thing to watch: **CHANGELOG entries for each version you
skip should be read**. Behaviour changes (e.g., 0.7.0's WIP-limit
default of 3, 0.8.0's event-based long-stale, 0.9.0's trend chip
appearance) all land at once, and someone on the team should be
prepared to explain to the rest of the team that the new
indicators are not a critique. See V2.1 brief §0.2 for the
non-evaluation framing if this comes up.

## Per-version notes

For migrations that warrant explicit operator attention, this
section will accumulate notes. Empty so far means the
migrations to date have been "safe additions" that don't require
hand-holding.

### 0.10.0 → 0.10.1 (this release)

Documentation-only release: five new operations docs added to
`docs/operations/`. No schema or behaviour changes; the migration
sequence is unchanged from 0.10.0. Upgrading is a binary swap
with no migration risk.

### 0.9.0 → 0.10.0

Adds `user_metrics_snapshots`. The background tick now does a
second pass (`capture_all_users`); see
[background-jobs.md](background-jobs.md). The
`<Sustainability>` panel on `/me` is hidden until both signals
have data, so users will not see anything new for the first
~24-48 hours. Plan to mention this once in team comms; otherwise
nothing operational to do.

### 0.8.0 → 0.9.0

Adds `metrics_snapshots`. Trend chips on project pages will show
`Trend::Unavailable` until 7-14 days of snapshot data has
accumulated. This is correct behaviour; no action needed.

### 0.7.0 → 0.8.0

Adds `issue_events`. Long-stale and personal-pace metrics for
*new* mutations from this point onward become event-based; legacy
issues continue to use the 0.7.0 `updated_at` approximation. See
the 0.8.0 CHANGELOG *Design* section for the full rationale on
why we don't backfill.
