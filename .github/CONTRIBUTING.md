# Contributing to peisear

Thanks for considering a contribution. This document describes the
local development loop, the expectations for a patch, and the pull
request process. Please be respectful and constructive in all
project communication.

## Ways to contribute

- **Bug reports** — open an issue with reproduction steps, version,
  and observed vs. expected behaviour.
- **Feature proposals** — open an issue first; large changes benefit
  from design discussion before code.
- **Documentation** — fixes to [`docs/`](../docs/README.md) and the
  root README are welcomed on the same footing as code.
- **Security issues** — do **not** open a public issue. See
  [SECURITY.md](SECURITY.md).

## Development setup

1. Install Rust 1.85+ — see
   [docs/getting-started/installation.md](../docs/getting-started/installation.md).
2. Clone the workspace and run the test suite:
   ```bash
   cargo check --workspace
   cargo build -p peisear-web --bin peisear
   ```
3. Run the application:
   ```bash
   cp .env.example .env
   cargo run --release -p peisear-web
   ```

For an in-depth walkthrough of the architecture before making
changes, start with
[docs/architecture/](../docs/architecture/README.md).

## Before you open a pull request

### Formatting

```bash
cargo fmt --all
```

### Linting

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

Clippy must pass clean. We treat `-D warnings` as the baseline; if
your patch produces warnings, either fix them or justify the
exception in the PR description.

### Build

```bash
cargo check --workspace
cargo build --workspace
```

### Tests

If your change is functional, it should come with tests. peisear
leans on integration tests that exercise the axum router end to end;
unit tests are appropriate where logic is pure and self-contained.

### Documentation

Public API changes require doc updates:

- User-facing behaviour change → update `docs/`.
- Architectural change → update the relevant file in
  `docs/architecture/` *and* `ROADMAP.md` if the change moves or
  completes a roadmap item.
- Breaking change → note it in the `## [Unreleased]` section of
  `CHANGELOG.md`.

### Commit style

- Use the imperative mood in the subject line: "Add X" not "Added X".
- Keep the subject line under 72 characters.
- Wrap the body at 72 characters.
- If the commit closes an issue, add a `Fixes #N` trailer.

Example:

```
Add per-issue effort estimate field

Introduces an `effort` column on `issues`, wires it through
`Issue` in peisear-core, and renders a selector on the new/edit
issue forms.

Fixes #42
```

## The pull request process

1. Fork the repository and create a topic branch from `main`.
2. Make your changes in logically organised commits.
3. Push to your fork and open a pull request against `main`.
4. Fill in the PR description: what changed, why, how you verified it.
5. Expect code review; it may request changes or additional tests.
6. Once approved, a maintainer will merge via a fast-forward or
   rebase merge (no merge commits).

## Design guidelines

peisear values consistency. If you're not sure, err toward:

- **Types over conventions.** If an invariant can be expressed in the
  type system, express it there.
- **Small surface area.** New dependencies are expensive; justify
  them in the PR description.
- **One crate, one concern.** If your change touches more than one
  crate, explain why in the PR — crate boundaries exist on purpose.
  See [docs/architecture/crate-boundaries.md](../docs/architecture/crate-boundaries.md).
- **Errors are explicit.** Use the layered error types (`AuthError`,
  `StorageError`, `AppError`) and their `From` conversions rather
  than reaching for `anyhow::Error` across crate boundaries.

## Licensing of contributions

peisear is licensed under Apache-2.0. By contributing, you agree
that your contributions will be licensed under the same terms. You
certify that you have the right to submit the work under this
licence (see the [Developer Certificate of Origin](https://developercertificate.org/)
for the spirit, though we do not require signed-off-by commits).

Substantial contributors may be listed in `NOTICE` at the
maintainers' discretion.

## Questions?

Open a discussion issue, or reach out via the channels listed in
[SECURITY.md](SECURITY.md) for anything sensitive.
