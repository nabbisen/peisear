# DOCS-007 — four screens, and one exclusion

**A short round closing 0.49.0.** Two packages' leftovers: `DOCS-004`'s
escalation, ruled; and `DOCS-005`'s one finding. **Plus §4.1, added
2026-10-11 after the owner's instruction that nothing committed as an image
may carry private or local information** — read it before writing §1's plain
string set, since it constrains what those strings may be.

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

## §4.1 — the privacy floor for anything committed as an image

**Owner instruction, 2026-10-11: a committed screenshot must not carry
anything private or local.** These images are published — to the repository,
to the book, to docs.rs readers and to crates.io — and an image is the one
artefact in this project that nothing scans. Rule 1 through rule 5 read text.
**No gate will ever read these pixels, so the check is yours and it is
manual.**

**What I verified already, so you do not repeat it.** I read your preview's
`board.png` and `issue_detail.png` at full size. The capture is **page pixels
only** — no window frame, no address bar, no tabs, no desktop, nothing of the
machine. The mechanism is sound at three points worth naming, because §1
changes one of them:

- `cdp.mjs:48` launches headless against a **`mkdtemp` profile**, so no real
  browser profile, history, cookie or extension is ever in frame.
- `doc-screenshots.mjs:61-73` creates a **fresh `mkdtemp` database per run**
  and pins `DATABASE_URL` in the child's environment. That pin is load-bearing:
  `Config::from_env` calls `dotenvy::dotenv()`, and dotenvy 0.15's `dotenv()`
  **does not override an already-set variable** (its `_override` twin exists
  for that) — so a developer's own `.env` cannot redirect the capture at a real
  database. **Keep it pinned.**
- The fixture invents every identity, and its addresses sit in
  **`example.org`**, a reserved domain.

**What §1 must preserve.** The plain string set you introduce replaces
invented adversarial values with invented plain ones — **not with anything
real**. No contributor's name, no real address, no real organisation, no
hostname. Addresses stay in `example.org` or `example.com`. A plain name that
reads like a real person is worse than a plain name that reads like a fixture;
prefer the latter.

**One element to keep out of frame.** `layout.rs:289` renders the signed-in
user's **email** in the navbar dropdown. It is closed in a capture and did not
appear in either preview — **keep it closed**, and do not add `/settings` or
the team members page (`teams.rs:533` renders every member's address) to the
set without asking first.

**Do not add a way to point this script at a running instance.**
`DOC_SCREENSHOT_DIR` and `PEISEAR_BIN` are fine — they choose an output
directory and a binary. A `DATABASE_URL` passthrough is not: it is §1's own
argument one level up, **a lever that makes a real database photographable**.

**And say it where it is run**: the regeneration command in
`browser-checks/README.md` gets one sentence — the script builds its own
throwaway database and must never be pointed at a real one.

**The check itself, before `git add`.** Open each of the four images at full
size and confirm, per image, by name: no real person, no real address beyond
the reserved domain, no host other than `127.0.0.1`, no filesystem path, and
nothing from the machine outside the page. **Report it that way — four
images, four confirmations.** "Looks fine" is not the report.

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
- **If any of the four cannot be captured without something of the machine in
  frame** — a path, a host, an address, anything §4.1 names. **Do not crop or
  retouch it out.** A retouched screenshot is an image that no longer shows
  what the product does, and nothing downstream can tell. Say so and stop.
- **If §5's exclusion cannot be written without looking arbitrary.**

## §8 — exit condition

A parameterised `createFixtures` with the gate's adversarial set explicit at
its call site and the gate still **120**; four 1× screenshots committed with
their regeneration command documented and their byte cost reported; the README
and `first-run.md` showing them; the pass-5 sentence written; rule 5 no longer
banning its own file; `DEC-007` **419**; `mdbook build` clean; and **§4.1's four
per-image privacy confirmations reported by name**.

**Then 0.49.0's content is complete** and the candidate is next.
