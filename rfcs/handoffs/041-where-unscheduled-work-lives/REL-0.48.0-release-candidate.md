# REL-0.48.0 — release candidate

**Contents**: `ROADMAP-001` (`8ee0d2d`), `POST-001` (investigation, no code),
`NAV-002` (`2d6f20b`), the Release-title format with all seven existing
releases retitled (`5b1f606`), and the specification amendments (`71dad6c`,
`b339cc7`, `4cb9873`). **Every handoff reviewed and closed.**

**`DEC-028` is already done** — both specifications cover `0.48.0`
(`4cb9873`), including §3's sweep. **Depends on**: nothing outstanding.

**Do not tag. Do not publish. Do not create the Release.** Produce the
candidate and stop.

---

## 1. Ordinary cut

`Cargo.toml`, `Cargo.lock`, `CHANGELOG.md` only. **`0.47.0` → `0.48.0`.**

**No migration** — `0020` remains the most recent, the **ninth** release
running.

**Expected: 407**, from 405. Two tests, both `NAV-002`'s.

**Production source changed in two files**: `components/sprint_plan.rs` (the
row's new `id`) and `handlers/sprints.rs` (the fragment on two redirects).
**Nothing else** — no migration, no other handler, no `static/*.js`.

**The Release title is the bare `X.Y.Z` from this release on**, and that is
now written down: `docs/development/changelog-and-releases.md`, with the
fourth leg, its own command line, and a row in the *what checks this* table.
**Nothing for you to do** — it is the architect's leg — but the notes mention
it (§2.5) and you should know the format changed and why.

## 2. The changelog

**This release has an unusually honest shape: its subject is the project's
own record-keeping, and the one user-facing fix came out of noticing that
record was wrong.** Five beats.

1. **Lead with the fix, because it is the only thing a user feels.** Moving
   an issue between the sprint plan's columns used to return the reader to the
   **top** of a page they were partway down — **1292 px of 3078** on a phone,
   with a row they had been reading ending 1990 px below the fold. The move
   redirects now name the row's own element, so **the row you acted on is in
   view**, in whichever column it went to.
2. **And say what it is not.** The previous scroll position is **not**
   restored, deliberately: after a move the row is in the other column, so
   restoring it would return the reader to where the row no longer is.
   **Write that explicitly** — it is the difference between the promise this
   keeps and one it does not.
3. **The fix splits, and the half that cannot be fixed this way matters.**
   On the calendar a moved block **leaves the day being viewed**, so a
   fragment has nothing to point at; the existing *"Moved to …"* banner is
   that surface's answer. Say that **a uniform "fragment on every redirect"
   change would have looked solved and done nothing for the case that raised
   this.**
4. **Why any of this was measured at all**: `ROADMAP.md` claimed its *current
   plan* was a release sequence **ending at `0.27.0`**, twenty minor versions
   behind. The way work gets chosen had changed — handoffs, the defect
   register, the per-release specification sweep — and the roadmap never
   moved with it. It is retired in place, with an **Unscheduled work** table
   added for quality gaps on shipped work that neither the register nor the
   deferred-features list covers. **The symptom is worth one sentence**: the
   list was being kept in the architect's head, and one item on it named a
   requirement that says something else entirely.
5. **The Release title is now the bare version**, documented rather than
   habitual, and **the seven existing Release pages were retitled** to match.
   Say that retitling touches no tag, no commit and no published crate — so
   unlike a tag rewrite it leaves nothing mismatched.

**Two things the notes must not claim.** That scroll position is preserved —
see beat 2. And that the calendar case is fixed; it is **explicitly not**, and
its remaining residual is in `ROADMAP.md`'s table as the owner's call.

## 3. Verification before the candidate commit

- **`DEC-028`: already done** (`4cb9873`). Confirm both specifications read
  `Covers release: 0.48.0` with a `0.48.0` baseline row each.
- **The sweep**: run by the architect; **its sixth pass found nothing new**
  and names the two claims it checked before saying so. Confirm rather than
  re-run, and **report explicitly**.
- `DEC-007` **407**, three consecutive runs, **from a block extracted out of
  the current `CONTRIBUTING.md`**.
- **All three browser scripts on the extracted tree**, in the job's order —
  overflow gate **120 cells**, trap script, then `drag-outcome-gate.mjs`.
- **`mdbook build` from the extracted tarball** — eighth release with the
  site, and `ROADMAP.md` was restructured, so the link check earns its keep
  here.
- `fmt`, `clippy --workspace --all-targets -- -D warnings`, `rustdoc-links`
  with **seven** `Documenting` lines and the private-link count **eight**.
- **No migration to exercise**, but start the candidate on a database built by
  the released **0.47.0** binary and confirm it comes up at 20 with nothing
  applied. **Build from the tag's own commit** — `0.47.0` is `6802f95`, one
  past its candidate `0ce629c`, the same distinction you have now caught for
  three releases running.
- `cargo publish --workspace --dry-run` from a **detached worktree of the
  candidate commit** — seven crates, **every `warning:` line read**.
- **A behavioural check, because a user-visible behaviour changed.** On the
  built artefact at a phone width: scroll a long sprint-plan backlog, move a
  row, and confirm you land with **that row in view**. `POST-001`'s harness
  measured `scrollY 1292 → 2177` with the row visible; **you need only confirm
  the row is on screen**, not reproduce the number.

## 4. Escalate rather than deciding

- **If the count is not 407**, or the overflow gate is not 120.
- **If the behavioural check lands you anywhere the moved row is not
  visible** — that is the release's one claim failing.
- **If `mdbook build`'s link check fails** on anything in `ROADMAP.md`.
- **If the sweep turns up anything**, given mine found nothing.

## 5. Exit condition

A candidate commit on `main` at **0.48.0**; a changelog whose lead is the
phone fix and which says plainly what it does **not** promise; 407 green three
times from a freshly extracted block; three browser scripts clean; the book
building from the tarball; the 0.47.0 database coming up clean from a binary
built at the tag's own commit; a dry run of seven crates with every warning
read; and **the moved row demonstrated in view on the artefact**. Then stop.
