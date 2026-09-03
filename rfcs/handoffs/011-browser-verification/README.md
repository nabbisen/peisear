# Handoffs — RFC 011, browser verification

Implementation companions for
[RFC 011](../../accepted/011-browser-verification.md).

**Steps 1 and 1b are committed.** Steps 2–4 are scheduled and re-decided at their
own releases against step 1's evidence.

| # | Handoff | Covers | Target | Depends on |
|---|---|---|---|---|
| JS-001 | [JS-001](./JS-001-policy-inventory.md) | Step 1 — classify every decision in the three scripts as movable policy or irreducible mechanics. **No code moves.** | 0.29.0 | — |
| JS-002 | [JS-002](./JS-002-pin-the-boundary.md) | Step 1b — pin the two-catch shape `dm.js`'s fallback boundary is made of. `JS-001` proved the rule itself cannot move. | 0.29.0 | JS-001 |
| JS-003 | [JS-003](./JS-003-one-outcome-authority.md) | Step 2 — one authority for response classification: the `409`/other/malformed decision, written three times in JS, moves into the server-authored copy island. Settles `board.js`'s silent malformed-body case. | 0.30.0 | JS-001, JS-002 |
| REL-0.30.0 | [REL-0.30.0](./REL-0.30.0-release-candidate.md) | Release candidate — `JS-003` alone. **Branches from `7845751`, not `main`**, because `main` already carries RFC 012. | 0.30.0 | JS-003 |
