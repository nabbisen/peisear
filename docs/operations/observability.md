# Observability

What to scrape, what to alert on, what's worth a dashboard tile.
peisear is small and the answer is "less than you might expect".

## Logging

peisear uses the [`tracing`](https://docs.rs/tracing) ecosystem,
formatted via `tracing-subscriber` with the `fmt` layer. By
default the format is human-readable text on stdout/stderr, with
ANSI colour when stderr is a TTY. For production, the
recommended setup is to let your supervisor (`systemd`,
container runtime, etc.) capture stdout / stderr as the log
stream.

### Levels

- **`info`** — startup, listening, shutdown, snapshot loop start /
  stop. Quiet.
- **`debug`** — per-tick snapshot pass summaries, request lines.
  Verbose.
- **`error`** — actual problems: per-row snapshot failures, request
  errors, database failures. Should be near-zero in normal
  operation.

The default filter (`info,sqlx=warn,hyper=warn,tower_http=info`)
hides `sqlx`'s and `hyper`'s noisier-than-helpful info-level
chatter while keeping our own at info. Override with
`RUST_LOG=...` if you want a different mix:

```bash
RUST_LOG=info,peisear_web=debug ./peisear   # see snapshot tick summaries
RUST_LOG=warn ./peisear                       # near-silent
RUST_LOG=debug ./peisear                      # everything (loud)
```

### Structured fields

Errors include structured key=value fields for the relevant
context. Example:

```
ERROR peisear_web::jobs: snapshot_loop: capture_one failed
  error=database is locked
  project_id=8a3f...
```

A log shipper that understands structured logs (Loki, Vector,
Filebeat, etc.) can index on those fields. The plain-text
formatter is the default; if you want JSON output for machine
ingestion, the `fmt` layer takes a `.json()` chainable —
that's a code change today, but a small one if your environment
needs it.

## Health checks

There is no `/healthz` or `/livez` endpoint today. The pragmatic
alternatives:

- **Liveness**: any HTTP 200 from any unauthenticated endpoint
  (e.g., `/login`). If the process is up, the auth-redirect
  middleware will respond.
- **Readiness**: the same. peisear's startup is "open the
  database, run migrations, listen". By the time it's listening,
  it's ready.
- **Database health**: peisear opens a connection pool to the
  database file and sends a `SELECT 1` per pool init. A failed
  pool open exits the process before listening.

This minimalism is fine for the deployment shape peisear is
built for (single binary behind a reverse proxy or directly
exposed). For container orchestration that requires a dedicated
endpoint, this is a small feature addition; open an issue.

## Alerting

The signals worth alerting on, in rough priority order:

### 1. Process not running

Standard service-level alert from your supervisor. systemd
restarts the process on crash; if it fails to start (most likely
because of a migration error after an upgrade), you want to know.

### 2. No successful snapshot tick in 24 hours

The snapshot loop is the easiest thing to silently miss. If the
tokio task panicked or got stuck, requests still serve, but trend
chips quietly turn into `Unavailable`. The signal is the absence
of `snapshot pass complete` log lines (debug-level) or the
absence of new `metrics_snapshots` rows.

A SQL-based check (run from monitoring):

```sql
SELECT (julianday('now') -
        julianday(MAX(captured_at))) * 24 AS hours_since_last_snapshot
  FROM metrics_snapshots;
```

Alert if `hours_since_last_snapshot > 24` and there are projects
with issues to snapshot.

### 3. Repeated `ERROR` in logs

Per-row snapshot failures are tolerated by the loop, but a sudden
spike in `ERROR` lines is operational signal. A simple
log-shipper rule like "more than 10 errors in 5 minutes" is a
fine starting point.

### 4. HTTP error rate

Standard reverse-proxy / load-balancer metric. peisear returns
4xx for unauth / not-found and 5xx for server errors. A 5xx
spike is the signal.

## Metrics

peisear does not export Prometheus / OpenTelemetry metrics today.
This is on the [scaling](scaling.md) discussion's roadmap; if
you need it now, a small middleware in `peisear-web::app`
exposing request counters / histograms is a contained addition.

The data points worth exposing eventually:

- HTTP request rate, latency, status-code distribution
- Snapshot tick duration and per-pass row counts
- Database query latency (sqlx instrumentation)
- WIP / capacity violations (already a per-user signal in
  `<HealthStrip>` but a Prometheus-reachable count is friendly
  to fleet-wide rollups)

## What to look at when debugging

A non-exhaustive checklist for "something feels wrong":

| Symptom | First thing to check |
|---|---|
| Trend chip stuck on `Unavailable` for a project that's been around for weeks | `SELECT COUNT(*) FROM metrics_snapshots WHERE project_id = ?` — if zero, the snapshot job hasn't run for this project. Check log for `snapshot_loop started`. |
| `/me` Sustainability panel never appears | The user must have at least one in-flight assigned issue *and* either an over-capacity streak or a stalled-issue streak. If neither, the panel is intentionally hidden. |
| Pace chip missing from `/me` | The user has no recently-done estimated issues, or the average days-per-point is below the display threshold (~0.05). See the comment in `peisear-web::components::me::format_skew`. |
| Long-stale count differs from intuition | Issues created with `status='in_progress'` directly have no `status_changed` event, so the long-stale clock falls back to `updated_at`. This is by design; see 0.8.0 CHANGELOG. |
| Snapshot rows growing faster than expected | Multiple peisear instances against the same database, perhaps. See [background-jobs.md](background-jobs.md). |
| A user's burnout panel mentions a streak after they've been on holiday | The over-capacity streak counts consecutive snapshots. If the user was over capacity right before the break and the next snapshot finds them still over (the holiday didn't *complete* anything), the streak persists. The intent is that the streak prompts reflection, not that it resets on calendar boundaries. |

## See also

- [background-jobs.md](background-jobs.md) — what the snapshot
  loop does, what it costs, when to expect log lines
- [data-retention.md](data-retention.md) — sizing and growth of
  the append-only tables
- [upgrade-runbook.md](upgrade-runbook.md) — what a startup log
  looks like during a migration
- [scaling.md](scaling.md) — when SQLite stops being enough
