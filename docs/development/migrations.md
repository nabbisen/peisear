# Migrations

Two directories, one schema history: `crates/peisear-storage/migrations/`
(SQLite) and `crates/peisear-storage/migrations-postgres/` (PostgreSQL,
`DIA-002`/`PG-002`). Both are embedded and run by `sqlx::migrate!` on
startup. [`migration_drift`](https://github.com/nabbisen/peisear/blob/main/crates/peisear-storage/src/migration_drift.rs)
checks the two directories carry the same filenames — a version number
and a description, both encoded in the filename — every `cargo test`,
regardless of which backend feature is active. It does not read SQL;
the two files behind a matching name can still disagree in content, and
nothing here checks that mechanically.

## `migrations/` is append-only, for one measured reason

**Once a migration has shipped, its file is never edited again.** `sqlx`
records each applied migration's checksum in `_sqlx_migrations` and
refuses to start a database whose checksum for an already-applied
migration no longer matches — demonstrated directly, not assumed,
when an in-place edit to `0015`/`0016` broke every real SQLite install
that had already applied them (`DIA-001`, 0.54.0). A new, additive
migration is always the right tool once a file has reached a real
install: `0022` dropped and re-created the same triggers `0015`/`0016`
defined, rather than editing either file.

**The rule's own reason is that a real database holding that checksum
exists.** It applies to `migrations/` unconditionally, because every
released version since `0.5.0` has shipped SQLite installs that may
have already applied any given migration.

## `migrations-postgres/` was append-only by the same convention, until `CON-006`

`migrations-postgres/` carried the identical rule by default — same
directory shape, same convention, no reason yet to ask whether it
actually applied. `CON-006` (0.56.0) asked, because it needed to amend
seven already-shipped files (`0001`, `0002`, `0005`, `0007`, `0008`,
`0009`, `0019`, `0020` — `INTEGER` columns, PostgreSQL's own fixed
4-byte type, widened to `BIGINT` to match the `i64` this crate's own
Rust layer binds every integer column as) rather than add a new one.

**The append-only rule's reason did not hold, measured rather than
assumed**: the `postgres` feature became buildable for the first time
the day before (`CON-001`, this same series), no released version has
ever documented a PostgreSQL install path, no archive and no container
image carries the feature, and `0.56.0` ships none. The only
PostgreSQL databases that have ever existed against this schema are
CI's own service containers, recreated empty on every run. **There was
no database anywhere holding a checksum the amendment could break**,
so `CON-006` amended the seven files in place rather than adding an
`ALTER COLUMN` migration behind them — the alternative would have left
every future reader of, say, `0007`, seeing `INTEGER` and needing to
know a later file silently changed it, for a rule whose own reason was
not engaged.

**This is a one-time exception, not a precedent for how
`migrations-postgres/` works from here.** `migrations/` was never
touched, and is not what this section is about — `INTEGER` is correct
there; it has shipped to real installs and nothing changes.

## The expiry

**The moment `ROADMAP.md`'s P5 documents a PostgreSQL install path —
an archive, a container image, or any released version a reader could
actually install and run against a real database — `migrations-
postgres/` becomes append-only on exactly the same terms `migrations/`
already carries, unconditionally, with no further exception.** At that
point a real database can hold a real checksum, and the measured
reason the rule exists starts applying to this directory too. A future
round that finds itself wanting to edit a shipped `migrations-postgres/`
file after that point has not found a second exception; it has found
the same defect `DIA-001` found in `migrations/`, and the fix is the
same one: a new, additive migration.
