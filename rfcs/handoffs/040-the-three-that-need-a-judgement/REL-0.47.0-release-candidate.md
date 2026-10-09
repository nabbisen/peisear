# REL-0.47.0 — release candidate

**Contents**: `PER-001` (`e954a58`), `HLT-003` (`edc6e78`), `GATE-005`
(investigation, no code), `GATE-006` (`89d3c5b`), and the specification
amendments (`2320762`, `38a3eba`, `0f0b0ea`). **Every handoff reviewed and
closed. `§10.35` closes with this release.**

**`DEC-028` is already done** — both specifications cover `0.47.0`
(`0f0b0ea`), including §3's sweep. **Depends on**: nothing outstanding.

**Do not tag. Do not publish. Do not create the Release.** Produce the
candidate and stop.

---

## 1. Ordinary cut

`Cargo.toml`, `Cargo.lock`, `CHANGELOG.md` only. **`0.46.0` → `0.47.0`.**

**No migration** — `0020` remains the most recent, the **eighth** release
running.

**Expected: 405**, from 392. **And the composition matters more than usual**:
`PER-001` +10 (nine unit tests in a new `#[cfg(test)]` module plus one HTTP
test), `HLT-003` +3, **`GATE-006` +0** — it is a browser gate, outside
`DEC-007`'s inventory entirely. **A release that added a gate and moved the
test count by zero for it is worth a sentence**, or a reader will think the
gate is in the 405.

**Production source changed in exactly one way**: `components/me.rs` gained a
`#[cfg(test)]` module. **No component markup, no handler, no route, no
migration, no `static/*.js`.** Check and say so — `git diff 0.46.0..HEAD --
'crates/*/src' static` should show that one file and nothing else.

**The browser job now runs three scripts.** `drag-outcome-gate.mjs` is the
third step, on port `4175`. Run all three from the extracted tree **in the
job's own order**, and confirm the third starts and tears down its own server
after the second has.

## 2. The changelog

**This release closes a class and ships no behaviour. Both halves need
saying, and the second must not be dressed up.** Five beats.

1. **Lead with the one thing a user could notice, which is a guarantee
   rather than a change**: on the **calendar day view** and the **sprint
   plan**, a drag's outcome announcement is now **measured** — a browser gate
   drives a real drop and reads the live region's text. Anyone relying on an
   announcement after a drag has that pinned on both surfaces for the first
   time. **Nothing moved on screen.**
2. **`FR-PER-006`: one cited test for a precedence chain**, and that test
   covered **the case where nothing applies.** Say the code has **four**
   conditions where its own sentence names three, and that **a chain is only
   a chain where two conditions compete** — which is why each tie-break was
   verified by **inverting the corresponding pair of arms** rather than by
   asserting a single outcome.
3. **`FR-HLT-007`: two of five indicators were covered**, by a test whose
   **plural name** made the entry read like a sample for six releases. And
   widening it **found a rule nobody had written down** — a non-empty basis is
   not enough for a basis link, because an indicator at `Good` renders no
   explanation row. **Found by running the test and reading the failure**,
   which is the sentence to keep.
4. **`NFR-A11Y-008` could not be closed by a Rust test at all**, and that is
   the interesting part: its regions are empty in the server-rendered HTML.
   Say that the question *"can a browser gate observe this?"* was asked
   **before** anything was built, answered **yes**, and that **a real
   OS-level drag gesture was declined on the record** with its capability gap
   named — so the next reader does not re-propose it.
5. **Two instrument failures, recorded rather than quietly fixed.** A gate's
   fixed 2000-character window over HTML **ran past `</section>` and accused
   the product of a defect it did not have**; and a risk the architect made
   structural **turned out already false** in a script the dev team re-read in
   full and corrected rather than benefiting from silently. **Both belong in
   the notes**: this release's whole subject is instruments whose scope does
   not match what they claim to measure.

**Three things the notes must not claim.** That any user-visible behaviour
changed — none did. That the test count includes the gate — it does not. And
that `§10.15` is closed: it is **not**, it has three gates now and 1,909 lines
still executed by almost nothing, and the entry's *almost* is load-bearing.

## 3. Verification before the candidate commit

- **`DEC-028`: already done** (`0f0b0ea`). Confirm both specifications read
  `Covers release: 0.47.0` with a `0.47.0` baseline row each.
- **The sweep**: run by the architect; **its fifth pass found one thing** —
  `FR-DM-006`'s Tab-order claim, which this release's own gate falsified.
  Confirm rather than re-run, and **report explicitly**.
- `DEC-007` **405**, three consecutive runs, **from a block extracted out of
  the current `CONTRIBUTING.md`**.
- **All three browser scripts on the extracted tree**, in the job's order:
  overflow gate **120 cells**, trap script clean on three surfaces, then
  `drag-outcome-gate.mjs` on both its surfaces with all three assertions.
  **Three servers, three ports, each torn down before the next** — say so.
- **`mdbook build` from the extracted tarball** — seventh release with the
  site.
- `fmt`, `clippy --workspace --all-targets -- -D warnings`, `rustdoc-links`
  with **seven** `Documenting` lines and the private-link count **eight**.
- **No migration to exercise**, but start the candidate on a database built by
  the released **0.46.0** binary and confirm it comes up at 20 with nothing
  applied. **Build from the tag's own commit** — 0.46.0's tag is `a420e4a`,
  one past its candidate `a6078c2`, and 0.46.0's own package caught that
  distinction for 0.45.0.
- `cargo publish --workspace --dry-run` from a **detached worktree of the
  candidate commit** — seven crates, **every `warning:` line read**.
- **No behavioural check is asked for.** No user-visible behaviour changed,
  and the browser gate is itself the behavioural check for what this release
  establishes. **If you disagree, say why rather than running one silently.**

## 4. Escalate rather than deciding

- **If the count is not 405**, or the overflow gate is not 120.
- **If `drag-outcome-gate.mjs` is flaky across repeated runs** — it is new,
  it drives real input, and `DEC-048` condition 3's quarantine policy exists
  for exactly this. **Flakiness found now is worth more than flakiness found
  in CI.** Run it three times, not once.
- **If three servers in one job collide** on the extracted tree.
- **If anything under `crates/*/src` or `static/` other than `me.rs`'s test
  module differs from `0.46.0`.**
- **If the sweep turns up a second thing.**

## 5. Exit condition

A candidate commit on `main` at **0.47.0**; a changelog that closes the class
and says plainly that no behaviour shipped, with both instrument failures in
it; 405 green three times from a freshly extracted block; all three browser
scripts clean in the job's order, **the third run three times**; the book
building from the tarball; the 0.46.0 database coming up clean from a binary
built at the tag's own commit; and a dry run of seven crates with every
warning read. Then stop.
