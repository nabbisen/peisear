# I18N-003 — `peisear-notify` copy through the table

**Issued by**: Architect
**Date**: 2026-08-03
**Priority**: P1
**Governing RFC**: [006](../../done/006-i18n-architecture.md) — requirement 4
**Depends on**: I18N-001 (landed)
**Parallel with**: I18N-002 — notify's copy is its own; no ordering between them

---

## 1. Purpose

Route notification titles, bodies, and email copy through `peisear-i18n`.

RFC 006 requirement 4 names this explicitly: the guard must cover
**notification bodies and email copy, not only HTML**. Notifications are the
product's most sensitive copy — they arrive unbidden, in a tool whose thesis is
that it does not chase people.

## 2. Change scope

- `crates/peisear-notify/src/edge.rs` — notification titles and bodies
- `crates/peisear-notify/src/email.rs` — subject and body path
- `crates/peisear-i18n/` — new `MessageKey` variants
- `Cargo.toml` — `peisear-notify` gains `peisear-i18n`

Not `channel.rs`, `config.rs` or `dispatch.rs` unless they carry user-visible
strings — check, and report what you find.

## 3. The copy as it stands

Two notification kinds in `edge.rs`, each a title plus a parameterised body:

- `BURNOUT_OVERLOAD` — *"Sustained over-capacity streak"* / body carrying
  `current_streak_days`
- `BURNOUT_STALLED` — *"Long-stalled assigned work"* / body carrying
  `current_max_days`

Both bodies are already carefully non-evaluative — *"a description of the
recent rhythm, not an evaluation of your work"*, *"May be worth a glance"*.
**Preserve that.** This is a relocation, not a rewrite. If the guard rejects
anything here, it is far more likely the guard's term list needs examining
than that this copy is wrong; escalate rather than reword.

## 4. A defect to correct while you are here

Both bodies tell the reader to *"review at /me"* and *"Visit /me for context"*.

**`/me` was renamed to `/today` in 0.17.0** (`FR-NAV-002`). It still resolves —
a 308 redirect preserves it — so nothing is broken, but the canonical route is
`/today` and `FR-NAV-001` names Today as the entry point. User-facing copy
should not send people through a compatibility redirect.

Correct it to `/today` as part of the conversion. Small, and it is the kind of
staleness that only surfaces when copy is finally looked at as copy.

Report it in the review request rather than folding it in silently — it is a
copy change, not a mechanical move.

## 5. Email is outside the browser's escaping

The one place this handoff carries real risk.

`email.rs` composes a subject and plain-text body and hands them to
`mail-builder`. Unlike HTML rendering, **nothing escapes on the way out**.
Parameters reaching email copy are the same user-derived values that reach the
page — issue titles, display names.

Required:

- Confirm what actually reaches email parameters today. If only numbers do, say
  so and the risk is nil for now.
- If any user-controlled string can reach a subject or body, state it plainly
  and describe what happens to it. Header injection through a subject line
  (`\r\n`) is the failure mode worth naming specifically.
- Do not introduce a new path by which user text reaches email copy in this
  handoff.

This is RFC 006's "Security and privacy considerations" applied to the one
surface where the browser is not there to help.

## 6. Required tests

1. Every new key has a rendering — I18N-001's exhaustive match; confirm it
   holds.
2. The guard covers the new entries and passes. **This is the first time
   notification copy has ever been checked against §1.7.**
3. `peisear-notify`'s existing 3 tests pass unchanged.
4. Rendered title and body are unchanged, except `/me` → `/today`. Assert on
   rendered output.

Run `cargo test -p peisear-notify -- --test-threads=1`. The suite shares a
SQLite file and flakes under parallel execution — a known, recorded issue, not
something to fix here.

## 7. Acceptance criteria

1. No user-visible English literal remains in `edge.rs` or `email.rs`.
2. Guard covers the new entries and passes.
3. Rendered output unchanged apart from the `/me` correction.
4. The §5 email-parameter question is answered explicitly, either way.
5. fmt and clippy exit 0; suite counts unchanged.

## 8. Prohibited

- Do not reword the non-evaluative framing in the two bodies. Escalate if the
  guard objects.
- Do not add new user-controlled data to email copy.
- Do not "fix" the notify test flake here.
- Do not weaken the guard.

## 9. Evidence

- Changed-file list.
- Before/after rendered title and body for both kinds.
- Guard output over the new entries.
- The §5 answer, with how you determined it.
- Confirmation the three existing tests pass, single-threaded.

## 10. Review focus to request

1. The §5 email-parameter finding — the part I most want a second opinion on.
2. Whether any string in `channel.rs`, `config.rs` or `dispatch.rs` turned out
   to be user-visible.
3. The `/me` → `/today` correction, since it is a copy change rather than a
   move.

**Escalate rather than deciding** if the guard rejects existing notification
copy, or if user-controlled text turns out to reach email subjects — the second
is a security finding, not a conversion detail.
