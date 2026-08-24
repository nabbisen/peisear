# RFC 0010: Destructive-action confirmation

**Status**: Proposed
**Target**: 0.25.0 — **before RFC 004's first substep**
**Related spec sections**: §7 rule 4 (destructive actions are confirmed), §5.8
(behaviour without JavaScript)
**Related requirements**: `DEC-021`
**Governing gap**: external design `§17.4`
**Last updated**: 2026-08-16

## Summary

Nine destructive actions confirm via `onsubmit="return confirm('…')"`. Without
JavaScript the handler never runs and **the action proceeds with no confirmation
at all** — so the un-enhanced path is more dangerous than the enhanced one,
which is the exact inversion `DEC-021` forbids.

This RFC replaces that with a server-rendered interstitial for the actions that
destroy data, keeps `confirm()` only as an enhancement that skips a round trip,
and — the part that shrinks the work — establishes that **five of the nine do
not need an interstitial at all**, because they are reversible through the
interface.

**Why before RFC 004.** RFC 004 is a phase of JavaScript enhancement. This is
the project's existing JavaScript-enhancement defect, under the same rule.
Opening the phase with the defect unresolved risks building more of its shape at
five times the surface area.

## Background

The nine sites, as shipped at 0.24.0:

| Action | Site | Reversible through the UI? |
|---|---|---|
| Delete project (cascades to issues) | `components/projects.rs:232` | **No** |
| Delete issue | `components/issues.rs:1478` | **No** |
| Delete planned sprint | `components/sprints.rs:496` | **No** |
| Delete completed sprint | `components/sprints.rs:520` | **No** |
| Remove capacity row | `components/settings.rs:320` | Yes — recreate it |
| Leave team | `components/teams.rs:470` | Yes — an admin re-adds you |
| Remove team member | `components/teams.rs:483` | Yes — re-add them |
| Detach project from team | `components/teams.rs:373` | Yes — re-attach it |
| Silence all notification kinds | `components/notification_preferences.rs:95` | **Yes, since 0.24.0** |

That last row changed underneath this gap. `§17.4` was recorded at 0.21.0, when
silencing everything was a one-way trip through a settings page. RFC 003 shipped
the resume banner at 0.24.0, so silence-all is now visibly and trivially
undoable from the inbox — and a confirmation dialog guarding a one-click undo is
friction, not protection.

## Requirements

1. **Every action that destroys unreconstructable data is confirmed on a path
   that does not depend on JavaScript.** The four `No` rows above.
2. **`confirm()` may remain, only as an enhancement** that reaches the same
   outcome in fewer round trips — never as the only confirmation.
3. **No confirmation is required for the five reversible actions.** Whether they
   keep one is open question 1.
4. **Cancel returns the user where they were** without relying on JavaScript
   history, and without accepting a caller-supplied destination.
5. **The interstitial names what will be destroyed**, specifically — not "this
   item" — including the cascade where one exists.
6. All copy through `peisear-i18n` (RFC 006 §D6, rule 7 included).

## Design

### D1 — One interstitial, four call sites

A single shared component, parameterised: heading, the specific thing being
destroyed, what else goes with it, a confirm button, a cancel link. Not four
screens — four uses of one screen.

```
GET  /projects/{id}/delete            → interstitial
POST /projects/{id}/delete            → performs it (route exists today)
```

Same shape for the issue and the two sprint deletes. The `GET` is additive; the
`POST` routes already exist and their handlers do not change.

**The originating control becomes a link, not a form.** Today it is a submit
button inside a form with an `onsubmit` guard. It becomes an ordinary `<a>` to
the `GET`. That is what makes the no-JS path correct by construction rather
than by remembering to guard it.

### D2 — Where the enhancement lives

With JavaScript, a small named enhancement may intercept the link, show
`confirm()`, and POST directly — one round trip instead of two.

**It must not be attached to the interstitial's own form.** A `confirm()` there
asks the user to confirm a screen they reached in order to confirm, which is the
kind of thing that survives review because everyone reads it as "extra safety".

Per `DEC-021` the enhancement is named, keyboard-equivalent, and optional. If it
is not written at all, the feature is complete without it.

### D3 — Cancel, and why not `return_to`

Cancel targets the **known parent** of the entity — the project's detail page for
an issue, the team's sprint list for a sprint, and so on. Derived server-side
from the route's own parameters.

**Not a `?return_to=` parameter.** A caller-supplied redirect destination is an
open-redirect vector, and this is a route reached from an email-shaped link
("confirm deleting X") more plausibly than most. The parent is derivable, so
there is nothing to gain by accepting it from the caller.

### D4 — `prose_scan`'s allowlist cleans itself

The nine `confirm()` strings are allowlisted in `prose_scan.rs`, each with the
reason that this decision was open. When four of them go,
`every_allowlist_entry_still_matches_something` **fails** until their entries are
removed — the test QA-001's review asked for, doing the job it was added for.

That is the intended sequence, not an obstacle: the guard notices the cleanup is
owed. Do not pre-emptively delete entries for dialogs that still exist.

## Test plan

| # | Check |
|---|---|
| 1 | `GET` on each of the four delete routes renders an interstitial naming the specific entity |
| 2 | The project interstitial names the cascade — that its issues go too |
| 3 | **The no-JS path**: following the link and posting the interstitial's form deletes; no JavaScript involved anywhere in the sequence |
| 4 | **Regression guard**: no `onsubmit`-only confirmation remains on any of the four. Written so it fails if one is reintroduced |
| 5 | Cancel returns to the entity's parent, and the interstitial accepts no destination parameter |
| 6 | `prose_scan` passes, with the four allowlist entries removed and the remaining five intact |
| 7 | Authorisation on the new `GET` matches the existing `POST` — a user who may not delete may not see the interstitial either |

Test 7 is the one most likely to be skipped: adding a `GET` beside a guarded
`POST` is exactly where an authorisation check gets forgotten, because the
`POST` looks like the dangerous half.

## Security and privacy considerations

- **The new `GET` routes are a read surface on the entity.** They must carry the
  same authorisation as their `POST`. Test 7.
- **No caller-supplied redirect** — D3.
- The interstitial names an entity's title, which the viewer can already read;
  no new disclosure.

## Out of scope

Undo. A confirmation is not a substitute for reversibility, but building undo is
a different and much larger design. Bulk delete. Any change to what the four
`POST` handlers do.

## Open questions

1. **Do the five reversible actions keep `confirm()` at all?** Requirement 3
   says they need no interstitial. Keeping a JS-only dialog on them is
   defensible — it is a speed bump, and its absence without JavaScript costs
   nothing that cannot be undone. Removing it is also defensible: a dialog that
   appears for some users and not others, guarding an action that is
   recoverable either way, is inconsistency without protection.
   *Default-if-no-decision: keep them, unchanged.* **Owner's call** — it is a
   product feel question, not a correctness one.
2. **Silence-all specifically.** It is now recoverable from the inbox banner, so
   its dialog is the weakest of the five. Fold into question 1 or decide
   separately.
3. **Does the enhancement in D2 get written in 0.25.0, or later?** The feature
   is complete without it. *Default: later — ship the correct path first, and
   let RFC 004's substeps carry the enhancement if they want it.*

## References

- External design `§17.4`, `§7` rule 4, `§5.8`
- `DEC-021` — JavaScript as named progressive enhancement only
- `prose_scan.rs`'s allowlist, and QA-001's
  `every_allowlist_entry_still_matches_something`
- RFC 003 / `INBOX-001` — the resume banner that made silence-all reversible
