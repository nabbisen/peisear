# Handoffs — yanked dependencies

**Not RFC-governed.** Surfaced by `0.43.0`'s `cargo publish --dry-run` and
correctly reported by the dev team as pre-existing rather than treated as a
candidate finding.

| ID | Link | What | Release |
|---|---|---|---|
| DEP-001 | [two yanked crates](./DEP-001-two-yanked-crates-in-the-lockfile.md) | `spin 0.9.8` (**shipped**, via `sqlx-sqlite`) and `chacha20 0.10.0` (**test only**, via `axum-test`) are both yanked. Both move within the same minor — `0.9.9` and `0.10.2` — and the architect verified that together they change **four lines of `Cargo.lock` and nothing else**. So the work is the gates, not the update, and **the MSRV build is the one that matters**: `NFR-CMP-001`'s declared `1.88.0` was once false precisely because nothing exercised it. A vulnerability scanner in CI is explicitly out of scope — that is a decision, not an implementation detail. | 0.44.0 |
