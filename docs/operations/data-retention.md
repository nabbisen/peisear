# Data retention

peisear has three append-only tables that grow over time:

- **`issue_events`** — one row per issue mutation (create, status
  change, assignee change, effort change, delete). Foundation of
  the trend / dwell-time / staleness numbers.
- **`metrics_snapshots`** — one row per active project per
  snapshot tick (default every 6 hours). Foundation of the
  project-level trend chip.
- **`user_metrics_snapshots`** — one row per active user per
  snapshot tick. Foundation of the personal sustainability panel.

This document covers:

- the rate at which each grows
- the privacy posture around them
- the retention policy options operators can choose between, and
  the tradeoffs of each
- how to actually delete old rows when you've decided to

The short version: peisear ships with no automatic retention. You
keep your data forever unless you delete it. For most installations
that's fine — these tables are small. The longer version below is
for installations where it isn't.

## Growth rate

### `issue_events`

Roughly **150 bytes per event**. One issue going through its
lifecycle (create → in_progress → done) generates 3 events.
Real-world rates we've observed:

- Solo developer: ~5 events / day → 1 MB / 4 years
- Small team (~5 people): ~50 events / day → 1 MB / 4 months
- Active project, ~20 contributors: ~500 events / day → 1 MB / 12
  days

These numbers are approximate. The point is that even a busy team
takes years to fill a typical disk with events.

### `metrics_snapshots`

Roughly **120 bytes per snapshot**. With the default 6-hour
interval, that's 4 snapshots / day per active project. A team
running 50 active projects writes 200 rows / day = 24 KB / day,
which is 8.5 MB / year.

### `user_metrics_snapshots`

Same shape. 4 snapshots / day per active user. Teams of 20 active
users write 80 rows / day, so the `user_metrics_snapshots` table
grows at about a third the rate of `metrics_snapshots` for a
typical team.

### Total

Combined, expect a few MB / year for small teams, tens of MB /
year for medium teams, and hundreds of MB / year for hundred-plus
teams running many projects. SQLite is comfortable with multi-GB
files; you have years before retention becomes operationally
necessary.

## Privacy posture

This matters for some compliance discussions and is good to know
even if compliance isn't a question:

- **`issue_events`** records `actor_id` for every event. That field
  is `ON DELETE SET NULL` against `users`, so deleting a user
  blanks their identity from past events but leaves the events
  themselves. The events are the operational audit trail —
  "this issue was moved" is a fact about the issue, not the user.
- **`metrics_snapshots`** is project-level only. No row identifies
  a user. Per V2.1 brief §2.5 (集計と個別を混同しない), this table
  is the aggregated bucket and is intended to be visible to all
  project members.
- **`user_metrics_snapshots`** is per-user. The `user_id` column is
  `ON DELETE CASCADE` against `users`, so deleting a user wipes
  their personal history. This is by design: the user's right to
  delete their own data outranks the operational value of keeping
  it.

If your jurisdiction requires deleting personal data on request
(GDPR Art. 17, etc.), removing a user via the standard delete flow
already wipes their `user_metrics_snapshots` rows. The
`issue_events` actor identity is also cleared. The only data that
survives is the activity timeline of issues they touched, which is
arguably no longer personal once the actor field is null.

## Retention policy options

The choice is yours; peisear doesn't impose one.

### Option 1: Keep everything forever

The default. Easy. For most installations the disk cost is a
non-issue and the value of being able to look back is real
(e.g., "how was the team going six months ago?"). Pick this unless
you have a specific reason not to.

### Option 2: Time-based cutoff per table

Delete rows older than N days from each of the append-only tables.
A reasonable default if you choose this is **365 days** —
long enough that "year-over-year" comparisons remain possible,
short enough to keep the tables from drifting unbounded.

The cutoff can be different per table:

- `metrics_snapshots` — 365 days is generous; 90 days is enough
  for trend math (which only looks 14 days back).
- `user_metrics_snapshots` — same as above.
- `issue_events` — be more conservative here; the dwell-time and
  staleness queries assume the event log is reasonably complete
  for in-flight issues. 730 days (two years) is a safer floor.

### Option 3: Volume-based cutoff

Keep at most N rows per table. Less common but reasonable if your
disk budget is the constraint rather than the calendar.

### Option 4: Per-issue policy

Delete `issue_events` for issues that have been in `done` status
for more than N days. Keeps history for active work, drops it for
ancient closed issues. This is the most defensible compromise for
operations-sensitive teams; it preserves the operational meaning
of the audit log (which is mostly about understanding *current*
work) while bounding total volume.

## How to actually delete old rows

peisear does not ship a built-in cleanup job today. The intent is
to add one once we're confident in a default policy that works for
most installations; until then, this is a manual operation.

### Snapshot tables (safe to truncate freely)

These are derived data. Trend math gracefully handles missing
history (renders `Trend::Unavailable` for empty windows). You can
safely delete arbitrarily-old rows:

```sql
DELETE FROM metrics_snapshots
 WHERE captured_at < datetime('now', '-365 days');

DELETE FROM user_metrics_snapshots
 WHERE captured_at < datetime('now', '-365 days');
```

### `issue_events` (be more careful)

The estimation-skew (Pace) and stalled-streak queries fall back to
`updated_at` for issues with no events, but that fallback is the
0.7.0 calendar-time approximation, not the 0.8.0 dwell-time
precision. Deleting events for *closed* issues is fine since they
no longer participate in either calculation; deleting events for
*open* issues regresses precision for those issues. So:

```sql
-- Safe: events for issues that have been done for >180 days
DELETE FROM issue_events
 WHERE issue_id IN (
   SELECT id FROM issues
    WHERE status = 'done'
      AND updated_at < datetime('now', '-180 days')
 );

-- Also safe: events whose issue has been deleted (issue_id IS NULL)
-- and which are themselves older than the audit-log retention you
-- want to keep
DELETE FROM issue_events
 WHERE issue_id IS NULL
   AND occurred_at < datetime('now', '-730 days');
```

Run these in a transaction and inspect `rowcount` before
committing if you want a dry-run.

### Reclaim disk after large deletes

SQLite does not automatically shrink the database file. After
significant deletions, run `VACUUM`:

```sql
VACUUM;
```

This rewrites the file in-place and may take some time on a large
database. Plan a maintenance window. Consider `VACUUM INTO
'path/to/new.db'` for a safer copy-then-swap pattern.

## Future direction

A periodic cleanup job in `peisear-web::jobs` is the natural place
for automated retention. We expect it to land once we've collected
enough operator feedback to pick a sensible default policy. If you
have an opinion, an issue describing your retention needs is the
fastest way to influence the design.

See also [background-jobs.md](background-jobs.md) for how to
read existing background-task behaviour, and
[upgrade-runbook.md](upgrade-runbook.md) for what changes between
versions.
