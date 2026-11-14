# Handoffs — the checks that could not see it

**`0.63.0`, authorized by the owner 2026-11-12** — the row, not the cut.
Three defects a reader meets, all shipped, **each invisible to every
gate this project owns for a structural reason**. The repairs are small.
**The guards are the release.**

---

## How work lands — read this before starting a round

**Commit and push to the shared `main`.** No topic branch and no pull
request: the owner coordinates the schedule so that no two workers are
ever on the same tree at once, so a branch and its merge step add a step
and remove nothing.

**Whoever pushes watches `main`'s own CI to completion.** A red `main` is
that person's only work until it is green again — fix forward or revert,
never leave it for someone else to find.

Exceptions exist and are narrow: parallel work the owner has
deliberately scheduled, a patch to an already-released version, a change
that cannot be made in steps that each leave the trunk working,
throwaway experiments, and any branch that is never pushed. A
contribution from outside the team is the public workflow, not this one.

---

## Why these are one release — re-scoped 2026-11-12

Each defect shipped because **the gate nearest to it asks a neighbouring
question**:

| the defect | the gate that was green | what it asks | what nobody asked |
|---|---|---|---|
| ten disclosures render no marker | `touch_target_scan` | is the control **big enough**? | does a disclosure still **look like one**? |
| five sentences read *"1 issues"* | three tests quoting the sentence | does the **tail** match? | does the word before it **agree**? |
| two header controls overlap at 768–800 px | `overflow-gate`, green at 768 | does the page **overflow**? | do two controls **cover each other**? |

**`MARK-001` then taught this series its own lesson, and the row was
re-scoped on it.** That round fixed six sites and added a scan for the
shape it had fixed. Planted with the defect's **other** shape — a
literal `inline-flex` inside the helper's argument rather than the
`grow_text()` helper itself — the scan passes. Four sites that were
already markerless were invisible to it, and the architect's own census
had missed them for the same reason.

So the principle this release is built on changed:

> **Make the property true by construction. Keep the check as a
> backstop, never as the mechanism.**

- A `<summary>` cannot lose its marker if **nothing can give it a
  display class** — so there is one helper for summaries, and the two
  exceptions the old approach needed are **deleted**.
- A sentence cannot disagree with its own number if **the number and
  both forms are chosen in the same place** — so a private helper in
  `en.rs` does it, and the allowlist the first draft needed never
  exists.
- Two controls cannot overlap if **the row that lays them out is allowed
  to shrink them** — so the fixed width goes, and the occlusion check
  becomes the backstop rather than the remedy.

**A round that fixes its instances and adds a check has done the
smaller half, and this series has already paid for that lesson once.**

## Rounds

| ID | Handoff | Subject | Blocked on | State |
|---|---|---|---|---|
| MARK-001 | [the other six](./MARK-001-the-other-six.md) | The six `<summary>` sites reachable through `grow_text(` | — | **Closed 2026-11-12, no findings — and it corrected the architect's census.** The population was **ten of fourteen, not six**: the defect is about **computed display**, and a literal `flex`/`inline-flex`/`btn` inside `grow()`'s own argument sets it just as well as the `grow_text()` helper does. Found by re-deriving rather than inheriting, and by checking each site's **rendering** rather than its call. Six built, `disclosure-marker-gate` extended to nine summaries across **four** pages — one more than this handoff named, because the velocity summary lives on the sprints *list* and the architect's page list would have left one of the round's own six unchecked. `alignment_scan` 3, `--lib` **246**, `DEC-007` **676 → 677** |
| MARK-002 | [a summary is never a flex container](./MARK-002-a-summary-is-never-a-flex-container.md) | The rule that ends the class | MARK-001 | **Closed 2026-11-14, no findings.** Thirteen of fourteen summaries compose `grow_summary(`, whose body is `format!("{base} {TOUCH_TARGET}")` and so **cannot contribute a display class**; the fourteenth is the escalation exclusion, untouched and cross-referenced rather than re-listed. **Both exceptions deleted**, as the round was asked to. The architect's own two plants fail by name on one consolidated test — including the literal `inline-flex` inside the helper's argument, the shape that defeated `MARK-001`'s scan. Gate across **thirteen summaries on seven pages**, both widths. **It corrected the architect twice more**: `/today`'s Rhythm is a single-child site, not one of the three needing a wrapper; and the gate's own coverage map had a gap, found because a regenerated `today.png` still showed Rhythm markerless after the source was fixed — the published artefact noticing what the gate's population did not contain. **Three findings by measurement**: a block child pushes the marker to its own line (72 px, not 48); an `inline-flex` box is atomic and wraps whole at 390 px, which killed the inner-span design; plain text in a 44 px control wraps to 104 px. `DEC-007` **677 → 677** — one test removed, one added |
| GRAM-002 | [every counted sentence](./GRAM-002-every-counted-sentence.md) | The five remaining strings — and agreement made structural by a private `counted()` helper, so the allowlist the first draft needed never exists | — | **Closed 2026-11-14, accepted with one correction required (round 2).** `en.rs::counted(n, singular, plural)` is private, prepends the number and takes the plural at `0` — the mechanism is right. **Fourteen fixed against the handoff's five**, including a duplicate code path for one of them the handoff did not know existed, found by re-deriving from the enum as the handoff asked. The architect's plant fails by name. **It reverses `CAL-002 §2.2`'s *"Do not add pluralisation"* and the reversal is ratified** — that argument was right and its prediction came true, and what changed is that the architecture is now ready; `RFC 006` rejects `fluent`, not agreement, so the old doc comment over-read its own citation (`§10.59`, amended). **The correction**: the guard this release's own row names — *render every count-bearing message at 1 and read it* — was built as `assert_eq!` lists of the strings already fixed, which is the shape the defect defeated twice. `MessageKey::all()` has rendered `ReadFirstLongStaleBody` at 1 on every CI run throughout, reading *"1 issue haven't been touched"*, in neither the fixed nor the reported list. `DEC-007` **677 → 680**, re-counted independently |
| GRAM-002 r2 | [the guard the row names](./GRAM-002-round-2-the-guard-the-row-names.md) | The guard as a property over `MessageKey::all()`, and the seventeen strings it finds | GRAM-002 | **Closed 2026-11-14, accepted.** `grammar_guard.rs` renders **all 748** `MessageKey::all()` entries and reads every one, collecting every offender with its rendered string — a property over the population, not a list of known strings. Built **before** the fixes and run first: sixteen of the seventeen had no sample at `1` in `all()` at all, so the samples came first and the pre-fix run is the round's own measurement. All seventeen through `counted()`; `ReadFirstLongStaleBody`'s hand-written `if` deleted for one call carrying noun, verb and pronoun in both arms. Hand-written word split rather than a new `regex` dependency; the skip set is five grammar words, closed. **Both gaps named rather than patched around**, and both confirmed by architect plant: a disagreement one adjective from its number, or one a verb away, leaves the guard green. Population **730 → 748**, `DEC-007` **680 → 683**. **It corrected the architect three times**: `FR-I18N` is not an id this repository has (`NFR-LANG-005` is), the three-run `cargo test --workspace` criterion was retired by `CONTRIBUTING.md` on 2026-10-15, and the `§3` rule's word *"immediately"* is what its own gap is made of |
| GRAM-002 r3 | [the rule that was too narrow](./GRAM-002-round-3-the-rule-that-was-too-narrow.md) | Adjectives between a number and its noun, and the hand-rolled inflection branch that cannot come back | GRAM-002 r2 | **Closed 2026-11-14, with one blocking finding filed and resolved mid-round.** The architect's widened rule said *three words*; at that width the guard flagged **three renderings that are already correct English**, reading past the noun phrase across a full stop, a comma and a semicolon. The round stopped at the first acceptance step, named the two escapes its own handoff prohibited, declined both, and **put the question in a file rather than chat** — the finding was reproduced by the architect before ruling. Resolution: `WINDOW = 2`, a narrower window rather than a wider stop set, green over all 748 and still catching `"1 consecutive snapshots"`; the three sentences became permanent negative-space inputs. `no_hand_rolled_inflection.rs` answers the verb case at the mechanism, with the one named exception proven visible to its own detector. **Both architect plants fail by name, and two further plants on shapes the round did not test** — a second adjective phrase, and a hand-rolled choice on a different key — fail too, so neither check is keyed to its one known site. The detector's boundary (`!= 1` escapes it) was probed by plant and kept deliberately; `§10.59` names the type that would close the class structurally. `DEC-007` **683 → 687**, population **748** unchanged |
| OCCL-001 | [two controls, one row](./OCCL-001-two-controls-one-row.md) | Let the header row shrink its own field so the overlap cannot happen, with the occlusion check as the backstop | — | **Closed 2026-11-14, no findings.** The cause was found live, by patching candidates into the running page and re-measuring after each, not read off the markup: `w-48` is not `flex-shrink: 0`, but a flex item's automatic minimum size floors at its specified width for a form control whose `overflow` is `visible`, so **`min-w-0` has to sit at every level of the nesting that floors** — on the `<input>` alone it changes nothing, on the `<form>` as well it clears, on `<nav>` too it only over-shrinks. `LAYOUT-006`'s own lesson, reaching the search field this time. `occlusion-gate.mjs` is a **fourth sibling gate**, not a second assertion inside `overflow-gate.mjs`, and it imports that gate's own widths, fixtures and page list rather than copying them. **It found more of the defect than the investigation that named it**: planted, it fails 48 of 120 cells — all 24 pages at 768 px **and all 24 at 1280 px**, a width `§10.55`'s own table calls safe, because the adversarial 77-character display name grows the account label far enough to reach. `--lib` 246 and `DEC-007` unmoved; no Rust test touched |

**Series `061` is complete.** `MARK-001`, `MARK-002`, `GRAM-002` and
its two further rounds, and `OCCL-001` are all closed, and `§10.58`,
`§10.59` and `§10.55` are closed with them. Each of the three defects
now has a check that could not pass while it was present, and in each
case the property is true by construction first and checked second —
one helper a `<summary>` may take, one `counted()` where a number meets
its noun, one row whose own shrink reaches every control in it.

## What this series deliberately does not carry

- **The typography and colour floor.** Its condition — *adopt after the
  views are decided* — has been met since unit three closed, and it is
  the next **design** row rather than a repair. Putting a new rule
  beside three repairs would blur what this release claims.
- **The ten ungoverned `<details>` elements.** A set, decided once;
  `external-design §5.10` says so in terms.
- `§10.15`, `§10.32`, `§10.50` and PostgreSQL P3, all carried.
- The two call-site counts whose population was never stated —
  `ROADMAP.md`, *Unscheduled work*.

## Where the numbers in these handoffs come from

Every figure was measured on 2026-11-12 against `0.62.0`'s own tree, and
one of them **corrects a previous round's report**: `DISC-001` named
*"`/today`'s own three summaries"* as what remained, and a source-level
census across every shipped component found **six markerless of
fourteen**, seven already correct, plus one site that uses no helper and
is not a defect. Read each handoff's own §1 before starting; where it
gives a population, re-derive it rather than inheriting it.
