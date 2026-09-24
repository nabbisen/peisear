# SPRINT-005 — the privacy gate reads live data over a captured record

**Issued by**: Architect
**Date**: 2026-09-25
**Target release**: 0.39.0
**Governing RFC**: [0013](../../accepted/013-the-sprint-record.md), `DEC-054`.
**Related requirement**: **`NFR-PRIV-007`** — *aggregates must not be
reversible to individuals*.
**Source**: found by the dev team in `SPRINT-004`'s review request §6, reported
as a display gate and left alone; ruled a privacy defect in
`.git-exclude/reviewed/SPRINT-004-review.md` §3.
**Depends on**: `SPRINT-004`, shipped.

Three items: **§1 is the privacy defect**; §2 and §3 are small and were ruled
in the same review.

---

## 1. The gate decides from data that keeps moving

`sprints::distinct_contributors` reads:

```sql
FROM sprint_issues si JOIN issues i ON i.id = si.issue_id
WHERE si.sprint_id IN (…) AND i.status = 'done'
```

**Live membership, live status.** It decides whether a sprint's burndown
trajectory (sprint detail) and the velocity aggregate (team sprints page) are
shown at all — `NFR-PRIV-007`'s two-contributor floor.

After `SPRINT-004`, **what is displayed for a completed sprint is the captured
record.** Whether that record is reversible to one person is a property of who
contributed to it, and that was fixed at completion. The gate is still asking
about now.

**It can open, not only close.** Membership of a completed sprint can only
shrink, but status can rise: an issue left in a completed sprint and finished
later raises the count. **A sprint that had one contributor at completion —
trajectory correctly hidden — can cross to two and begin disclosing a
per-person trajectory that nobody decided to disclose.** That is the
disclosure `NFR-PRIV-007` exists to prevent, arriving by itself.

## 2. The fix

**Capture the gate's basis with the record, and read it for completed
sprints.**

**2.1 — Two columns on `sprint_records`** (migration `0020`; do not edit
`0019`, it is applied history):

- `contributor_count` — `COUNT(DISTINCT i.assignee_id)` among done issues at
  capture, the query's existing `known`;
- `had_unassigned_contributor` — the existing `unassigned > 0`, which today
  returns `None` and suppresses.

**Store the basis, not the decision.** The two-contributor floor is policy and
belongs in code where it can change; a stored verdict would freeze today's
policy into old rows.

**2.2 — Backfill both columns in `0020`** for existing records, from the same
live computation `0019` used, with the same honesty in the header: it is
correct as of the migration, not as of completion, for any sprint that has
already drifted. **The backfill is what stops a mixed estate**, which is
`0019`'s reasoning.

**2.3 — The single-sprint call reads the captured columns** when the sprint is
`completed`, and computes live otherwise. Same shape as `summary` and
`burndown`.

**2.4 — The multi-sprint call is the part that needs care.** The velocity
caller passes several sprint ids and `COUNT(DISTINCT assignee_id)` **across**
them is not the sum, the max, or any function of per-sprint counts — one person
may have contributed to all of them.

**Take the conservative reading: the set of completed sprints passes the gate
only if at least one of them does on its own** (`max(contributor_count) >= 2`,
and no member had `had_unassigned_contributor`). It may suppress where today's
live union would show.

**That is the right direction and it is not only caution.** The case it
newly suppresses is several sprints each with a single, different contributor —
where the union is two people but **each sprint's number is one person's
output**, so the aggregate is reversible per sprint. Today's gate shows that.
**Suppressing it is a second hole closed, not a cost.** Say in the review
request how many sprints in a realistic fixture change visibility.

**Do not store contributor identities** to make the union exact. Copying
person-identifying rows into a second table to decide a privacy gate is the
wrong trade, and the conservative reading is sound without it.

## 3. Also ruled in the same review — small

**3.1 — `SprintReopenedFlash`.** `start` and `complete` both flash on success;
reopen should. You left it out because it would exceed a string count I wrote;
**that count was an estimate, not a budget**, and consistency with the two
sibling actions is the requirement.

**3.2 — The overflow gate's fixture carries a completed sprint, permanently.**
You found the fixture's sprint is planned, so the gate never renders *Reopen
sprint* or `SPRINT-004`'s two headings, and covered it with a scratch variant.
**That is `§10.27`**: a gate that cannot reach a surface reports 90/90 about
something else. Make it permanent — a completed sprint with a captured record
in the fixture — so the next control on that page is covered by default.

Expect the cell count to stay **90** (18 pages × 5 widths) unless a page is
added; **if it changes, say why before assuming it is fine.**

## 4. Verification

- **The disclosure, demonstrated**: a completed sprint with **one** contributor
  among its done issues — trajectory absent — then finish another member's
  issue **in place**. Before: the trajectory **appears**. After: it stays
  absent. Report both runs; the before is the defect.
- **The reverse still works**: a sprint completed with two contributors keeps
  its trajectory when an unfinished issue is carried out afterwards.
- **Reopen re-derives**: after reopen the gate is live again, and completing
  recaptures the columns.
- **Velocity**: a fixture where the live union is two and no single sprint has
  two — assert it is now suppressed, and say how many sprints that is.
- **`had_unassigned_contributor`** suppresses as the live `unassigned > 0`
  does today.
- **Backfill**: the 0.38.0-built database from `SPRINT-004` §3, migrated
  through `0020`; every record has both columns; **a page-text diff against the
  `SPRINT-004` tree** — the only expected differences are the flash key and
  any sprint whose visibility §2.4 deliberately changes, each named.
- `DEC-007`: report the new count, last recorded **340**.
- Three consecutive workspace runs; `fmt`; `clippy`; **the overflow gate with
  the amended fixture**.

## 5. Escalate rather than deciding

- **If the conservative reading suppresses a case you think a user will miss.**
  Report the case; do not soften the gate.
- **If `distinct_contributors` has a third caller** I have not found. Two were
  in `handlers/sprints.rs`.
- **If capturing the basis turns out to need the identities after all.** Stop —
  that is a decision above this handoff.
- **If the gate's fixture change moves the cell count** (§3.2).

## 6. Exit condition

A completed sprint's trajectory is shown or hidden on the basis that existed
when it completed, and later work cannot open it; the velocity aggregate
suppresses unless a single sprint clears the floor; the flash matches its
siblings; the gate's fixture reaches the completed-sprint page.
