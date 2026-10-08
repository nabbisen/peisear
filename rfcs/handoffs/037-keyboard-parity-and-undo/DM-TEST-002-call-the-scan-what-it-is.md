# DM-TEST-002 — call the scan what it is, and wire the browser script

**Review:** `.git-exclude/reviewed/DM-TEST-001-review.md`
**Round 1:** `c7de37f`, accepted. **376** confirmed, every negative
demonstrated, both halves of `DEC-007` registered, no production source
touched. Nothing in round 1's reasoning was wrong.

**This handoff changes no assertion.** Names, one doc sentence, and two CI
lines. If you find yourself editing an `assert!`, stop and report instead.

---

## §1 — F-1: the name claims a runtime fact the mechanism cannot reach

Your module doc comment says it plainly — *"the toast these four scripts
build does not exist until the script runs, so there is no server-rendered
HTML a Rust integration test could inspect for it"* — and the `test.yml`
comment matches it. **The names do not.**

A reader meets `undo_dom_order: 4 passed` in a CI log, and in a failure
report meets a test name **alone**, with no doc comment anywhere near it.
`FR-DM-006`'s `Met` status now cites this work, so the name is what a future
auditor will take the strength of the evidence from. What is actually
established is that **three substrings are still present in
`showUndoToast`** — if that chain is correct and something else moves the
toast afterwards, all four still pass. That is `§10.17`'s open class, and
`§10.28` is the precedent that a doc-versus-mechanism mismatch is worth
fixing here.

Four changes, all naming:

1. **Rename the file** so its subject is the attachment chain in the source,
   not DOM order. `undo_toast_attachment_scan.rs` is a suggestion, **not a
   mandate** — pick what reads best; the test is whether the name would
   mislead someone who sees only the name.
2. **Rename the four tests** in the same direction — *…appends the undo toast
   after…* rather than *…undo lands right after…*. The assertion is about
   what `showUndoToast` **is written to do**.
3. **Correct the doc's summary line.** It reads *"A DOM-order assertion, not
   a Tab-walk"*; it is **not a DOM assertion either**. *A source scan, not a
   DOM assertion and not a Tab-walk.* The paragraph beneath it is already
   correct and should not change.
4. **`DEC-007` both halves move with the filename** —
   `.github/CONTRIBUTING.md`'s loop and `test.yml`'s job and job name — and
   `cargo test -p peisear-web --lib dec_007` must be green again afterwards.

**Do not change an assertion, add a test, or try to make this a real DOM
check.** The scan is the right instrument for this property at this cost. It
needs to be called what it is.

## §2 — item 6 is wired, into the job that already exists

You left this to me; the ruling is in the review's §3. **Yes, wired — and as
a step in `browser-overflow-gate`, not as a new job.**

That job already runs `cargo build -p peisear` and already has Chromium on
`ubuntu-latest`. One added step reuses both:

```yaml
      - run: node browser-checks/undo-mousedown-trap.mjs
```

after the existing `overflow-gate.mjs` step. **No second compile, no second
runner.** A separate job would pay for both again to run one script.

**Your reading of `browser-checks/README.md` was right** and the ruling keeps
it: `DEC-048` condition 3's *"one assertion only, do not widen this gate"*
governs `overflow-gate.mjs`'s **own** assertion set, and a sibling script in
the same job is a different property rather than a second assertion on that
gate. **Record that distinction in the job's comment** so nobody re-derives
it: *one job may hold two gates; one gate holds one assertion.* Rename the
job's `name:` if *"browser overflow gate"* now undersells it — your call, but
say which you chose.

**Keeping it unwired was the one option I rejected**, and the reason belongs
in the comment too if it fits in a line: a script sitting unrun beside a
gated one reads as coverage, which is `§10.15`'s class arriving as a *test*
executed by nothing.

## §3 — verification

- `cargo test -p peisear-web --lib dec_007` — **4/4**, after the rename.
- `DEC-007`, three consecutive runs, **376** and unchanged. The count must
  not move: this handoff renames tests, it does not add or remove any. **If
  the figure is not 376, stop and report** — something was lost in the
  rename.
- `fmt`, `clippy --workspace --all-targets -- -D warnings`.
- **Both browser scripts from the wired job's own command lines**, locally:
  `cargo build -p peisear`, then each `node …` step in order. The overflow
  gate at **120 cells** and the trap script clean on all three surfaces. The
  point is that the two scripts coexist in one job without the first's server
  or port interfering with the second's — **say explicitly whether each
  script starts and stops its own server**, because two scripts sharing a
  port in one job is the failure this step exists to find.
- The overflow gate is otherwise **not implicated**; no component markup
  changes here.

## §4 — escalate rather than deciding

- **If the two browser scripts cannot share one job** — a port collision, a
  leftover server, an ordering dependency. That changes my §2 ruling and it
  is mine to change, not yours to work around.
- **If the rename moves the test count.**
- **If renaming the file breaks a reference I have not anticipated** — a
  `§10.x` citation, a requirement's acceptance line, `browser-checks/README.md`.
  Report the reference; do not edit the specification.

## §5 — exit condition

A commit touching names, one doc sentence, `CONTRIBUTING.md`, `test.yml` and
nothing else; `dec_007` 4/4; `DEC-007` 376 three times; both browser scripts
run from the wired job's own commands with the port question answered
explicitly. Then stop.

**`DM-TEST-001` closes with this, and its closure unblocks `RFC 0015`'s
0.45.0 control handoff** — which is why this is a naming handoff and not a
rebuild.
