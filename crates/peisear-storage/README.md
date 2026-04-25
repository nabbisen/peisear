# peisear-storage

[![crates.io](https://img.shields.io/crates/v/peisear?label=me)](https://crates.io/crates/peisear)
[![crates.io](https://img.shields.io/crates/v/peisear-storage?label=peisear)](https://crates.io/crates/peisear-storage)
[![Rust Documentation](https://docs.rs/peisear-storage/badge.svg?version=latest)](https://docs.rs/peisear-storage)
[![Dependency Status](https://deps.rs/crate/peisear-storage/latest/status.svg)](https://deps.rs/crate/peisear-storage)

Persistence layer for [peisear](https://crates.io/crates/peisear).
SQLite via `sqlx` today, designed to grow to PostgreSQL via either a
feature flag or a sibling `peisear-storage-postgres` crate without
changing public function signatures.

## API surface

The crate exposes:

- A `Pool` type alias (currently `sqlx::SqlitePool`) for backend-swap
  preparation.
- A `StorageError` enum carrying `Database`, `Migration`,
  `InvalidData`, `NotFound`, and `Bootstrap` variants — abstract over
  any specific backend's error vocabulary.
- Per-table query modules: `pool`, `users`, `projects`, `issues`. All
  queries use parameter binding (`?N`) — no string interpolation, no
  injection surface.
- Embedded migrations via `sqlx::migrate!()`, baked into the binary
  at compile time.

## Example

```rust
use peisear_storage::{pool, projects};

let db = pool::connect("sqlite://data/app.db").await?;
pool::migrate(&db).await?;

let projects = projects::list_for_user(&db, "user-id").await?;
```
