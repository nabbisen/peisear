# Backup

**Everything from here down to [PostgreSQL](#postgresql) is about the
SQLite backend** — peisear's default, and the only backend any
released archive, container image or `cargo install peisear` builds
against. If you built with `--features postgres` (not a supported
configuration at 0.56.0; see [`scaling.md`](scaling.md)), skip to that
section.

peisear's persistent state lives in the database file and, if
`JWT_SECRET` is not set explicitly, a generated signing-key file beside
it (`jwt_secret`, `RFC 017` §4, `0.53.0`). **The backup unit is the
directory, not the database file alone** — the database and the key
that validates its sessions belong together, and a restore of one
without the other is a restore that logs everyone out. Backup is
still cheap and restore still obvious — there are two right ways and
one wrong way.

## Online backup (while peisear is running)

SQLite in WAL mode — which peisear enables on startup — allows
**concurrent readers during writes**, so a read-only snapshot while
the server is live is safe:

```bash
sqlite3 /var/lib/peisear/data/app.db ".backup /var/lib/peisear/backups/app-$(date +%F).db"
cp /var/lib/peisear/data/jwt_secret /var/lib/peisear/backups/jwt_secret-$(date +%F) 2>/dev/null || true
```

`.backup` produces a single-file database snapshot that is internally
consistent, the same guarantee as before. The key file is written once
at generation and never modified after, so a plain `cp` of it is
always safe regardless of server state — the `|| true` is only there
because the file doesn't exist at all when `JWT_SECRET` is set
explicitly, which is not a failure. Wrap both in a cron job or a
systemd timer and copy the output offsite.

## Cold backup (while peisear is stopped)

If you can afford the downtime, the simplest strategy is also the
most correct — and now a directory copy, not a single file:

```bash
systemctl stop peisear
cp -r /var/lib/peisear/data/ /var/lib/peisear/backups/data-$(date +%F)/
systemctl start peisear
```

The server stops holding the WAL and SHM files, the database
consolidates, and a plain recursive `cp` gets a consistent copy of the
database **and** the key file beside it.

## The wrong way

**Do not `cp` the live `app.db` file while the server is running.**
Without coordinating with the running sqlite writer, you'll capture
the main file without its associated WAL/SHM, producing an incomplete
snapshot. Use `.backup` (above) instead — the key file has no such
restriction and may be `cp`'d live.

## Restore

Restoration is the obverse of the cold copy — **restore the database
and the key file together**, from the same backup:

```bash
systemctl stop peisear
cp /var/lib/peisear/backups/data-2026-04-23/app.db /var/lib/peisear/data/app.db
cp /var/lib/peisear/backups/data-2026-04-23/jwt_secret /var/lib/peisear/data/jwt_secret 2>/dev/null || true
systemctl start peisear
```

No migration step is required: the backup captures the schema along
with the data, and peisear's embedded migrations know how to
no-op-forward from whatever version that backup had. **Restoring the
database without its key file is not an error** — peisear generates a
new one on the next start — but it invalidates every session that
existed at backup time, same as a deliberate `JWT_SECRET` rotation.

## Offsite rotation

A simple daily-weekly-monthly rotation strategy:

```bash
#!/bin/bash
# /etc/cron.daily/peisear-backup
BACKUP_DIR=/var/lib/peisear/backups
STAMP=$(date +%F)
sqlite3 /var/lib/peisear/data/app.db ".backup ${BACKUP_DIR}/app-${STAMP}.db"
find "$BACKUP_DIR" -name 'app-*.db' -mtime +30 -delete
rsync -a "${BACKUP_DIR}/" offsite:/backups/peisear/
```

Adjust the retention and offsite target to match your needs.

## Durability of a single write

Separate from backups, which protect against losing history: peisear
fsyncs the write-ahead log **on every commit** (`synchronous = FULL`,
measured and chosen in `STORE-002`/`RFC 018` P0b). A change that
reports success survives a power cut or an OS crash immediately
after — it does not depend on the next backup running.

**Measured cost**: about 2 ms on a ~440 ms fixture-building write path,
on NVMe. **On storage where `fsync` is slow** — a network-backed volume,
a spinning disk, anything without a power-loss-protected write
cache — the trade is different; the guarantee is the same, the latency
it costs is not.

**This is not configurable.** There is deliberately no environment
variable for it: a lever that makes a safety property optional is
worse than not having the property documented at all — a knob here
could silently trade away a durability guarantee. If you need
`synchronous = NORMAL`'s fewer-fsync profile for a workload this
product's size doesn't anticipate, that is a question for an issue,
not an environment variable.

## What's NOT in the backup

- **An explicitly-set `JWT_SECRET` environment variable.** If you set
  it yourself rather than letting peisear generate one, it lives in
  your infrastructure-as-code or secrets manager, not in `data/`, and
  is on you to back up separately. The *generated* key file, when
  there is one, is in `data/` and is covered above.
- **`static/` assets**. They come from the release tarball; there's
  no point backing them up.
- **Per-session data**. The JWT is signed with the current signing
  key, generated or explicit; if the key changes, all existing
  sessions invalidate. This is intentional.

## PostgreSQL

**Not a supported configuration at 0.56.0** — selectable at build
time (`--features postgres`), proven against a real server in CI, and
not something any release documents an install path for; see
[`scaling.md`](scaling.md) for what that does and does not mean. This
section exists anyway, for the same reason `scaling.md` names the
feature's status at all: the `[features]` section is public API the
moment a crate publishes it, and an unexplained feature is worse for a
reader than either answer.

**The backup unit is the operator's own PostgreSQL instance, backed up
by the operator's own means — peisear coordinates nothing for a server
it did not start and does not own.** This is the same posture
`NFR-CMP-002` already takes toward the operating system underneath
peisear: self-hostable, not self-managing everything beneath it. A
managed provider's own snapshot mechanism, a `pg_dump`/`pg_restore`
pair run by the operator, or WAL archiving and point-in-time recovery
configured on the server itself, are all the operator's choice to make
and the operator's own documentation to follow — not peisear's,
because peisear never configures the server, only connects to it.

**If you want a logical dump anyway**, `pg_dump`/`pg_restore` work
against whatever `DATABASE_URL` already names, with no peisear-specific
step:

```bash
pg_dump "$DATABASE_URL" --format=custom --file="peisear-$(date +%F).dump"
```

Restore into a fresh, empty database (never the live one a server is
still serving from) — same connection details as `DATABASE_URL`, a
different database name, so there is nothing new to configure:

```bash
psql "$DATABASE_URL" -c "CREATE DATABASE peisear_restored"
pg_restore --dbname "${DATABASE_URL%/*}/peisear_restored" "peisear-2026-04-23.dump"
```

(`bash`'s own `${VAR%/*}` strips everything from the last `/` onward —
swap in your shell's own equivalent if it isn't `bash`.)

Both commands were run against this project's own CI service container
(`postgres:17`) before this section was written, including a full
round trip — seed a row, dump, create a fresh database, restore into
it, confirm the row reads back — not merely typed and assumed to
work.

**No generated signing-key file exists on this backend to back up.**
`JWT_SECRET` must already be set explicitly for peisear to have started
at all on PostgreSQL (`scaling.md`'s own PostgreSQL note) — there is no
`jwt_secret` file beside a PostgreSQL connection string, so unlike the
SQLite sections above, a PostgreSQL backup is the database alone. Keep
the explicit `JWT_SECRET` value backed up the same way you back up any
other secret in your infrastructure-as-code or secrets manager — it
is not peisear's to back up, the same way it is not generated by
peisear here.

## Next

- [Deployment](deployment.md) — where `/var/lib/peisear/data/` came from
- [../security/hardening.md](../security/hardening.md) — `JWT_SECRET`
  rotation as a compromise-recovery primitive
