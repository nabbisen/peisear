# FIV-005 — the binary that is not one

**Target**: 0.53.0. **Not in `RFC 017`'s five barriers** — found
2026-10-14, after the series was scoped, by reading the Quick Start as
a newcomer rather than as its author. `§10.41`.

**`README.md:52-56` is the first instruction a reader meets, and it
cannot produce a working instance:**

```bash
cargo install peisear
peisear
#   → http://localhost:3000
```

The server starts. The page has **no CSS and no JavaScript.**

---

## §1 — what is actually there

Three facts, each checkable in one command. **Check them rather than
taking them from here** — this round's whole subject is a claim that
was written down and never walked.

1. **`static/` is in no published crate.** It sits at the workspace
   root, outside every package directory, and no `Cargo.toml` in the
   workspace carries an `include`. **Measured 2026-10-14, all seven
   crates, `cargo package -p <crate> --list`: zero `static/` entries in
   each.** The `peisear` crate publishes seven files in total —
   `.cargo_vcs_info.json`, `Cargo.lock`, `Cargo.toml`,
   `Cargo.toml.orig`, `README.md`, `src/lib.rs`, `src/main.rs`.
   **Re-run it anyway and quote your own output**; a figure taken from
   a handoff is a figure nobody checked.
2. **The router resolves it against the current working directory.**
   `crates/peisear-web/src/app.rs:259` is
   `.nest_service("/static", ServeDir::new("static"))`, and its own
   comment says so: *"the working directory of the running binary. For
   typical `cargo run`-from-workspace-root usage…"*. That assumption
   is true for a contributor and false for every installed copy.
3. **There is no way to point it anywhere.** `config.rs` reads exactly
   `DATABASE_URL`, `JWT_SECRET`, `BIND_ADDR`, `COOKIE_SECURE` — no
   static path among them.

**What the reader gets**: `layout.rs:52-54` requests three
stylesheets, `layout.rs:90` and four components request `search.js`,
`board.js`, `plan.js`, `calendar.js`, `dm.js`. Eight requests, eight
404s. An unstyled login form, no drag-and-drop, no search.

**And the data lands wherever they were standing.** `DATABASE_URL`
defaults to `sqlite://data/app.db`, also relative. Run `peisear` from
`~` once and from `~/projects` later and there are two databases and
two accounts, with nothing on screen explaining why the first one's
issues are gone. After `FIV-001` the generated `jwt_secret` follows the
database, so it is two keys as well.

**The README contradicts itself about this, in the same file.**
Features says *"Ship one executable plus `static/` and you're live"*;
Design Notes says *"One binary deploy."* The first is today's truth and
the second is the promise. This round makes the second one true.

## §2 — the remedy, and what it must not become

**Embed `static/` into the binary at compile time** — `include_dir` or
equivalent — and serve the embedded set. The files stay in the
repository exactly where they are; what changes is that the binary
carries a copy.

**Why this shape and not the obvious alternative.** A `STATIC_DIR`
environment variable would also work, and it is the wrong answer here:
it adds a fifth configuration knob, makes the correct value something
the reader has to know, and leaves the failure silent when they get it
wrong. Embedding **removes** a way to be wrong rather than documenting
one, which is the direction this project has taken for migrations and
templates already — both are compiled in, and nobody configures them.
It is also what makes *"One binary deploy"* true rather than aspirational.

**The cost, measured before you start**: `static/` is 3.0 MB across
eight files, and **2.93 MB of that is `static/daisyui.min.css`** — 98%
of it, one unpurged vendored file. Report the binary's size before and
after. If the growth is unacceptable, that is an escalation (§6), not a
licence to purge Tailwind in this round.

**Keep a development path.** A contributor editing `app.css` should not
need to rebuild to see it. Whether that is a debug-only `ServeDir`
fallback, a feature flag, or nothing at all is **yours to decide and to
state in the report** — including "nothing at all, because the rebuild
is cheap", if you measure it and that is true.

**Not a Tailwind purge.** `daisyui.min.css` being 2.93 MB is a real
question and a separate one; `ASSET-001` (0.32.0) tracked that file
deliberately. Measure it, say what you think, do not act on it here.

**Not a change to what is served.** The eight files served today are
the eight files served after. `static_js_scan`, `undo_toast_attachment_scan`
and `dm_fallback_boundary_scan` all read `static/*.js` **from the
filesystem** (via `manifest_dir`, not via HTTP), so they keep working
unchanged — *confirm that rather than assuming it*, because a guard
that silently stops reaching its subject is `§10.39`'s class and this
project has had it twice.

## §3 — the rules this round runs into

- **`docs_scan` rule 3** compares `docs/operations/deployment.md`'s
  deployment manifest against the real `static/`. If the binary now
  carries those files, **the manifest's meaning changes** — it is no
  longer a list of what you must copy. Decide what rule 3 should assert
  after this change and say why; do not delete it to make a test pass.
- **`docs/operations/deployment.md`** itself tells an operator to ship
  `static/` alongside. Every such instruction has to become correct.
  **Enumerate them and state the count.**
- **`README.md`'s two contradicting sentences** — Features and Design
  Notes, §1 above. One of them changes.

## §4 — specification work (`DEC-028`)

- **`NFR-CMP-002`** (self-hostable) and **`NFR-REL-002`** (archive
  naming and layout) both describe what a deployment consists of.
  **`NFR-REL-002` is this round's to amend, not `FIV-002`'s** — see
  below. Say what the archive layout becomes when the binary is
  self-contained, and amend the requirement here, once.

**This round runs before `FIV-002` and `FIV-003`, and that is an
ordering, not a preference** *(added 2026-10-15)*. `FIV-002` §4 asks
whether `static/` belongs in the archive; `FIV-003` says its *"stage
carries the binary and `static/`"*. **Embedding changes both answers.**
Building either first means building it around a layout this round
invalidates, and amending `NFR-REL-002` twice in one release. This
section previously said *"coordinate rather than both editing it"* —
coordination between rounds that do not run at the same time is just an
order nobody wrote down.
- **`§10.41`** is open in `requirements.md` and is this round's to
  close — but only on evidence, which means §6's walk, not the build
  succeeding.

## §5 — gates

- `DEC-007` from a fresh block, against `FIV-001` round 2's baseline.
- `fmt`, `clippy -D warnings`, `cargo doc` with broken-intra-doc-links
  denied.
- **The walk, which is the point of the round**: `cargo install --path
  crates/peisear --root <a scratch prefix>`, then run the installed
  binary **from a directory that is not the repository** — `cd` into an
  empty scratch directory first — and load the board in a browser.
  **Report what renders**, not that the process started. A 200 with no
  stylesheet is the defect, and it does not show up in an exit code.
- **Before/after binary size**, stated in bytes.
- The three filesystem scans named in §2, confirmed still reaching
  `static/*.js`.

## §6 — escalate rather than deciding

- **If the binary grows by more than you think the project should
  accept.** Say the number and what you would do instead; the purge
  question and the "ship a tarball instead" question are both mine.
- **If embedding breaks a guard you cannot repoint without widening
  it.** A wider rule is a design change (`§10.39`), so report the shape.
- **If `cargo install peisear` turns out to work** — if `cargo package
  --list` shows `static/` and this entire handoff is wrong. Say so
  plainly and stop; being wrong in writing is cheaper than a round
  spent fixing nothing.

## §7 — exit condition

`cargo package -p peisear --list` quoted; the embedded set serving all
eight files from an installed binary run **outside the repository**,
with the board confirmed rendered rather than merely returning 200;
before/after binary size in bytes; the development path decided and
stated; `deployment.md`'s manifest instructions re-counted and
corrected with the count given; rule 3's post-change assertion decided
with reasoning; `README.md`'s self-contradiction resolved; the three
`static/*.js` scans confirmed still reaching their subject; and
`§10.41` closed **on the walk's evidence**, not on the build.
