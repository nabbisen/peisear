# Background jobs

peisear runs a small background tokio task alongside the web server,
co-located in the same process. Today this task is the **snapshot
writer**, which feeds the trend chip on `<HealthStrip>` and the
sustainability panel at `/me`. Future tasks (optional cleanup,
notification dispatch) will live as siblings in
`peisear-web::jobs`.

This document is for operators who want to understand:

- what runs in the background
- what it costs
- how to observe it
- what to expect after restarts and long downtime
- when (and how) to tune it

If you are looking for what a task *is*, the source-of-truth is
`crates/peisear-web/src/jobs.rs` — it is small (~150 lines) and
heavily commented. This document explains the operator-visible
behaviour.

## What runs

One tokio task: `snapshot_loop`. Each tick it does two passes:

1. **`capture_all`** — for every project that has at least one
   issue, compute the project-level health (the same numbers the
   `<HealthStrip>` shows on a project page) and insert one row into
   `metrics_snapshots`.
2. **`capture_all_users`** — for every user with at least one
   in-flight assigned issue, compute their personal-load metrics
   and insert one row into `user_metrics_snapshots`.

The default tick interval is **6 hours**. It's defined as
`SNAPSHOT_INTERVAL` in `crates/peisear-web/src/jobs.rs`. Six hours
is a deliberate compromise: tighter would multiply rows without
adding signal (the HealthScore doesn't change minute-to-minute);
looser would mean week-over-week trend lines have only a couple of
points to median over.

There's also an **initial pass on startup** before the first
sleep, so a fresh process or a restored backup has comparison data
within the first second instead of waiting six hours for the first
tick.

## What it costs

Each tick is bounded by the number of projects with any issues
plus the number of users with active assignments. For each:

| Pass | Per-row work |
|---|---|
| `capture_all` | one `project_health::for_project` query (small `SELECT` with grouped sums) + one `metrics_snapshots` `INSERT` |
| `capture_all_users` | one `personal_metrics::for_user_global` query (similar shape) + one `user_metrics_snapshots` `INSERT` |

In practice, each row's work is on the order of a few milliseconds
on SQLite. Even a 1000-project / 1000-user installation is a
few-second tick every 6 hours. Memory cost is negligible — no
per-row allocations beyond the row itself.

Database growth is modest:

- `issue_events` grows linearly with mutation traffic
  (~150 bytes / event)
- `metrics_snapshots` grows by 1 row × `num_active_projects` × 4
  ticks/day ≈ ~50 KB / 100 projects / day
- `user_metrics_snapshots` grows similarly per active user

A small team using peisear for a year accumulates a handful of
megabytes of history. A larger team should think about
[data retention](data-retention.md) policy after a couple of
years.

## How to observe

The task uses [`tracing`](https://docs.rs/tracing) for logs. Two
log lines are guaranteed at info level:

```
INFO peisear_web::jobs: snapshot_loop started
INFO peisear_web::jobs: snapshot_loop shutting down
```

The first is logged once at startup; the second is logged on
graceful shutdown via the oneshot signal. Together they bracket
the lifetime of the task.

At debug level (`RUST_LOG=peisear_web=debug` or
`RUST_LOG=info,peisear_web::jobs=debug`), each tick logs a one-line
summary:

```
DEBUG peisear_web::jobs: project snapshot pass complete succeeded=12 total=12
DEBUG peisear_web::jobs: user snapshot pass complete succeeded=4 total=4
```

A `succeeded != total` line is the operator's signal that something
is going wrong for some specific projects or users without taking
the whole job down.

Per-row failures log at error level with structured fields:

```
ERROR peisear_web::jobs: snapshot_loop: capture_one failed
  error=database is locked
  project_id=8a3f...
```

The errors are skipped (the loop continues with the next row), so
a transient SQLite lock from a request running at the wrong moment
won't cascade. If the same project / user keeps failing, that's
worth investigating.

### Health-check signal

There is no dedicated `/healthz` endpoint that reports on the job
specifically. The pragmatic alerting signal is **"no successful
tick within the past 24 hours"** — see
[observability.md](observability.md) for how to wire that up.

## After a restart or long downtime

The initial-tick-on-startup behaviour means a freshly-restarted
process gets one `metrics_snapshots` row per active project and
one `user_metrics_snapshots` row per active user almost
immediately. From the user's perspective, the first project page
loaded after a restart will see the recent past as expected.

A *very* long downtime (say, the process was off for a month) is a
slightly different story: the snapshot history will have a gap
of one month, and the trend baseline window (7-14 days back) may
end up empty. In that case `<HealthStrip>` shows
`Trend::Unavailable` until the next tick — same behaviour as a
fresh install. This is correct. Inferring trend across a downtime
gap would be making things up.

## Tuning

### Changing the tick interval

`SNAPSHOT_INTERVAL` is a `const` in `peisear-web::jobs`. Changing
it is a code change. We picked six hours deliberately and don't
expose it as configuration today; if you have a use case that
needs a different value (e.g., a single-developer instance where
six hours is too coarse), open an issue describing the operational
shape and we'll consider exposing it via env var.

### Disabling the task

For a short-lived test or migration scenario, you may want to run
peisear without the background job. There is no flag for this
today; the supported workaround is to comment out the
`jobs::spawn_all` line in `crates/peisear/src/main.rs` and
rebuild. For a permanent disabling story, see the same advice as
above — open an issue with the use case.

### Multiple peisear instances

Today peisear is a single-process deployment. If you run multiple
instances against the same database, each instance will run its
own snapshot loop independently. The result is duplicate snapshot
rows at slightly different timestamps. Median-based trend math
handles this fine (it just means `n` is bigger), but the database
grows faster than necessary. The intended deployment is one
process per database file; PostgreSQL multi-instance is a
[scaling](scaling.md) topic.

## Future jobs

The runner is shaped for siblings. Adding a task is a
`tokio::spawn` call in `jobs::spawn_all` plus a sibling `*_loop`
function. Already shaped to land here:

- **Periodic cleanup** — old `issue_events` / `metrics_snapshots`
  beyond a configured retention window. See
  [data-retention.md](data-retention.md). Ships when retention
  policy is configurable.
- **Notification dispatch** — pushing warnings out via email /
  webhook so users see them without having to look at the page.
  Ships with the planned notification surface work; see ROADMAP.
