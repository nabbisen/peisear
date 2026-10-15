# FIV-006 — two pictures for a newcomer

**Owner request, 2026-10-14**: *"a few (two or around) screenshots in
`README.md` or some documentation may be helpful to new comers. However,
when the screenshots taken, my privacy must be respected and any local
secrets must not be exposed."*

**Target**: 0.53.0. **Independent of the other five rounds** — it needs
only the capture harness, which already exists. Start it whenever there
is capacity; nothing waits on it and it waits on nothing.

---

## §1 — the privacy floor, first, because it is the condition

The owner's constraint is not a caveat on this round; it is the reason
the round has a shape at all. **Read `browser-checks/doc-screenshots.mjs`
before touching it.** The floor is already built into that script and is
structural rather than procedural, which is why it holds:

- It spawns **its own server on its own port** (`4176`) against a
  database in `mkdtempSync` — never a running instance, never a real file.
- `DATABASE_URL` is set **explicitly in the child's `env`**, and the
  file carries the reason in a comment: *a developer's own `.env` cannot
  redirect a capture at a real database.* **Do not add a passthrough.**
- Fixtures are `PLAIN_STRINGS`, with the account on `demo@example.com`
  — a reserved domain, chosen deliberately (`DOCS-008`).

**Every property above is a thing that could be quietly undone by a
convenience.** If any change you make would make the script read an
ambient value instead of an explicit one, that is the escalation in §5,
not a judgement call.

**One gap, and it is this round's to close.** The script does not set
`JWT_SECRET`, so `Config::from_env`'s `dotenvy::dotenv()` will load the
machine's own `.env` for that variable if one exists — a real local
secret signing a throwaway capture session. It does not reach an image,
so this is hygiene rather than exposure, but it is the same reasoning
the `DATABASE_URL` comment already states and the script should be
consistent with itself. **Set it explicitly to a throwaway value**, with
a comment saying why, next to the existing one.

**And the floor applies to the report as well as the images.** The
review request must not quote a local absolute path, a hostname, or
anything from the machine the capture ran on. Relative paths only.

## §2 — what to add, and the one that is actually missing

`README.md` carries **one** screenshot today (the board). The capture
script produces four: `board`, `list`, `issue_detail`, `sprint_plan`.

**`/today` has never been captured.** `DOCS-004` listed it as a
candidate in 0.49.0 — *"the personal dashboard, which nothing currently
illustrates"* — and it did not happen. It is also the screen the README
sells hardest and shows least:

> *"answers two questions ordinary trackers leave invisible… **am I
> overloaded?**"*
> *"A dashboard that is yours alone. `/today`'s load, rhythm and
> sustainability panels, private to you."*

**So: one new capture (`/today`), and one image reused.** The second
README image should be one the script already produces — `list` costs
no new bytes and shows the view toggle, but **argue for a different one
if you think it reads better to a stranger**; the *what* is yours.

**Two images in the README, not five.** The owner said *"two or
around"*. A README that opens with a gallery is a README nobody scrolls.

## §3 — what reading `board.png` found

This is `DOCS-004`'s own rule — *"Taking the first screenshots of a UI
often finds things — say what you see"* — applied to an image that has
been in the README since 0.49.0.

**The health section reads as empty.** The capture shows `HEALTH`, the
summary sentence *"Throughput and Bus factor are worth a glance."*, then
a bare label **`Indicators`** with blank space under it and `WORKLOAD`
below that. A reader sees a section that failed to load.

**It is not a defect.** `HealthStrip` (`crates/peisear-web/src/components/issues.rs:185`)
puts the per-indicator breakdown in a `<details>` collapsed by default,
on purpose, documented in its own doc comment — *"so the standard board
view stays uncluttered"*. The product is behaving as designed.

**It is a screenshot defect.** The README's first feature bullet is
*"Project health in plain language. Six indicators; a non-healthy one
explains itself in a sentence rather than a number"* — and the image
directly beneath that claim shows zero indicators.

**Expand the `<details>` in the doc capture before shooting**, and say
in the alt text that it is expanded. One line of CDP before the
screenshot; the gate captures are untouched.

**This is a decision, so make it deliberately and record it.** The
honest objection is that the image then shows a state the reader's own
first load will not be in. The counter is that it shows a state one
click away, illustrating a documented feature, and that an image reading
as *broken* misinforms more than an image reading as *expanded*. **I
think expanding is right and the alt text is what keeps it honest — but
if you disagree after seeing both, capture both and argue from them.**

## §4 — where they go

- **`README.md`** — two images, each **beside the claim it
  illustrates**, not collected at the top. The board already sits under
  Overview; `/today` belongs with the *"A dashboard that is yours
  alone"* bullet or the *"am I overloaded?"* sentence.
- **`docs/getting-started/first-run.md`** already uses four and needs no
  more. **If `/today` deserves a sentence there too, add the sentence** —
  a newcomer who has just made one issue has a `/today` with something
  on it, which is the one moment that page explains itself.
- **`docs_scan` rule 6** checks that `README.md`'s raw image URLs
  resolve. A new README image means a new URL in that population —
  **confirm the rule covers it and that it fails if the file is
  missing**, by removing the file and watching it fail. A rule that has
  never been seen to fail is `§10.36`.

## §5 — escalate rather than deciding

- **If the capture needs anything ambient** — a real `.env`, a
  pre-existing database, a value read from the machine rather than set
  by the script. Stop and report; §1 is not negotiable.
- **If `/today` captures badly** — empty panels, or panels whose data
  the fixture cannot produce. `DOCS-006` has a precedent exactly here
  (the sprint plan's two empty panels). **Report the image, do not
  fabricate a fixture to flatter it**, and do not hand-capture.
- **If expanding the `<details>` turns out to need more than a line of
  CDP.** That is a signal the page is harder to illustrate than it
  looks, and worth my attention rather than your workaround.

## §6 — gates

- `DEC-007` from a fresh block. This round should move it very little;
  **say by how much and why**, including "zero" if that is the answer.
- `mdbook build` with linkcheck, and the new image present in the built
  book if it lands under `docs/`.
- `docs_scan` rule 6 seen to **fail** against a removed image, then pass.
- Byte count for each committed image, and the total the repository
  gains.

## §7 — exit condition

One new `/today` capture and one reused image in `README.md`, each
beside the claim it illustrates; the health `<details>` decision made,
recorded, and reflected in the alt text; `JWT_SECRET` set explicitly in
the capture script with its reason written beside the `DATABASE_URL`
one; rule 6 seen to fail and then pass; byte counts stated; and **every
image opened and looked at by a person before it is committed** —
confirmed in the report, per image, against the owner's constraint in
§1: no local path, no real address, no account that is not fixture data.
A scan reads text, not pixels, which is why this last one is a person's
job and cannot be delegated to a gate.

---

# FIV-006 round 2 — one image is half empty, and it showed me a defect

Round 1 is accepted (see the review). The privacy floor held, `/today`
is captured at last, and reading `board.png` did what reading a
screenshot is for.

## §8 — your reading of "one image reused" was right, and my handoff was not

Round 1 flagged, rather than guessed, that it read `§2`'s *"one image
reused"* as `board.png` continuing in place — two README images total,
not three. **That reading is correct**, and `§4` of this handoff says
so explicitly: *"two images, each beside the claim it illustrates. The
board already sits under Overview; `/today` belongs with…"*.

**`§2`'s sentence contradicted `§4`** by suggesting `list` as the
second image when `§4` had already spent both slots. Mine, not the
round's. Nothing to change in the repository — recorded so the next
reader of `§2` is not sent after a third image.

## §9 — `today.png` is 900px tall and its content stops at 500

Look at the image as a README reader meets it: the three panels end at
*"What do these mean?"* and **roughly the lower 40% is blank white**.
Beside a feature bullet, that reads as a broken or half-loaded
capture, which is the same failure mode `§3` set out to fix on
`board.png` — an image that misinforms by looking wrong.

`board.png` does not have this problem because a board is tall.
`/today` is short, and the capture is a fixed 1280×900 viewport.

**Remedy is a capture-side change, not a product one.** Either shoot
this page at a shorter viewport, or clip to content height. **Mind
`cdp.mjs`'s own behaviour**, which this script's comments already
record: `screenshot()` applies `scale: 3` when a `clip` is given, so a
naive clip returns a 3× image — which is why the round that wrote that
comment deliberately passed no clip. **Decide which way and say
why**; a 3× README image is a worse defect than the whitespace.

## §10 — the basis link reads as a broken sentence

In `board.png`, the second explanation row reads:

> In-flight work is currently carried by one person. What this is based on

**"What this is based on" is a link** (`issues.rs:312`,
`render_explanation_row`) carrying `class="link link-hover …"` —
daisyUI's `link-hover` shows the underline **only on hover**, so at
rest it is visually identical to the sentence it follows. Its sibling
chips all carry a `▶` marker; this one carries nothing. A reader sees
a sentence that ends in a fragment.

**Why this is in scope for a round about pictures.** `DOCS-004`'s rule
is that a screen looking wrong in a screenshot is a finding for the
architect, and it is — this is me deciding it. It lands here because
**this round publishes that image into `README.md`**, the project's
front door, and because the fix is one class in one call site, which
is smaller than the round it would otherwise need.

**Measured before being called a defect**: `link link-hover` appears
**10 times** across `components/`, and `link link-primary` **4 times**
— both are house style. The other `link-hover` uses sit on project
names, team names and search results, where position alone says
"link". **This one is a fragment appended to the end of a sentence,
where position says the opposite.** So the finding is this call site,
not the convention.

**Remedy**: give it the affordance its siblings have — `link-primary`,
already used four times in this codebase, or the `▶` the chips use.
**Not a sweep of all ten.** Then regenerate `board.png`, which is this
round's own output anyway.

## §11 — exit condition

`today.png` re-captured so its content fills its frame, with the
viewport-versus-clip decision stated and the `scale: 3` trap
explicitly avoided; the basis link given a resting affordance at that
one call site and `board.png` regenerated to show it; both images
opened and checked against `§1`'s privacy floor again, per image;
byte counts restated; `DEC-007` stated — it should move by zero again
unless the link change touches a test that reads that markup, in which
case **say which**.
