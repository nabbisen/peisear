# DOCS-008 — images a stranger can read

**The last round of 0.49.0.** `DOCS-007` implemented all three rulings
correctly; the images it produced are not yet publishable. Three findings and
one small addition.

**Review**: `.git-exclude/reviewed/DOCS-007-review.md`. **Target**: 0.49.0.

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
- **The three images regenerated**, byte cost reported, and **§4.1's
  per-image privacy confirmations repeated for the new captures** — they are
  new files, so the old confirmations do not carry over.
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

Three images in which **nothing reads as a fixture and nothing contradicts
the prose beside it**; the per-image privacy check repeated and reported; the
exclusion proved by a positive assertion; rule 6 written or declined with a
reason; `DEC-007` stated and explained; the gate at 120; `mdbook build`
clean.

**Then 0.49.0's content is complete** and the candidate is next — this time
with nothing held back behind it.
