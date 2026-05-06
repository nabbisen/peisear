# Operations

Running peisear in places other than your laptop.

## Day-one tasks

- [Deployment](deployment.md) — systemd, working directory, asset
  resolution, the whole production dance
- [Backup](backup.md) — how to make (and restore) a copy of your
  SQLite database
- [Tailwind self-hosting](tailwind-local.md) — pulling the CSS off
  the CDN

## Day-two tasks

- [Background jobs](background-jobs.md) — what runs in the
  background, what it costs, how to observe it
- [Data retention](data-retention.md) — sizing of the append-only
  tables, privacy posture, retention policy options
- [Upgrade runbook](upgrade-runbook.md) — how migrations work,
  pre-upgrade checklist, dry-running, rollback boundary
- [Observability](observability.md) — what to scrape, what to
  alert on, how to read the logs
- [Scaling](scaling.md) — when SQLite is enough and when it isn't

For configuration of a running instance (env vars etc.), see
[../getting-started/configuration.md](../getting-started/configuration.md).
