# I18N-002 — `peisear-core` emits message descriptors, not prose

**Issued by**: Architect
**Date**: 2026-08-03
**Priority**: P1 — closes compliance gap §10.8
**Governing RFC**: [006](../../done/006-i18n-architecture.md) §D3
**Depends on**: I18N-001 (landed)
**Blocks**: I18N-004a–e — the web surfaces render what this produces
**Parallel with**: I18N-003 (notify has its own copy)

---

## 1. Purpose

`peisear-core` currently returns finished English sentences. Make it return
**message descriptors** — a key plus typed parameters — and let presentation
render them.

This is the substantive half of RFC 006 and the largest single change in
0.21.0.

## 2. Why this matters beyond i18n

Two requirements are currently satisfied only in appearance:

- **`FR-HLT-009`** — "computation and presentation are separate concerns." The
  computation crate is writing user-facing prose. The 0.19.1 baseline recorded
  this as `Partial` for the wrong reason; §B4 of the amendments corrects it.
- **`NFR-MNT-001`** — metric computation as pure functions in the domain crate.
  A function that decides English wording is not purely computing a metric.

And the practical consequence: **the vocabulary guard cannot see these
sentences.** A guard covering only the web layer would have a hole exactly
where `FR-HLT-006` (explanation neutrality, P0) applies. That is why this
handoff exists at all rather than being deferred as tidying.

Recorded as compliance gap **§10.8**.

## 3. Change scope

- `crates/peisear-core/` — the four prose-producing functions and their
  supporting types
- `crates/peisear-i18n/` — new `MessageKey` variants for what core emits
- `crates/peisear-web/` — **forced call sites only**; the surfaces themselves
  are I18N-004
- `Cargo.toml` — `peisear-core` gains a `peisear-i18n` dependency

## 4. The four functions

Surveyed as of 0.20.0. If you find a fifth, report it before converting it.

| Function | `lib.rs` | Returns |
|---|---|---|
| `Indicator::human_explanation` | ~729 | `Option<String>` — the per-indicator sentence, `FR-HLT-005` |
| `format_value` | ~953 | `String` — an indicator's value as displayed |
| `summarize` (project health) | ~1184 | `String` |
| `summarize` (burnout) | ~1566 | `String` |

**`slugify` is not in scope** — it produces a URL slug, not prose.

## 5. Required implementation

### 5.1 The direction of dependency

`peisear-i18n` is a leaf (I18N-001). `peisear-core` gains a dependency **on
it**, not the reverse. Core emits `MessageKey` values; it never renders them.

RFC 006 §D2 considered the alternative — core returning plain `&str` keys with
the table in `-web` — and rejected it: nothing would then guarantee that a key
core emits has a rendering anywhere. The whole point is that a missing
rendering fails to compile.

### 5.2 Return descriptors

`human_explanation` returns `Option<MessageKey>` rather than `Option<String>`.
The others likewise return keys.

The sentences carry embedded values — *"3 issues haven't moved in over two
weeks"*. Those become **typed parameters on the key variant**, per RFC 006
requirement 7. A count is a number, not a pre-formatted string. If you find
yourself passing a `String` that was already formatted, the formatting belongs
on the presentation side.

### 5.3 What must not change

**Every explanation must still say what it said**, modulo wording the guard
rejects. This is a relocation of prose, not a rewrite of it. If a sentence has
to change to fit the key/parameter shape, say so and why — that is a finding,
not a detail.

Do not change any threshold, classifier, or normalisation. `FR-HLT-005`'s rule
stands: a sentence for non-healthy indicators, none for healthy or
insufficient.

### 5.4 Call sites

Changing core's public API forces `peisear-web` call sites to render. Update
them **minimally** — enough to compile and behave identically. Do not convert
surrounding copy; that is I18N-004 and mixing the two makes both unreviewable.

List the forced call sites separately from the core changes in the review
request.

## 6. Required tests

1. **Every key core emits has a rendering** — guaranteed by I18N-001's
   exhaustive match; confirm it still holds with the new variants.
2. **The guard covers the new entries.** These sentences are the ones
   `FR-HLT-006` is about. This is the moment that requirement becomes
   checkable for the first time.
3. **Rendered output is unchanged** where wording was not deliberately
   corrected. Assert on the rendered sentence, not on the key.
4. `health_explainability`'s existing 7 tests pass unchanged. If one needs
   editing, that is a signal the behaviour moved — report it.

## 7. Acceptance criteria

1. None of the four functions returns prose; all return descriptors.
2. `peisear-core` has no user-visible English string literal left in the
   converted paths.
3. The guard covers every new table entry and passes.
4. Rendered output is unchanged except where a guard rejection forced a
   correction — each such correction listed.
5. fmt and clippy exit 0; full suite unchanged counts.

## 8. Prohibited

- Do not convert web surfaces beyond forced call sites.
- Do not add a rendering path to `peisear-core`. It emits; it does not render.
- Do not weaken the guard to admit an existing sentence. A rejection means the
  sentence is wrong — report it and propose the correction.
- Do not change computation.

## 9. Evidence

- Changed-file list, core separated from forced call sites.
- Before/after rendered output for every converted sentence, side by side.
- The guard's output over the new entries.
- Any sentence the guard rejected, with the correction and its reason.

## 10. Review focus to request

1. Whether the key granularity matches I18N-001's grain — one variant per
   message, typed parameters for closed sets.
2. Any sentence whose meaning shifted to fit the descriptor shape.
3. Whether any remaining `String` return in core is genuinely not user-visible.

**Escalate rather than deciding** if a sentence cannot be expressed as key plus
typed parameters without losing meaning, or if the guard rejects an existing
explanation — that is a `FR-HLT-006` finding and its correction is a wording
decision, not an implementation one.
