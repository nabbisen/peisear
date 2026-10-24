# SPEC-004 — the table re-derived

**0.57.0, added after `SPEC-002`.** Written for one of three possible
answers to a question that has been open since 0.43.0. **Do not start
it until the series README's State column says that answer arrived.**

`SPEC-002` built rule (b) — Appendix A's cells against the 168 real
entries — and it is **red in nine areas, twenty cells**. The table is
stale; the rule is right. This round makes the table true.

---

## §1 — the question this round is conditional on

`requirements.md:7409`, written at 0.43.0 and never answered:

> *"Whether this table should be kept, derived from the entries by a
> scan, or dropped is the owner's decision."*

**This round is written for "keep it."** If the answer is *derive it*
or *drop it*, this handoff is replaced rather than amended — the work
is different in each case, and the architect writes the replacement.

## §2 — what is already known, so none of it is rediscovered

`SPEC-002` measured all of this. Do not re-derive the *finding*; it
is the *table* that needs re-deriving.

| Area | Real entries say | Appendix A says |
|---|---|---|
| Sub-issues | Implemented 10 | 9; Partial 0 vs 1 |
| Sprints | Implemented 4 | 3; Specified 0 vs 1 |
| Sprint planning | Implemented 4 | 0; Specified 1 vs 5 |
| Health | Implemented 8 | 5; Partial 1 vs 2; Divergent 0 vs 2 |
| Notifications | Implemented 9 | 6; Partial 0 vs 1; Specified 0 vs 2 |
| Calendar | Implemented 8 | 1; Specified 0 vs 7 |
| Privacy | Implemented 8 | 6; Divergent 0 vs 2 |
| Accessibility | Deferred 0 vs 1; Divergent 3 vs 2 |
| Language | Implemented 2 | 1; Partial 1 vs 2 |

**Why each is stale, already traced**: `requirements.md:7403` records
that the 0.43.0 sweep re-derived four rows and left *"the other
fifteen … rather than re-stated with more confidence than they have
earned."* Calendar, Sprint planning, Health, Notifications,
Sub-issues, Sprints and Language have **never** been re-derived.
Privacy and Accessibility were, at 0.43.0, and drifted after.

**The architect confirmed two of the nine independently** — Calendar
(all eight entries `Met`/`Implemented`, five carrying *"recorded
Specified until 0.40.0; `REQ-001`"*) and Privacy (all eight
`Implemented`/`Met`, `NFR-PRIV-005` reading `Met at 0.43.0`). The
other seven are the round's to verify as it rewrites them.

## §3 — two entries to fix first, because the rule reads them wrong today

**1. `NFR-A11Y-009` → `Deferred`.** Its status reads `Not implemented
— Phase E.`, whose leading token maps to Divergent, while
`requirements.md:7408` says in the document's own words *"`NFR-A11Y-009`
is the Deferred one."*

**Fix the entry, not the rule.** A per-entry exception inside
`status_column` would be `§10.39` — a rule keyed on a string — and
this project has paid for that. When a rule and ambiguous prose
disagree and the document elsewhere states the intent, the prose is
what is wrong. One word.

**2. `FR-CAL-001`'s correction note is stale, and the tree says so.**
The entry reads `*Status*: **Met**` with a later
`*Correction (0.40.0, REQ-001)*: read Specified (RFC 002, target
0.21.0).` **Settled against the tree, not by re-reading it**:
`/today/calendar` is a real route (`app.rs:150`, handler at
`handlers/calendar.rs`) and the entry's own named test
`personal_calendar_renders_only_my_issues`
(`tests/calendar_surfaces.rs`) **passes**.

So the entry is `Met` and the correction note is the stale thing. Its
four siblings went Specified → `Met` at the same `REQ-001`; this one
reads as the only one that went the other way, and it did not.
**Withdraw the note as superseded, with its date and the reason** —
do not delete it silently; `§10.47`'s convention is to leave the
original and record the correction beneath it.

## §4 — what re-deriving means, and what it must not become

**Count the entries; write what you counted.** For each of the nine
areas: every entry with that ID segment, its governing status (the
last `*Status*:` in its own span — `SPEC-002`'s decision), mapped
through `status_column`, summed per column.

**The entries are the source of truth. The table is a summary of
them.** So:

- **Never change an entry's status to make a cell match.** If an
  entry's status looks wrong, that is a finding about the
  requirement, and it stops the round (§6).
- **Change only the table**, plus §3's two entries, whose statuses are
  wrong *as prose* for reasons already established.
- **Recompute both grand-total rows from their own member rows**, not
  by adding the deltas. Appendix A has been *"wrong in two
  compensating cells, so the total looked right"* before — that is
  `§10`'s own first row.

**Record what you did, where it will be read.** The 0.43.0 note names
which rows were re-derived and when, and that note is the only reason
this round could tell stale from fresh. **Extend it**: name the nine
areas and this round, so the next person inherits the same evidence.

## §5 — gates

- **Rule (b) green on the real tree**, and its known-violations test
  replaced by a clean assertion — the same flip `SPEC-001` round 2
  made for rule 1.
- Rule (a) still green; both grand totals recomputed from their rows
  and stated in the round's report.
- **The population assertion still holds**: 168 entries, 25 area
  segments. If this round changes either, something is wrong.
- `DEC-007` delta stated — a test changing shape may not change the
  count; say which.
- `docs_scan`, `changelog_scan`, `mdbook` + linkcheck, `fmt`,
  `clippy` both ways, `rustdoc`, `publish-dry-run`, CI.

## §6 — escalate rather than deciding

- **If any entry's own status looks wrong** — not its spelling, its
  substance. That is a requirement question and it is the
  architect's, with the entry and the evidence.
- **If an area's count cannot be derived unambiguously** because an
  entry's governing status is genuinely unreadable, the way
  `FR-CAL-001`'s was. **Settle it against the tree if the tree can
  settle it** — a route, a passing test, a shipped migration — and
  escalate only what the tree cannot answer.
- **If a tenth area disagrees** that `SPEC-002` did not name. That
  means its violation set was bounded where it was thought total;
  report the full list.

## §7 — exit condition

The nine areas re-derived from their own entries and the table
rewritten to match; both grand totals recomputed from their member
rows rather than adjusted; `NFR-A11Y-009` reading `Deferred` and
`FR-CAL-001`'s correction note withdrawn as superseded with its
reason recorded beneath the original; **no entry's status changed to
make a cell match**; rule (b) flipped from its known-violations
assertion to clean; rule (a) and the population assertion still
green; the 0.43.0 re-derivation note extended to name these nine
areas and this round; `DEC-007` reconciled; all gates re-run after
the last edit.
