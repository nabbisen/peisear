# Changelog

All notable changes to peisear are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://example.invalid/peisear/compare/v0.2.1...HEAD
[0.2.1]: https://example.invalid/peisear/releases/tag/v0.2.1
[0.2.0]: https://example.invalid/peisear/releases/tag/v0.2.0
[0.1.0]: https://example.invalid/peisear/releases/tag/v0.1.0
