# CSS-002 — a class with no rule

**Series**: `059` (unit two, user workflows). **Blocked on**: nothing.
**Opens**: `§10.52`'s own unfinished half.

## 1. What already happened, so this round does not redo it

`CSS-001` (`838fb24`, 2026-11-07, architect, directly applied) fixed the
defect and both of its causes: `tailwind.config.js`'s content glob now
reads `crates/peisear-web/static/*.js`, `style/tailwindcss/README.md`
and the config comment name the file the binary actually serves, and
`crates/peisear-web/static/tailwind.css` is regenerated — **zero rules
removed, five added**, banner unchanged.

**This round adds no CSS and changes no component.** It builds the one
thing `CSS-001` deliberately did not: something that runs and fails when
a component writes a class the shipped stylesheet has no rule for.

## 2. Why a README sentence is not enough

`style/tailwindcss/README.md` has said *"a Tailwind class was added and
the build is stale"* since `ASSET-001`. That sentence was true, correct
and useless for three weeks, because the procedure it documented wrote
to `static/tailwind.css` — a path `FIV-007` had removed — and nothing
compared what a component asks for against what the product serves.
`cargo build` runs nothing in `style/tailwindcss/`, by design
(`DEC-051`), and that design is right: a self-hoster needs no Node. The
consequence is that the stylesheet is the one shipped artefact with no
reader at all. `static/*.js` has `static_js_scan`,
`undo_toast_attachment_scan` and three browser gates; the CSS has a
sentence.

## 3. What to build

A `css_scan` module in `crates/peisear-web/src/`, in the shape
`docs_scan`, `spec_scan`, `contrast_scan` and `touch_target_scan`
already have — a `--lib` test over the repository's own files, no
browser, no CI job of its own.

**The rule, one sentence**: *every class token a component writes must
have a rule in one of the three stylesheets the binary serves, or be
named in this file's own allowlist of JavaScript hooks.*

**Where the tokens come from** — bounded deliberately, and the bound is
the design:

- files: `crates/peisear-web/src/components/**/*.rs` only;
- literals: `class="…"`, `grow("…")` and `grow_text("…")`;
- a literal containing `{` or `}` at a token position is skipped — it is
  interpolated and this scan cannot know its value;
- Rust line-continuations (`\` at end of line inside a string) are
  joined before splitting on whitespace. **This is load-bearing**: a
  scan that skips them reads `\` as a class and misses every token on
  the continued line.

**Where the rules come from**: `crates/peisear-web/static/tailwind.css`,
`daisyui.min.css` and `app.css`, read from `CARGO_MANIFEST_DIR` — the
same resolution the three existing `static/`-readers use, and the whole
reason they survived `FIV-007` while the two repo-root-relative readers
did not. **Do not resolve either side from the repo root.**

A token is present if the escaped selector (`.md\:flex`, `.w-1\/2`) is
followed by a selector or declaration boundary — `{ , : > + ~ [ ) .` or
whitespace — so `.btn` does not match `.btn-ghost`.

**The allowlist**: `issue-card` and `js-status-form`, both in
`components/issues.rs`, both selectors `static/*.js` queries and neither
intended to carry style. Each entry carries a comment naming the file
that queries it. **An allowlist entry that nothing queries is itself a
defect** — assert that each name appears in at least one
`crates/peisear-web/static/*.js`.

## 4. Acceptance

- `the_repository_satisfies_the_live_rule` — on the real tree, over the
  real stylesheets: **291 tokens examined, 2 allowlisted, 0 unexplained**.
  The population count is asserted, not implied: a scan that examined
  nothing must not be able to pass (`§10.36`).
- A plant, shown red and then green, and it must be **this** plant:
  delete `.md\:flex{display:flex}` from
  `crates/peisear-web/static/tailwind.css` and run the scan. It must
  fail **naming `md:flex` and `components/layout.rs`** — not "N tokens
  unexplained". Restore and re-run.
- A second plant for the allowlist's own assertion: rename
  `js-status-form` in `static/dm.js` only, and the allowlist entry must
  fail as unused.
- Unit tests over the extractor itself, on literals written in the test:
  a continued string, an interpolated `class=` expression, a
  `grow_text(MENU_FOCUS_RING)` constant reference (which this scan
  cannot follow — assert that it is skipped, so the limit is pinned
  rather than assumed).

## 5. Bounds — state them in the module's own doc comment

This scan does **not** see: classes built by `format!`; `class:foo=`
bindings; classes returned as strings from other crates
(`peisear-core::project_health`'s badge classes, which is exactly why
`tailwind.config.js`'s content glob covers `crates/**`); classes written
in `static/*.js`; and `grow_text(CONST)` call sites. Name all five. A
guard that implies more reach than it has is `§10.36`'s own class of
defect, and this project has paid for it before.

## 6. What this round must not do

- **No new CI job.** It is a `--lib` test; `test.yml`'s existing
  `peisear-web --lib` block runs it, and `DEC-007`'s count moves by
  however many tests are added — state the from and the to, measured
  with the runner, never a grep.
- **No regeneration of the stylesheet**, and no edit to
  `style/tailwindcss/`. `CSS-001` left those correct.
- **No widening to `handlers/**` or to the other crates.** The 291
  tokens above are the measured population of the bound as written; a
  wider bound is a separate decision with its own measurement.
