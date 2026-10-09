# REL-0.46.0 — release candidate

**Read this first: the version is 0.46.0, not 0.47.0.** `0.46.0` was never
cut. The last tag is **`0.45.0`**, `Cargo.toml` says `0.45.0`, and everything
in this thread is one unreleased body of work. The architect had been
labelling amendments `0.46.0` *and* `0.47.0` for two releases that do not
exist; **the specification is relabelled** (`f260b8b`) and the deferred three
moved to `0.47.0`. If you see a stray `0.47.0` referring to *this* work
anywhere, that is a miss and I want it reported.

**Contents**: `TT-007` (`b89cf01`), `REQ-004`, `REQ-005`, `COV-001`
(`7e365e7`), `NAV-001` (`d13768a`), and the specification amendments
(`056123e`, `2d0440c`, `82d8df4`, `f260b8b`). **Every handoff reviewed and
closed.**

**`DEC-028` is already done** — both specifications cover `0.46.0`
(`f260b8b`), including §3's sweep. **Depends on**: nothing outstanding.

**Do not tag. Do not publish. Do not create the Release.** Produce the
candidate and stop.

---

## 1. Ordinary cut

`Cargo.toml`, `Cargo.lock`, `CHANGELOG.md` only. **`0.45.0` → `0.46.0`.**

**No migration** — `0020` remains the most recent, the **seventh** release
running.

**Expected: 392**, from 383. Nine net: `COV-001` six, `NAV-001` one, and
`TT-007` **none** — it widened an existing test in place rather than adding
one, which is why the figure is not ten. *The composition is worth stating
even though the total is unsurprising.*

**Production source changed**: one component (`teams.rs`'s breadcrumb block),
nine `class=` literals becoming `grow(…)` calls across four components, the
touch-target guard's population, and a fourth `NavSection` variant with its
two match arms. Nothing in any handler, no route, no migration, no
`static/*.js`.

**The overflow gate is 120 cells** and this release changed component markup
twice — the nine declarations and team detail's trail. Run it from the
extracted tree, and the trap script after.

## 2. The changelog

**This release is one story and it should be told as one**: a defect class
found by accident, audited to its edges, and closed except for three gaps
deliberately left. Six beats.

1. **Open with what was wrong, in a sentence a reader can hold.**
   `NFR-A11Y-007` says **every interactive element** presents a 44 × 44 px
   touch target, and the guard it names as its acceptance read **three tag
   names of six** — **146 elements inside its population, 102 outside**. The
   requirement read `Met`, the guard was green, **and both were true.**
2. **Name the class, because it is new.** `§10.35`: *a requirement's cited
   acceptance narrower than the requirement.* Say what it is not — the code
   runs (`§10.15`), the job exists (`§10.16`), and the assertions pass for
   exactly the right reason over exactly the elements they name (`§10.17`).
   **Nothing was broken. Nothing was watching.**
3. **Then the audit, and its denominator.** Of 165 entries, **48 cite a
   mechanism**; seven more were narrower and had never said so; **four were
   reported as *unresolved* rather than scored**, and that honesty is worth a
   clause — it is why a second pass happened at all.
4. **The finding behind the findings**, which is the transferable part: a
   population-to-citation ratio **overstates** risk when the mechanism is
   structural, and **understates** it when the citation names no artefact.
   `FR-SUB-002`'s *"trigger-enforced"* named a mechanism class while **three
   of its four conditions had zero executed assertions**, next to a sibling
   that was correctly tested — which is how a gap reads as covered.
5. **One coverage audit found a compliance defect**, and this is the beat not
   to bury: writing the test meant to prove `FR-NAV-003` covered found **team
   detail with no back link at all**. Its trail was hand-rolled; the
   requirement's sentence has two conjuncts and that screen met one. **A user
   on that page now gets a back link and a trail starting at Today.** Say the
   test **failed before the fix existed**, on *no `<nav>` present*.
6. **What is deliberately left**, with the reason: `FR-PER-006`,
   `FR-HLT-007` and `NFR-A11Y-008` at 0.47.0, because each needs a judgement
   about *what* a test should assert — and for `NFR-A11Y-008`, a Rust test
   asserting that a script-updated region *exists* would build `§10.35`'s own
   defect one release after recording it.

**Two things the notes must not claim.** That the touch-target guard is now
complete — it reads six tags and the nine `<textarea>`s pass on a **browser
measurement**, which the entry records. And that the audit found compliance
defects **plural** — it found one, and judged the other eleven correct on
inspection.

**Mention the relabelling in Internal.** A reader comparing the register to
the tags should not have to work out why amendments say `0.46.0` for work
that was partly labelled `0.47.0` yesterday.

## 3. Verification before the candidate commit

- **`DEC-028`: already done** (`f260b8b`). Confirm both specifications read
  `Covers release: 0.46.0` with a `0.46.0` baseline row each.
- **The sweep**: run by the architect; **its fourth pass found nothing new**,
  which is the first time. Confirm rather than re-run, and **report *swept,
  nothing further found* explicitly** — a clean pass reported as silence is
  the shape the procedure exists to prevent.
- `DEC-007` **392**, three consecutive runs, **from a block extracted out of
  the current `CONTRIBUTING.md`**.
- **Both browser scripts on the extracted tree**, in the job's own order:
  overflow gate **120 cells** with the new trail and the nine declarations
  present, then the trap script clean on three surfaces.
- **`mdbook build` from the extracted tarball** — sixth release with the site.
- `fmt`, `clippy --workspace --all-targets -- -D warnings`, `rustdoc-links`
  with **seven** `Documenting` lines and the private-link count **eight**.
- **No migration to exercise**, but start the candidate on a database built by
  the released **0.45.0** binary and confirm it comes up at 20 with nothing
  applied.
- `cargo publish --workspace --dry-run` from a **detached worktree of the
  candidate commit** — seven crates, **every `warning:` line read** rather
  than grepped for keywords.
- **A behavioural check, because a user-visible page changed**: on the built
  artefact, open a team's detail page and confirm the trail reads **Today →
  Teams → the team** and that a **Back to teams** link is present and works.
  Ten seconds, and it is the one thing in this release a user meets.
- **Grep the tree for `0.47.0`** and confirm every remaining hit refers to the
  **deferred** work and not to this release. That is the relabelling's own
  check and I would rather you found a miss than a reader did.

## 4. Escalate rather than deciding

- **If the count is not 392**, or the gate is not 120.
- **If any `0.47.0` reference turns out to describe work in this release.**
- **If the team detail page's trail or back link does not work** on the
  artefact.
- **If `rustdoc-links`' private-link count is not eight** — `test.yml`'s
  comment and `NFR-REL-008` both cite it.
- **If the sweep turns up anything**, given mine found nothing — two clean
  passes disagreeing is more interesting than either.

## 5. Exit condition

A candidate commit on `main` at **0.46.0**; a changelog telling one story
with the compliance defect unburied and the three deferrals explained; 392
green three times from a freshly extracted block; the gate at 120 and the trap
script clean; the book building from the tarball; the 0.45.0 database coming
up clean; a dry run of seven crates with every warning read; the team
breadcrumb demonstrated on the artefact; and **no `0.47.0` reference
describing this release**. Then stop.
