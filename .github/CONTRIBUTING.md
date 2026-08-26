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
   cargo build -p peisear --bin peisear
   ```
3. Run the application:
   ```bash
   cp .env.example .env
   cargo run --release -p peisear
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

**Run tests per crate and per target, not `cargo test --workspace`
in one shot** (`DEC-007`):

```bash
cargo test -p peisear-core    --lib
cargo test -p peisear-auth    --lib
cargo test -p peisear-storage --lib
cargo test -p peisear-i18n
cargo test -p peisear-notify  -- --test-threads=1
for t in aggregate_privacy assignee_candidates auth_boundary board_keyboard \
         breadcrumb calendar calendar_surfaces confirmation \
         health_explainability inbox_refinements issue_edit_url \
         optimistic_lock search smoke sprint_plan status_control \
         status_segment sub_issues today_panel view_state \
         workload_privacy; do
  cargo test -p peisear-web --test "$t" -- --test-threads=1
done
cargo test -p peisear-web --lib
cargo test -p peisear
```

This isolates each test binary's link step. That is the live,
present-tense reason it exists: peak linker memory stays bounded
this way, because the combined `--workspace` build links several
large crates at once (sqlx, leptos, axum, the wasm-smtp family). It
also gives clean per-target failure attribution.

**The `--lib` shape on `peisear-core`, `peisear-auth`, and
`peisear-storage` skips doctests.** There are none in those three
crates today — the workspace contains exactly one doctest, on the
`peisear` facade's `src/lib.rs` — so nothing is uncovered right now.
It becomes a hole the day someone writes a documented example in one
of them; `cargo test -p peisear` (bare, no `--lib`) is what picks up
the facade's own doctest, which is why that line has no `--lib` where
the other single-crate lines do. `QA-004`: this block used to omit the
`peisear` line entirely, discovered when a release candidate's own
gate table carried a count no command in this block actually produced.
`dec_007_scan` (`peisear-web`'s test suite) now asserts every workspace
member's crate name appears somewhere in this block, so the list
cannot drift from the workspace silently again.

**History, for a specific defect that is now guarded a different
way**: `cargo test --workspace` used to also fail intermittently for
an unrelated reason — a `TestApp::spawn`/`fresh_pool` temp-directory
collision named from the clock alone (`QA-001`, baseline `§10.13`).
That defect is fixed (`peisear-web/tests/common/server.rs`,
`peisear-notify/tests/dispatch_integration.rs`) and is now caught
**deterministically**, every time, at no runtime cost, by a test
that scans every `crates/*/tests/**.rs` file for the pattern
reappearing (`peisear-web`'s `test_harness_scan` module) — not by
running `--workspace` repeatedly and hoping to land on a bad roll.
`QA-001`'s review measured that hope directly: on a loaded machine
the same command failed 3 of 6 runs; on a quiet machine, 0 of 6,
*with the defect fully present both times*. A repeated run is not a
reliable detector for a deterministic property, so `test_harness_scan`
is the guard for this specific class now, not the recommendation
below.

**Still run `cargo test --workspace` three times in a row before
opening a PR that touches `crates/peisear-web/tests/` or
`crates/peisear-notify/tests/`, or before cutting a release
candidate** — for flakes of *other* kinds, the ones a structural scan
cannot see because they're not "a file contains a bad pattern" but
"two things interact badly at runtime." This is deliberately the
condition a contributor reaches for without thinking, by typing the
obvious command, because the per-crate procedure's isolation is
exactly what would hide a class of defect like that from every other
gate result — which is what happened here, once, before it had a
better guard. Re-measure and adjust the count if a new defect's
observed rate suggests three isn't enough.

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
