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

## §5 — the one-time walk, and the one file nobody here can read

*This section was written against a draft exit criterion 9, which was
withdrawn on 2026-10-15 before first use. The walk below still happens,
**once, for this release**, because 0.53.0 is the release that creates
all three install paths — looking once at something new is a different
act from looking every time at something settled. The permanent form is
`NFR-REL-011`, which CI enforces and `0.54.0` completes.*


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

## §5b — `NFR-REL-011`'s two missing walks, in this release

*Added 2026-10-15, reversing a one-day-old deferral. The reasoning is
in the entry itself.*

`image.yml` walks the container path on every run. **`release.yml`
builds five archives and never extracts one**, and nothing runs an
installed binary. Close both here:

- **`release.yml`**: after packaging, extract the archive this job
  just built and **run the binary from a scratch directory**, to an
  authenticated session — the same shape as `image.yml`'s own *"Walk
  it"* step, which is the model to copy rather than invent.
- **`cargo install`**: `--path crates/peisear` into a scratch root,
  run from outside the repository. `FIV-005`'s own walk is the
  precedent.

**Exercise both by `workflow_dispatch` before the tag**, against a
real ref, and paste the run IDs. `FIV-002` and `FIV-003` each did this
three times; a workflow step proven by dispatch is not a step taken on
faith at leg 4. **If a dispatch run cannot be made to pass, stop and
report** — that is the escalation, not a reason to ship the walk
untested.

**This makes `NFR-REL-011` `Implemented` rather than `Partial`** — on
the jobs existing and passing, not on §5's human walk, which stays
what it is: a one-time look at three paths that did not exist a week
ago.

## §6 — what closes *after* the tag, and is not yours

`NFR-REL-009` (five archives on the Release) and `NFR-REL-010` (a
`ghcr.io` tag) **stay `Divergent` in this candidate.** `NFR-REL-011`
(the documented paths exercised by CI) moves to **`Implemented`** on
§5b's two new jobs — **on the jobs, not on §5's walk**: a human
walking a path is not what that entry requires, and never was. Their artefacts
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
`NFR-REL-010` left `Divergent` with their conditions stated;
`NFR-REL-011` moved to `Implemented` **only** on §5b's jobs passing a
real dispatch run, with the run IDs quoted; and a
recommendation to the architect — **not a tag**. The four legs are the
architect's, on the owner's approval.

---

# REL-0.53.0 round 2 — one of three documents got the caveat

The candidate is accepted on every gate (see the review). One pass-5
miss, and it is on the front page.

## §9 — `README.md` promises a URL its own commands do not reach

`docs/getting-started/configuration.md` was caveated properly this
round: *"As of 0.53.0, it is not yet true… comment out or delete that
one line before copying the file."* Good, and exactly right.

**Three documents give that instruction. One was caveated.**

- `README.md:62-67` — *"To read or change it"*: `git clone`,
  `cp .env.example .env`, `cargo run --release -p peisear`, and then
  **`#   → http://localhost:3000`**. That arrow is false. The server
  refuses, as this round's own walk proved.
- `.github/CONTRIBUTING.md:28-31` — step 3 of *Development setup*, the
  same two commands, no caveat. The round reported this file as *"read
  in full, already current"*; it is current about everything except
  the instruction that now fails.

**This is the release whose subject is the first five minutes**, and
its front page ends a copy-paste block with an arrow to a URL that
block does not reach. `§10.40`'s own cause, reaching the one document
every visitor sees.

**It is also a pass-5 miss of a known shape.** `NFR-MNT-002` said *"six
crates"* at 0.52.0 because the sweep read every document except the
one it was amending; this read the other documents but not all three
carrying the sentence it had just caveated in the first. **Pass 5's
rule is now: grep for the instruction, not for the document.**

## §10 — resolved: Option 1, applied, walked, and evidenced

*Updated 2026-10-16. The fork below is closed — read this section, not
the options.*

**The owner applied the `JWT_SECRET` fix.** Reviewing the file with the
read rule temporarily lifted found the other half still live —
`BIND_ADDR=0.0.0.0:3000`, uncommented, after `FIV-001` moved the code
default to loopback. **Both are now fixed**, and the file follows one
rule instead of two corrections: *every line is commented out, each
value shown is already the built-in default*, so a copied `.env`
changes nothing and this file can never hand anyone a setting they did
not choose (`be9e2fa`).

**Walked, not assumed** — `Env Example Walk` against `main`,
run [38974525616](https://github.com/nabbisen/peisear/actions/runs/38974525616):

```
RESULT: started
  starting peisear database=sqlite://data/app.db addr=127.0.0.1:3000
  listening addr=127.0.0.1:3000
  → http://localhost:3000
```

**Three things that proves at once**: the documented path starts;
it binds **loopback**, so `BIND_ADDR` is no longer overridden; and
`README.md`'s own `#   → http://localhost:3000` is now literally what
the binary prints.

**So `§9`'s finding is closed by the file, not by caveats.**
`README.md` and `.github/CONTRIBUTING.md` need **no change** — they
became true. What remains is to take the caveat back out and move the
two entries.

### What round 2 now is

1. **`docs/getting-started/configuration.md`** — remove the *"As of
   0.53.0, it is not yet true"* caveat added this round. The section
   says what it always meant again.
2. **`CHANGELOG.md`** — the `### Security` section's second bullet
   states the placeholder still ships and the documented path still
   fails. **Both are now false.** Rewrite it to what shipped: the
   refusal exists *and* the file no longer triggers it. Keep the
   refusal in the entry — it is the durable protection, and it is what
   makes the file's state checkable rather than assumed.
3. **`requirements.md`** — `§10.40` **closed** and `NFR-SEC-004`
   **`Implemented`**, on the run above and on the file's own content;
   Appendix A recomputed from its own rows, not adjusted by the delta.
   Appendix C's `0.53.0` baseline row drops `§10.40`/`NFR-SEC-004` from
   what the release does not prove — `NFR-REL-009`/`NFR-REL-010` stay
   there.
4. **`§11`'s two items**, unchanged and still open.

**Do not re-run the walk to justify the status** — it has been run,
the output is above, and re-running it proves the same thing twice.
Quote that run.

## §10b — the options, kept for the record

**Option 1 — the owner comments out `.env.example`'s `JWT_SECRET`
line.** Then all three documents are true as written, the caveat added
to `configuration.md` this round comes **out** again, `§10.40` closes,
and `NFR-SEC-004` moves to `Implemented` — in this release rather than
carried.

**Option 2 — the file ships unchanged.** Then `README.md` and
`CONTRIBUTING.md` carry the same caveat `configuration.md` already
has, and the arrow in `README`'s second block goes or gains a
condition.

*Option 1 was chosen and is done (`§10`). This record stays because
the reasoning — that the two were opposite edits to the same three
files, so guessing would have cost a round either way — is why the
round waited rather than started.*

## §11 — two smaller ones, not blockers

- **`env-example-walk.yml` reintroduces the two third-party actions**
  `FIV-002` round 2 deliberately removed — `dtolnay/rust-toolchain@stable`
  and `Swatinem/rust-cache@v2`, both at moving refs. The risk here is
  far lower than in `release.yml` (this job holds `contents: read` and
  publishes nothing), but the decision was three days old and was not
  revisited. **Use the bare `rustup install` this project adopted**,
  or say why this workflow is the exception.
- **The walk cannot fail, which is right today and wrong the moment
  the file is fixed.** `set +e` and two `echo` branches: it reports,
  it does not assert. Correct while a refusal is the expected answer.
  **Under Option 1 it must become an assertion** — `RESULT: started`
  or exit non-zero — or it is a permanently green job that examines
  nothing (`§10.36`). Note also that the contributor start path it
  walks is a **documented start path `NFR-REL-011` does not cover**;
  that entry names `install.md`'s three. Say whether it should.

## §12 — exit condition

`configuration.md`'s caveat removed; `CHANGELOG.md`'s security
bullet rewritten to what actually shipped; `§10.40` closed and
`NFR-SEC-004` `Implemented`, citing run 38974525616 rather than
re-running it; Appendix A recomputed from its rows and Appendix C's
baseline row updated; `§11`'s two items each fixed or reasoned; `DEC-007`
unchanged at **486**; and the gates re-run after the last edit.
