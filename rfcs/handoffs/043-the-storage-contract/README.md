# Handoffs — the storage contract

**`RFC 0018`-governed**, plus two security items that came out of the same
assessment. **0.50.0's theme is the correctness and security of what already
ships** — not the PostgreSQL backend, which starts at 0.53.0.

**Why this release exists at all.** Scoping `RFC 0018` meant reading how
`updated_at` is maintained, and the reading found that **the optimistic-lock
token does not advance between two writes in the same second**. SQLite's
`CURRENT_TIMESTAMP` is whole-second and `NFR-CONC-002` reuses it as the lock
value, so the P0 requirement that exists to prevent a lost update has a
window in which it cannot see one. **A backend port three releases away would
have masked it rather than fixed it.**

| ID | Link | What | Release |
|---|---|---|---|
| STORE-001 | [the lock token that does not move](./STORE-001-the-lock-token-that-does-not-move.md) | **`RFC 0018` P0.** Demonstrate the lost update end to end — it must fail before the fix — then move the eleven triggers and the column defaults to millisecond precision. The stored format changes, and pre-migration rows keep the old shape until their next write, so **both must parse** | 0.50.0 |
| STORE-002 | [the backstop, and the cost of durability](./STORE-002-the-backstop-and-the-cost-of-durability.md) | **`RFC 0018` P0b** — measure the write cost of `synchronous = FULL` against `NORMAL` and decide it on the number — **and `STORE-001` review §2**: a storage-level test of the **five `julianday(updated_at) = julianday(?)` guards** (`projects` ×2, `sprints` ×2, `user_capacities` ×1), which `STORE-001` never reached because the handler's Rust check refuses a stale token first | 0.50.0 |
| SEC-001 | [what leaves the process, and what the operator knows](./SEC-001-what-leaves-the-process-and-what-the-operator-knows.md) | **Three items from `RFC 0018` §2**, plus the durability note and the rename from `STORE-002`'s review. Redact the DSN in the startup log (`main.rs:22-27` logs it at `INFO`, no redaction anywhere; harmless as a path, a credential the day P3 lands). And **put data at rest in the threat model** — 194 lines across `docs/security/` contain zero occurrences of "encrypt", "at rest" or "disk" | 0.50.0 |
| REG-001 | *(with the candidate)* | **`§10.36`**, a register entry: *a check that cannot tell "nothing wrong" from "nothing examined."* Three instances in two releases, all named in `REL-0.49.0`'s review | 0.50.0 |

**Deliberately not in this release**, and recorded so the order is visible
rather than remembered:

- **0.51.0 — documentation accuracy.** `DOCS-006` (both architecture
  documents, the deployment manifest, the citation count), `RFC 0016`
  proposal 2 (`README.md`'s repositioning, including `README.md:77`'s
  PostgreSQL claim), `scaling.md`'s own version of the same overclaim,
  `CONTRIBUTING.md`'s stale MSRV **and rule 2 reading one file too few**, the
  scratch-hygiene convention, `requirements.md §11`'s two migration rows from
  `RFC 0017`, the sprint-plan fixture with the fourth screenshot, and the
  `grow()` alignment defect.
- **0.52.0 — the first five minutes** (`RFC 0017`).
- **0.53.0–0.55.0 — the backend** (`RFC 0018` P1–P4).

**Decided and closed**: database-level encryption is **not** required
(owner, 2026-10-11). Volume encryption is the control, documented by
`SEC-001`; SQLCipher and column-level encryption are both declined on the
record in `RFC 0018 §2`.
