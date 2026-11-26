# Changelog

All notable changes to peisear are documented in this file. **This file holds
the current series only**; older series are archived under
[`changelog/`](changelog/) and linked at the end. How release notes are written
and found is in
[`docs/development/changelog-and-releases.md`](docs/development/changelog-and-releases.md).

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Security

- **Pressing "Log out" now actually ends the session, everywhere.**
  Until now, it only removed the browser's own copy of the
  credential — anyone who still had it (a copied cookie, a browser
  left signed in) could keep using it for the rest of its normal
  week-long lifetime. Logging out now invalidates that credential on
  the server, immediately, on every device signed in as that
  account.

### Changed

- The account menu now says, next to "Sign out", that it ends
  sessions on every device — the one thing about logging out that
  used to be a surprise.
- Signing in again right after signing out works immediately, with
  no delay.

## [0.70.0] — 2026-11-26

**The colour the browser actually paints.** Four releases taught this
project's own checks to read colour out of the source: a class name, a
family of theme colours, a stylesheet rule, a border setting. **Every one of
them reads what was written down. None of them asks the browser what colour
ended up on the screen** — and the project had already learned, on a
navigation bar that read as visible in the file and was invisible in a real
window, that those two answers are not the same.

**This release asks the browser.** Every page the product's own test
fixture can build is now rendered at five window widths and the real colour
is read back out of the picture, for text and for the outline of a control
alike. **It found one place where a reader would have struggled**: the
highlighted half of the board/list switch on a project. Everything else it
flagged was a colour this project had already looked at, decided about, and
written the reason down for — which is the result that says the new check is
measuring the product rather than arguing with it.

### Fixed

- **Four releases taught the specification's own static check to read a
  class name or a stylesheet rule for colour contrast. None of them
  ever asked a browser what colour it actually painted.** A new
  check now renders every page the product's own test fixture builds
  and reads the real colour back — covering both text contrast and
  the contrast of a control's own border and focus outline together,
  in one pass. It found one real site this way: the selected view on
  a project's board/list toggle read below the readable threshold,
  fixed by reusing the same highlighted-tab look already used
  elsewhere in the product rather than inventing a new colour.
- The one list naming which two sites are deliberately excluded from
  the contrast check now has a single home that both the specification
  check and the new rendered check read, instead of being duplicated.
- **These release notes start a new series, so the previous ten move
  out.** `0.60.0` to `0.69.0` are now in
  [`changelog/0.60-0.69.md`](changelog/0.60-0.69.md), moved
  word-for-word, and this file holds `0.70.0` onward. The move was
  checked by comparing the moved text against the previous release's own
  copy of it: 959 lines, identical to the byte.

### What this release does not fix

- **The new check sees only the pages the test fixture can build.** Four
  situations the product can really be in — a finished sprint with no work
  in it, a team with no projects, an unassigned item, a day with nothing
  planned — cannot be set up by that fixture, so no colour on those screens
  has been looked at this way. Known, written down, and the reason the older
  source-reading checks stay in place rather than being replaced.
- **A tick in a checkbox is not measured by anything.** Once a checkbox is
  ticked, the filled square is what a reader sees as its edge, and the new
  check deliberately steps past the edge it had when empty. The filled
  square itself is measured by no check. It was measured by hand for this
  release and is comfortably readable; what is missing is something that
  keeps watching it.
- **Fading text by fading the whole element is still unreadable to both
  checks.** Nineteen places in the product dim a piece of text by dimming
  everything inside it, which neither the source check nor the new rendered
  one can account for.
- **The lines around cards and panels are still treated as decoration**,
  the previous release's decision, left exactly where it was rather than
  quietly reopened by a new check that could now reach them.
- **A documentation check cannot see links written on internal items**, and
  two such links are broken today. Not caused by this release and not fixed
  by it — a check whose reach is narrower than its name.
- **The smaller text and size items remain**: seventeen pieces of small
  print still at 12px, nine links invisible until pointed at, and the
  small-control size tier.
- **One requirement still says its own evidence does not hold.** The
  human-language explanation on the health view is implemented and the test
  meant to prove it does not really test it. The status says *unverified*
  and stays that way.
- **Moving release notes into an archive breaks the links written inside
  them.** A link written in a release note points at a path from this file's
  position; once the note moves one directory deeper, it no longer resolves.
  **Four links in the notes moved this release are broken by the move**
  (twice to the install guide, once to the build guide, once to the previous
  archive), and **three in the previous archive have been broken the same way
  since it was written on 1 November** and went unnoticed. The rule for
  these archives says a moved note is moved word-for-word and an archive is
  never edited after it is written, which is exactly what makes the breakage
  unavoidable without changing that rule — so the links are left broken,
  counted and reported here rather than quietly repaired. Nothing checks for
  this, which is the other half of the problem.
- **Two earlier releases' tags do not point at their notes.** Every release
  tag is supposed to carry a link to the notes for that exact version, so
  that someone landing on the tag can find them. `0.68.0`'s and `0.69.0`'s
  carry nothing. They are left alone on purpose — those versions are already
  published, and changing a published tag is worse than the gap it closes.
  This release's tag carries the link.
- **The documentation site has not published since 18 November**, stuck in a
  hosting state its own interface refuses to cancel.

## Earlier series

Older release notes are archived, one file per series of ten minor
versions:

- [0.60 to 0.69](changelog/0.60-0.69.md)
- [0.50 to 0.59](changelog/0.50-0.59.md)
- [0.40 to 0.49](changelog/0.40-0.49.md)
- [0.30 to 0.39](changelog/0.30-0.39.md)
- [0.20 to 0.29](changelog/0.20-0.29.md)
- [0.10 to 0.19](changelog/0.10-0.19.md)
- [0.1 to 0.9](changelog/0.1-0.9.md)
