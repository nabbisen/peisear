# LAYOUT-007 — the display name is user text too: two subtitles, and a fixture that cannot see them

**Target release**: 0.33.0. **Governing RFC**: none — defect fix.
**Source**: my verification of `LAYOUT-006` (`LAYOUT-006-review.md` §3);
`§10.25`.

## 1. The defects

`LAYOUT-006` made the navbar give up the name. It did not make the *pages*
give it up. With a display name that is one 80-character unbroken run
(the form's own ceiling), measured on a release build of `188056e`:

| Page | 320 | element | shape |
|---|---|---|---|
| `/today` | **385** | `p.text-sm.text-base-content/70.mb-4`, `components/me.rs:254` — the subtitle quoting the name | **B** |
| `/settings` | **385** | `p.text-sm.text-base-content/70.mb-6`, `components/settings.rs:~99` — the subtitle quoting the name | **B** |

A 40-character run gives 55 at 320 on both. A name with spaces gives 0
everywhere — which is why `LAYOUT-006`'s package measured 0 with its
80-character name, and why nobody saw this before. Injecting `break-words` on
the culprit `<p>` takes both to **0**; the box is the right width and only the
text overflows it. Confirmed by a text-range walk, because a box walker cannot
see shape B at all.

**Both pages are in the gate's list, and the gate is green on them.** Its
fixture's display name, `"Overflow Gate Fixture"`, has a space at every point.
This is `BROWSER-002` §1 again, one field over: an input that can carry an
unbroken run and a fixture that does not carry one there.

The navbar is not a site here. Its `<span>` text runs to 852 px inside a
269 px box, and `truncate`'s `overflow: hidden` keeps that off the document.
That is `LAYOUT-006` working.

## 2. What to do

- **Two shape-B fixes**: `break-words` on each subtitle `<p>`. One-line
  reference to the rule in `components.rs` at each; no copy of it. Classify
  by injection first anyway — the cost is a minute and the rule says so.
- **The gate's fixture display name gains an unbroken run**, the way the title,
  project name and team name did in `BROWSER-002`: ordinary spaced words plus
  a 64-character run, inside the 80-character ceiling (for instance
  `"Gate Fixture "` + 64 hexadecimal characters = 77). **Watch it go red
  first** on `today` and `settings` — expect 320 and 390, possibly wider —
  then fix, then green. Two commits, as `LAYOUT-005` did.
- The navbar must still show something a user recognises at 320 with the new
  name: report the rendered text at 320 / 360 / 390, as `LAYOUT-006` did.

## 3. What must not change

- **Nothing else in the fixture.** The title, project name, team name and the
  long email stay as they are.
- **The navbar.** `LAYOUT-006`'s classes are untouched; the new fixture name
  is longer, and the button still measures 44 px high with the name
  truncated inside it — measure it, since plant D depends on it.
- **Ordinary subtitles.** Pixel-identical at 768 and 1280 for a spaced name.

## 4. Verification

- `/today` and `/settings` at **0** at 320 / 360 / 390 / 768 / 1280 with the
  80-run name and with the new fixture name.
- Gate **70/70**, exit 0, 0 non-local requests; the red run in the package
  with its cells.
- `browser-checks/README.md`'s fixture section names the display name as the
  fourth deliberate fixture property.
- `DEC-007` **256**, three consecutive workspace runs, `fmt`, `clippy`.

## 5. Escalate rather than deciding

- **If the new fixture name turns any other page red.** Report the cell; the
  display name may be quoted somewhere I did not look.
- **If either `<p>` is not shape B.**
- **If the navbar's rendered text at 320 is unrecognisable** with the new
  name (nothing but the run's first characters). Then the fixture name's
  spaced part goes first, which the suggestion above already does; if that
  is not enough, say so.

## 6. Exit condition

Two subtitles at 0 at five widths, the gate at 70/70 having been watched red
on `today` and `settings` first, the fixture's fourth property documented,
`DEC-007` at 256, three consecutive workspace runs.

---

**Who holds what**: dev team. **Depends on**: nothing. **What's next**: review
request.
