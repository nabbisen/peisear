# A11Y-006 — the mobile completion audit

**Target release**: 0.33.0. **Governing RFC**: none — this closes a requirement
that has been unverified since Phase E.

## 1. What is being asked

> **`NFR-A11Y-006` — Mobile completion of key flows.** The following MUST be
> completable on a phone: reviewing and reading notifications; viewing `/today`;
> changing issue status from the issue detail; viewing today's calendar.
> *Status*: **Not verified — Phase E audit.**

**It is the last open limb of Definition of Done item 5** — the oldest condition
in that table, unmoved from 0.19.1 until 0.32.0. When this is answered, item 5
either reaches *"Met"* outright for the first time, or it does not and we know
exactly why.

## 2. The distinction the whole audit turns on

**Rendering is not completing.**

I checked these four flows informally with a browser and found they render and
operate at 390 × 844. **That is evidence the audit will pass. It is not the
audit**, and I am not asking anyone to confirm my impression.

**Drive each flow to its end and assert the outcome changed.** Not "the button is
present" — **tap it and prove the state moved**. A flow whose control is visible,
tappable, 44 px, and silently does nothing is exactly what `NFR-A11Y-006` exists
to catch, and it is what a presence check cannot see.

## 3. Method

**Touch, not mouse.** Use `Input.dispatchTouchEvent` with
`Emulation.setEmitTouchEventsForMouse` and `mobile: true` device metrics. **A
flow that works with a synthetic mouse click and not with a tap is a real
finding** — and a mouse-driven audit would never see it.

**Three viewports**, because "a phone" is a range: **320 × 568** (small),
**390 × 844** (typical), **414 × 896** (large). Report per flow per viewport.

**Fixtures with content.** More than one issue, more than one notification, a
long issue title. **`§10.21` was my own false finding because my fixtures were
empty**, and `LAYOUT-001` was invisible without a long email — this project has
been caught twice by fixtures that were thinner than reality.

**`browser-checks/cdp.mjs`** is the harness. It is tracked now.

## 4. The four flows, and what "completed" means for each

| Flow | Completed means |
|---|---|
| **Reviewing and reading notifications** (`/inbox`) | An unread notification is **read afterwards** — driven by tapping, and confirmed by re-fetching the page, not by the click returning |
| **Viewing `/today`** | The page's own content is **reachable**: every section can be scrolled to and read, nothing clipped, nothing requiring hover to reveal |
| **Changing issue status from the issue detail** | The issue's status **is different afterwards**, confirmed server-side. **Both paths** — with JavaScript (`dm.js`) and with JavaScript disabled (`DEC-021`'s native form). A phone user with a broken script is still a phone user |
| **Viewing today's calendar** (`/today/calendar`) | Content reachable, **and its navigation works** — previous/next reach a different day and the page reflects it |

## 5. Validate the method before trusting it

**Prove each check can fail.** Before reporting that a flow completes, break it
and confirm the check notices:

- **Notifications**: assert-read against a notification you never tapped → must
  report incomplete.
- **Status change**: assert the status moved after tapping a control you did not
  tap → must report incomplete.

**This is planting, applied to an audit.** It is here because I have just had a
finding of my own withdrawn — `§10.21` — for exactly this reason: **a check I had
not validated against a known positive, used to draw a conclusion.** A green
audit from an unvalidated method is worth nothing, and this requirement has been
unverified for thirteen releases; it deserves better than a fourth false reading.

## 6. What this is not

- **Not a fix.** If a flow does not complete, **stop and report it.** The fix is a
  separate handoff with its own scope. An audit that repairs what it finds stops
  being an audit.
- **Not a gate.** `BROWSER-001` is the only browser check in CI, deliberately and
  under `DEC-048`. **Do not add a CI job.** If the audit suggests one is
  warranted, that is a finding for a later decision.
- **Not the other flows.** The requirement names four. Others may well work; they
  are not what is being certified.

## 7. Exit condition

A report stating, **per flow per viewport, completed or not, with the evidence
that says so** — and for anything that did not complete, what stopped it.

**And one sentence I need**: whether `NFR-A11Y-006` is met. That sentence moves
Definition of Done item 5, and it is the last one it needs.

`DEC-007` clean at **256** — **this handoff adds no test.** If the count moves,
something was misunderstood.

---

**Who holds what**: dev team — the audit. **What's blocked**:
`NFR-A11Y-006`'s status and Definition of Done item 5, both mine, both waiting on
§7's sentence. **What's next**: review request.
