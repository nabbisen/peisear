# I18N-005d — Today, inbox, settings, search

**Issued by**: Architect
**Date**: 2026-08-10
**Priority**: P1
**Depends on**: I18N-005a (pattern settled)
**Parallel with**: 005b, 005c, 005e

Pattern rules are in the queue README.

---

## 1. Scope

`components/me.rs` (~35), `settings.rs` (~21),
`notification_preferences.rs` (~19), `notifications.rs` (~11),
`search.rs` (~8), plus user-visible strings in the matching handlers.

**Validation messages are not in scope.** `AppError::Validation(...)` calls in
these handlers belong to **I18N-005e**, which owns validation copy across
`handlers/*` — they are a coherent class with a shared §1.7 failure-framing
risk, and converting them together lets one reviewer catch inconsistency across
the set. Other handler strings — flash messages, redirect copy — are yours.

*(Scope corrected 2026-08-10 after I18N-005b hit the same overlap; the original
wording was ambiguous between this handoff and 005e.)*


The largest group by string count, and the one whose copy carries the most
product weight.

## 2. `/today` is the surface the product is *about*

`me.rs` renders personal sustainability signals. §1.7 exists because of this
screen: it is where evaluative language would do real harm, and where the
product's claim not to judge is either true or not.

The copy here is careful and was written deliberately. **Convert it, do not
improve it.** If a sentence reads awkwardly to you, that is a finding to report.

Two things it already gets right and must keep:

- The subtitle states the dashboard is **"Visible only to you"**
  (`FR-PER-001`, `NFR-PRIV-001`). It is the screen's privacy claim.
- Panel and callout copy is descriptive, never directive. `FR-PER-006`'s
  callout is "what to read first", not "what to do".

`ISSUE-006` found two broken sentences on the health path by looking at prose
carefully during conversion. This screen is where the same attention is most
likely to pay off again.

## 3. Notification kinds are a closed set

Notification kind names and severity labels appear on both `/inbox` and the
preferences screen. Parameterise per rule 3.

**Severity is `Info` and `Watch` only** (`FR-NTF-003`). If you find a key or
branch implying a third, that is a finding — the same class of defect as
`ISSUE-006`, and worth escalating rather than converting faithfully.

## 4. Settings copy explains rules, and the explanations are load-bearing

`SCR-22` requires capacity-period overlap rejections to explain the overlap
**in text, not only visually**. That explanation is copy and must survive
conversion intact.

The WIP-limit section explains the default and its effect. Same.

## 5. Search

Blank query renders guidance rather than an error (external design §6 SCR-24).
That guidance is copy. The type-ahead is rendered by `static/search.js` from
JSON — **out of scope**; JS-side strings are not part of this workstream and
converting them would need a mechanism that does not exist.

Report any user-visible string you find in `search.js` rather than converting
it. That is a gap worth recording, not closing here.

## 6. Watch for

- **Empty states** across all five files (external design §5.4): they explain
  what an area is for without implying emptiness is a deficiency.
- **Pace and cycle-time figures** (`FR-PER-008`) carry an explicit caution
  against over-interpretation. That caution is copy; losing it is a requirement
  regression.
- **Relative timestamps** ("2 days ago") — if composed by concatenation, they
  violate rule 1 the same way `BackToLabel` did. Convert to a parameterised key.

## 7. Tests

Guard covers new entries; exhaustiveness holds; rendered output semantically
identical; `today_panel`, `search` and `auth_boundary` exercise these surfaces.

Add an assertion that `/today`'s "visible only to you" claim still renders — it
is a privacy statement, and a conversion that dropped it would be invisible to
every other test.

## 8. Acceptance

1. No user-visible literal left in the five components or their handlers.
2. Notification kinds and severities parameterised; no third severity exists.
3. The `/today` privacy subtitle asserted by test.
4. Guard passes; rendered output semantically identical.
5. fmt and clippy exit 0; suite counts unchanged.
6. Survey reported, including any `search.js` strings found and left.

## 9. Prohibited

Do not improve `/today`'s copy. Do not drop the over-interpretation caution. Do
not convert JavaScript strings. Do not introduce a third severity even if a
branch appears to want one.

## 10. Review focus to request

1. Any sentence on `/today` that read oddly during conversion — the
   `ISSUE-006` question, asked of this screen.
2. Relative-time composition: parameterised, or still concatenated?
3. What you found in `search.js`.
