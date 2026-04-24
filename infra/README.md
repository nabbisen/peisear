# infra/

Intentionally empty scaffold. Contents to be added in roadmap work:

- `docker/` — a single-binary container image (scratch or distroless) with the `peisear` binary plus `static/`.
- `compose.yaml` — `docker compose` stack for local/dev end-to-end, wiring the app to a PostgreSQL container once the `storage-postgres` crate lands.
- `terraform/` — minimal IaC to stand the app up on a single-node VM with TLS, backups, and a health probe.
- `github/` — GitHub Actions workflows for `cargo test`, `cargo clippy`, `cargo build --release`, and release tagging.
- `k8s/` — a minimal Helm chart or kustomize overlay for clusters.

Keeping this directory separate from `crates/` means Rust tooling
(cargo check, cargo fmt, rust-analyzer) never traverses infrastructure
files, and infra engineers can iterate on Dockerfiles and pipelines
without rebuilding the Rust tree.
