# `CLR-003` round 2 — The exclusion that is two words

**Round 1 is accepted.** The last two headings are repaired, the failing
population is **zero**, `NFR-A11Y-005` is amended with its own *"the theme's
own tokens were never the problem"* sentence corrected rather than deleted,
`§10.61` is opened and closed with the measured figures, and the guard
**fails on the shape it bans** — which I verified by re-planting
`btn btn-outline btn-error btn-sm` in a throwaway worktree rather than
taking the quoted failure: baseline 3 passed, planted FAILED, naming the
file and the class value.

**The round also corrected my own arithmetic** — my §3 said *"thirteen
repairs"* where the series' total is **fifteen** — and recorded the
correction in the entry instead of quietly fixing the number. Third of
mine this series.

**One item. Then the series closes.**

---

## 1. The exclusion list has a hole shaped like its own defect

`NAMED_EXCLUSION_CLASS_VALUES` is
`["text-5xl font-bold text-error", "text-primary"]`, matched as a whole
class value.

The first is long and distinctive, and anything that matches it is a 48 px
bold error number, which passes on the large-text route anyway. **The second
is two words**, and `class="text-primary"` is precisely what a future site
would write for ordinary body text in primary blue — **4.21:1**, the exact
violation this series exists to prevent. It would be excluded by name, and
**both** of the new tests would stay green: the ban skips it, and
`every_named_exclusion_still_matches_a_live_class_value` asserts only that
each exclusion matches *something* (`found[i]` is a bool,
`contrast_scan/tests.rs:96`).

The `alignment_scan` precedent this cites works *because* its literal is a
long opening tag. This one cannot carry that weight.

---

## 2. What to do

**Assert the exact count of live matches per exclusion** — the `css_scan`
population shape, which this project already trusts for precisely this
reason:

- `"text-primary"` → **2** (the brand bullets at `auth.rs:15` and
  `layout.rs:211`);
- `"text-5xl font-bold text-error"` → **1** (`error_page.rs:18`).

Re-derive both counts yourself before writing them. A third occurrence then
fails **by name**, and whoever adds it has to justify it or exclude it
deliberately — rather than inheriting an exemption written for a glyph.

**Plant it**: add a third `class="text-primary"` site in a scratch copy of
the tree (not on `main`), watch the count assertion fail, remove the plant,
re-run green. The failure message should say which exclusion drifted and
from what to what — a count that moves silently is the thing this fix is
against.

**Record the reason beside the numbers**, as every population assertion in
this codebase does: these are decorative glyphs and one large-text number,
and the count is what stops the exemption spreading.

**Do not** widen the ban, add a family, change the theme, or touch anything
else. If the recount disagrees with the figures above, **your measurement
wins** and that is the finding.

---

## 3. Acceptance

1. Exact-count assertions for both exclusions, counts re-derived rather than
   copied, with the reason recorded beside them.
2. **Planted and watched failing**, then the plant removed and the suite
   re-run green. Quote the failure message.
3. `cargo test -p peisear-web --lib` — **248** as it stands; say what it
   became and why (an added assertion inside an existing test adds no test).
4. `css_scan`'s population: **`(287, 3)`**, unchanged — verify, do not
   assume; nothing here touches a component class.
5. The rest of the gate line per `README.md §5`, with real exit codes,
   including the `DEC-007` block (**687**).
6. **No `CHANGELOG.md` entry**: this changes a test's own strictness, not
   anything a reader of the product can see. Say so rather than adding one
   for symmetry — `LBL-001` round 2 set that precedent in series `063`.
7. Privacy floor per artefact; scratch under `.git-exclude/tmp/`.
8. **Review request** at
   `.git-exclude/review-request/CLR-003-round-2-the-exclusion-that-is-two-words/README.md`.

**After this round, series `064` is closed** and `0.66.0` goes to a release
candidate against exit criteria 1–6, which is the architect's to write. No
round cuts a release.
