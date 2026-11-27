# `REL-0.71.0` — release candidate

**Recommendation: cut it, today.** Exit criteria 1–6 are met, measured at this
tree. **Criterion 7 is the owner's**, and none of `DEC-056`'s four legs runs
until that approval.

Written 2026-11-27 against `main` at `1770697` plus this cut's own `DEC-028`
sweep.

---

## 1. What this release is

**One security fix, and the reason to cut it now is not that the work is
ready.**

`FR-AUTH-003` is P0 and read `Implemented`: *a user MUST be able to end their
session, after which protected routes are inaccessible with the prior
credentials.* `logout` removed the client's own cookie and touched no storage.
There was no `jti`, no session table, no denylist — **nothing server-side to
end** — so a credential replayed after logout kept working on every route,
`/api/*` included, for the rest of `SESSION_TTL_SECS`: **seven days**, with
nothing the user or the operator could do to shorten it.

**The repository has published the reproduction since 2026-11-26 19:38.**
`origin/main` is public. `ROADMAP.md`'s own row names the release commit
(`cfa45d9` = `0.70.0`) and gives the sequence: register, `GET /projects` 200,
`POST /logout` 303, then the **same token** → `/projects` 200, `/today` 200,
`/api/search` 200. `RFC 024` in `rfcs/accepted/` supplies the *why*. **Every
day between that push and this tag is a day the public record explains how to
use a credential that cannot be revoked, against the latest release, with the
fix sitting untagged.** That asymmetry is the timing argument and it runs one
way only.

**Fixed with a session generation, not a clock.** One integer column on
`users`, advanced by `logout`, stamped into the token at issue, and checked in
**exactly one place** — `verified_requester_id`, the choke point all five
extractors already funnel through, with the read memoised in
`parts.extensions`.

**The mechanism is not the one I recommended.** I proposed a stored instant
compared against the token's own whole-second `iat`, and withdrew it on the
owner's philosophy review once `0021_updated_at_millisecond_precision.sql`'s
own header proved it reproduces the identical sub-second window that
migration had already been paid to close for `updated_at`. `RFC 024` §4.4
records the withdrawn mechanism rather than deleting it.

**Nobody is signed out by the upgrade**, by construction and by test: the
column is `DEFAULT 0` and `Claims.ses` is `#[serde(default)]`, so a `0.70.0`
token deserialises as the generation every row starts at. Verified in the
built artefact, not only in the source — §4.

**And the series is the first in seven to meet entry criterion 1.** `063`
through `068` cite no RFC anywhere; `069` is governed by `RFC 024`, accepted
before the work started, which is the gate the owner settled on 2026-11-26.

---

## 2. Exit criteria

| # | Criterion | State |
|---|---|---|
| 1 | Every handoff reviewed and accepted, one commit per handoff, in dependency order | **Met**, with one thing said plainly — one handoff, **two commits**: `1372883` and the CI fix-forward `c636c55` (`BIGINT` not `INTEGER` on PostgreSQL, a version literal 23 → 24). One review unit, so the criterion's purpose — telling one unit's work from another's — holds; recorded rather than papered over |
| 2 | `fmt`, `clippy -D warnings`, the full per-crate suite, with captured logs | **Met** — §3, and **re-derived independently by the dev team at this same tree**: all 42 commands, 0 non-zero exits, 703/0/0 over 53 result lines, every figure matching. Its evidence row was also where this cut found `§10.36` instance 12 (§3) |
| 3 | Requirements amendments applied; nothing `Divergent` without a gap entry | **Met** — `FR-AUTH-003`'s acceptance replaced, `FR-AUTH-001`'s second clause given its first real test, `§10.66` opened and closed, `§10.36` gains instances 10, 11 and 12 — the last two found at this cut, and 12 in this document's own criterion-2 row. **No entry is left `Divergent`**, and `FR-AUTH-003` never carried that status: the gap and its repair landed in the same round, so `§10.66` is the durable record instead (reviewed and accepted as a deliberate departure from the handoff's own instruction) |
| 4 | `CHANGELOG.md` records the *why* | **Met** — `[Unreleased]` carries one Security entry and two Changed, and gained its **What this release does not fix** section at this cut (§6). The section's narrative lead is written in the release commit, when the heading is dated |
| 5 | Affected documentation updated in the same release | **Met, and by amendment rather than by reading** — the first for `external-design.md` since `0.62.0`. §5 |
| 6 | Tarball built and extraction verified | **Met**, and the walk reproduced the published exploit against the artefact and watched it refused — §4 |
| 7 | Release recommendation issued; **owner approves** | **This document. Awaiting the owner.** |
| 8 | **What ships next** reconciled as of the tag | **Not yet** — in the release commit, with the `0.71.0` row removed per that criterion |

---

## 3. Criterion 2, measured at this tree

```
commands (+++ markers):   46   (42 DEC-007 + 4 gates)
DEC-007 block:            42   (expected 42)
non-zero exits:            0
tests passed (summed):   703
tests failed (summed):     0
tests ignored (summed):    0
result lines:             53
```

**`DEC-007`: 699 → 703.** The four are `SES-001`'s own: the replay test
(`a_credential_captured_before_logout_is_refused_after_it_html_and_api`),
three mechanism tests, and `FR-AUTH-001`'s plaintext-password test — against
one retired assertion. Log: 2,627 lines, kept privately.

| gate | result |
|---|---|
| `cargo fmt --all -- --check` | exit **0** |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit **0** |
| `… --no-default-features --features postgres -- -D warnings` | exit **0** |
| `cargo build --release -p peisear` | exit **0** |
| `CI` (`test.yml`) on `1770697`, `c42171d`, `efdcd40` and `65271fe` | **50 jobs, 0 not-success** on each, read per job with no `--limit`, `headSha` matched in full |
| `Docs` (`docs.yml`) on the same three, and on `65271fe` | `mdbook build (with link check)` **success** on all four. No workflow conclusion is success — `cancelled`, `cancelled`, `pending`, `pending` — and **every one of those comes from `Deploy to GitHub Pages` alone** (below) |
| `peisear-web and peisear-storage (postgres feature, real server)` | **success** on all four, read per job. The second backend is exercised against a real server by CI on every commit, which §4's walk is not and does not claim to be |
| `env-example-walk.yml` | did not run, correctly — its path filter matches none of these commits' files |

**The block ran before this cut's document edits, so the document scans were
re-run against the tree that will be tagged** — every one of them reads its
document from the filesystem at run time, not through `include_str!`, so this
is a real re-measurement and not a rebuild:

```
peisear-web --lib   258 passed, 0 failed      peisear-core --lib  3/0
peisear              6 passed, 0 failed  (3 binaries)

changelog_scan 9/9   spec_scan 42/42   docs_scan 46/46   prose_scan 4/4
page_title_scan 2/2  contrast_scan 8/8  css_scan 9/9
```

**This table named one workflow and was read as naming the tree, which is
`§10.36` instance 12 and mine.** Its earlier wording was *CI on `c42171d` —
50 jobs, 0 not-success*. True of `test.yml`, whose workflow is literally
called `CI`; silent about the fact that **two** push-triggered workflows run
on `main`. **An independent re-verification at this same tree then read the
row as a statement about the tree and wrote that down**, from a single
`--limit 1`-shaped read of the one workflow the row named. **A row whose name
is wider than its reach does not fail; it gets believed** — and that reading,
made by someone who was deliberately checking rather than trusting, is the
measurement.

**Measured properly, nothing is broken, and that is why this is an instance
rather than a defect.** Every non-success conclusion on all three commits
comes from **one job**: `Deploy to GitHub Pages`, the deployment wedged since
2026-11-18 that `ROADMAP.md`'s *Unscheduled work* already records.
`docs.yml` moved the `pages` lock off workflow level on 2026-11-21 **so that
this wedge could not stop the link check** — and these three commits are the
evidence it worked, with `mdbook build (with link check)` green on each.
**`0.70.0`'s own release commit `cfa45d9` was tagged in exactly this state**,
as was every commit on `main` back past `245314f`.

**Before leg 2**, the release commit's own runs are read per job across
**both** workflows, and the tag waits on `CI` plus `Docs`'s build job — not
on the Pages deployment, which no commit has cleared since 2026-11-18.

**Nothing but this section and `§10.36`'s new row has changed since
`c42171d`.** No code, no specification requirement, no changelog text, so
every measurement in §3–§5 still describes the tree that will be tagged.

---

## 4. Criterion 6 — the archive, extracted, and the published exploit replayed against it

```
archive: peisear-0.70.0-x86_64-unknown-linux-gnu.tar.gz   8,402,065 bytes
tar tzf → peisear      ← one member, at the root (NFR-REL-002)
extracted binary        23,939,264 bytes
```

The archive says `0.70.0` because `Cargo.toml` still does — the bump is held
for the release commit (§7), and `release.yml` refuses an archive whose
version does not match the tag.

**Unattended start, then `ROADMAP.md`'s own sequence, against the extracted
binary on `127.0.0.1:3000`:**

```
health 200            wizard ran: no          (NFR-REL-011)
register              303, session cookie 236 chars
before logout         /projects 200      /api/search 200
POST /logout          303
---- the replay, same token ----
/projects             303      /today 303      /api/search 401
303 →  /login?flash=You+were+signed+out+because+this+session+was+ended+on+...
login page            renders the notice;  markup: alert alert-warning
immediate re-login    303, then /projects 200
old token afterwards  303  — still refused, generation advanced not reset
account-menu hint     present in the served HTML
server stopped by pid; listeners on 3000 afterwards: 0
```

**Three of those lines are the release.** `/projects` 303 and `/api/search`
401 are §9's two new rows, measured; the `/login?flash=` landing is §5.3's
session notice, measured; and `alert alert-warning` is what made §5.3's and
§3.1's old `role="status"` claim demonstrably wrong (§5), read out of the
artefact rather than inferred from the source.

**What this walk does not cover**: one backend (SQLite), one platform, one
user, one locale. **No PostgreSQL instance was started *in this walk*** —
and that caveat was worded narrowly enough to be true and widely enough to be
read as a gap in the release, which it is not. **PostgreSQL is covered, by
CI**: `test.yml` runs a job called **`peisear-web and peisear-storage
(postgres feature, real server)`** against a real server, and it is
`success` on **all four** commits this document cites — `1770697`,
`c42171d`, `efdcd40` and `65271fe`, read per job on each. `clippy`'s postgres
feature set and `migration_drift.rs`'s filename pairing are *additional* to
that, not substitutes for it. This correction is `§10.36` instance 12's
second occurrence, in the same document and at the same cut — a caveat read
wider than its reach, this time understating the project's own coverage
rather than overstating it.

---

## 5. Criterion 5 and the `DEC-028` sweep

**`external-design.md` is amended for the first time since `0.62.0`**, ending
a run of eight releases that amended nothing in it. Four sections, and each is
a state or surface the document could not previously express:

- **`§9`** gains **Credential revoked, HTML** (`303 → /login?flash=`) and
  **Credential revoked, API** (`401`). Until this release a signed-out
  credential was indistinguishable, to that table, from one that was never
  valid — because it *was* still valid. The HTML row is deliberately a
  different landing from `Unauthenticated, HTML`: a bare redirect cannot be
  told apart from an expiry or from never having signed in.
- **`§5.3`** gains a fourth kind, the **session notice** — not a success
  (nothing the reader asked for succeeded) and not a recoverable error
  (nothing to correct, no input to preserve).
- **`SCR-02`** gains a fourth state and a fourth entry path, **the first it
  reaches without the reader having done anything on this screen**. No field
  is pre-filled, email included: the screen cannot assume the person holding
  the browser now is the one who was signed out.
- **`§3.1`'s diagram and `§4.2`'s `/logout` row** record one line of new copy
  and an action whose reach is now written down.

**And the sweep found two statements that were already wrong, which is the
part worth reading.** `§5.3`'s `Role` column read `role="status"` for
`?flash=`, and `§3.1`'s region-obligation row read *`role="status"` or
`role="alert"`, `alert` reserved for immediate errors*. **Neither describes
either renderer, and neither has for as long as both have existed**: the
authenticated shell's flash is `role="alert"` with `alert alert-info` — on an
ordinary success flash, which that reservation forbids — and the public
shell's, the one this release routes a security notice through, carries **no
role at all**. Counted rather than asserted: the product has **eight**
`role="status"` sites — five JavaScript announcement regions
(`#status-announcements`, `NFR-A11Y-008`) and three static value or notice
regions — and **not one is a `?flash=` carrier**.

**Corrected to what ships, on the merits rather than by default**: a flash
arrives *with* its document, and a live region exists to announce an insertion
into a page already on screen, so neither renderer is wrong about
announcement. The rows were wrong about the renderers. **`§10.36` instance
11** is the durable record, and the absent check is in `ROADMAP.md`'s
*Unscheduled work* beside the three rules instance 9 left there — a rendered
role needs a plant, which is a round, not an edit at a cut.

**Both specifications' header fields moved together**, per `DEC-028`:
`Covers release` to `0.71.0` in each, `Amended` to 2026-11-27, and a
`0.71.0 (this baseline)` row in each Appendix C with the previous baseline
demoted. The `0.71.0` narrative is the one drafted during the cycle and
preserved in `c636c55` when I reverted the field for being ahead of its own
release. Paren depth on both header fields verified **0**, and both new
Appendix C rows carry exactly three pipes.

**Both documents re-rendered with the pinned toolchain** — `mdbook` 0.5.4 and
`mdbook-linkcheck2` 0.13.0, the versions `docs.yml` pins — exit **0** with the
link check running, and the real exit code read rather than a pipeline's:

```
requirements.html      44 tables; Appendix C = 57 rows (header + 56)
external-design.html   50 tables; Appendix C = 51 rows (header + 50)
                       §5.3 flash table   5 rows (header + 4 kinds)
                       §9  status table  15 rows (header + 14 situations, +2)
both files             0 <td><p>,  0 stray pipe paragraphs
```

---

## 6. What is not fixed

In the changelog, for a reader, and here for the record:

- **There is still no password change or reset anywhere in the product.** **0**
  hits for `password` in `app.rs` (the router, 68 `.route(` registrations)
  and **0** in `handlers/settings.rs`, re-measured at this tree. Someone who
  believes their password has been seen can now genuinely end every session —
  a partial remedy, and **not** the missing one, since whoever knows the
  password signs straight back in.
- **Signing out is all or nothing.** No device list, no way to end one session
  and keep another. Revocation is account-wide by construction, which is why
  the specification states no per-device scope.
- **Upgrading signs nobody out**, on purpose. Stated in the changelog because
  a reader could reasonably expect the opposite from a release with this
  headline.
- **Two nits stay with the round**, neither a defect: the log-half absence
  assertion has no population floor (my own probe measured **854 bytes**
  captured, carrying the `tower_http` span for the very `POST /register`, so
  it examines the right thing today — one line asserting that would keep it
  true), and three tests capture the credential as `cookies().iter().next()`,
  *the first* cookie rather than the session cookie by name. To fold into the
  next round that touches `smoke.rs`.
- **The flash-role check does not exist** (§5), and three rules the owner
  settled on 2026-11-26 are still decided-but-unwritten. Four checks, one
  round, the architect's to bring.
- **One warning inside a gate that exits 0, and it is not this cut's.**
  `mdbook build docs` prints `unclosed HTML tag <p> found in
  specification/requirements.md while exiting TableCell` and exits **0**.
  **Measured rather than assumed**: built at `1770697` in a throwaway
  worktree, where it appears exactly once with the identical message, so it
  predates this sweep. Nothing a reader sees is wrong today — the rendered
  output carries **0** `<td><p>` and **0** stray pipe paragraphs in either
  specification. It is `proc-macro-error2`'s shape, one tool over: a warning
  in a green gate's log that nobody had read. Written down here rather than
  chased at a cut.
- **`proc-macro-error2`** is unchanged, and **the Pages deployment is still
  wedged** — no `Docs` run on `main` has concluded `success` in the last 60,
  `0.70.0`'s release commit included. The link check is unaffected by design
  and green throughout (§3); nothing in this release touches the deployment,
  and the tag does not wait on it.

---

## 7. What the release commit will contain

`Cargo.toml`'s bump to `0.71.0` and `Cargo.lock`; `CHANGELOG.md`'s
`[Unreleased]` becoming `## [0.71.0] — <cut date>` with its narrative lead
written; and `ROADMAP.md`'s criterion-8 reconciliation with the `0.71.0` row
removed. One commit — `changelog_scan` derives the version from `Cargo.toml`
and requires a dated section for it, so the two cannot be split.

**If approval falls on a later day than 2026-11-27**, both specifications'
`Amended` fields and the changelog's date move with it in that commit rather
than being left at today's.

Then `DEC-056`'s four legs, in order: signed tag · push · `cargo publish
--workspace` from a worktree of the tag · `gh release create` titled bare
`0.71.0` with the changelog section as its body. None of them before criterion
7.

---

## 8. The honest part

**The mechanism in this release is not the one I recommended, and the owner's
review is why.** He asked for three open decisions to be re-read against his
own two sentences; reading my own standing recommendations against them
reversed two of three, this one included. The `iat` comparison would have
shipped a sub-second window that this repository had already paid a migration
to close, and the evidence was in that migration's own header the whole time.

**The dev team corrected me twice more, and both corrections were right.** My
handoff said *add `0023_*`* — `0023` was already spent on a PostgreSQL-only
migration and `migration_drift.rs` compares full filenames, so the first
number free on both sides is `0024`. And they declined the RFC's own
`sessions_epoch` for `session_generation`, because *epoch* reads as a
timestamp to anyone who has not read §4.1 — which is the exact confusion a
counter exists to avoid.

**My own `§2.4` was wrong when I wrote it** ("in both extractors"; the tree
has one choke point), and so was my cost estimate for the check. Both were
corrected before any work started, by reading the tree instead of my memory
of it.

**And this cut's sweep found two normative rows that had been wrong for
longer than this release has existed**, in the document `§10.36` instance 7
was about. They were found because `DEC-028` forces the sweep and the sweep
was done by reading rather than by assuming — which is the only reason any of
the last eight "nothing amended" readings is worth anything.

**One question, and it is the only one this document asks:** cut `0.71.0`, or
hold it?
