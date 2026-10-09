# Handoffs — the three that need a judgement

**Not RFC-governed.** `§10.35`'s last three coverage gaps, held back from
0.46.0 on purpose: **each needs a decision about *what* a test should assert**,
not just a test written against current behaviour. Writing them against
today's code would have produced three tests that pass and prove little —
which is the defect class `§10.35` names.

| ID | Link | What | Release |
|---|---|---|---|
| PER-001 | [a precedence chain is its tie-breaks](./PER-001-a-precedence-chain-is-its-tie-breaks.md) | `FR-PER-006` (P1) cites **one** test — the case where **nothing applies**. The code has **four** early returns, not three tiers (burnout is two conditions), and two boundaries its own comments call deliberate: **`>` not `>=`** on WIP, and `long_stale_count >= 1`. **A chain is only a chain where two conditions compete**, so four isolated tests would test four independent conditions. The test *level* is the dev team's judgement, with reasoning required. | 0.47.0 |
| HLT-003 | [three indicators, no basis test](./HLT-003-three-indicators-no-basis-test.md) | `FR-HLT-007` (P2): of five non-excepted indicators, **two are tested**. `Activity`, `bus-factor` and `long-stale` have none. **Not one shared path** — `basis_for` forks per kind, returning a different field for each, so three untested kinds are three untested computations. **The slugs are hyphenated**, and `REQ-005`'s negative for two of them came from grepping underscores: right answer, wrong needle. Also renames or widens a **plural-named test covering one indicator** — the reason this entry read like a sample. | 0.47.0 |
| GATE-005 | [what should the browser gate cover?](./GATE-005-what-should-the-browser-gate-cover.md) | **An investigation; the deliverable is a ranked proposal, not tests.** Two items have now been deferred into one question: `NFR-A11Y-008`'s script-updated live regions, and the real-drag-gesture test from `DM-TEST-001`. A Rust test asserting a region merely *exists*, cited as acceptance, **would build `§10.35`'s own defect one release after naming it**. `RFC 011`'s refusal to buy a harness is not reopened. **"Nothing further earns a gate" is a complete result** — `REQ-003`'s precedent. | 0.47.0 |

| GATE-006 | [one script, three properties](./GATE-006-one-script-three-properties.md) | **`GATE-005`'s proposal, accepted.** Candidates 1–3 as one script at port `4175`; **candidate 4 declined on the record**, with its CDP capability gap now in `§10.15` as the named remainder. The answer to `GATE-005`'s question was **yes** — a CDP gate *can* see that a live region received text — so `NFR-A11Y-008` gets a mechanism rather than a permanent limit. **The risk the investigation named is now the handoff's**: the synthetic-drop technique has only ever driven `dm.js` and `board.js`, so the drop's effect is asserted **before** the region's text, making a setup failure legible instead of looking like a defect. | 0.47.0 |

| REL-0.47.0 | [candidate](./REL-0.47.0-release-candidate.md) | **The release, and `§10.35` closes with it.** Expect **405** from 392, and the composition needs saying: `PER-001` +10, `HLT-003` +3, **`GATE-006` +0** — a browser gate is outside `DEC-007`'s inventory, so a reader would otherwise think it is in the 405. One production file changed, `me.rs`'s `#[cfg(test)]` module. **The new gate is run three times, not once**: it is new, it drives real input, and flakiness found now beats flakiness found in CI. | 0.47.0 |

**Why `GATE-005` is an investigation and the other two are not.** `PER-001`
and `HLT-003` know what they must assert once someone decides at which level;
`GATE-005` does not know whether its property is observable at all. Asking a
question whose answer may be *no* is cheaper than building a mechanism that
cannot carry the requirement it would be cited for.
