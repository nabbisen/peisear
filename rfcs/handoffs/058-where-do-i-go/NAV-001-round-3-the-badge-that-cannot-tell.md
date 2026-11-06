# NAV-001 round 3 — the badge that cannot tell

**Round 2 found one page where the bell's count is always zero. It is
twelve of twenty-six.**

Round 2's `§2` reported, honestly and without generalising, that
`handlers::me::page` never calls `unread_count_for_user`, so `/today`
renders with `unread_count = 0` whatever the reader actually has. **The
architect measured the population: of 26 `<AppShell>` render sites, 14
thread the real count and 12 do not.**

| file | sites without the count |
|---|---|
| `issues.rs` | 5 — issue detail, new, edit, sub-issue forms |
| `projects.rs` | 3 — list, new, detail |
| `me.rs` | **2 — `/today`, the screen a reader opens first** |
| `search.rs` | 1 |
| `settings.rs` | 1 |

*(Measured 2026-11-07 by parsing each `<AppShell` tag's own attributes.
**Two earlier counts of the architect's were wrong** — one truncated at a
400-character window, one at the `>` inside `None::<String>` — and
reported 19. **Re-measure before quoting 12.**)*

## §0 — the mechanism, which is the actual defect

`components/layout.rs:73`:

```rust
#[prop(default = 0)] unread_count: i64,
```

**A missing count becomes a plausible zero.** Nothing fails, nothing
warns, and the badge is simply absent. This is the shape this project
names over and over — a value that cannot tell *nothing unread* from
*nobody computed it* (`§10.36`'s family) — and here it is a defaulted
component prop rather than a scan.

**And `AppShell`'s own rustdoc states the opposite** (`layout.rs:65-67`):

> `unread_count` drives the bell badge in the navbar. **Threaded through
> every handler that returns the shell** so the badge stays in sync with
> reality across page loads.

**False for twelve of twenty-six sites**, in published rustdoc, which is
`DIA-004`'s class exactly — a comment describing code that does not do
what it says.

**Why it matters more after round 2.** Round 2 put the count into the
**accessible name** (`"Inbox (3 unread)"`). So on these twelve pages a
reader using a screen reader or voice control now also hears *"Inbox"*
with no count — the fix was right, and it widened the reach of this
defect from a missing badge to a missing fact.

## §1 — the decision: make the prop required

**Remove the default.** `unread_count: i64`, no `#[prop(default)]`.

Then **the compiler names every site that does not thread it**, none can
recur, and `AppShell`'s doc claim becomes **structurally true** rather
than a convention someone must remember. That is the simplest available
rule and it needs no scan, no census and no reviewer.

**Not** a different default, **not** an `Option<i64>`, and **not** a
database call inside a component — the handler owns data access here and
that boundary is not this round's to move.

## §2 — the twelve handlers

Each of the twelve call sites' handlers computes the caller's own count
and threads it, the way the fourteen that already work do:

```rust
let unread_count = notif_store::unread_count_for_user(&state.db, &rid).await?;
```

**Read `handlers/teams.rs:54` and copy that shape** rather than inventing
one; six handlers in `sprints.rs` and four in `teams.rs` are the model.

**Check each handler has a `Requester` before assuming it can.** If one
renders `AppShell` without knowing who is asking, **stop and report it** —
that is a design question about that page, not a line to improvise.

## §3 — the cost, which must be stated and not waved away

This adds one `unread_count_for_user` query to twelve page renders. The
owner's stated concern is **continuous** cost, so:

- **Say what the query is** — read it, don't assume it is a cheap
  `COUNT(*)`; report whether it is indexed.
- **Say how many queries each affected page already makes**, at least in
  order of magnitude, so one more is in proportion.

If it turns out not to be cheap, **report that and stop**: caching or a
single shared fetch per request is a design decision and it is the
architect's.

## §4 — seen red

**The compiler is the guard for recurrence**, but it is not evidence that
the badge is now right. So also:

- **A test on a page that previously could not show it.** `/today` is the
  one to pick, because it is the reader's first screen and it was round
  2's own blocker. Seed an unread notification, load `/today`, assert the
  badge **and** the accessible name carry the count.
- **Seen red by removing the threading from that one handler** — which,
  with the prop required, will now be a *compile* error rather than a
  silent zero. **Quote the compiler error**: that is the proof the trap is
  closed, and it is better evidence than a failing assertion.
- Round 2's two containment tests must still pass unchanged.

## §5 — the doc comment

Correct `layout.rs:65-67` to say what is true **after** this round, and
say **why it is now true** — the prop is required, so the compiler
enforces it. Do not leave a sentence whose truth depends on nobody making
a mistake. **This one is the round's to write**, not the architect's: it
describes the code the round changes, which is `DIA-004`'s own division.

## §6 — gates

`fmt`; `clippy --workspace --all-targets -- -D warnings`, **both default
and `--no-default-features --features postgres`** (twelve handlers change,
and `PLT-001`'s Windows job plus the postgres clippy step both now exist);
all 34 integration binaries, since every one of these pages' chrome
changes; `contrast_scan`, `touch_target_scan`, `browser-checks`;
`mdbook`; rustdoc baseline **6**; `DEC-007` from the runner, **from and
to** — remember `test.yml`'s own block lines define it.

## §7 — exit conditions

1. `unread_count` required on `AppShell`; no default, no `Option`.
2. All twelve handlers thread the caller's own count; any that cannot is
   reported rather than guessed at.
3. The query's cost stated, with the affected pages' existing query order
   of magnitude.
4. A `/today` test asserting badge **and** accessible name carry a seeded
   count, and **the compiler error quoted** from removing the threading.
5. `AppShell`'s doc comment true, and saying why it is now structurally
   true.
6. `§6`'s gates after the last edit, `DEC-007` from and to, CI quoted.
