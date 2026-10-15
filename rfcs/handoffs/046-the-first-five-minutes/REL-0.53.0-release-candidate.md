# REL-0.53.0 — release candidate

**Target**: 0.53.0, *the first five minutes*. `RFC 017`, six rounds,
all closed 2026-10-15.

**What is different about this release, and it governs everything
below**: 0.53.0 is the first whose own Release **produces artefacts
after it exists**. Five archives and a container image are pushed by
workflows that fire on `release: published` — minutes after
`DEC-056`'s fourth leg, not before it. **Two requirements therefore
close after the tag, not in this candidate**, and a candidate that
marks them `Implemented` would be the exact defect `§10.40` is about.

---

## §0 — already done this cycle; reconcile, do not redo

Six rounds are closed and reviewed. **Do not re-open them.** What this
round owes them is reconciliation, not repetition:

| round | what it left for the cut |
|---|---|
| `FIV-001` | `§10.40` **open**, `NFR-SEC-004` **Divergent** — on `.env.example`'s own state, which §5's walk establishes |
| `FIV-002` | `NFR-REL-009` **Divergent** — closes after the tag |
| `FIV-003` | `NFR-REL-010` **Divergent** — closes after the tag |
| `FIV-004` | `docs_scan` rule 2 repointed; the book has two new pages |
| `FIV-005` | `§10.41` **closed**; `NFR-REL-002` amended |
| `FIV-006` | six screenshots in the book |

`DEC-007` has moved **451 → 486** across the release. That figure is
the one to account against, and `FIV-004` round 2 left it unchanged.

## §1 — the ordinary cut

- **Version**: `0.52.0` → `0.53.0` in the workspace `Cargo.toml`.
  **This is also what `release.yml` and `image.yml` compare against
  the tag** (`FIV-002` round 2 §9) — if the bump is missed, both
  workflows fail the Release rather than publishing something named
  wrong. That is the guard working; do not route around it.
- `CHANGELOG.md`: `## [Unreleased]` becomes `## [0.53.0] — <date>`.
- `ROADMAP.md`: **What ships next** reconciled — the 0.53.0 row
  removed, the remaining rows' numbers and blockers correct as of the
  tag (exit criterion 8).

## §2 — `DEC-028`: both specifications, five passes

`requirements.md` and `external-design.md`, amended **before** the
tag. The five-pass sweep as always, with two additions this release
earns:

- **Pass 5 greps the document it is amending**, not only the others.
  `NFR-MNT-002` said *"six crates"* when there were seven at 0.52.0
  because pass 5 read every file except the one being edited.
- **Appendix A is recomputed from its own rows**, not adjusted by
  hand. It was wrong in two compensating cells before `FIV-002`'s
  review, which is why the total looked right. It currently reads
  `68 | 52 | 5 | 0 | 2 | 9` and each row's own cells sum to its own
  total; **check that property again after your amendments**, because
  this release adds entries to it.

**`§11`'s new baseline row** is this round's to write. The 0.53.0 row
should say what the release did and **what it did not**: the upload
and publish paths were unexercised until the Release itself fired
them.

## §3 — the changelog

Lead with the security change, not the packaging. A current operator
reads this file for what changes under them, and what changes under
them is: **the published signing-key fallback is gone**, startup
refuses the published placeholder and an empty secret, and the bind
default is loopback.

Then the first five minutes: three install paths, a container image,
assets embedded in the binary, a startup line naming the URL.

**Name the two unexercised paths as shipped-but-unproven**, in the
changelog and not only in the specification. A reader deciding whether
to upgrade is entitled to know that the archive and image workflows
have never run on a real Release before this one.

## §4 — the gate table

Every figure re-measured against the candidate commit, none carried
from a round's own report:

- `DEC-007` from a block extracted fresh, three consecutive clean runs.
- `fmt --check`, `clippy --workspace --all-targets -D warnings`
  (both profiles — `static_assets.rs` has a `cfg`-gated branch).
- `rustdoc` with `-D rustdoc::broken_intra_doc_links`, **run after the
  candidate's last edit**. `FIV-005` round 1 reported this clean from
  a run that predated its own `Cargo.toml` change; do not repeat it.
- `mdbook build` with linkcheck, and **the built book's screenshot
  directory read** rather than counted from a document.
- **The three browser gates**, including `overflow-gate.mjs` —
  `FIV-004` round 2 edited its `createFixtures` and did not run it.
  It passes; run it anyway, because the candidate is where that stops
  being somebody's recollection.

## §5 — exit criterion 9's walk, and the one file nobody here can read

**Walk the documented start on a `workflow_dispatch` build of the
candidate commit** — all three paths in `install.md`, each run rather
than read, from a directory that is not this repository.

**This is also what establishes `.env.example`'s state.** No tool in
this project may read that path, and three rounds reported its
contents as hearsay quoted from a review request. The walk does not
need to read it: `cp .env.example .env` then start, and the server
either runs or refuses. **Report what happened, verbatim.**

- **If it starts**: `§10.40` closes and `NFR-SEC-004` goes to
  `Implemented`, on that evidence.
- **If it refuses**: say so plainly and **leave both where they are**.
  A refusal here is the product working — it means the file still
  carries the published placeholder, and the owner has not applied the
  one edit no tool here can make. **That is a reportable state, not a
  blocker you work around**, and it does not stop the release: the
  refusal protects every reader of that path.

**Do not route around the deny rule** to answer this — not with
`git show`, not with a copy to an intermediate file. `FIV-001` round 2
did, and `§9b` was rewritten because of it.

## §6 — what closes *after* the tag, and is not yours

`NFR-REL-009` (five archives on the Release) and `NFR-REL-010` (a
`ghcr.io` tag) **stay `Divergent` in this candidate.** Their artefacts
do not exist until the Release publishes and the two workflows run.

**Write them as pending with the condition stated**, and leave the
status alone. The architect verifies them after leg 4 by reading the
Release and pulling the image — by the artefact being there, which is
what both entries say the check is.

A candidate that marks either `Implemented` because a workflow exists
is `NFR-REL-008`'s own lesson inverted.

## §7 — escalate rather than deciding

- **If the version bump makes a workflow fail on the real Release.**
  That is leg 4 already past; report it rather than re-cutting.
- **If a gate moves and you cannot account for the delta.** A changed
  `DEC-007` count with no new test is the shape `QA-004` opened.
- **If the walk in §5 fails for a reason that is not the
  placeholder.** A broken install path at the candidate is a stop.

## §8 — exit condition

Version bumped and the changelog section written, leading with the
security change and naming the two unexercised paths; both
specifications amended under `DEC-028` with pass 5 reading the
document it edits and Appendix A recomputed from its rows; `§11`'s
0.53.0 baseline row written, including what the release did not prove;
`ROADMAP.md` reconciled; every gate in `§4` re-measured against the
candidate commit with the browser gates among them; `§5`'s three-path
walk performed and reported verbatim, with `§10.40` and
`NFR-SEC-004` moved **only** on what it showed; `NFR-REL-009` and
`NFR-REL-010` left `Divergent` with their conditions stated; and a
recommendation to the architect — **not a tag**. The four legs are the
architect's, on the owner's approval.
