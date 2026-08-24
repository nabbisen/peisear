# CONF-001 — The confirmation interstitial

**Issued by**: Architect
**Date**: 2026-08-16
**Priority**: P1 — first work of 0.25.0, **before any RFC 004 substep**
**Governing RFC**: [010](../../accepted/010-destructive-action-confirmation.md)
**Depends on**: nothing

---

## 1. Scope

Four destructive actions get a server-rendered confirmation interstitial. Five
others are untouched.

**In scope** — project delete, issue delete, planned-sprint delete,
completed-sprint delete. One shared component, four `GET` routes, four
originating controls changed from form-with-`onsubmit` to link.

**Out of scope** — the five reversible actions (leave team, remove member,
detach project, remove capacity row, silence all). RFC 010's open question 1 is
**settled: they keep their `confirm()` dialogs, unchanged.** Do not touch them,
and do not remove their `prose_scan` allowlist entries.

**Also out of scope** — the JavaScript enhancement in RFC 010 §D2. Settled:
later. The feature is complete without it, and shipping the correct path first
is the point.

## 2. Why this is first in 0.25.0

RFC 004 is a phase of JavaScript enhancement. This is the project's existing
JavaScript-enhancement defect, and both live under `DEC-021` — JavaScript only
as a named enhancement over a working path.

Right now the un-enhanced path is *more dangerous* than the enhanced one: with
JavaScript off, `onsubmit="return confirm('…')"` never runs and the delete
proceeds unconfirmed. Fixing that before building five substeps of new
enhancement is the whole reason for the ordering.

## 3. The shape

### 3.1 One component, four uses

Not four screens. One interstitial, parameterised with: what is being destroyed
(by name), what else goes with it, the confirm action, the cancel destination.

```
GET  /projects/{id}/delete
GET  /projects/{id}/issues/{issue_id}/delete
GET  /teams/{slug}/sprints/{sprint_id}/delete
```

The sprint route serves both the planned and completed cases — the difference is
copy, not structure, and the two existing dialogs differ only in wording.

**The four `POST` routes already exist. Their handlers do not change.** If you
find yourself editing one, stop and report: this handoff adds a path to them, it
does not alter what they do.

### 3.2 The originating control becomes a link

Today: a submit button inside a form carrying an `onsubmit` guard.
After: an ordinary `<a>` to the `GET`.

**This is the part that makes the fix structural.** A link cannot silently lose
its confirmation when JavaScript is absent, because the confirmation is a page.
Keeping a form and adding a fallback would leave the same shape that produced
the defect.

### 3.3 Cancel, and what not to accept

Cancel goes to the entity's **known parent**, derived server-side from the
route's own parameters: an issue's project detail page, a sprint's team sprint
list, a project's project list.

**Do not add a `?return_to=` parameter.** A caller-supplied redirect is an
open-redirect vector, and a "confirm deleting X" URL is more plausibly pasted
into a message than most routes in this app. The parent is derivable; accepting
it from the caller buys nothing and costs a vulnerability class.

If a cancel destination is genuinely not derivable for one of the four, stop and
report rather than reaching for a parameter.

### 3.4 Name the thing, and the cascade

Requirement 5: the interstitial names the specific entity, not "this item".

The project case must also say that its issues go with it — that is the one
cascade among the four, and the current dialog already says so
(`"Delete this project and all its issues? This cannot be undone."`). Do not
lose that in the move.

## 4. Authorisation — the thing most likely to be missed

Each new `GET` must carry **the same authorisation as its `POST`**.

Adding a read route beside a guarded write route is exactly where a check gets
forgotten, because the `POST` looks like the dangerous half. It is not: a `GET`
that renders "you are about to delete *Q3 Planning*" to someone with no access
to that sprint is a disclosure, and it is reachable by guessing an id.

Test 7 asserts this per route. Write it before the routes work.

## 5. `prose_scan` will fail, and that is the sequence

The nine `confirm()` strings are allowlisted in `prose_scan.rs`, each with the
reason that this decision was open. When the four dialogs go,
`every_allowlist_entry_still_matches_something` **fails** until their four
entries are removed.

That is QA-001's review doing the job it was added for. Remove exactly those
four; the other five dialogs still exist and their entries must stay.

**Do not pre-emptively empty the allowlist**, and do not adjust the test.

## 6. Copy

All interstitial copy through `peisear-i18n` — RFC 006 §D6, rule 7 included:
one key per sentence, composed in one `en.rs` arm, no `format!` assembling
prose at the call site.

`§1.7` applies. "Delete", "cannot be undone" and "permanently" are factual, not
failure-framed, and are fine. Do not add urgency, warning glyphs, or danger
colouring beyond what the existing state-badge vocabulary provides — a
confirmation screen is a statement of consequence, not an alarm.

## 7. Tests

New target `crates/peisear-web/tests/confirmation.rs`, with a CI job and a
`CONTRIBUTING.md` line.

| # | Check |
|---|---|
| 1 | `GET` on each of the four routes renders an interstitial naming the specific entity |
| 2 | The project interstitial states the cascade to its issues |
| 3 | **No-JS path**: following the link and posting the interstitial's form deletes the entity — no JavaScript anywhere in the sequence |
| 4 | **Regression guard**: none of the four originating controls carries an `onsubmit` confirmation any more, and none is a form. Written so it fails if one is reintroduced |
| 5 | Cancel targets the entity's parent; the route accepts no destination parameter (a `?return_to=` is ignored, not honoured) |
| 6 | The five reversible dialogs are untouched — still present, still `onsubmit` |
| 7 | **Authorisation**: for each new `GET`, a user who may not delete the entity may not see the interstitial. Expect what the corresponding `POST` already gives |

Test 4 is the one that keeps this fix from being undone by someone restoring a
"quick" inline guard. Test 7 is the one that would otherwise be skipped.

## 8. Escalate rather than deciding

- If a cancel destination is not derivable — §3.3.
- If a `POST` handler needs changing to make the `GET` work, stop. That would
  mean the split is wrong.
- If the sprint interstitial cannot serve both planned and completed without
  branching beyond copy, say so; two routes would be acceptable, but not
  unremarked.
- If `prose_scan` fails in any way other than the four expected allowlist
  entries, report it before adjusting anything.

## 9. Acceptance

1. All seven §7 tests pass; test 4 and test 7 written before the routes work.
2. Four `GET` routes, one shared component, four originating controls now links.
3. No `POST` handler changed.
4. Exactly four allowlist entries removed from `prose_scan.rs`; the other five
   intact and the test passing.
5. The five reversible actions unchanged.
6. All new copy through `peisear-i18n`; `prose_scan` and `test_harness_scan`
   pass.
7. fmt and clippy exit 0; `DEC-007` gate set green; three consecutive
   `cargo test --workspace` runs.

## 10. Prohibited

No `?return_to=` or any caller-supplied redirect. No change to the four `POST`
handlers. No change to the five reversible confirmations. No JavaScript
enhancement — that is settled as later. No undo. No danger colouring or urgency
copy beyond the existing vocabulary. Do not weaken or edit `prose_scan`'s tests.

## 11. Required review-request format

Workflow §9.2. Include test 4's and test 7's pre-implementation failing
transcripts, and state which cancel destination each of the four routes derives.
