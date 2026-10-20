//! The drift guard between `migrations/` and `migrations-postgres/`
//! (`PG-002`, `RFC 018` P3's own §2: "propose it; do not build a
//! comparison that reads SQL semantics"). **Filename-level only** —
//! a version number and a description, both encoded in the
//! filename itself (`NNNN_description.sql`), never the SQL inside
//! it. Two directories with the same twenty-two filenames says
//! nothing about whether their *contents* agree — that is a
//! different, harder project this round declines, per the bound
//! above — only that neither set has gained or lost a step the
//! other does not know about.
//!
//! **Unconditional, not feature-gated**: this reads the filesystem,
//! not a connection, so it runs in every `cargo test` regardless of
//! which backend feature is active — the one guard in this crate
//! that checks both directories every time, rather than only the
//! one the active feature would otherwise read.

use std::collections::BTreeSet;
use std::path::Path;

/// Filenames allowed to exist only in `migrations-postgres/`, with no
/// SQLite counterpart expected (`PG-004`). Every other migration in
/// this crate ports content SQLite already has; these add schema
/// SQLite does not need, because SQLite already provides the same
/// thing implicitly — a placeholder on the SQLite side would claim a
/// port that never happened. `0023` adds a surrogate `rowid` column
/// (`GENERATED ALWAYS AS IDENTITY`) standing in for SQLite's own
/// implicit `rowid`, which nine production queries' `ORDER BY`
/// tiebreaks rely on and PostgreSQL has no equivalent for.
const POSTGRES_ONLY: &[&str] = &["0023_rowid_surrogate_for_ordering.sql"];

/// Every `.sql` filename directly under `dir`, sorted. Not
/// recursive — this crate's migration directories are flat.
fn sql_filenames(dir: &Path) -> BTreeSet<String> {
    std::fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("read_dir({}): {e}", dir.display()))
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| name.ends_with(".sql"))
        .collect()
}

/// Every filename present in one migration directory but not the
/// other, as a human-readable line per mismatch. Empty means the
/// two sets agree. Takes both directories as arguments (rather than
/// hardcoding `CARGO_MANIFEST_DIR`-relative paths itself) so a test
/// can call it with a planted directory without touching the real
/// one.
pub(crate) fn mismatches(sqlite_dir: &Path, postgres_dir: &Path) -> Vec<String> {
    let sqlite = sql_filenames(sqlite_dir);
    let postgres = sql_filenames(postgres_dir);

    let mut out = Vec::new();
    for name in sqlite.difference(&postgres) {
        out.push(format!(
            "{name}: in migrations/, missing from migrations-postgres/"
        ));
    }
    for name in postgres.difference(&sqlite) {
        if POSTGRES_ONLY.contains(&name.as_str()) {
            continue;
        }
        out.push(format!(
            "{name}: in migrations-postgres/, missing from migrations/"
        ));
    }
    out
}

#[cfg(test)]
mod tests;
