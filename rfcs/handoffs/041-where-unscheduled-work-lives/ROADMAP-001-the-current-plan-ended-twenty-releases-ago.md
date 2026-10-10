# ROADMAP-001 — the current plan ended twenty releases ago

**A decision handoff. The deliverable is a proposal and one edit, not a
rewrite.** **Target**: 0.48.0.

`ROADMAP.md` says, in its own words:

> *"**The current plan is the Milestones, Release sequence, and Release cycle
> sections below**, agreed 2026-08-01. The phase narrative that follows
> immediately is retained as history."*

**That current plan's release sequence ends at `0.27.0` and its milestones
end at `M5`. We released `0.47.0` yesterday.** Twenty minor versions past the
end of the part the document explicitly labels current — and the file's last
commit is `908c539`, **2026-09-24**, seven releases ago.

**This is not a request to update a stale file.** The document distinguishes
*history* from *current plan* carefully; what has happened is that **the way
work is chosen changed** — releases are now driven by handoffs, the `§10.x`
register, and the specification sweep — and no one moved the roadmap to
match.

---

## §0 — the symptom that found it, which is the real argument

The architect has been reciting the unscheduled-work list **from memory**
each time the owner asks what is next. Two things that produced:

- **One item did not exist.** *"`FR-DM-005`'s keyboard undo (RFC-sized)"* was
  carried across many releases. **`FR-DM-005` is *Conflict message
  vocabulary*, and it is `Implemented`.** There is **no keyboard-undo
  requirement anywhere in the document** — `NFR-A11Y-009` is list navigation
  shortcuts (`j`/`k`/`Enter`), and undo's keyboard path was settled at 0.41.0
  and is now measured on two surfaces by `GATE-006`. **A label carried across
  releases without once being checked against the thing it named** — this
  session's own recurring class, in the planning record rather than in code.
- **One item is recorded nowhere a clone contains.** The *top-of-page landing
  after a POST* was measured during `CAL-004` and lives only in
  `.git-exclude/reviewed/` — **gitignored** — and in one closed thread's
  handoff README.

## §1 — where things actually live today, measured

| kind of work | home | state |
|---|---|---|
| open defect classes | `§10.x` register | **works** — `§10.32` and `§10.15`'s declined remainder are both there, correctly |
| deferred future features | `§11`, *Deferred and future requirements* | **works** — thirteen entries, feature-shaped, with sources and phases |
| requirement compliance | each entry's `*Status*` | **works**, and audited twice |
| **unscheduled quality work on things already shipped** | **nowhere** | the POST landing; a possible audit of the 117 entries citing no mechanism; file sizes beyond `NFR-MNT-004`'s own entry |

**The gap is the fourth row only.** Do not propose replacing the first three —
they are doing their jobs, and `§11` in particular is not the right home: its
entries are *features accepted in principle*, not *known rough edges on
shipped surfaces*.

## §2 — the question

> **What is `ROADMAP.md` now, and where does the fourth row live?**

Three shapes, and they are not exclusive:

- **(a) Retire and re-point.** Move the superseded Milestones and Release
  sequence into the historical part the document already has, and let the
  current-plan section say how work is chosen **now**: handoffs, the
  register, the sweep, the owner's scheduling. Short.
- **(b) Give the fourth row a section** in `ROADMAP.md` — unscheduled work,
  each line with **what, why not now, and whose it is**.
- **(c) A separate document** under `docs/development/`, beside
  `changelog-and-releases.md`.

**The architect's view, offered and not binding**: **(a) plus (b)**, in
`ROADMAP.md`, because a third document is a third thing to go stale and this
handoff exists because one already did. But **you will have read the file in
full and the architect has read its structure and two sections** — if it is
too far gone for (a) to be honest, say so.

## §3 — what the deliverable is

1. **A proposal**, with which shape and why — one or two paragraphs.
2. **One edit, made**: whatever minimum stops the document asserting a
   current plan that ended twenty releases ago. **Even if the full answer is
   deferred, that sentence must not survive this release** — it is the
   `NFR-REL-007` shape (a document claiming something untrue of itself) and
   this project has paid for that twice.
3. **The fourth row's two known items written down** wherever your proposal
   puts them: the **POST landing** (measured, product-wide, fix shape
   undecided — `POST-001` investigates it in this same release) and the
   **117 entries citing no mechanism** (`REQ-004`'s denominator; deliberately
   not commissioned).

## §4 — what this is not

- **Not a rewrite of 791 lines.** The Medium-term, Long-term and Out-of-scope
  sections are not in scope and no one has said they are wrong.
- **Not a new requirement.** `NFR-REL-006` requires the repository to
  *contain* `ROADMAP.md`, and it does — **a stale file satisfies it.**
  Whether that requirement should also demand currency is a question worth
  one sentence in your proposal, **and the architect's lean is no**: a
  currency MUST is hard to measure and the specifications already have the
  sweep. **Say if you disagree.**
- **Not a backlog-grooming exercise.** Two known items, written down. Do not
  go looking for more.

## §5 — escalate rather than deciding

- **If the Milestones or Release sequence turn out still to be live** in some
  way the architect missed — that reverses the premise.
- **If the right answer is that `ROADMAP.md` should be deleted.** That is an
  owner decision, not yours or the architect's, and `NFR-REL-006` names the
  file explicitly.
- **If the fourth row turns out to have more than two items** already written
  somewhere committed.

## §6 — gates

Documentation only. `DEC-007` confirmed once at **405**, unchanged. `fmt`
clean. **`mdbook build` with linkcheck** — `ROADMAP.md` is linked from the
book and a restructure can break an anchor. No component, no test.

## §7 — exit condition

A proposal naming the shape and the reasoning; the *current plan* sentence no
longer asserting a plan that ended at `0.27.0`; and the fourth row's two items
written somewhere a clone contains.
