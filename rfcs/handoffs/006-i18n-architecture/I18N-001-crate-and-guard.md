# I18N-001 — The `peisear-i18n` crate and the vocabulary guard

**Issued by**: Architect
**Date**: 2026-08-03
**Priority**: P1 — closes two standing P0 verification gaps
**Governing RFC**: [006](../../accepted/006-i18n-architecture.md)
**Depends on**: nothing. First unit of 0.21.0.
**Blocks**: I18N-002, I18N-003, I18N-004a–e — all of them.

---

## 1. Purpose

Build the mechanism, prove it works, and wire the guard into CI. No surface
conversion in this handoff.

The deliverable is a crate that can hold every user-visible string, a
compile-time guarantee that no key lacks a rendering, and a **blocking** test
asserting no table entry contains prohibited §1.7 vocabulary.

## 2. Why this comes first, alone

Handoffs 2–4 convert real surfaces against this crate's API. Writing them
before the API exists would be guessing. And converting surfaces before the
guard exists would mean converting them twice — once to move the strings, once
to fix what the guard then finds.

## 3. Change scope

- `crates/peisear-i18n/` — new workspace member
- `Cargo.toml` — add it to `[workspace] members` and `[workspace.dependencies]`
- `.github/workflows/test.yml` — a CI job for the guard

**Nothing else.** No existing crate is modified. No surface is converted. If
you find yourself editing `peisear-web`, stop — that is handoff 4.

## 4. Required implementation

### 4.1 The crate

`peisear-i18n`, a **leaf crate with no workspace dependencies**. `-core`,
`-notify` and `-web` will depend on it; it depends on none of them. This is
what lets the domain crate emit keys in I18N-002 without a cycle.

Follow the workspace conventions: edition 2024 inherited, `version.workspace`,
`rust-version.workspace`, 2018+ module style (`foo.rs` beside `foo/`, no
`mod.rs`), tests separated from implementation per `NFR-MNT-005`.

### 4.2 `Locale`

An enum. **English only** ships (`NFR-LANG-005`; `DEC-022`). Do not add
Japanese — see §4.5 for how the design gets validated without it.

### 4.3 The message key and table

A key **enum**, not string constants. Rendering is an exhaustive `match` per
locale, so a key without a rendering **fails to compile**. That is RFC 006
requirement 2 and it is the reason for the enum: a `HashMap<&str, &str>` would
turn a missing key into a runtime fallback, which is how untranslated keys leak
into production interfaces.

Seed it with a small, real set — enough to exercise the mechanism honestly.
Suggested: a handful of `AppError` and validation messages, since handoff 4e
converts those and they are the shortest, least ambiguous copy in the system.
Do **not** attempt breadth here.

**Parameters are typed, not positional.** A message needing an issue title
takes a typed parameter, not `{0}`. RFC 006 requirement 7.

**No concatenation.** Composing user-visible sentences from fragments is
prohibited — the guard cannot see through it, and it is how a prohibited word
gets assembled at runtime from innocent parts.

### 4.4 The guard — the point of the whole handoff

A test that walks **every entry of every locale table** and asserts no
prohibited §1.7 term appears.

The prohibited set is §1.7 **in full**, not the mockup's five-word subset. From
the requirements baseline §1.7: evaluative phrasing ("good progress", "bad
pace", "performance is increasing/decreasing"), judgement ("concerning trend",
"underperforming", "failing to meet"), directives ("you should", "you must"),
`velocity`, ranking vocabulary ("ranking", "top performer"), completion-rate
emphasis, and failure framing ("Failed to", "Error:").

Read §1.7 from
`.git-exclude/specs/peisear-0.20.0-requirements-en.md` and transcribe it
completely. If a term is ambiguous to encode, **report it rather than
dropping it**.

Matching is **case-insensitive and word-boundary aware**. `"velocity"` must
match `"Velocity"`; it must not match a substring inside an unrelated word.

The guard **blocks CI** from this release (RFC 006 open question 3, default
standing). An advisory guard on a P0 requirement is a guard nobody fixes.

### 4.5 Prove the mechanism is not English-shaped

Add a **fixture locale** in test code — not shipped, not `Locale`-visible in
production — with deliberately distinct values, and assert that rendering
switches wholesale.

This is RFC 006 open question 4's default: it validates the design without
committing to a Japanese table that would then drift unmaintained. It costs
almost nothing and it catches the failure where a "locale" system quietly hard-
codes English somewhere in the middle.

### 4.6 CI

A job for the guard, following the existing per-crate pattern (`DEC-007`).
A test crate without a CI job does not exist.

## 5. Required tests

1. **Exhaustiveness** — a key with no rendering fails to compile. Demonstrate
   how this is guaranteed; if it is by exhaustive `match`, say so and show it.
2. **The guard itself** — over every entry of every table, including the
   fixture locale.
3. **Guard efficacy** — a test proving the guard *catches* a violation. Add a
   deliberately prohibited entry to a test-only table and assert the guard
   rejects it. **A guard never observed failing is not known to work.**
4. **Locale switching** — the fixture locale renders differently, wholesale.
5. **No key leakage** — no rendered output contains a raw key-shaped literal.

Test 3 is the one most likely to be skipped and the one that matters most. This
release exists because a gate that had never passed was recorded as passing.

## 6. Acceptance criteria

1. `peisear-i18n` builds as a workspace member; nothing else is modified.
2. A missing rendering is a compile error, demonstrated.
3. The guard runs in CI as a **blocking** job and passes.
4. The guard is demonstrated to fail on a planted violation.
5. The §1.7 prohibited set is transcribed in full; anything omitted is reported.
6. `cargo fmt --all -- --check` and `cargo clippy --workspace --all-targets --
   -D warnings` both exit 0 — the 0.20.0 state must not regress.
7. Full per-crate suite passes with unchanged counts.

## 7. Prohibited

- Do not convert any surface. That is handoffs 2–4.
- Do not add a second shipping locale.
- Do not use a runtime map keyed by string.
- Do not weaken the guard to make a seeded entry pass. If a real message needs
  a prohibited word, that is a finding to report — the message is wrong, not
  the guard.
- Do not add `#[allow]` to silence anything.

## 8. Required evidence

- Changed-file list.
- fmt and clippy output, both exit 0.
- Full test output including tests 1–5.
- **The planted-violation run**, showing the guard failing as intended, and the
  run after removing it.
- The §1.7 transcription, with anything you could not encode called out.

## 9. Review focus to request

1. Whether the key enum's granularity is right — one variant per message, or
   grouped by surface.
2. The §1.7 transcription: completeness and the word-boundary handling.
3. Whether the seeded message set is representative enough to have exercised
   the mechanism, or too thin to have proved anything.

**Escalate rather than deciding** if the exhaustiveness guarantee cannot be
achieved with an enum as RFC 006 assumes, or if a §1.7 term resists encoding.
