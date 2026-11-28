# `REL-0.72.0` — release candidate

**Series**: `070` — [the password a user can change](./README.md).
**Governing RFC**: [`RFC 025`](../../accepted/025-the-password-a-user-can-change.md),
accepted by the owner 2026-11-27.
**Rounds**: [`PWD-001`](./PWD-001-password-change.md) (accepted 2026-11-28) and
[`PWD-002`](./PWD-002-the-generation-it-committed.md) (accepted 2026-11-28).
**Row**: authorized 2026-11-28.
**Cut**: approved in advance — *"The cut is authorized if ready"* — and the
architect's answer the **first** time that was asked was **not ready**.

---

## 1. What this release is

`0.71.0` made *Log out* genuinely end every session on every device, and its
own release notes had to say that this was **a partial remedy and not the
missing one**: ending sessions answers a stolen *credential*; nothing in this
product answered a stolen *password*. Whoever knew it signed straight back in.

There was no way to change a password anywhere in peisear, and **no
requirement asking for one** — measured at `RFC 025`'s acceptance: `0` hits
for `password` across `app.rs`'s 68 `.route(` registrations, `0` in
`handlers/settings.rs`, and the word appearing 13 times in `requirements.md`
without ever being a requirement to change one.

`FR-AUTH-006` is now `Implemented`. `FR-AUTH-007` (reset) is accepted as a
requirement and `Deferred`, with three measured prerequisites — SMTP is
optional here, nothing in this product rate-limits anything, and a correctly
hidden reset affordance leaves a locked-out user on a mail-less instance
needing an operator action that does not exist. **A reset that silently
cannot deliver is worse than no reset.**

## 2. Exit criteria

| # | Criterion | State |
|---|---|---|
| 1 | Every handoff reviewed and accepted, one commit per round | **Met** — `PWD-001` at `b72a743`, `PWD-002` at `2a69c76`, both reviewed and accepted, reviews private under `.git-exclude/reviewed/` |
| 2 | `fmt`, `clippy -D warnings`, full per-crate suite | **Met** — §3 |
| 3 | Requirements amendments applied, nothing left pending | **Met** — `FR-AUTH-006` `Implemented` with its acceptance citation corrected at this cut; `FR-AUTH-007` `Deferred`; `§10.36` gains instances 14 and 15 |
| 4 | `CHANGELOG.md` records the *why* | **Met** — dated `0.72.0` section, with a *What this release does not fix* note naming the absent reset |
| 5 | Affected documentation updated in the same release | **Met** — both specifications amended per `DEC-028`, §5 |
| 6 | Tarball built and extraction verified | **Met** — §4 |
| 7 | Release recommendation issued; owner approves | This document. Approved in advance, conditional on readiness |
| 8 | *What ships next* reconciled | **Met** — the `0.72.0` row becomes a `| — |` cut row; *Unscheduled work* grows to **11** |

## 3. Criterion 2, measured at this tree

The 42-command per-crate block `DEC-007` defines (`.github/CONTRIBUTING.md`),
run here after the last edit, counted **with the runner** rather than from any
list:

```
commands run:          46      (42 test commands + 4 gates)
non-zero exits:         0
result lines:          53
tests passed (sum):   709
tests failed (sum):     0
tests ignored (sum):    0
```

```
cargo fmt --all -- --check                                      exit 0
cargo clippy --workspace --all-targets -- -D warnings            exit 0
… --no-default-features --features postgres -- -D warnings       exit 0
cargo build --release -p peisear                                 exit 0
mdbook build docs (0.5.4 / linkcheck2 0.13.0)                    exit 0
```

**`DEC-007` moved 703 → 708 → 709** across the two rounds: `PWD-001` added
five acceptance tests, `PWD-002` one storage plant. Both deltas were checked
against `git diff -U0` on `#[tokio::test]` as well as the runner, so the
figure rests on two independent methods.

**`mdbook build` now prints zero warnings**, which it has not done in at
least three releases — see §5.

**CI, read per job by workflow name on each pushed commit** (`--commit`
omits queued runs, which is `§10.36`'s own lesson from the `0.71.0` cycle):
`CI` 50/50 success on `b72a743`, `68f8130`, `4b08c94` and `2a69c76`; `Docs`'
own `mdbook build (with link check)` job success on each, with
`Deploy to GitHub Pages` left pending — the wedged-since-2026-11-18 job
`ROADMAP.md` already records, not a new defect.

## 4. Criterion 6 — the archive, extracted, and the property walked

Built the way `release.yml` builds it — `cp` into `dist/`, then
`tar czf … -C dist peisear`, so files sit at the archive root
(`NFR-REL-002`):

```
archive: peisear-0.72.0-x86_64-unknown-linux-gnu.tar.gz   8,431,987 bytes
tar tzf → peisear      ← one member, at the root
extracted binary       24,028,480 bytes
```

**What the archive's version claim actually rests on, measured rather than
assumed**: there is **no `CARGO_PKG_VERSION` anywhere in this codebase**, so
the binary embeds no version string and nothing inside it can be checked
against the name. The guarantee is `release.yml`'s own step comparing
`Cargo.toml`'s version to the release tag and refusing to attach a
mismatched archive. Stated because an unexamined assumption here would be
`§10.36`'s own class.

**Unattended start of the extracted binary on `127.0.0.1:3118`, then the
whole of `FR-AUTH-006` walked against it:**

```
GET /login                   200        wizard ran: no   (NFR-REL-011)
register                     303        session cookie 252 chars
device B login               303        tokens_differ(A,B): no
device B /projects  (before) 200
POST /settings/password      303        → /settings?flash=Password+changed.
                                          +Every+other+device+has+been+signed+out.
                             exactly 1 Set-Cookie, token differs from A
---- the rule, both halves ----
acting session, re-issued    200        ← MUST remain valid
acting session, old cookie   303
device B /projects  (after)  303        ← MUST become invalid
device B /api/search         401            both §9 rejection shapes
---- the password itself ----
old password at /login       400
new password at /login       303
---- the copy, on the real screen ----
"… signs you out of every other device. This one stays signed in."
```

**`tokens_differ(A,B): no` is the finding this release ships with rather than
fixes.** Two logins for one account inside one second are the same bearer
token, so device B's "own" credential is byte-identical to device A's. The
requirement is still proven — a credential issued under the superseded
generation is refused, on both shapes — but **no walk and no test can show
that a *distinct* second session died.** `§6` carries it; one test name was
corrected for it at this cut.

Server stopped by pid, **0** listeners on 3118 afterwards, **0** privacy hits
in the artefact directory.

## 5. Criterion 5 and the `DEC-028` sweep

**`requirements.md`** — `FR-AUTH-006` moved `Specified` → `Implemented`;
`FR-AUTH-007` `Deferred`; Appendix A's Authentication row 5 → 7 entries and
the functional total 100 → 102 (both landed with the entries at `RFC 025`'s
acceptance and re-checked here); `Covers release`, `Amended` and a new
Appendix C baseline row; **the acceptance citation corrected**; `§10.36`
gains instances 14 and 15.

**`external-design.md`** — amended for the **second consecutive release**,
after a run of eight that amended nothing. `SCR-22` gains a **fifth
section**, placed second, immediately after Profile — identity beside
identity. **The position is argued rather than assumed**: `§5.10` governs
destructive confirmations and does not reach this screen. The section's own
paragraph states the sign-out scope *before* the control is used and records
the re-issue; `§8` gains the route and the form; the screen-to-requirement
table gains `FR-AUTH-006`; Appendix C gains its baseline row.

**Deliberately absent, and recorded as absent** because an absence is what a
later reader cannot reconstruct: no new `§9` row (all three rejections are
ordinary validation errors that table already expresses) and **no second use
of `§5.3`'s session notice**, so that flash kind keeps the single meaning
`0.71.0` gave it.

**One departure from `§9`'s sign-in neutrality, stated rather than implied**:
the rejections name which field was wrong. The architect's own instruction
requiring neutrality here was **withdrawn at review** — `FR-AUTH-002`'s
neutrality defeats account enumeration at *sign-in*, and an authenticated
screen has no account to enumerate. The dev team's proposed copy was right.

## 6. What this release does not fix

- **No password reset.** `FR-AUTH-007` is `Deferred`, and the changelog says
  so in the user's own terms: someone locked out entirely still has no path
  back in.
- **Sign-out is still all-or-nothing.** Revocation is account-wide by
  construction; there is no device list and no per-session scope.
- **Two sessions for one account inside one second are the same bearer
  token.** `jwt::issue` stamps `iat` in whole seconds, measured twice on
  release binaries. Nothing is unproven about either requirement — every
  credential under a superseded generation is refused — but **no test can
  show that a *distinct* second session died**, because the product cannot
  mint two distinguishable tokens within a second. One test name was
  corrected here; a second, in `SES-001`, is recorded unscheduled.
- **The acceptance-citation check is not written.** `§10.36` instance 14's
  remedy is a round, not an edit at a cut.
- **The torn-table-cell check is not written.** Instance 15's remedy is the
  same shape.

## 7. What the release commit contains

- `Cargo.toml` `0.71.0` → `0.72.0` and `Cargo.lock` (7 workspace crates).
- `CHANGELOG.md`: `[Unreleased]` → `## [0.72.0] — 2026-11-28`, with the
  narrative lead and the *does not fix* note. No `[Unreleased]` heading is
  left behind, matching this project's own convention.
- Both specifications, per `DEC-028` (§5).
- `ROADMAP.md`: the `0.72.0` row reconciled to a `| — |` cut row per exit
  criterion 8; *Unscheduled work* at **11** rows, including the two new
  checks and the corrected `mdbook` row.
- This candidate.

## 8. The honest part

**`PWD-002` exists because of my own handoff.** `PWD-001` `§3` step 5 told
the implementer to take the re-issued generation from *"the value you just
wrote, not from a re-read, and not from the stale row you loaded"* — and the
statement as written produced no such value, so the only available reading
was the stale row. The dev team implemented that reading and documented it.
The result was a **measured 16.6 ms** window — two Argon2 operations wide —
in which a concurrent `logout` signed the user out of the browser
`FR-AUTH-006` promises to keep signed in.

**I held the release for it, and the reason was the register rather than the
window.** The window is low-likelihood and fails safe. What could not ship
was a requirement marked `Implemented` carrying a `MUST` that measurement
contradicted, citing a test that could not show what the citation claimed.

**And then this cut's own sweep caught me doing the same kind of thing
twice.**

First: I wrote into `ROADMAP.md` that the long-standing `mdbook` `<p>`
warning was a scanner artefact and the rendered output was fine — **from
reading the source and not asking the renderer.** Then I rendered the book.
`§10.36` instance 8's own cell — the entry about table rows that are not
rows — had been rendering **torn** in the published specification for at
least three releases: an unescaped `|` inside a code span ends a GFM table
cell, so the cell closed early and the rest of its sentence rendered as the
next column's content. Three consecutive round reports called that warning
*cosmetic*, two of them my own candidates. Escaped here; the warning is gone
and the page renders whole, which is the proof it was never cosmetic.

Second: the two register rows I wrote **about** that defect contained the
same unescaped pipe. The scanner I had just written to measure the
population found them in my own new text before they shipped. That is the
argument for instance 15's check existing in code rather than in a habit.

**What the dev team found that I had not**: a second test name making the
same unprovable claim, in `SES-001`. They declined to fix it — right call,
different requirement, different round — and credited the finding to my
`PWD-001` review. **That credit was wrong and I have corrected it**: my
review and `ROADMAP` row named the class and `PWD-001`'s own test, and
neither named theirs.
