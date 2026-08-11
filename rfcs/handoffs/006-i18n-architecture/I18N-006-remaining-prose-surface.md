# I18N-006 — The remaining prose surface

**Issued by**: Architect
**Date**: 2026-08-10
**Priority**: P1 — **the last item before 0.21.0**
**Governing RFC**: [006](../../accepted/006-i18n-architecture.md)
**Depends on**: I18N-005b–e (all landed)

Pattern rules: RFC 006 §D6.

---

## 1. Purpose

Four categories of user-visible text that the 005 series did not reach, because
they live outside `peisear-web`'s components or outside Rust's reach entirely.

This handoff exists because it was named in three consecutive reviews and grew
in each — from two prose functions, to three, to five consumer sites plus two
more categories. **That growth happened because nobody measured it.** §2 is a
measurement, not an estimate.

## 2. The surface, measured

| Category | Where | Extent |
|---|---|---|
| `DisplayHealthState::glyph()`'s state word | `peisear-core/src/lib.rs:437` | 3 words (`no data` / `good` / `watch`), **5 consumer sites** |
| `IndicatorKind::description()` | `:690` | 6 sentences |
| `peisear-storage` validation / conflict text | `peisear-storage/src/**` | **At least one** (`"Sprint is already active."`) — a floor, not a count. §5 |
| `BurnoutSignal.label` | `handlers/api_users.rs` ~`:105`, `:115`, `:134-146` | 3–4 constructions, JSON response prose |
| `validator` derive messages | 7 sites across `peisear-web` | **Not converted** — see §6 |

## 3. `glyph()` — split the symbol from the word

`glyph()` returns `(&str, &str)`: a symbol and a state word.

The **word** is copy and goes through the table, following the
`IndicatorLabel` precedent — a typed label in `peisear-i18n` with a
`to_i18n_label()` conversion at the boundary.

The **symbol** (`—` / `✓` / `⚠`) may stay in `peisear-core`. It is not language.
Note in passing that symbols are not culturally neutral either, but a second
locale is deferred and inventing a mechanism for three glyphs today would be
premature — record it, do not solve it.

Update all five consumers. Two are `me.rs`'s WIP and long-stale chips, found by
I18N-005d; three are `HealthStrip`'s trend chip, composite row and indicator
row, found by I18N-005b. If you find a sixth, that is the finding.

## 4. `description()` — six keys, no subtlety

Straight conversion. These are the indicator explanations shown on the health
surface.

## 5. `peisear-storage` — the one place with real design weight

**Survey first.** My grep found a single user-facing string; the 005d and 005e
reviews both reported storage text reaching users through passthrough arms in
`sprints.rs`, `teams.rs` and `settings.rs`. Those cannot both be right. Establish
the real count before converting anything, and report it.

**Design decision, made here so you do not have to**: user-facing
`StorageError` variants carry a `MessageKey`, not a `String`.

`peisear-i18n` is a leaf crate; `peisear-storage` depending on it mirrors what
`peisear-core` and `peisear-notify` already do. The alternative — mapping
storage errors to keys by string-matching at the web boundary — recreates
`DEC-011`'s fragility, where `translate_trigger_error` already couples to SQLite
`RAISE` text and a reworded trigger silently breaks the mapping.

Two constraints:

- **Internal-only variants stay `String`.** The test is the same as I18N-005e's:
  does this ever reach a rendered page or an API response? Log-only text stays.
- **`translate_trigger_error` maps `RAISE` text to a `MessageKey`**, not to a
  string. Its coupling to trigger wording is pre-existing (`DEC-011`) and is not
  yours to fix, but do not deepen it.

**Escalate** if changing the error type ripples beyond the storage boundary and
its direct callers — that would mean the variant is doing more than carrying a
message, and the design needs a second look before it spreads.

## 6. The `validator` derive messages — checked, not converted

`#[validate(message = "…")]` requires a literal, so `t()` cannot reach the seven
sites. Both options I18N-005e raised are wrong: refactoring to manual validation
is structural work for a vocabulary problem, and a permanent exclusion leaves
seven validation messages — the category where §1.7 failure framing concentrates
— unchecked forever.

**The guard's purpose is that copy is checked, not that it lives in the table.**

Add a test running `peisear_i18n::find_violations` over those seven literals.
Collect them in one place — a `const` array beside the forms, referenced by the
derives if the crate allows it, otherwise listed in the test with a comment
naming each site.

If a literal cannot be referenced from the test without duplicating it,
duplication plus a comment is acceptable here and only here — say so in the
review request, because a duplicated literal is the drift risk this release has
found four times.

## 7. Not in scope

`static/search.js`. It needs a mechanism that does not exist, and inventing one
for a type-ahead dropdown would be disproportionate. **Permanently excluded and
named as such** — the review request should say so, so the exclusion is a
decision rather than an omission.

## 8. Tests

1. Guard covers every new entry and passes.
2. Exhaustiveness holds in both `en.rs` and the fixture locale.
3. The §6 scan test over the validator literals.
4. Rendered output semantically identical per §D6 rule 5 — the health surfaces
   and the burnout JSON both change shape internally without changing what a
   user sees.
5. Existing suites unchanged: `health_explainability`, `today_panel`,
   `auth_boundary` and `optimistic_lock` all exercise these paths.

## 9. Acceptance

1. No user-visible prose left in `peisear-core` or `peisear-storage`.
2. `BurnoutSignal.label` renders from the table.
3. The seven validator literals are scanned by test.
4. Guard passes; rendered output semantically identical.
5. fmt and clippy exit 0; suite counts unchanged apart from the new test.
6. Survey reported per 005a §4.1 — especially the storage count in §5.

## 10. Prohibited

No rewording — report instead. Do not convert `search.js`. Do not deepen
`translate_trigger_error`'s coupling to trigger text. Do not weaken the guard.
Do not change the burnout JSON's `code` field — it is a contract
(`FR-API-004`), like `error` and `entity_type`.

## 11. On completing 0.21.0

When this lands, **every user-visible string in the product is either in the
table or scanned by the guard**, with `static/search.js` the single named
exclusion.

Say that in the review request **only if it is true after your own survey**.
I18N-005e was invited to make the same claim and correctly declined, listing
four gaps instead — three of which are this handoff. If a fifth exists, saying
so is worth more than completing the release on schedule.
