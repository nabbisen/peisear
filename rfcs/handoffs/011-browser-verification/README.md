# Handoffs — RFC 011, browser verification

Implementation companions for
[RFC 011](../../done/011-browser-verification.md).

**Steps 1 and 1b are committed.** Steps 2–4 are scheduled and re-decided at their
own releases against step 1's evidence.

| # | Handoff | Covers | Target | Depends on |
|---|---|---|---|---|
| JS-001 | [JS-001](./JS-001-policy-inventory.md) | Step 1 — classify every decision in the three scripts as movable policy or irreducible mechanics. **No code moves.** | 0.29.0 | — |
| JS-002 | [JS-002](./JS-002-pin-the-boundary.md) | Step 1b — pin the two-catch shape `dm.js`'s fallback boundary is made of. `JS-001` proved the rule itself cannot move. | 0.29.0 | JS-001 |
| JS-003 | [JS-003](./JS-003-one-outcome-authority.md) | Step 2 — one authority for response classification: the `409`/other/malformed decision, written three times in JS, moves into the server-authored copy island. Settles `board.js`'s silent malformed-body case. | 0.30.0 | JS-001, JS-002 |
| REL-0.30.0 | [REL-0.30.0](./REL-0.30.0-release-candidate.md) | Release candidate — `JS-003` alone. **Branches from `7845751`, not `main`**, because `main` already carries RFC 012. | 0.30.0 | JS-003 |
| BROWSER-001 | [BROWSER-001](./BROWSER-001-overflow-gate.md) | Step 4 — one assertion, `scrollWidth <= clientWidth`, as a CI job rather than a `cargo test`. Binding on `DEC-048`. | 0.33.0 |
| LOCK-001 | [LOCK-001](./LOCK-001-board-card-lock-value.md) | Step 3's **replacement** (`DEC-052`) — assert every board card renders a non-empty `data-updated-at`, the guarantee `board.js` was covering for. | 0.33.0 |
| BROWSER-002 | [BROWSER-002](./BROWSER-002-fixture-and-320.md) | The gate is green on a tree with three overflow defects, because its fixture has a space at every point. An unbroken run in the fixture, then 320 px. **Fixture first, and watch it go red.** ✅ Done — fixture (`1890916`, watched red) and 320 px (`188056e`, 70/70 on fourteen pages); three plants held. | 0.33.0 | LAYOUT-006 |
| REL-0.33.0 | [REL-0.33.0](./REL-0.33.0-release-candidate.md) | Release candidate — the gate, its fixture and 320 px, `LOCK-001`, `LAYOUT-003`–`008`, `NFR-A11Y-006` Met. **After `LAYOUT-008`.** The changelog must not read as "layout is now covered". | 0.33.0 | LAYOUT-008 |
