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

---

# FIV-005 round 2 — the assets are never cached now

Round 1 is accepted (see the review). This is the one thing it traded
away, found by asking what `ServeDir` used to do that the new handler
does not.

## §8 — every page navigation re-downloads every asset

`static_assets::serve` sets exactly one response header,
`Content-Type`. **No `ETag`, no `Last-Modified`, no `Cache-Control`.**
`ServeDir` set `Last-Modified` and answered conditional requests; this
does not. Measured against round 1's own binary:

```
$ curl -D - -o /dev/null http://…/static/daisyui.min.css
HTTP/1.1 200 OK
content-type: text/css; charset=utf-8
vary: accept-encoding
content-length: 2930630

$ curl -H 'If-Modified-Since: …' …        # a conditional request
status=200 bytes=2930630                   # not 304
```

**A browser has nothing to revalidate with, so it refetches.** Every
page in this application loads three stylesheets and at least one
script from `/static/`: **~184 KB gzipped per navigation**
(`daisyui.min.css` 176,891 · `tailwind.css` 4,098 · `search.js` 2,856 ·
`app.css` 325), re-sent and re-decompressed on every page view, for
bytes that cannot change while the process is running.

**This is a real trade, not a slip.** Round 1 fixed a cold-start defect
— the app did not work at all when installed — and introduced a
warm-path one. The first is worse and the order was right. But *"the
pages feel slow"* is a first-five-minutes problem too, and this release
is the one that owns that.

**Why no gate saw it.** Every test asserts status, bytes and
content-type. **Not one asserts a header the asset pipeline depends
on**, so the suite could not tell a correctly cached response from an
uncached one — `§10.36`'s question, aimed at an HTTP contract.

## §9 — what the fix has to satisfy

Two constraints, and they pull against each other:

- **An upgraded binary must never serve its new page against the old
  CSS.** The URLs are not fingerprinted (`/static/app.css` is the same
  URL forever), so a long `max-age` alone would do exactly that, and
  the symptom — a half-styled page after an upgrade, cleared by a hard
  refresh the user does not know to perform — is worse than the slow
  load it replaces.
- **A repeat navigation must not re-send the body.**

**A strong `ETag` over the embedded bytes satisfies both**: content
that changes with the build changes the tag, and a revalidation that
matches costs a 304 with no body. Whether to pair it with
`Cache-Control: no-cache` (store, always revalidate) or a short
`max-age` is yours — **state which and why**. `Last-Modified` is the
weaker option here and says so by itself: embedded bytes have no
meaningful mtime, and the build time is not one.

**Compute the tag once, not per request.** Hashing 2.93 MB on every
request trades bandwidth for CPU and is not the point.

**Handle `If-None-Match` yourself if nothing in the stack does.**
Setting an `ETag` and still returning 200 with a full body is the
defect with extra steps.

## §10 — gates

- **A test per header this round adds**, asserting the response
  carries it — and **one asserting a matching `If-None-Match` returns
  304 with an empty body.** That second one is the whole round; the
  first without it is `§10.36` again.
- A test that **two different asset bodies produce two different
  tags**, so the tag is over content and not a constant.
- `DEC-007` against **475**, the review's own figure.
- **Re-walk round 1's installed-binary walk**, and report the wire
  bytes for a second navigation. Round 1 proved the page renders; this
  round has to prove it renders twice without paying twice.

## §11 — exit condition

`ETag` over the embedded content, computed once; a matching
`If-None-Match` answered **304, no body**, with a test; the
`Cache-Control` choice made and reasoned against the upgrade-staleness
constraint in `§9`; a test proving different content yields different
tags; `DEC-007` accounted for against 475; and the installed-binary
walk re-run with **second-navigation wire bytes reported**, not first.

---

# FIV-005 round 3 — the map and the lookup disagree about subdirectories

Round 2 is accepted (see the review). One defect, latent today and
certain the first time anyone organises `static/`.

## §12 — `ETAGS` cannot see a nested file; `get_file` can

```rust
static ETAGS: LazyLock<HashMap<&'static str, String>> = LazyLock::new(|| {
    STATIC_DIR.files().map(…).collect()
});
…
let etag = ETAGS.get(path.as_str())
    .expect("every file get_file can find has a precomputed entry in ETAGS -- both walk the same STATIC_DIR");
```

**They do not walk it the same way.** From `include_dir` 0.7.4's own
source:

- `Dir::files()` is `self.entries().iter().filter_map(as_file)` —
  **direct children only.**
- `Dir::get_file()` calls `get_entry()`, whose own doc comment reads
  *"Recursively search for a `DirEntry`"*.

So for `static/vendor/foo.js`: `get_file` returns `Some`, `ETAGS.get`
returns `None`, and **the handler panics on a request** — in release,
where the `expect` is the only thing standing there.

**This is not hypothetical, and the codebase says so itself.**
`static_js_scan.rs:193` already reads: *"a future `static/vendor/*.js`
or similar would…"*. That scan walks `static/` recursively on purpose;
`docs_scan` rule 3 compares the manifest against its real contents.
**Nothing forbids a subdirectory, and one module already plans for
one.** The first person to tidy eight flat files into folders turns a
page load into a panic.

**The `expect` message is the finding.** It states the invariant —
*"both walk the same STATIC_DIR"* — and the invariant is false. An
`expect` whose message argues for a guarantee the code does not make is
worse than an unwrapped `None`, because it tells the next reader not to
check.

**Remedy**: build `ETAGS` from the same recursive walk `get_file`
uses — `STATIC_DIR.find("**/*")`, or an explicit recursion over
`entries()`. **And then the `expect` is true**, so keep it and keep the
message, corrected.

**Test it with a nested file, not by reading the fix.** The existing
tests all use top-level paths, which is exactly why the suite is green
today. If the embedded tree cannot be given a subdirectory in a test
without adding one to the real `static/`, **say so** — that is a
finding about testability, and inventing a second `include_dir!` root
for tests is the kind of thing to raise rather than do.

## §13 — a 304 should carry what a 200 would

`not_modified` sets `ETag` alone. RFC 9111 §4.3.4 has the server send
the headers a cache would need to update its stored response —
`Cache-Control` among them. With `no-cache` the practical cost is near
zero, which is why this is a clause and not a section: **add it, or say
why not.**

## §14 — exit condition

`ETAGS` built by the same recursive walk `get_file` performs, with the
`expect` message left in place and now true; **a test that serves a
nested path**, or a stated reason the embedded tree cannot be given one
in a test; `§13` decided; `DEC-007` accounted for against **483**, the
review's own figure; and **rustdoc re-run after the last edit of the
round**, not before it — round 2's own `§4` is why that sentence is
here.
