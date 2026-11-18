# `REL-0.64.0` — release candidate

**Series:** [062 — the smallest text on the screen](./README.md)
**Row:** authorized 2026-11-17 — the row, not the cut.
**Status:** **criterion 7 is the only open criterion. Awaiting the
owner.** Nothing in `DEC-056`'s four legs has run.

---

## §1 The exit criteria, and where each is answered

Measured at `d64f29c` plus the held cut edits (§4), which together are
exactly the tree the release commit will contain.

| # | Criterion | Where |
|---|---|---|
| 1 | Every handoff reviewed and accepted, **one commit per handoff, in dependency order** | **Met.** Five work commits — `9b9120a` `TYPE-001`, `5404f45` its round 2, `e78d43e` `TYPE-002`, `90d16af` its round 2, `297de0d` `TYPE-003` — each followed by the architect's own review commit, in order. All five closed in this series' `README.md` |
| 2 | `fmt`, `clippy -D warnings`, the full per-crate suite, **with captured logs** | **Met.** `fmt --check` exit 0. Clippy exit 0 on **both** the default features and `--no-default-features --features postgres`. `DEC-007`'s own 42-command block: **42 commands, 42 zero exits, 687 passed, 0 failed, 0 panics.** Logs kept at `.git-exclude/tmp/cand/{dec007,clippy}.log`. §2 |
| 3 | Requirements amendments applied; nothing left `Divergent` without a gap entry | **Met, and by identity rather than by inspection**: all **189** `*Status*` lines are byte-identical to the `0.63.0` cut. No requirement added, amended or re-statused. `§10.60` is the only register entry touched, and it **closes** |
| 4 | `CHANGELOG.md` records the *why* | **Met in content; the heading is dated at tag time, not now.** §4 and §5 |
| 5 | Affected documentation updated in the same release | **Met.** `DEC-028` sweep over both specifications — headers to `0.64.0`, new Appendix C baseline rows in each. `external-design.md` states *nothing to amend* **and why**: `§5.10` governs section order, and that document describes no text sizes at all, which is the finding `§10.60` opened on |
| 6 | Real binaries, and an upgrade that is a non-event | **Met, walked rather than reasoned.** §3 |
| 7 | Release recommendation; **owner approves** | **§6 — issued 2026-11-18. The only open criterion.** |
| 8 | *What ships next* reconciled | **After approval, with the tag** — the criterion asks for the rows to be correct *as of the tag*, so doing it now would date it wrong |

## §2 Criterion 2, in full — and one thing it found

```
cargo fmt --all -- --check                                                    exit 0
cargo clippy --workspace --all-targets -- -D warnings                         exit 0
cargo clippy --workspace --all-targets --no-default-features \
             --features postgres -- -D warnings                               exit 0
DEC-007's 42-command block     42 commands / 42 zero exits / 687 passed / 0 failed
cargo test -p peisear-web --lib                                          246 passed
```

`687` is unchanged from `0.63.0`'s baseline, which is what five rounds
that added and removed no test should produce.

**The counting was got wrong once and corrected.** The first tally
reported *"687 passed"* as **0**, because it summed the log's fifth
whitespace field — the word `passed` — rather than the fourth. Recounted
from the right field over the same log. *A counter is a claim like any
other*, which is this project's own standing lesson one level further
in.

**And reading the log rather than the exit code found something.**
Both clippy runs exit 0 **with a warning present**:

> `warning: the following packages contain code that will be rejected by
> a future version of Rust: proc-macro-error2 v2.0.1`

It arrives **transitively through `leptos 0.8.19`** (`leptos_macro` and
`reactive_stores_macro`), so it is not a direct dependency and not
removable here. **`-D warnings` cannot catch it** — `cargo` emits it,
not `rustc` as a lint — so every build this project has ever run has
carried it while every gate passed. **It was recorded in no document
before today.** Not caused by this release, not fixed by it, and now in
`ROADMAP.md`'s *Unscheduled work* with the reason it is watched rather
than scheduled.

## §3 Criterion 6 — walked, not reasoned

**The archive.** Packed the way `release.yml` packs it —
`tar czf … -C dist peisear`:

- **One entry, `peisear`, at the archive root** — `NFR-REL-002`'s own
  property, no intermediate directory.
- Extraction **byte-identical**: `sha256` of the built and extracted
  binaries match (`0d141cb9…c55c25`), executable bit preserved.
- **The extracted binary starts and serves**: `listening
  addr=127.0.0.1:4311`, and `/` answers `303` to the login page for an
  unauthenticated reader, which is the correct answer rather than
  merely a non-error.

**The upgrade.** `0.63.0` was built from **its own tag** in a throwaway
worktree, not reconstructed, and the walk went through the real HTTP
path:

1. `0.63.0` on a fresh SQLite database — registered a user and created a
   project through the app's own forms (`303`, `303`), and the project
   **renders** on `/projects`. Database state: **22 migrations, 1 user,
   1 project.**
2. `0.63.0` stopped by pid.
3. **`0.64.0` started on that same database file.** No migration
   applied — `_sqlx_migrations` is **identical, 22 rows, byte for
   byte** — and **zero** lines in the startup log match
   `migrat|error|panic`.
4. Logged in with the same credentials and **the project created by
   `0.63.0` renders under `0.64.0`.**

**What the walk does not cover, stated rather than implied**: one
backend (SQLite, not PostgreSQL), one platform (x86-64 Linux), and a
database with one user and one project. The five platform archives and
the container image are built by CI on the tag, and that is where their
evidence comes from — as it did for `0.63.0`.

## §4 What the release commit will contain, and why it is not committed yet

**Held, uncommitted, as a verified patch** at
`.git-exclude/tmp/cand/0.64.0-cut.patch`:

| file | change |
|---|---|
| `CHANGELOG.md` | the `0.64.0` section — **heading dated on the day of the tag** |
| `Cargo.toml` / `Cargo.lock` | `0.63.0` → `0.64.0` |
| `docs/specification/requirements.md` | `Covers release` header, new Appendix C baseline row |
| `docs/specification/external-design.md` | `Covers release` header, new Appendix C row stating *nothing to amend* and why |
| `ROADMAP.md` | criterion 8, **written at the tag** so it is correct as of it |

**Why held.** `0.63.0`'s own release commit (`922f976`) contained
exactly these files, and it is **leg 1 of `DEC-056`, which runs only
after criterion 7.** Two further reasons, both specific:

- **The version bump and the changelog date are coupled.**
  `changelog_scan` derives the version from `Cargo.toml` and requires a
  dated section for it, so bumping early *forces* dating early.
- **`0.60.0` is where those two dates first diverged** — section
  2026-11-01, tag 2026-11-03 — and that divergence is already a
  standing `ROADMAP.md` candidate for a `changelog_scan` rule. Writing
  today's date into a section a later tag will carry would be the same
  mistake with the evidence already in hand.

`release.yml:113` refuses to attach an archive whose `Cargo.toml`
version does not match the tag, which is the same property guarded from
the other side.

## §5 What this release is

**The smallest text in the product, read one screen at a time — and the
decision not to write a rule about it.**

**Sixty of sixty-one distinct sites** across three screens move up one
step, every cost measured at 390 and 1280 rather than assumed. **The
four sites whose contrast nothing had ever checked were measured and all
four pass** — `5.51` / `5.91` / `4.98` / `5.86` resting, as rendered,
with hover reached by a real dispatched mouse move and a sampling rule
that is a conservative floor rather than a point estimate. The one site
kept was kept for a reason that has nothing to do with type.

**`§10.60` opened and closed inside the same release**, on the second of
the two branches its own closing condition named: *a recorded decision
that none should exist.* No requirement governs text size, deliberately,
because the size answer converged — which is the argument against a
rule rather than for one — because seven of this release's costs needed
a reading no class-token scan can do, and because 68 sites on seven
unread screens would otherwise be governed by a rule written from three.

**What recurred is not text size.** It is that a size change propagates
into arithmetic written in comments and into literal class strings held
inside checks, and **nothing reads either** — four `<summary>`
touch-target sums, one `overflow-gate.mjs` fixture regex, one
`min-h-[2rem]` that silently stopped meaning *two lines*. Carried to
`ROADMAP.md` as a candidate rather than bolted on here.

## §6 What it does not fix, stated rather than implied

- **The seven screens nobody read** — 68 of these sites remain,
  unchanged, deliberately left rather than swept.
- **The calendar's day-view project badge** — still the smallest text in
  the product, still clipped entirely on any event under about an hour.
  A defect with nothing to do with its size, found while measuring and
  recorded as found-and-not-fixed.
- **Layout.** This release answers the typography half of the owner's
  2026-11-04 question. Colour was already discharged; layout is
  untouched.
- **`proc-macro-error2`** — §2. Not ours to fix.
- `§10.15`, `§10.32`, `§10.50`, `§10.54`, `§10.57` and PostgreSQL P3 are
  carried, unchanged by this release.

## §7 The honest part

**Four of this release's corrections were the architect's own**, and
each reached a document before it was caught: a published entry saying
three of four sites moved when all four had; every population figure
inflated by a `grep -c` that counted comments as sites; an unmeasured
*"already fully legible"* claim sitting in the very entry that recorded
the testing of that claim; and a headline figure that double-counted two
sites. **Three of the five rounds corrected me.** All four are recorded
in `§10.60` as corrections rather than edited away.

The rounds' own misses were smaller and none reached a user: a grep
reported as two hits when it was four, a CI run read as absent while it
was in flight, and one self-contradicting exception.

**The measurement discipline is the thing this release actually bought**,
more than the twelve pixels.

## §8 If approved

`DEC-056`'s four legs, in order, none before approval:

1. **Release commit** — the held patch (§4) plus criterion 8's
   `ROADMAP.md` reconciliation, the changelog heading dated that day.
2. **Signed tag** `0.64.0`.
3. **`cargo publish --workspace`** from a worktree of the tag.
4. **`gh release create`** with the changelog section as the body, title
   the bare `0.64.0`.

Publish before Release, so the Releases page never announces a failed
publish.
