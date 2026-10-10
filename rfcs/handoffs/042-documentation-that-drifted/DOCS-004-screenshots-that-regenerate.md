# DOCS-004 — screenshots that regenerate

**Owner request, 2026-10-11**: *"it may be better for new visitors and users
in trouble that some screenshots are shown."* **Target**: 0.49.0, after
`DOCS-003`.

**There is exactly one image in the whole repository** — `docs/src/assets/logo.png`.
A product whose README tagline reads *"Sophisticated. Solid. Really easy.
**Good UI.**"* shows no UI, and `first-run.md` walks a new user through a
board, a project and an issue **in prose**, so a user in trouble has nothing
to compare their screen against.

**The capability already exists and is unused for this.** `browser-checks/`
drives the real application through `cdp.mjs` across **24 pages at 5 widths**
with real fixtures created through real forms. **Screenshots here should be
generated, not hand-captured** — because a hand-pasted PNG is exactly the
artefact that goes stale, and staleness is what all four of this release's
audits were about.

---

## §1 — what to build

**A script in `browser-checks/` that produces documentation screenshots from
the same harness the gates use.** Not a gate — it asserts nothing and must
not run in CI's gate job.

**Reuse `overflow-gate.mjs`'s fixture setup.** It already registers a user,
creates a project, issues, a team and a sprint through the real forms —
**that is the hard part and it is done.** A screenshot script that builds its
own fixtures would be a second copy of that logic to keep in step.

**Four or five images, not twenty-four.** My suggestion, and the *what* is
yours to argue with:

| image | why |
|---|---|
| the kanban board with issues | the product's signature screen |
| the issue list view | the dense alternative, and shows the view toggle |
| an issue detail | where most time is spent |
| `/today` | the personal dashboard, which nothing currently illustrates |
| the sprint plan **or** the calendar | one planning surface |

**One width, not five.** A desktop width for the README. **If you think a
phone-width pair earns its place for the mobile story, propose it** — but the
gate's five widths exist to find overflow, not to illustrate.

## §2 — the two questions that are actually decisions

**Answer both in the report; do not pick silently.**

1. **Where do the files live, and are they committed?** Committed PNGs grow
   the repository and must be regenerated when the UI changes — **which is
   the staleness this handoff is trying not to recreate.** But an uncommitted
   image cannot render in a README. **`DEC-051`'s vendoring precedent
   applies**: vendored assets are acceptable when **regeneration is a named,
   watched operation**. So the shape I expect is *committed, with the
   regeneration command documented beside them* — but say what you conclude
   and what it costs in bytes.
2. **How does anyone know they have gone stale?** This is the real question.
   A screenshot of a redesigned screen is worse than no screenshot, because
   it looks authoritative. **Options**: the sweep's new **pass 5** covers it
   by eye; or the script is run in CI and the output compared; or the images
   carry the version they were taken at. **Propose one.** *Do not build a
   comparison gate in this handoff* — that is a `GATE-00x` decision and mine.

## §3 — what this must not become

- **Not a gate.** No assertions, not in the gate job, and **not a fourth
  script in `browser-overflow-gate`** — that job runs three gates and this
  is not one.
- **Not twenty-four images.** The gate sweeps 24 pages because overflow hides
  anywhere; documentation needs the few screens a reader must recognise.
- **Not hand-captured.** If the harness cannot produce a usable image of some
  screen, **report that** rather than opening a browser and cropping.
- **Not a redesign.** If a screen looks wrong in a screenshot, that is a
  finding for me, not something to fix here. *Taking the first screenshots of
  a UI often finds things* — say what you see.

## §4 — where they go

- **`README.md`** — one, after the Overview, where a visitor decides whether
  to read on.
- **`docs/getting-started/first-run.md`** — the walkthrough's steps, so a user
  in trouble can compare. **This is the half the owner asked for and the half
  a README screenshot does not serve.**
- **The book** renders both, so `mdbook build` must stay clean.

## §5 — gates

- **`DEC-007`** confirmed once at **407**, unchanged.
- **`mdbook build` with linkcheck** — new image paths, and `book.toml` has
  `follow-web-links = false`, so **local image paths are checked and matter.**
- **The three existing browser gates still clean**, in the job's order — you
  are adding a file to `browser-checks/` and must show you did not disturb
  them.
- **`browser-checks/README.md` gains the script**, in the shape the other
  three have, and **says it is not a gate** — the directory's title is *gates,
  not tests*, and a non-gate living there needs saying.
- **Report the byte cost** of whatever lands committed.

## §6 — escalate rather than deciding

- **If the fixtures do not produce a screen worth showing** — an empty board
  photographs badly, and making it look good is a fixture question I would
  rather decide than have improvised.
- **If any screenshot would contain plausible-looking personal data.** The
  fixtures use invented users; **keep it that way and say so.** A screenshot
  showing `alice@example.com` is fine, one showing anything resembling a real
  address is not.
- **If the staleness answer (§2.2) needs a gate** — mine.

## §7 — exit condition

Four or five generated screenshots, committed with their regeneration command
documented, placed in the README and `first-run.md`; a script in
`browser-checks/` that is clearly not a gate and is documented as such; the
three real gates still clean; `mdbook build` clean; **both §2 decisions
answered with their reasoning**; and the byte cost stated.
