# QS-001 — the first command

**The Quick Start leads with the path that needs nothing**, and
`NFR-REL-007` gains the clause whose absence let this sit for seven
releases.

Read the series `README.md` first; it carries the measurement and why
nothing caught this.

## §0 — what this round is, and what it is not

**A documentation round plus one specification clause.** No code, no
new artefact, no release. It merges to `main` and GitHub serves it
immediately.

`NFR-REL-007`'s **ordering is met and stays met** — the Quick Start
keeps its place between *why/when* and *features*. Only its contents
change.

## §1 — the shape, which is the architect's, and the words, which are yours

**Four paths, in this order, because the order is the whole point:**

1. **Download and run.** First, because it needs **nothing** — no
   toolchain, no container runtime, no `git`. Link straight to
   `https://github.com/nabbisen/peisear/releases/latest` so the reader
   does not navigate; name that five archives are there, one per
   platform, and point at `install.md`'s own table rather than
   repeating it.
2. **`docker run`.** Second, because it is **one command** for a
   reader who has Docker:
   `docker run -p 3000:3000 -v ./data:/data ghcr.io/nabbisen/peisear`.
   Keep the `-v ./data:/data`; it is what makes the database and the
   generated key survive a restart, and a Quick Start that drops it
   teaches a data-losing habit.
3. **`cargo install`**, kept, and marked *if you already have Rust* —
   **mirror `install.md`'s own conditional heading rather than
   inventing wording**, so the two documents cannot drift into
   disagreeing about the same path.
4. **The clone**, kept, under its own heading — *to read or change
   it*. That is what it is for, and the current Quick Start's second
   block already says so; it is not an install path and should not sit
   among them.

**Keep it short.** `NFR-REL-007` says the README stays concise, and
the owner's *"less is more"* applies hardest to the section a reader
meets first. Four short blocks, a line each saying what it needs, and
one link to `install.md` for the full story. **If this section grows
past what fits on one screen, it has failed at the thing it is being
fixed for.**

## §2 — the trap, verified, which this round must not walk into

**The current Quick Start's comment is `# asks two quick questions on
first run (enter twice for the defaults)`. It is true of paths 1, 3
and 4, and false of path 2.**

`should_run_wizard(can_prompt, configuration_exists)` requires
`can_prompt`, which requires stdin to be a terminal
(`wizard.rs:30–38`, `TTY-001`, `§10.49`). `docker run` **without
`-it`** — which is how `install.md` writes it and how this round should
write it — gives the process no terminal, so the wizard does not run:
the instance starts on defaults and says so once on stderr.

**So do not copy that comment onto the container block.** Say what
actually happens there instead. Copying it would ship a Quick Start
that promises a prompt that cannot appear — which is the same defect
class this whole series is about, committed inside the fix for it.

*This is the one thing in this round I would most expect to go wrong,
which is why it has a section rather than a footnote.*

## §3 — `NFR-REL-007` gains a clause

The entry requires the README's ordering and nothing about the quick
start's contents, which is exactly why the sweep never caught this.
**Amend it** so a future sweep can:

> the quick start MUST offer at least one path that reaches a running
> instance **without a language toolchain**, and any path that needs
> one MUST say so in the line that introduces it.

**Write the clause in the entry's own voice, not this one's** — the
wording above is the property, not necessarily the prose.

- **Date the amendment and name this round**, per `DEC-028` pass 1.
- **`*Status*` stays `Met at 0.41.0`** unless you find a reason it
  should not, in which case **escalate rather than moving it** — a
  status move is a specification judgement, and `§10.34` is this
  project's own entry about statuses that moved when they should not
  have.
- **Record what the clause does not do**: it is a property a reader
  and a scan can both check, and `QS-002` is the scan. Until that
  round lands, nothing enforces it — **say so in the amendment**
  rather than letting a clause imply a guard that does not exist yet
  (`§10.36`).

## §4 — `CHANGELOG.md`

Under `## [Unreleased]`, not a new version section. **Confirm
`[Unreleased]` is empty first by reading it** — it was at `293df27`,
read it again.

What belongs there: that the Quick Start now leads with a path needing
no toolchain, **and that it did not for seven releases while
`install.md` did.** The project's own standard is to name its mistakes
rather than fix them quietly, and a reader of the changelog is exactly
who benefits from knowing the front door was wrong.

## §5 — gates

Nothing here compiles, but three things read these files:

- **`docs_scan`** — it reads `README.md`. Run the suite rather than
  assuming a prose edit is invisible to it; `docs_scan` rule 1's
  needle is pointed at files like this one.
- **`prose_scan`** and **`changelog_scan`** — both run under
  `cargo test -p peisear-web --lib`.
- **`mdbook build docs`** plus the link check — the README is linked
  from the book, and the new `releases/latest` link is external, so
  state whether the link checker reaches it or skips it rather than
  assuming either.
- **`DEC-007`** should not move. Say the number you measured, with the
  runner.

## §6 — escalate rather than deciding

- If `NFR-REL-007`'s `*Status*` looks like it should move.
- If `docs_scan` objects to the new README shape — that is a finding
  about the rule, not a reason to reshape the section around it.
- If making the four paths fit on one screen means dropping something
  a reader needs; **say what you would drop and why** rather than
  shipping a section that is comprehensive and long.

## §7 — exit conditions

1. The Quick Start offers the four paths in `§1`'s order, with the
   toolchain-free one first and a direct link to the latest release.
2. `cargo install` carries `install.md`'s own conditional wording, not
   new wording.
3. **The container block does not claim a prompt** (`§2`), and says
   what happens instead.
4. The section still fits one screen, and you say how you checked.
5. `NFR-REL-007` amended, dated, this round named, the missing guard
   named, `*Status*` unmoved or escalated.
6. `[Unreleased]` entry written, naming the seven-release gap.
7. `§5`'s gates run after the last edit, `DEC-007` counted with the
   runner, and the external link's treatment by the checker stated.
