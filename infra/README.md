# infra/

A scaffold, partly filled. Contents, roadmap work unless marked:

- `docker/` — **done, `FIV-003`.** A single-stage-visible, two-stage-build
  `Dockerfile`: the builder carries the Rust toolchain, the published
  image carries the `peisear` binary alone on `gcr.io/distroless/cc`.
  Not "plus `static/`" as this file used to say — `FIV-005` embedded
  `static/` into the binary at compile time, so there is nothing left
  to copy separately.
- `compose.yaml` — `docker compose` stack for local/dev end-to-end, wiring the app to a PostgreSQL container once the `storage-postgres` crate lands.
- `terraform/` — minimal IaC to stand the app up on a single-node VM with TLS, backups, and a health probe.
- `k8s/` — a minimal Helm chart or kustomize overlay for clusters.

**Not `github/`** — GitHub Actions workflows must live at
`.github/workflows/` at the repository root; GitHub does not read them
from anywhere else. This file said `infra/github/` before `FIV-003`;
that was never going to work and is corrected here rather than carried
forward. The actual workflows this scaffold anticipated are
`.github/workflows/test.yml`, `docs.yml`, `release.yml`, and now
`image.yml`.

Keeping this directory separate from `crates/` means Rust tooling
(cargo check, cargo fmt, rust-analyzer) never traverses infrastructure
files, and infra engineers can iterate on Dockerfiles and pipelines
without rebuilding the Rust tree.
