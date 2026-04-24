# Backup

peisear's entire persistent state lives in one SQLite database file.
This makes backup cheap and restore obvious — but there are two
right ways and one wrong way.

## Online backup (while peisear is running)

SQLite in WAL mode — which peisear enables on startup — allows
**concurrent readers during writes**, so a read-only snapshot while
the server is live is safe:

```bash
sqlite3 /var/lib/peisear/data/app.db ".backup /var/lib/peisear/backups/app-$(date +%F).db"
```

This produces a single-file snapshot that is internally consistent.
Wrap it in a cron job or a systemd timer and copy the output offsite.

## Cold backup (while peisear is stopped)

If you can afford the downtime, the simplest strategy is also the
most correct:

```bash
systemctl stop peisear
cp /var/lib/peisear/data/app.db /var/lib/peisear/backups/app-$(date +%F).db
systemctl start peisear
```

The server stops holding the WAL and SHM files, the database
consolidates, and a plain `cp` gets a consistent copy.

## The wrong way

**Do not `cp` the live `app.db` file while the server is running.**
Without coordinating with the running sqlite writer, you'll capture
the main file without its associated WAL/SHM, producing an incomplete
snapshot. Use `.backup` (above) instead.

## Restore

Restoration is the obverse of the cold copy:

```bash
systemctl stop peisear
cp /var/lib/peisear/backups/app-2026-04-23.db /var/lib/peisear/data/app.db
systemctl start peisear
```

No migration step is required: the backup captures the schema along
with the data, and peisear's embedded migrations know how to
no-op-forward from whatever version that backup had.

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

## What's NOT in the backup

- **Environment variables** (including `JWT_SECRET`). Keep those in
  your infrastructure-as-code or secrets manager separately.
- **`static/` assets**. They come from the release tarball; there's
  no point backing them up.
- **Per-session data**. The JWT is signed with the current
  `JWT_SECRET`; if the secret rotates, all existing sessions
  invalidate. This is intentional.

## Next

- [Deployment](deployment.md) — where `/var/lib/peisear/data/` came from
- [../security/hardening.md](../security/hardening.md) — `JWT_SECRET`
  rotation as a compromise-recovery primitive
