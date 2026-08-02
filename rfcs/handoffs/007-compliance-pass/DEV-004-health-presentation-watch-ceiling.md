# DEV-004 — Clamp health presentation to the Watch ceiling; retire the headline score

**Issued by**: Architect
**Date**: 2026-07-31
**Priority**: P0 / P1 — product-defining commitment
**Governing decision**: `DEC-023`
**Resolution already specified**: requirements baseline §10.2; external design §17.1
**Depends on**: nothing. Can run in parallel with DEV-001 and DEV-003.

---

## 1. Purpose

Three related contradictions in project-health presentation, all recorded in
the 0.19.1 baseline as open divergences:

1. A headline `"Score N / 100"` badge is rendered. `FR-HLT-008` forbids it.
2. `Concern` — a severity above `Watch` — is reachable in presentation.
   `NFR-LANG-002` (**P0**) forbids it.
3. `Concern` maps to `badge-error`, i.e. danger colouring for health state.
   `NFR-A11Y-004` and `NFR-LANG-002` forbid it.

The baseline already prescribes the resolution. Implement it.

## 2. Background

`SPEC §28.6` anticipated exactly this failure mode by separating computation
from presentation: the internal model **may** keep `Concern` for accuracy, but
the presentation layer must clamp it. The internal model was built correctly;
the clamp was never applied.

External design §17.1 is explicit that the code moves to the specification,
not the reverse: *"the ceiling is a product-defining commitment, not a
stylistic preference."*

Known sites:

| Site | Problem |
|---|---|
| `components/issues.rs:156-192` | Builds and renders `"Score"` + `{score} " / 100"` |
| `components/issues.rs:160` | Accessible label `"Project health score: {} of 100"` |
| `peisear-core/src/lib.rs:389-397` | `badge_class()` maps `Concern => "badge-error"` |
| `components/issues.rs:306`, `components/me.rs:151` | `indicator_glyph` has a `Concern => ("✗", "concern")` arm |
| `handlers/api_users.rs:167-175` | `indicator_str` serialises `Concern => "concern"` on `/api/users/{id}/burnout` |

The API site matters even though the burnout classifiers do not currently
produce `Concern` (`peisear-core/src/lib.rs:1609` notes the ceiling applies
there). External design §8.3 states the `indicator` field observes the severity
ceiling. **Clamp at the boundary regardless of current reachability** — relying
on "the classifier never emits it" is the same structural weakness that
produced this defect.

## 3. Applicable requirements

| ID | Requirement | Priority |
|---|---|---|
| `NFR-LANG-002` | No user-visible severity above `Watch`; `Concern`/`danger`/`failing` MUST NOT appear; no danger colouring for health state | P0 |
| `FR-HLT-008` | Composite shown at equal weight beside the indicators; no headline score; no 0–100 gauge or bar | P1 |
| `FR-HLT-009` | Computation and presentation are separate concerns | P1 |
| `NFR-A11Y-004` | Meaning not carried by colour alone; no red/green win-lose contrast | P1 |
| `FR-HLT-005` | Plain-language explanation for non-healthy indicators | P1 |

## 4. Change scope

- `crates/peisear-core/src/lib.rs` — presentation mapping only
- `crates/peisear-web/src/components/issues.rs` — health section
- `crates/peisear-web/src/components/me.rs` — glyph mapping
- `crates/peisear-web/src/handlers/api_users.rs` — `indicator_str`
- `crates/peisear-web/tests/health_explainability.rs` and a new ceiling test
- `CHANGELOG.md`

## 5. Non-change scope

- **The internal four-state model stays.** Do not delete
  `HealthIndicator::Concern` or collapse the enum. §10.2 is explicit: retain
  internal accuracy, clamp presentation.
- Do not change indicator computation, thresholds, or normalisation.
- Do not change which indicators exist. The `SPEC §28.1` vs implemented-set
  divergence (§10.1) is a separate, still-open question.
- Workload chips — DEV-003.

## 6. Required implementation

1. **Clamp at the render boundary.** Introduce one presentation-level mapping
   that reduces the four-state internal model to the three displayable states
   (`Insufficient` / `Good` / `Watch`), mapping `Concern → Watch`.

   **It must be structurally impossible to render an unclamped state.** Do not
   implement this as a call each site is expected to remember — that is how the
   current defect exists. Prefer a distinct presentation type that the render
   path requires, so a missed site fails to compile rather than failing
   silently. Justify your approach in the review request.

2. **Clamp the badge class.** `Concern` must not resolve to `badge-error` or
   any danger palette. After clamping, the reachable classes are
   `badge-ghost` / `badge-success` / `badge-warning`.

3. **Clamp the glyph.** The `("✗", "concern")` arm must not be reachable in
   presentation. `✗` is failure iconography.

4. **Clamp the API.** `indicator_str` must never emit `"concern"` on
   `/api/users/{id}/burnout`. `"watch"` is the ceiling
   (external design §8.3).

5. **Remove the headline score.** Delete the `"Score"` label, the
   `{score} " / 100"` badge, and the `"… score: {} of 100"` accessible label.
   Render the **composite indicator at equal weight beside the individual
   indicators**, per `FR-HLT-008` and external design §6 SCR-08.

   The composite keeps its state badge, trend chip, and summary sentence — it
   simply stops being the headline and stops carrying a number. No 0–100 figure,
   gauge, progress bar, or percentage anywhere in health presentation.

6. **Preserve explanations.** `FR-HLT-005` explanation sentences stay. Note
   that `human_explanation()` currently returns text for `Watch | Concern`;
   after clamping, `Concern`-derived explanations surface under a `Watch`
   badge. That is correct and intended — the sentence stays factual, only the
   severity label is clamped.

7. **Vocabulary.** No new string may contain `Concern`, `danger`, `failing`,
   `critical`, or failure framing (§1.7).

## 7. Required tests

New `crates/peisear-web/tests/severity_ceiling.rs` (or extend
`health_explainability.rs`):

1. For a project whose indicators compute to `Concern`, the rendered project
   detail contains no `"Concern"`, `"danger"`, `"failing"`, or `"critical"`.
2. The same page contains no `badge-error` on a health element.
3. The same page contains no `"/ 100"`, no `"of 100"`, and no `"Score"` label.
4. The composite renders beside the individual indicators, not above them as a
   headline.
5. `/api/users/{id}/burnout` never returns `"indicator": "concern"`.
6. The explanation sentence for a `Concern`-computed indicator is still
   present — clamping must not silence the explanation.

Test 1 requires fixture data that actually reaches `Concern`. If the existing
harness cannot construct one, build it — a ceiling test over data that never
approaches the ceiling proves nothing. This is the single most important test
in the task.

## 8. Acceptance criteria

1. No user-visible surface and no API response exposes a state above `Watch`.
2. No danger colouring represents health state.
3. No 0–100 score, gauge, or percentage appears in health presentation.
4. The composite appears at equal weight beside the individual indicators.
5. Internal computation is unchanged — same states computed, same explanations.
6. fmt, clippy `-D warnings`, and the full web test suite are clean.

## 9. Prohibited shortcuts

- **Do not** delete `Concern` from the internal enum to make the problem go
  away. That destroys computational accuracy §10.2 explicitly preserves.
- **Do not** clamp by renaming the variant or its label string while leaving a
  distinct colour class reachable. The mockup makes exactly this mistake — it
  translates `level_concern` to `"Watch"` but still emits a `badge--concern`
  class with its own palette. Clamp **the whole badge**, not the text.
- **Do not** keep the score "for now" behind a flag or a collapsed panel.
  `FR-HLT-008` forbids the form, not its prominence.
- **Do not** amend `FR-HLT-008` or `NFR-LANG-002`. External design §17.1
  settles the direction: the implementation moves.

## 10. Known risks

| Risk | Mitigation |
|---|---|
| Removing the score changes a familiar surface | Intended. Record the rationale in `CHANGELOG.md` per `NFR-MNT-009` |
| A missed render site keeps leaking `Concern` | Step 1's compile-time approach exists for this. If you choose a runtime approach instead, justify it and add a test per site |
| Fixture data may not reach `Concern` | Build it. See §7 |

## 11. Required evidence

- Changed-file list.
- fmt and clippy output.
- Full test output including the new ceiling tests.
- Confirmation that test 1 **fails** before the change — paste that run.
- Before/after rendered HTML for the health section.
- A short statement of how step 1 makes an unclamped render impossible or
  detectable.

## 12. Required review-request format

Per workflow §9.2, into `.git-exclude/review-request/`. Request focused review
on the step-1 clamp mechanism and the §7 fixture that reaches `Concern`.

**Escalate rather than deciding** if clamping reveals that indicator
computation itself depends on the presentation mapping (it should not — that
would be a `FR-HLT-009` violation worth reporting), or if removing the score
leaves the health section without a coherent summary for the user.
