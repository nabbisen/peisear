# QS-003 — the page that never clones

**`install.md` tells the reader that `build-from-source.md` clones the
repository. That page never clones anything.**

`install.md:57`, inside Option C's own footnote:

> (If you want to read or change the code rather than just run it, see
> [Build from source](build-from-source.md) instead — that path clones
> the repository and builds from a checkout, which this one does not.)

`build-from-source.md` installs `rustc` two ways, verifies the version,
and then its `## Next` jumps to Configuration and First run. **There is
no `git clone` on the page, and no build command outside the
distro-package alternative** — where `cargo-1.91 build --release -p
peisear` assumes a checkout the page never told the reader to make.

**So one published document asserts that a second teaches a step the
second does not contain**, and the page skips the step its own title
promises. This is `NFR-REL-007`'s class: a reader following written
instructions does not reach a working instance. **Found by `QS-002`**
while measuring its rule against the real tree, and given a round of its
own rather than folded into the round that noticed it, on this project's
standing exception.

**And the same shape exists one layer down.**
`.github/CONTRIBUTING.md:23`, step 2 of Development setup, reads
*"**Clone the workspace** and run the test suite:"* — and the fence
under it contains `cargo check --workspace` and `cargo build -p peisear
--bin peisear`, **no clone**. A reader following it literally runs
`cargo check` in whatever directory they are standing in.

## §0 — what this round is

**Documentation, plus the one decision about where a command lives.**
No rule changes. `QS-002`'s rule 9 stays byte-exact against
`install.md` alone, and **this round must not extend it** — that was
option (a)'s rule half and it is declined in `QS-002` `§7`.

## §1 — the decision already made, so the round does not re-open it

**`build-from-source.md` owns the clone-and-build step.** Not the
README, not `CONTRIBUTING.md`. The reason is that the chain already
says so: `install.md:57` names that page as the one that clones, so
making the claim true is cleaner than moving the claim. *State a fact
once* then settles the other two documents:

- **`build-from-source.md`** gains the missing step — get a checkout,
  build it, run it — placed after `## Verify` and before `## Next`, and
  **`## Next` stops skipping it.**
- **`.github/CONTRIBUTING.md` step 2 drops its clone claim** rather than
  gaining a clone command. Step 1 already links
  `build-from-source.md`; once that page teaches the checkout, step 1's
  link owns it and step 2 is about the test suite. **Three copies of a
  clone command in three documents is the defect this series exists to
  stop**, and adding a third copy to close a prose-versus-fence gap
  would be that defect wearing a fix's clothes.
- **The README's source block is not touched.** Its two lines are
  `QS-001`'s and `QS-002` `§7` moves them under their own heading; this
  round gives them a page that teaches them.

## §2 — what the commands should be, and what proves them

**`.github/workflows/env-example-walk.yml` already walks this path** —
`actions/checkout@v6`, `rustup install`, `cp .env.example .env`,
`cargo build --release -p peisear`. Write the page's build-and-run step
to agree with what that job actually runs.

**Correction, 2026-11-03 — this handoff first said "so the documented
commands are ones CI exercises on every push." That was false, and it
was the architect's.** `env-example-walk.yml` is
`on: workflow_dispatch:` **only**, with a required `ref` input: it runs
when a person asks it to and never on a push. Its whole run history is
**three manual dispatches, the last on 2026-10-16** — before `0.57.0`,
`0.58.x`, `0.59.0` and `0.60.0` shipped. `§10.40` describes it
accurately as *"a new, one-off `workflow_dispatch`-only job"*; the
overstatement was in this handoff alone. **The round did not propagate
it** — its report wrote *"on every manual dispatch"*, which is right.
What the job's commands are still worth agreeing with; what they are
not is continuously proven.

**Two bounds to state rather than imply:**

1. **CI does not prove the clone line.** Actions checks out by its own
   mechanism; no job runs `git clone`. The clone is one standard command
   and that is fine — **but do not write or report that CI covers it.**
2. **`NFR-REL-011` does not cover this path and must not be widened to.**
   That entry decided it explicitly in `0.53.0` round 2: its text names
   the paths `install.md` offers — a newcomer with no Rust — and
   build-from-source is *"a different document, for a different reader,
   who already has a toolchain"*. The walk *"stands on its own merit as a
   regression guard for the contributor path; it does not need this
   entry's authority to justify existing."* **Quote that decision if the
   question comes up; do not re-decide it.**

## §3 — the trap

**The page's existing build command is `cargo build --release`, the
README's is `cargo run --release`, and the workflow's is `cargo build
--release`.** Three near-identical commands, which is how this series'
original defect started. **Pick deliberately and say why in the round's
report**: the page teaches both a build and a run if a reader needs
both, or it teaches one and the other document agrees with it. What it
must not do is add a fourth variant without noticing it is a fourth.

**Nothing checks any of this.** Rule 9 guards the README's Quick Start
against `install.md`, and after `QS-002` `§7` the source block is
outside that section. So this round's own correctness rests on reading,
and the residual is named in `QS-002`'s closing rather than hidden.

## §4 — gates

`cargo fmt --check`, `clippy --workspace --all-targets -- -D warnings`,
`mdbook build docs` (the linkcheck backend runs with it — this round
edits a page the book serves), and the full per-crate suite with
`DEC-007`'s count from the runner. **Run them after the last edit**, not
after the last code edit.

## §5 — escalate rather than deciding

- If completing `build-from-source.md` would need a command the page
  cannot state without a second decision — a Windows checkout, a
  `--no-default-features` variant for a PostgreSQL contributor — **name
  it and stop**, rather than inventing a path nobody has walked.
- If `.github/CONTRIBUTING.md`'s step 2 turns out to need the clone for a
  reason this handoff has not seen, **say so with the reason** instead of
  applying `§1`'s decision against the evidence.

## §6 — exit conditions

1. `build-from-source.md` teaches the step its title promises: a
   checkout, a build, a run.
2. `## Next` reaches that step instead of skipping it.
3. **`install.md:57`'s claim is true when read against the page it
   points at** — quote both in the report.
4. `.github/CONTRIBUTING.md` step 2's prose and its fence agree.
5. The command variants are reconciled deliberately, with the choice and
   its reason stated — including which commands `env-example-walk.yml`
   runs and which of them CI does **not** prove.
6. `§4`'s gates run after the last edit, `DEC-007`'s delta stated with
   the runner (this round should move it by zero; say so if it does not).
