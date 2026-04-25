# Changelog

All notable changes to peisear are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.3] — 2026-04-25

### Added

- New facade crate `peisear` at `crates/peisear/`. It is the
  crates.io public entry point: `cargo install peisear` installs the
  runnable server, and the `peisear` library re-exports the four
  implementation crates as `peisear::core`, `peisear::auth`,
  `peisear::storage`, and `peisear::web`.
- Per-crate `README.md` files for each of the five crates, with
  crates.io / docs.rs / deps.rs badges and crate-specific
  descriptions. `readme = "README.md"` declared in each sub-crate
  manifest so crates.io picks up the correct README per crate.
- Workspace-wide crates.io publishing metadata in `[workspace.package]`:
  `description`, `repository`, `documentation`, `categories`,
  `keywords`. Sub-crates inherit these via `*.workspace = true`.

### Changed

- The `[[bin]] name = "peisear"` target moved from `peisear-web` to
  the new `peisear` facade crate. `peisear-web` is now library-only.
- The user-facing run command is now `cargo run --release -p peisear`
  (was `-p peisear-web`). Documentation updated throughout.
- Workspace inter-crate version pins bumped to `0.2.3`,
  matching `[workspace.package].version`.
- `thiserror` workspace pin updated from `1` to `2`.
- README heading capitalised to "Peisear".

## [0.2.2] — yanked

A draft release that was published with an incomplete facade
scaffold (empty `lib.rs`, missing workspace dependency entry).
Users should skip 0.2.2 and use 0.2.3 instead. The version number
on crates.io is unavailable for re-use per crates.io policy; the
scope originally planned for 0.2.2 has been shipped under 0.2.3.

## [0.2.1] — 2026-04-24

### Added

- `docs/` tree organised by reader intent: `getting-started/`,
  `architecture/`, `operations/`, `security/`, `guides/`. Each
  section has its own `README.md` landing page.
- Root-level governance files: `CHANGELOG.md`, `ROADMAP.md`,
  `NOTICE`, `TERMS_OF_USE.md`.
- Community health files in `.github/`: `SECURITY.md` and
  `CONTRIBUTING.md`.

### Changed

- `README.md` slimmed to hero section, overview, quickstart,
  features, and design notes. Detailed content migrated into `docs/`.
- Licence simplified from `MIT OR Apache-2.0` to `Apache-2.0`. The
  two licence files `LICENSE-MIT` and `LICENSE-APACHE` are replaced
  by a single `LICENSE` containing the Apache-2.0 terms.

### Removed

- `LICENSE-MIT` (see above).

## [0.2.0] — previous release

### Added

- Cargo workspace layout with four crates: `peisear-core`,
  `peisear-auth`, `peisear-storage`, `peisear-web`.
- Leptos 0.8 in SSR-only mode as the template engine.
- `infra/` directory reserved for CI/CD and IaC artifacts.

### Changed

- Project renamed to **peisear**; binary renamed to `peisear`.
- axum upgraded from 0.7 to 0.8 (path syntax `/:id` → `/{id}`,
  removal of `#[async_trait]` on `FromRequestParts`).
- Error handling split into three layered types: `AuthError` in
  `peisear-auth`, `StorageError` in `peisear-storage`, and
  `AppError` in `peisear-web` with `From` bridges.

### Removed

- askama / askama_axum dependency.

## [0.1.0] — initial release

### Added

- Initial implementation of a minimal issue-tracking web application
  with projects, issues, and a kanban board.
- User registration, login, and logout backed by argon2id and JWT.
- axum 0.7 + askama templating + sqlx on SQLite.

[Unreleased]: https://github.com/nabbisen/peisear/compare/0.2.3...HEAD
[0.2.3]: https://github.com/nabbisen/peisear/releases/tag/0.2.3
[0.2.2]: https://github.com/nabbisen/peisear/releases/tag/0.2.2
[0.2.1]: https://github.com/nabbisen/peisear/releases/tag/0.2.1
[0.2.0]: https://github.com/nabbisen/peisear/releases/tag/0.2.0
[0.1.0]: https://github.com/nabbisen/peisear/releases/tag/0.1.0
