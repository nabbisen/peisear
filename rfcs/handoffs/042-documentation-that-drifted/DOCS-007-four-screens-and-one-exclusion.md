# DOCS-007 — four screens, and one exclusion

**A short round closing 0.49.0.** Two packages' leftovers: `DOCS-004`'s
escalation, ruled; and `DOCS-005`'s one finding.

**Reviews**: `.git-exclude/reviewed/DOCS-004-review.md` (§3) and
`…/DOCS-005-review.md` (§1). **Target**: 0.49.0.

---

## §1 — the fixture: parameterise, do not add an env var

You offered three options and called the second and third *"more invasive…
same effect."* **They are not the same effect.**

**An env var that swaps the adversarial strings is a lever that can silently
disable the overflow gate.** `BROWSER-002` chose that 64-character SHA-256 run
deliberately, and `§10.25`/`LAYOUT-006` depend on unbreakable text being
present. **A gate whose adversarial property can be switched off by an ambient
variable is the shape this project keeps finding.**

**So: parameterise `createFixtures`.** The overflow gate passes its
adversarial string set **explicitly at its own call site**;
`doc-screenshots.mjs` passes a plain one. One fixture function, two call
sites, no hidden state — and `BROWSER-002`'s intent becomes **visible where
it is used** instead of a default nobody reads.

**Which fields need a plain variant is yours to determine from what actually
renders** on the four screens below — the display name certainly, the project
name, the card titles. **Do not plainify what the gate needs and the
screenshots never show**; a smaller parameter set is a smaller thing to keep
in step.

**And the gate must still be adversarial after this.** Re-run it: **120
cells**. If the parameterisation makes it possible to run the gate with plain
strings by accident, that is worse than the env var and you should say so.

## §2 — capture at 1×, not 3×

`cdp.mjs`'s `screenshot()` applies `clip.scale = 3`, sized for evidence-log
clarity. **README and walkthrough images want 1× at 1280** — smaller *and* the
right resolution for the use. **Report the new byte cost.**

## §3 — four screens, not five

**Board, list view, issue detail, sprint plan. Drop `/today`.**

A blank dashboard is **worse than no dashboard image** — it photographs as
though the feature is empty rather than as though the fixture is. Populating
it honestly needs burnout signals, WIP state and stale issues, which is
`compute_read_first`'s whole input set and a bigger fixture question than this
round. **Recorded as wanted, not now.**

## §4 — then the images, and one sentence

- **`docs/src/assets/screenshots/`, committed**, beside `logo.png`, with the
  regeneration command in `browser-checks/README.md` — `DEC-051`'s shape,
  which you proposed and I accepted.
- **One in `README.md`** after the Overview, **and the walkthrough's screens
  in `docs/getting-started/first-run.md`** — the second is the half the owner
  asked for and the half a README image does not serve.
- **Then the pass-5 sentence** in `docs/development/changelog-and-releases.md`,
  which you correctly declined to write while the directory did not exist.
  **Now it will.** One sentence: the screenshots are regenerated and looked at
  during pass 5.

## §5 — `DOCS-005`'s finding: rule 5 cannot pass

`git_exclude_citations` (`docs_scan.rs:565`) counts across **all** tracked
files and `no_committed_file_cites_git_exclude` asserts **0**. **`docs_scan.rs`
contains the string nine times**, necessarily — in the doc comments explaining
the rule and in the test expectations.

Measured: **139 tracked occurrences, 9 of them in `docs_scan.rs`, 130
elsewhere** — exactly the figure you reported, so nothing was miscounted; the
file that reports the count raised it by nine.

**So when `DOCS-006` closes the 130 and flips the `#[ignore]`, the test fails
at 9 — its own.** That is `§10.17`'s family.

**Exclude the scan's own file from the count, with the reason in a comment.**

**Do not settle what else may legitimately cite the path.** A handoff
*discussing* the problem cites it — three times in `042/` alone — and whether
that is acceptable is **`DOCS-006`'s** question. Here, just stop the rule
banning its own existence. **If excluding one file feels arbitrary, say so** —
the alternative is that the rule's subject is narrower than its name, which
would be worth knowing before `DOCS-006` relies on it.

## §6 — gates

- **`DEC-007`** three consecutive runs. **Expect 419 unchanged** — nothing
  here adds a test. **If it moves, say why.**
- **All three gates clean, in the job's order, with logs this time** —
  `overflow-gate.mjs` at **120 cells** is the one that matters, since §1
  changes its fixture's call site.
- **The screenshot script still runs** and produces the four images.
- **`mdbook build` with linkcheck** — new image paths, and `follow-web-links =
  false` means local paths are checked and matter.
- `fmt`, `clippy -D warnings`, `rustdoc-links` — seven crates, private-link
  count **eight**.
- **Report the byte cost** of what lands committed.

## §7 — escalate rather than deciding

- **If parameterising lets the gate run non-adversarially by accident.**
- **If any of the four screens still photographs badly** once the strings are
  plain. **Say so rather than shipping it** — that was this round's whole
  lesson.
- **If a screenshot would contain anything resembling real personal data.**
  The fixtures invent their users; keep it that way and confirm it.
- **If §5's exclusion cannot be written without looking arbitrary.**

## §8 — exit condition

A parameterised `createFixtures` with the gate's adversarial set explicit at
its call site and the gate still **120**; four 1× screenshots committed with
their regeneration command documented and their byte cost reported; the README
and `first-run.md` showing them; the pass-5 sentence written; rule 5 no longer
banning its own file; `DEC-007` **419**; and `mdbook build` clean.

**Then 0.49.0's content is complete** and the candidate is next.
