# REL-0.58.1 — release candidate

**0.58.1, the cut.** Both work rounds are closed: `TTY-001` (two
rounds) replaced the predicate, `TTY-002` (two rounds) restored the
archive, divided the walks and built the guard. `§10.49` is closed.

**This is a patch release that exists because 0.58.0 shipped a
defect and dropped a platform.** The accounting below is therefore
not a formality — the honest record of what went wrong is most of
this release's value, and the only part a future reader can learn
from.

## §0 — whose round this is, and in what order

**This is a dev-team round, and it is the largest one left in the
series.** `§1` and `§3`–`§7` are its work: the version bump, the
`[0.58.1]` section and its accounting, the `ROADMAP` line, `DEC-028`'s
five passes, the gate table re-measured, and a falsification walked
with real binaries. `§10` lists the seven exit conditions.

**The four `DEC-056` legs are not part of it.** Tag, push tag,
`cargo publish --workspace` and `gh release create` are the
architect's, on the owner's approval, per
`docs/development/changelog-and-releases.md` (lines 59–60: *"The
architect, on the owner's approval… The dev team never runs it."*).
They happen **after** this round is reviewed and accepted, in this
order:

1. the dev team does `§1`, `§3`–`§7` and reports;
2. the architect reviews;
3. the owner approves the cut;
4. the architect runs the four legs and the `§8` asset check.

**No gate blocks step 1** — unlike `REL-0.58.0`, which waited on an
owner's answer before it could start, nothing here is pending on
anyone. *(An earlier wording of this line and of the series README's
own State column said "nothing waits but the four legs", which read as
"this round has no work in it". It has; that was my sentence, and this
`§0` replaces it.)*

## §1 — the ordinary cut

1. **Version** — `0.58.1` in `[workspace.package]`, nowhere twice.
   Currently `0.58.0` at `Cargo.toml:24`.
2. **`CHANGELOG.md`** — a `[0.58.1]` section, dated. Confirm
   `[Unreleased]` is empty first, by reading it.
3. **`ROADMAP.md`** — `## What ships next`: `0.58.1`'s row comes out
   and the shipped line is written. **Word the legs as pending until
   they are run** — `0.56.0`, `0.57.0` and `0.58.0` each carried that
   caveat, and `0.58.0`'s own first attempt at this paragraph was
   titled for a release that had not happened. What becomes next is
   whatever the table then shows; it currently shows that nothing is
   authorized beyond this release.
4. **Release cycle exit criteria 1–8**, each stated with what
   satisfied it. **There is no criterion 9.**

## §2 — the claim

**A first run started the way servers are actually started serves
instead of stopping**, and the five documented archives are five
again.

Stated precisely, because the 0.58.0 claim's imprecision is what this
release repairs: peisear prompts only where a human can actually
answer — on Unix, when stdin is a terminal *and* this process is that
terminal's foreground job, the kernel's own `SIGTTIN` test; on
Windows, where no such test exists, when stdin is a terminal, with
`PEISEAR_SETUP=never` available to declare the answer. Where it
cannot prompt it starts on defaults and says so once, on stderr, in
the one case where a reader might be waiting.

## §3 — the accounting, which is this release's actual content

**`0.58.0`'s own `CHANGELOG` entry is wrong, and the `[0.58.1]`
section must say so plainly.** It reads:

> *"a start with no configuration asks, writes the file it describes,
> and **is skipped entirely where a prompt would be an outage**."*

Two shapes were tested — no terminal, and configuration already
exists. A third was not: a terminal attached that this process cannot
read. There, a prompt *was* the outage. I wrote that sentence, and the
same claim in `ROADMAP.md` (corrected 2026-10-28) and in `§10.49`.

**Do not rewrite `[0.58.0]`'s text.** It is the published record of
what was claimed, and a changelog that edits its own history is worse
than one that carries a correction forward. **Do** add a short
parenthetical pointer in that section — *"corrected in 0.58.1"* or
equivalent — so a reader of `[0.58.0]` is not left with the false
claim alone. Confirm `changelog_scan` still passes with it; if the
guard objects, report that rather than working around it, and the
pointer is dropped in favour of the `[0.58.1]` text alone.

**Three things belong in `[0.58.1]`:**

1. **What broke, concretely.** `should_run_wizard` asked whether a
   terminal was attached when the question was whether anyone could
   read it. `./peisear > log 2>&1 &` stopped under `SIGTTIN`:
   nothing listening, no `.env`, the only trace a question in a
   redirected log. The Windows release job hit the same cause through
   a `Start-Process` console and never attached the archive it had
   already built, so `0.58.0` shipped four of the five archives
   `install.md` offers — a platform `0.56.0` and `0.57.0` both had.
2. **That the defect was found by a check, too late to help.** The
   only thing in the project that ever put a terminal in front of the
   binary was the Windows release walk, which runs on a tag. It
   worked — and the earliest it could speak was while the release was
   being produced.
3. **That the fix is now guarded.** `test.yml` case 7 exercises the
   defining case on every pull request, seen red on a planted
   predicate with `[1]+ Stopped` and bash's own `wait: warning: job
   stopped` in the log, green at the commit that shipped.

## §4 — `DEC-028`: both specifications, five passes

- **Pass 5 greps for the instruction, not the document.**
- **Appendix A recomputed from its own rows** — read from rule (a)
  and rule (b); run them, do not re-derive by hand.
- `docs_scan`, `changelog_scan` and `spec_scan` green.
- **`§10.49` is closed and its entry already carries its own closing
  note** (set by the architect across two rounds, with the coverage
  table and the corrected run citation). **Do not restate it in the
  baseline row** — cite it. The baseline row's job is what the
  release shipped, not a second copy of the register.
- `requirements.md`: does anything here move a `*Status*` line?
  `PEISEAR_SETUP` is a new operator-facing variable and the skip
  behaviour changed — check whether any `FR-*`/`NFR-*` entry should
  name it, and **if none should, say so rather than leaving the
  question unasked.** `NFR-REL-*` is the likely neighbourhood for the
  archive count.

## §5 — the gate table, re-measured against the candidate commit

- **`DEC-007` — 601** at `b9e4172`; re-measure at the candidate.
  `586` at `0.58.0`, `+13` `TTY-001` round 1, `+2` `TTY-001` round 2,
  `+0` both `TTY-002` rounds. `peisear-web --lib` **205**,
  `wizard::` **44**, `peisear` **3**. **Count with the runner**, never
  a grep.
- **The wizard cases green, and one seen red by a plant this
  release** — it was: `TTY-002` round 2's case-7 plant
  ([40819438956](https://github.com/nabbisen/peisear/actions/runs/40819438956)).
  Cite it; do not re-plant.
- `fmt`; `clippy --workspace --all-targets`, both profiles; `clippy`
  under `--features postgres` for all four crates that carry it;
  rustdoc with `.github/CONTRIBUTING.md`'s own documented flag, not a
  stricter one invented here; `mdbook` + linkcheck; `publish-dry-run`.
- **The local `publish --workspace --dry-run` artefact should
  disappear at the bump.** It fails locally at an unchanged version
  because the sandbox's registry cache resolves a stale
  `peisear-web@0.58.0`; the same prediction held at `0.58.0`. If it
  still fails after the bump, that is a real finding — report it.
- CI on the candidate: **quote the run's own conclusion and its
  non-success count; do not sum the jobs.** One `SKIPPED` (`Deploy to
  GitHub Pages`) is correct on a PR and is not a failure.

## §6 — the claim to falsify, with real binaries

**An upgrade must be a non-event for a configured instance.** 0.58.0
is installable, so this is no longer hypothetical.

1. Install `0.58.0` from crates.io into a scratch root, register a
   real account, create a project, keep the session cookie.
2. Run this candidate against that same database. **Assert**: the
   migration count does not move; the 0.58.0-issued cookie still
   authenticates; startup output for a configured instance is
   byte-identical to 0.58.0's.
3. **And the new one**: `./peisear > log 2>&1 &` in an empty
   directory serves, where 0.58.0 stopped. That is the release's own
   claim, run against the archive rather than a debug build.

This release adds no migration. Say so, and show the count rather
than asserting it.

## §7 — what this release does not prove, stated not implied

Both of these came out of this series and neither is fixed. They
belong in the record, not in a footnote.

- **An attended start prompting is unproven on Windows.** Five
  measured attempts at console-input injection, each fixing a
  diagnosed defect in the last, ended with every Win32 call reporting
  success and the wizard still not advancing. The account lives in
  `release.yml`'s own comment. The reasoning that this is not a
  product gap — on Windows *attended* is just *a terminal is
  attached*, which `Start-Process` already provides — is accepted,
  and the property is proven on a real pty on Unix and by `test.yml`
  case 3.
- **No CI job compiles for Windows at all** outside the release tag.
  `test.yml` is `ubuntu-latest` throughout, so platform-asymmetric
  `#[cfg]` code — this series added three pieces — is first compiled
  for Windows while the archive is being produced. Twice in this
  series `release.yml` found something `test.yml` structurally could
  not (this, and BSD `script` on macOS). **Carried as a finding, not
  a register entry**; the architect brings it to the owner with the
  next proposal.
- Seven `SMTP_*` variables remain documented nowhere operator-facing.
- The wizard is still proven on a runner's pty and in CI, not inside
  the published container image.

## §8 — the legs, and the check `0.58.0` taught us to run

`DEC-056`'s four legs are the architect's, on the owner's approval:
signed annotated tag · push tag · `cargo publish --workspace` from a
detached worktree of the tag commit · `gh release create` with the
changelog section as its body and `--verify-tag`.

**Then, before any record says this shipped**: `gh release view 0.58.1
--json assets` and count. **Five archives, not four.** 0.58.0's record
was one template-filled sentence away from claiming five when the
Release carried four, and the only reason it did not is that the asset
list was read. Read it again. Verify the crates by per-crate query and
the container tags against the registry's own manifest API, not
against the workflows' exit codes.

## §9 — escalate rather than deciding

- If `changelog_scan` objects to the `[0.58.0]` pointer in `§3`.
- If any `*Status*` line looks like it should move and the right
  wording is not obvious — that is a specification question, not a
  release-mechanics one.
- If the `publish --dry-run` artefact survives the version bump.

## §10 — exit condition

1. Version bumped; `[0.58.1]` section written and dated;
   `[Unreleased]` confirmed empty by reading it.
2. `§3`'s three items present in `[0.58.1]`, and `[0.58.0]`'s text
   **not rewritten**.
3. `ROADMAP.md`'s shipped line written with the legs worded as
   pending.
4. `DEC-028`'s five passes run, `§10.49` cited rather than restated,
   and the `*Status*` question answered either way.
5. `§5`'s gate table re-measured at the candidate, counted with the
   runner, CI's own conclusion quoted.
6. `§6`'s falsification walked with real binaries, including the
   backgrounded start, with the migration count shown.
7. `§7`'s four limits stated in the candidate's own words.
