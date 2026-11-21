# `SIZE-003` round 2 — The colour the alert owns

**Round 1 is accepted.** Six sites raised, the final census stated, the
rounds table reconciled, and what the series leaves behind named rather than
implied. **Two things come back**, and then series `063` closes.

**And one thing this round should know it got right**: it measured the two
colour spellings *in isolation* and reported a **2.4-point** difference where
every earlier round — and both of the architect's handoffs — had carried
`§10.60`'s *"composite identically, within pixel noise"* forward as if it
were general. A measurement beat an inherited sentence. That is the series
working, and it is also why item 1 below goes the way it does.

---

## 1. Revert the spelling at `settings.rs:81`; keep the size raise

`§10.60` never permitted that conversion generally. **It attached a
condition** — the two spellings composite identically at single nesting
depth — and this round **measured that the condition is not met here**, then
converted anyway because both readings clear 4.5:1.

By this round's own numbers, the conversion:

- **lowers** the contrast of a warning a reader is being asked to act on,
  from **9.01:1** to **6.61:1**;
- **changes the hue**, replacing `alert-warning`'s own content token
  (`--wac`, near-black in the pinned `corporate` theme — confirmed in
  `daisyui.min.css`: `.alert-warning{… color:var(--fallback-wac,…)}`) with
  the page's base text colour at 80 %. That token is a component contract,
  not an incidental colour;
- buys **guard visibility** — a future dimming becomes refusable — at the
  price of a measured, immediate legibility loss, **in a series whose whole
  purpose is legibility.** The wrong direction of trade.

**So: `text-base-content/80` → back to `opacity-80`, and the `text-sm` raise
stays.** Then:

1. **Re-measure** the site's contrast after the revert and report it. It
   should return to ~**9.01:1**; if it does not, something else moved and
   that is the finding.
2. **Record the exception at the site**, in one short comment: this element's
   colour comes from `alert-warning`'s own token, so the guarded spelling is
   a colour change rather than a spelling change, and `§10.60`'s condition
   fails here. A later sweep of the bare-`opacity` set must not convert it
   again, and the comment is what tells it so.
3. **Expect `css_scan`'s population to move back**: `(288, 3) → (289, 3)`,
   because `opacity-80` re-enters `components/` as a distinct token at that
   one site. **Verify, do not assume** — and if the number is anything else,
   stop and report.

---

## 2. Fix the module doc this round found — it is worse than it looked

`css_scan.rs:56–63` says *"two selectors"* where there are now **three**,
and it cites `` [`tests::every_allowlist_entry_is_queried_by_some_static_js`] ``
— **an intra-doc link to an item that no longer exists**, since `SIZE-002`
renamed that test to `…_by_some_consumer`. Nothing fails today only because
no job runs `cargo doc`.

Reconcile that paragraph with the local doc comment above
`JS_HOOK_ALLOWLIST`, which `SIZE-002` already got right: three entries, two
kinds of consumer (`static/*.js` and `browser-checks/*.mjs`), and the live
test name. **`SIZE-002` introduced the drift and the architect accepted that
round without catching it** — it is two sentences in a file this series has
already opened, so the series fixes it rather than leaving it for someone to
stumble on.

---

## 3. Acceptance, and then the series is closed

1. The spelling reverted, re-measured, and the exception recorded at the
   site.
2. `css_scan.rs`'s module doc reconciled — count, consumers, and the live
   test name, with the intra-doc link resolving.
3. **Census unchanged at 38** — nothing here touches a `text-xs` site.
   Re-derive it and say so.
4. **`(289, 3)`** confirmed by running the scan, with the reason in the
   assertion's own comment as every round in this series has done.
5. **`CHANGELOG.md`**: the `[Unreleased]` entry for `SIZE-003` currently
   describes the conversion. Correct that sentence — what ships is the size
   raise, and the colour stays as it was; released prose should not describe
   a change the tree does not contain.
6. **The gate line in full**: the `DEC-007` block's count with real exit
   codes (**687**), `fmt`, `clippy` both variants, `contrast_scan`,
   `touch_target_scan`, `alignment_scan`. **No browser gate is needed** —
   nothing here changes rendered markup beyond one class token's colour, and
   `overflow-gate.mjs`/`disclosure-marker-gate.mjs` both passed on it this
   round; say that rather than running them for form.
7. **The series' closing stands as written** in round 1 — the rounds table,
   the final census, and what is left behind. Add only the one line this
   round changes: the bare-`opacity` set's second condition (does the
   element's colour come from a component token?), which the architect has
   recorded against the candidate in `ROADMAP.md`.
8. **Privacy floor per artefact**, per `README.md §8.7` — and note that
   round 1's own contrast PNG went to `/tmp`, which is outside the
   repository and outside the project's scratch rule; write any such
   artefact under `.git-exclude/tmp/` instead, and delete it when the round
   closes.
9. **Review request**:
   `.git-exclude/review-request/SIZE-003-round-2-the-colour-the-alert-owns/README.md`.

**After this round, series `063` is closed** and `0.65.0` goes to a release
candidate against exit criteria 1–6, which is the architect's to write. **No
round cuts a release**: `DEC-056`'s four legs run on the owner's approval at
criterion 7 and nowhere else.
