# FIV-004 — the page a newcomer lands on

**Target**: 0.53.0, last. `RFC 017` proposals **3** and **5**.

**Last for a reason.** This round writes down what the other three
produce. An Install page written before the download URLs and the
`docker run` line are real is a page describing an intention, and the
screenshots beside it would be of a first run that no longer behaves
that way.

---

## §1 — two pages where there is one

`docs/getting-started/installation.md` is **48 lines and all of them
are about building from source**. It becomes two pages:

- **Install** — download for your platform, or `docker run`, or
  `cargo install peisear` for people who already have Rust. **Each
  path ends the same way: *open `http://localhost:3000`*,** with the
  first-run screenshots beside it.
- **Build from source** — today's 48 lines, **moved intact**, where a
  contributor will look for them. Nothing is lost; it stops being the
  first thing a user meets.

**Three paths, one ending.** The ending is the point of the RFC: the
reader should not have to work out which of three sets of next steps
applies to them.

**`cargo install peisear` is third, not absent.** It is the right path
for someone who already has Rust and the wrong first instruction for
everyone else.

## §1b — three sentences that are wrong on the way in

**Added 2026-10-14**, from walking Installation and the Quick Start as
a newcomer. Each is small; each is on the path this round owns.

1. **`docs/getting-started/README.md:3`** — *"Three steps from tarball
   to working kanban board."* **There is no tarball** (that is
   `FIV-002`'s entire premise), and the three steps it then lists begin
   with *Install Rust*, not with unpacking anything. Wrong twice in one
   sentence, and it is the first sentence of the section. After
   `FIV-002` a tarball exists — **but the three steps still will not
   start from one**, so rewrite the sentence against the Install page
   this round creates rather than deleting one word from it.

2. **`installation.md:26-30`** — Option B tells a newcomer to
   `sudo ln -sf /usr/bin/rustc-1.91 /usr/local/bin/rustc` (and the same
   for `cargo`), **shadowing the system compiler machine-wide**, pinned
   to a version that will age out of trixie. For a first-contact
   document that is a large irreversible-feeling action in exchange for
   avoiding `rustup`. **My recommendation: Build from source keeps
   `rustup` as the one path, and the distro packages become a short
   note without the symlinks** — but this is a judgement about a
   reader, so argue it if you see it differently. Whatever you decide,
   **the symlink instruction does not survive unchanged.**

3. **`first-run.md:23`** — *"Subsequent runs are instant."* They are
   not, after an edit, in `--release`. That page's audience at that
   point is the *"To read or change it"* path, which is exactly the
   audience that will edit something. One honest clause.

**None of these is a product defect, and that is why they lasted.**
They are the class `DOCS-003` through `DOCS-008` were about — *the
documents outside this repository never get swept* — reaching the one
set of pages this release exists to make correct.

## §2 — the line of output

`crates/peisear/src/main.rs:58` logs `listening addr=0.0.0.0:3000`
through `tracing`. It is correct and it asks the reader to translate
`0.0.0.0` into `localhost` — **barrier 5** in the RFC's own table.

**Print, once, on a successful bind, in plain words: the URL to
open.** And **when the bind is not loopback, say that the instance is
reachable from the network** — after `FIV-001` that is no longer the
default, so it is a sentence for the operator who chose it and for the
container, where it is correct.

Two things to get right:

- **It is a successful-bind message.** `main.rs` binds and then logs;
  keep that order, so the line is never printed by a process that
  failed to start.
- **It is for a person, not for a log aggregator.** The existing
  `tracing::info!(%addr, "listening")` is the structured record and
  has its own consumers — **decide whether the plain line replaces it
  or joins it, and say which and why.** Two lines saying the same
  thing to two audiences is defensible; two lines saying it to the
  same audience is noise.

## §3 — the rules this round runs into

- **`docs_scan` rule 2** reads `installation.md` for the MSRV
  sentence (`**X.Y or later**`) and its Verify block's
  `# rustc X.Y.x` / `# cargo X.Y.x` lines. **Those belong with Build
  from source.** Splitting the page without moving the rule's needle
  is a test failure, which is the guard working — fix the scan's
  `Input`, do not weaken the rule.
- **`docs_scan` rule 1b** reads `docs/README.md`'s *"{N} crates and
  what belongs in each"* line and rules 1/3/4/6 read other fixed
  documents. A new page under `docs/getting-started/` changes the
  book's structure: **`SUMMARY.md` must list both pages**, and
  `mdbook build` with linkcheck is what proves nothing dangles.
- **Every cross-reference to `installation.md`** in the repository has
  to land on whichever of the two pages is now correct, and *correct*
  is per-reference: `CONTRIBUTING.md`'s is almost certainly Build from
  source, `README.md`'s is almost certainly Install. **Enumerate them
  and state the count**; a link that still resolves but now points at
  the wrong half is the failure mode a link checker cannot see.

## §4 — the screenshots

The first-run screenshots sit beside the Install page's ending. They
are generated by `browser-checks/doc-screenshots.mjs`, and the
candidate's own gate confirms four of them present in the built book.

**`FIV-006` adds a `/today` capture and puts two images in `README.md`.**
If that round has landed before this one, the count above is five, not
four — **read the directory rather than this sentence**, and if it has
not landed, do not wait for it. The two rounds touch different files.

**Regenerate only if what they show has changed.** `FIV-001` changes a
default and a log line, neither of which is in a screenshot of the
application. **Check rather than assume, and say which you did** — a
regenerated screenshot that differs for no reason is churn, and a
stale one that should have moved is the `DOCS-006` failure this
project already has a register entry for.

**The privacy floor is a named exit condition for every image.** Any
screenshot produced or regenerated here must contain no local path, no
real address, no account that is not fixture data. Check each one and
say you did.

## §5 — specification work (`DEC-028`)

- **`NFR-CMP-002`** (self-hostable) is the entry the install story
  serves. Check whether its text still describes the paths that now
  exist; report rather than rewrite if it reads narrower than reality.
- **`external-design.md`** describes what the product presents. A
  startup line is output, not a screen — **say whether you think it
  belongs there.** This document has been wrong in both directions
  before; a reasoned "no" recorded in the review is worth more than a
  row added defensively.

## §6 — gates

- `DEC-007` from a fresh block, against `FIV-003`'s baseline. **Rule
  2's scan tests will move** with the page split; account for each.
- `fmt`, `clippy -D warnings`, `cargo doc` with broken-intra-doc-links
  denied.
- **`mdbook build` with linkcheck, and `SUMMARY.md` listing both
  pages** — a page that exists and is not in the book is a page nobody
  reads.
- **Walk the Install page as written, on each of its three paths**,
  on the artefacts `FIV-002` and `FIV-003` produced. Not "the
  instructions look right": run them, and report where a reader would
  stop.
- Four screenshots present in the built book.

## §7 — escalate rather than deciding

- **If rule 2's needle cannot follow the MSRV sentence** to Build from
  source without widening the rule's population. A wider rule is a
  design change; report the shape rather than adopting one.
- **If a path in the Install page cannot be walked** because its
  artefact is not yet real. That means this round started before its
  dependencies closed — stop and say so, rather than documenting the
  intention.

## §8 — exit condition

Two pages where there was one, the 48 lines moved intact and nothing
lost; three install paths with one shared ending; every
`installation.md` reference re-pointed at the correct half, with the
count stated; `SUMMARY.md` and the book clean; rule 2 following the
MSRV sentence with its tests accounted for; a plain line of output on
successful bind naming the URL, with a network warning when the bind
is not loopback, and a stated decision on whether it replaces or joins
the structured log line; **§1b's three sentences corrected, with the
symlink instruction's replacement reasoned rather than deleted**; the
screenshots checked rather than assumed,
each confirmed against the privacy floor; and **all three install
paths walked on the real artefacts**, with what a reader would see
reported for each.

---

# FIV-004 round 2 — the page promises one screen and shows five others

Round 1 is accepted (see the review). The split is right, all three
paths were walked on real artefacts, and the startup line does what
barrier 5 asked. One thing on the new page itself.

## §9 — "an empty project list", followed by five pictures of a full one

`install.md`'s *What you'll see* reads:

> Register an account, and you land on an empty project list:

and is followed by **five screenshots**: the board with issues, the
list view, an issue detail, a sprint plan, and `/today`. **Not one of
them is an empty project list.**

**This is the defect class the release exists to remove**, on the page
it exists to produce — a caption contradicted by the image beneath it
is `DOCS-006`'s own finding, and `DOCS-008` was a whole round about
images a stranger can read. A newcomer following Option A reaches a
screen that looks nothing like the five they were just shown, at the
exact moment they are least able to tell whether they did something
wrong.

**And four of the five are already on the next page.** `first-run.md`
carries `board`, `list`, `issue_detail` and `sprint_plan` — so the
book now shows the same four images twice, on adjacent pages, with the
second showing them *in the walkthrough that explains them*.

**My own `§4` invited this** by saying *"the first-run screenshots sit
beside the Install page's ending"* without saying how many or which.
Recorded as mine; the fix is still yours.

## §10 — what the page's ending is actually for

The Install page's job is **three paths and one shared ending**. The
ending is one moment: *the server is up and you are looking at it.*
One image serves that; five turn the ending into a gallery and bury
the `Next` link under it.

**Two ways to make it honest, and the choice is yours:**

- **Capture what the sentence promises.** The harness registers an
  account before it creates fixtures, so an empty `/projects` is a
  state it already passes through. A new `projects_empty.png` is a
  real option — `FIV-006` is closed, but the script it left is not.
- **Keep one image and make the sentence match it.** If the kept image
  is `board.png`, then the sentence is about what the product looks
  like once there is work in it, and *"Register an account"* needs to
  say so.

**Do not keep five**, and **do not keep an image whose own page is one
click away** unless you can say what the Install page's copy adds that
`first-run.md`'s does not.

## §11 — exit condition

`install.md`'s *What you'll see* carrying **one** image, with the
sentence above it true of that image — either a new capture of the
empty project list or a reworded lead-in, and **say which you chose
and why**; `first-run.md` unchanged; the duplication across the two
pages resolved and stated; `mdbook build` with linkcheck clean and the
built book's screenshot directory read rather than assumed;
`DEC-007` at **486**, which this should not move.
