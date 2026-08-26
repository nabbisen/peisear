# QA-016 — Aggregate inferability: the two charts, not the chip

**Issued by**: Architect
**Date**: 2026-08-26
**Priority**: P2 — `NFR-PRIV-007` is a **SHOULD**, and this is an audit
**Governing RFC**: [005](../../proposed/005-quality-consolidation.md) §7
**Depends on**: nothing.

**Audit and report. Change nothing.** Same shape as `QA-012`: the measurement
comes back, the decision is the owner's, and a separate handoff acts on it if
they choose to.

---

## 1. Read RFC 005 §7 first — it was rewritten today, and the reason matters

The original proposed suppressing the **workload chip** at N < 2 and applying
the same rule to the sprint plan's capacity hint and a per-assignee rollup.

**Do not implement any of that.** All three are settled, and one of them was
settled by doing precisely what §7 proposed and then undoing it:

- The workload chip's N < 2 suppression **shipped in `DEV-003` and was
  withdrawn at 0.20.0** as a misapplication — a chip bearing a person's name is
  not an aggregate, it is individual workload under `NFR-PRIV-002`. Building
  §7 as written re-introduces a removed defect.
- The **capacity hint was withdrawn at 0.22.0**, the product's first genuine
  `NFR-PRIV-007` case. There is no capacity code in `sprint_plan.rs`.
- The **per-assignee mini-rollup** never landed.

**Confirm all three before going further.** If any turns out not to be settled,
stop and report — this handoff's whole scope rests on it.

## 2. What to audit

Two aggregates that did not exist when §7 was written:

- **The sprint burndown** — `sprints.rs:641`, `render_burndown`, plotting
  `cumulative_committed` and `cumulative_completed` per day.
- **The team velocity chart** — a median across recent completed sprints.

For each, answer three questions and show your working:

### 2.1 Can it resolve to one person?

A burndown for a sprint with one contributor is that contributor's series. **How
would you know?** `BurndownPoint` is `{ day, cumulative_committed,
cumulative_completed }` — it carries no contributor count, and I found nothing
plumbing one through. Confirm that, and say what it would take: is the count
reachable at the render site, or does it need a new query?

Same for velocity: a one-person team's velocity history is one person's
throughput history.

### 2.2 What does it disclose that no other surface does?

**This is the question that decides whether anything needs doing**, and it is
where the workload chip's answer differed.

I checked one thing and it is the crux: **`issue_events` — where completion
timestamps live — is referenced nowhere in `peisear-web`.** No screen exposes
when an issue was finished. The issue list shows *what* is done; the burndown
shows *when*, day by day.

**Verify that independently.** Grep the whole web crate, not just handlers. If
some screen does expose completion timing, the finding collapses and I want to
know before anything moves.

Then, for each chart, state plainly: what could a viewer learn from it that
they could not assemble from the issue list, the project detail, and the sprint
detail?

### 2.3 Who can see it?

A burndown on a team sprint is visible to team members. Establish the actual
audience for each chart — including whether a `viewer`-role member sees it.
`RFC 009` gave `viewer` read access; whether that extends here is a fact, not
an assumption.

## 3. What not to do

- **No suppression.** Not a threshold, not a tooltip, not a flag. `QA-012`'s
  shape: the table comes back first.
- **No new query, no plumbing of a contributor count.** §2.1 asks what it
  *would* take, not for it to be built.
- **No change to `NFR-PRIV-007`.** It is a `SHOULD` at P2 and it stands.
- **No touching the workload chip.** It was settled twice; a third pass at it
  is how a reverted decision comes back.

## 4. What I want in the report that is not a fact

For each chart, describe **what a one-contributor instance actually looks
like** — how many points, what shape, how legible. A burndown of a solo sprint
may be two points and a straight line, in which case "work-rate profile"
overstates it and the finding should shrink. Or it may be a fourteen-point
daily series, in which case it does not.

I have not looked, and the answer changes what this is worth.

## 5. Escalate rather than deciding

- **If any of §1's three surfaces is not settled**, stop.
- **If `issue_events` turns out to be exposed somewhere**, stop and report —
  §2.2's crux would be wrong.
- If either chart already has suppression logic I missed, stop.
- If the audience in §2.3 turns out to include someone outside the team, that
  is a different and larger finding — report it separately and do not fold it
  into the inferability question.

## 6. Acceptance

1. §1's three settled surfaces confirmed settled, from the code.
2. §2's three questions answered for both charts, with the `issue_events` check
   re-run independently.
3. §4's shape description for a one-contributor instance of each.
4. Nothing changed — `git status --short` empty at the end.
5. fmt and clippy exit 0; three consecutive `cargo test --workspace` runs
   (unchanged count expected).

## 7. Required review-request format

Workflow §9.2. §2.2's answer as prose per chart — it is the part the decision
turns on. Say plainly if the finding shrinks.
