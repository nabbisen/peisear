# DOCS-008 — images a stranger can read

**The last round of 0.49.0.** `DOCS-007` implemented all three rulings
correctly; the images it produced are not yet publishable. Three findings and
one small addition.

**Review**: `.git-exclude/reviewed/DOCS-007-review.md`. **Target**: 0.49.0.

**Amended 2026-10-11, after the round was written**: `RFC 0016` was raised by
the owner, accepted the same day, and puts **one change ahead of the
captures** — §0 below. Read it first; it changes what the images must show.

---

## §0 — before you capture anything: the product's own name

**`RFC 0016` (accepted, owner decision): the interface brand string becomes
`peisear`.** Today it is **"Issue Tracker"** — in the navbar of every
authenticated page and in seven page titles — so the product never says its
own name, and `DOCS-007`'s committed screenshots publish the wrong one.

**Do this before `node browser-checks/doc-screenshots.mjs` runs**, or the
images are captured twice.

In `crates/peisear-i18n/src/en.rs`, eight string literals:

| Line | Now | Becomes |
|---|---|---|
| `:205` | `AppBrandName => "Issue Tracker"` | `"peisear"` |
| `:221` | `"Error — Issue Tracker"` | `"Error — peisear"` |
| `:273` | `"{project_name} — Issue Tracker"` | `"{project_name} — peisear"` |
| `:327` | `"Projects — Issue Tracker"` | `"Projects — peisear"` |
| `:333` | `"New project — Issue Tracker"` | `"New project — peisear"` |
| `:345` | `"Edit {project_name} — Issue Tracker"` | `"Edit {project_name} — peisear"` |
| `:1040` | `"Sign in — Issue Tracker"` | `"Sign in — peisear"` |
| `:1041` | `"Create account — Issue Tracker"` | `"Create account — peisear"` |

**Lowercase `peisear`**, matching the crate, the command, the documentation
prose and the specifications. Not *Peisear*; the logo's wordmark is not the
interface's.

**My search for anything that asserts the old text was bounded**:
`git grep -F 'Issue Tracker'` over tracked files returns the eight above plus
`RFC 0016`'s own quotations — **no test, fixture or document asserts it**
(`fixture_locale.rs:131` uses `[fx-brand]`, so the i18n guard is unaffected).
**Re-run that search yourself before trusting it**, and say what you found.

## §0.1 — and the same claim in four published one-liners

`RFC 0016 §9`. Each is **published by the 0.49.0 release itself**, so leaving
them would pair a corrected hero image with an uncorrected crates.io card:

- `crates/peisear/Cargo.toml:3` — the **crates.io description**.
- `crates/peisear/src/lib.rs:1` — the `//!` line, the **docs.rs front page**.
- `crates/peisear-web/Cargo.toml:3` — "…for peisear (issue tracker)".
- `docs/book.toml:17` — the book's own description.

**Take the wording from `RFC 0016`'s hero draft**, not from a fresh
invention, so one phrasing governs: *project and issue tracking that also
shows how the project — and the people in it — are holding up*. Keep the
owner's own adjectives where they still fit. **These are provisional** —
0.50.0's `README.md` pass owns the final phrasing and these lines follow it.

**Do not touch `README.md`'s prose in this round.** The repositioning is
0.50.0's, deliberately, and starting it here would mix a careful copy pass
into a release-closing round. The hero *image* is still this round's.

---

## §1 — the fixture language still in two committed images

**`board.png` and `list.png` both publish "Fixture project for the
BROWSER-001 overflow gate."** as the project's subtitle, under the heading
*Website Launch*, and both show cards titled **"An ordinary fixture issue"**
and **"An assigned fixture issue"**.

At source: `browser-checks/overflow-gate.mjs:243`, `:254`, `:285` — hardcoded
inside `createFixtures`, outside the eight parameters.

**Why this is release-blocking and not cosmetic.**
`crates/peisear/Cargo.toml:4` sets `readme = "../../README.md"`, so
`board.png` is the hero image of the **crates.io page for 0.49.0**. The first
sentence a stranger reads under the product's own name would tell them they
are looking at a test fixture for an internal gate ID.

**Do not fix the three I named and stop.** Re-derive the set **from the
regenerated images**, the way §4.1 asks you to check them — reading the
pictures is the only method that cannot miss a fourth. I found three by
reading two images; that is a bound on my search, not a count.

**The gate keeps its own set.** Whatever you add to the parameter list gets
an adversarial value in `ADVERSARIAL_STRINGS` too, and the gate stays at
**120 cells**.

## §2 — the issue detail image contradicts its own caption

`docs/getting-started/first-run.md` step 3 promises *"title, status,
priority, effort, **a description**, and a place for sub-issues"*. The image
immediately under it reads **"No description provided."**

**This is `/today`'s rule applied to the screen you did commit** — a page
that photographs as though the feature were missing. Sharper here, because
the prose and the picture disagree within one step of a walkthrough, which is
the confusion the owner's philosophy names before any other.

The captured issue is `secondIssueTitle`'s, created with `description: ''`
(`overflow-gate.mjs:265`).

- **Required**: the issue in the committed capture **has a description**, and
  the image shows it.
- **Optional, only if it is one more form POST**: one sub-issue, so the
  second panel shows the feature rather than its empty state. **If it is not
  cheap, leave it** — the sentence says *"a place for sub-issues"*, which an
  empty panel honestly is.

## §3 — the exclusion proof passes vacuously

`the_scan_does_not_count_its_own_file` asserts `SELF_PATH` is **absent** from
`tracked_files()`. **Rename `docs_scan.rs` and `SELF_PATH` names a file that
no longer exists — absent, so the test still passes**, while the renamed file
is counted again and rule 5 silently can never reach zero.

**The one failure mode the test exists to guard is the one it cannot see.**
That is `§10.17`.

**Make it positive**: assert `SELF_PATH` **is** present in git's own tracked
list before the filter, and absent after it. A rename then fails loudly on
the first half, naming the path it could not find.

## §4 — rule 6: the one reference nothing checks

`mdbook`'s linkcheck verifies `first-run.md`'s three relative image paths and
**cannot see `README.md`'s absolute one** — `follow-web-links = false`, which
is the right setting. So the hero image is the single reference in this
project that no check reads: rename the file and crates.io shows a broken
image at the next release, silently, with nothing red anywhere.

**Rule 6, in `docs_scan.rs`**: every
`https://github.com/nabbisen/peisear/raw/main/<path>` URL in `README.md`
names a file that exists at `<path>`. Same shape as rule 3, same module, text
and `fs::exists` only — no network, ever.

**If it costs materially more than rule 3 did, say so and defer it** rather
than growing it.

## §5 — what is explicitly *not* in this round

**The sprint plan's empty fixture.** `sprint_plan` is one of the gate's own
24 pages (`overflow-gate.mjs:515`), and your evidence image shows both panels
empty — so **five of the 120 cells sweep a page with no list in it**, which
is `GATE-001` and `GATE-002` repeating. **Your option (a) is accepted in
principle and scheduled for 0.50.0 with `DOCS-006`**, where the fourth
screenshot falls out of it for free.

**Not now, deliberately**: populating a surface the gate has never swept with
content can turn it red, and a newly-red gate inside a release candidate
blocks 0.49.0 on a defect that has been latent for releases. **That is a
reason to schedule it, not to rush it.** Option (c), the calendar, is
declined — it swaps a known-empty page for one whose fixture state nobody has
measured.

## §6 — gates

- **`DEC-007`** three consecutive runs. **It will move** — §3 changes a test
  and §4 adds at least one. **Say the number and what it is.**
- **`overflow-gate.mjs` at 120 cells**, since §1 touches its string set
  again. The other two gates if anything they read changes; say so either way.
- **The three images regenerated after §0**, byte cost reported, and
  **§4.1's per-image privacy confirmations repeated for the new captures** —
  they are new files, so the old confirmations do not carry over. **The
  navbar in all three must read `peisear`**; if it does not, the capture ran
  against a stale binary.
- **`DEC-028`**: check whether either specification records the old brand or
  the old one-line description. I searched and found none, but that search
  was mine and the rule is yours to satisfy.
- **`mdbook build` with linkcheck**, `fmt`, `clippy -D warnings`,
  `rustdoc-links` (seven crates, eight private-link warnings, zero broken).

## §7 — escalate rather than deciding

- **If a fourth or fifth fixture string turns out to render** that §1 does not
  name. Fix it and say which — that is the measurement working, not a
  deviation.
- **If giving the captured issue a description weakens the gate** in any way
  you can name.
- **If rule 6 cannot be written without a network call.** Then it is not rule
  6 and should not be built.

## §8 — exit condition

The interface saying **`peisear`** and the four published one-liners no longer
calling the product an issue tracker; three images, captured after that, in
which **nothing reads as a fixture and nothing contradicts the prose beside
it**; the per-image privacy check repeated and reported; the
exclusion proved by a positive assertion; rule 6 written or declined with a
reason; `DEC-007` stated and explained; the gate at 120; `mdbook build`
clean.

**Then 0.49.0's content is complete** and the candidate is next — this time
with nothing held back behind it.
