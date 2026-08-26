# RFC 0005: Quality consolidation

**Status**: Proposed
**Target**: 0.27.0 (Phase E) — §9 shipped at 0.22.0, §10 pulled forward to 0.25.0,
§12-13 added from `REL-0.26.0`'s review
**Related spec sections**: §40 (Phase E plan), §11.5.5 (API
authorization QA), §21.4 (optimistic-lock conflict),
§30-34 (ABDD axes)
**Last updated**: 2026-08-25

## Summary

A QA pass over everything Phase A–D shipped: keyboard,
screen reader, mobile, language consistency, colour contrast,
and security (authorization boundaries + optimistic-lock
behaviour). No new feature work; the goal is that every
shipped surface satisfies the six ABDD axes (§30) plus the
two security axes (§40.1.6).

This is the Phase that closes the few `#[ignore]`d tests
left in 0.19.0 and audits the surfaces that grew piecemeal
across PRs.

## Background

Phases A–D each shipped surfaces with their own feature
requirements; ABDD acceptance was checked at the PR level.
Phase E is the "step back and look at the whole" pass. Two
specific drivers:

1. The spec §40 explicitly schedules QA work as its own
   phase rather than baking it into every feature PR. Phase
   E is the slot where keyboard navigation (`j/k`), WCAG AA
   contrast, and mobile completion get systematically
   addressed instead of opportunistically.
2. The security axes (§40.1.6) were added in v2.1
   specifically because the §11.5 boundary and §21.4
   optimistic-lock contract are easy to *almost* implement
   correctly — the "team admin reads member burnout"
   edge case in B-PR2 was a textbook example. Phase E
   inventories every entry point and confirms the
   boundaries hold.

## Requirements

### Six ABDD axes (§30, every surface)

The six axes restated for clarity:

1. **Keyboard** — every action reachable.
2. **Focus** — visible focus ring; predictable post-action
   focus location.
3. **Screen reader** — meaningful announcements; no
   dead-end "graphic" labels.
4. **Color** — information conveyed by more than colour
   (icon, text, position).
5. **Mobile** — the four key flows (Today / Inbox / Issue
   detail / Calendar today-view) complete on a phone.
6. **Live update** — dynamic changes (status flip,
   notification arrival) announce via aria-live.

### Two security axes (§40.1.6)

7. **Authorization** — every surface that handles personal
   data refuses cross-user reads (other user's auth token
   → 403; admin reading member personal data → 403; no
   auth → 401 JSON for `/api/*`, 303 to login for HTML).
8. **Concurrency** — every mutation that owns an entity
   honours the §21.4 optimistic-lock contract; conflicts
   surface as 409 with the structured body and the UI
   rolls back without celebratory language.

### Must-haves

1. Every surface listed in §40.2 receipt 1–6 passes.
2. Authorization audit table populated (see Design §1).
3. Optimistic-lock audit table populated.
4. The single ignored test from 0.19.0
   (`cross_user_settings_post_returns_403`) is activated
   or deleted with cause.
5. Keyboard navigation `j/k` works on issue lists and
   kanban (long-promised in §32).
6. Locale audit complete: English UI strings only (the
   mixed-language drift the spec calls out in §40.1.4 is
   resolved).
7. Colour contrast audit run against every page: WCAG AA
   4.5:1 minimum.

### Nice-to-haves

- Snapshot-test contrast values per theme so future Tailwind
  upgrades don't silently regress them.
- Lighthouse score ≥ 95 for the four key flows (proxy
  metric; not a hard requirement).
- Bundle size: `static/dm.js` (RFC 0004) under 8 KB
  uncompressed.

### Explicitly out

- New features. If a new feature is found *necessary* during
  the audit, it gets its own RFC and ships in a later
  version.
- Cross-team aggregation surfaces. The spec is unambiguous
  about not adding them; an audit isn't license to invent
  them.
- Refactoring for the sake of refactoring. Phase E touches
  code only where the audit found a defect.

## Design

### 1. Authorization audit

Build a table — owned by this RFC and updated as the audit
proceeds — listing every endpoint that carries personal
data (§11.5.1) or per-user mutation:

| Endpoint | Auth check | Cross-user test | Status |
|---|---|---|---|
| `GET /today` | AuthUser → self via cookie | implicit (no user_id) | ✓ 0.18.0 |
| `GET /today/calendar` | AuthUser | implicit | (added in PR3) |
| `GET /inbox` | AuthUser | implicit | ✓ |
| `GET /api/users/{id}/burnout` | ApiAuthUser + require_self | `auth_boundary::burnout_endpoint_walls_off_other_users` | ✓ 0.18.0 |
| `GET /api/users/{id}/capacity` | same | `..._capacity_..._other_users` | ✓ |
| `GET /api/users/{id}/notifications` | same | `..._notifications_..._other_users` | ✓ |
| `GET /settings` | AuthUser | implicit | ✓ |
| `POST /settings/wip-limit` | AuthUser | implicit — session-scoped, single row | ✓ |
| `POST /settings/capacity*` | AuthUser, lock-checked | implicit (no user_id in URL) | ✓ |
| `POST /inbox/resume` | AuthUser | implicit (no user_id) | ✓ 0.24.0 |
| `POST /inbox/email-opt-in` | AuthUser | implicit | ✓ 0.24.0 |
| `POST /inbox/mark-all-read` | AuthUser | **bulk** — negative ✓ (`QA-007`), positive ✓ (`QA-008`) | ✓ |
| `POST /inbox/resume` (bulk half) | AuthUser | **bulk** — positive ✓ (`inbox_refinements`), **negative missing** | open |
| `POST /settings/notifications/silence-all` | AuthUser | **bulk** — positive ✓ (`inbox_refinements`), **negative missing** | open |
| `GET /projects/{id}/delete` | AuthUser + `find_accessible` + **owner check** | **unaudited** | 0.25.0 |
| `GET …/issues/{iid}/delete` | AuthUser + `find_accessible` | **unaudited** | 0.25.0 |
| `GET …/sprints/{sid}/delete` | AuthUser + `can_manage_team` | **unaudited** | 0.25.0 |
| `POST …/status/detail` | AuthUser + `find_accessible` | **unaudited** | 0.25.0 |
| `POST …/status/list` | AuthUser + `find_accessible` | **unaudited** | 0.25.0 |

*Reconciled 2026-08-25.* The "fill in as they ship" row was a promise to a
future that has now arrived twice — 0.24.0's two inbox routes and 0.25.0's
five. Filling it in was nobody's step, which is the same shape as a status
nothing checks.

**The three `GET` delete rows are new in kind, not only in number.** Before
0.25.0 no `GET` in this application rendered a page whose only purpose was to
authorise a mutation. `GET /projects/{id}/delete` carries an owner check that
`GET /projects/{id}` does not, so the interstitial is a **narrower** boundary
than the screen that links to it. Audit whether the `POST` half enforces the
same narrowing — a `GET` that refuses and a `POST` that does not is a boundary
that exists only in the user interface.

The audit fills in the right-hand column for every row. New
test entries land in `tests/auth_boundary.rs`.

*Corrected 2026-08-25 from `QA-007`.* The `wip-limit` row said `partial`,
"needs test once user_id surface lands — see ignored test", and this paragraph
described what to do with that test. **There is no ignored test.** It was
**withdrawn** at 0.20.0 with cause — settings mutations are self-scoped by
session rather than addressed by `user_id` in the path, so the boundary it
asserted cannot exist — and the requirements baseline §9.3 has said so since.
This RFC was written after that and cited it anyway. Two of my own documents
disagreed for five releases; the dev team settled it by reading
`auth_boundary.rs` rather than by choosing a document.

**"Implicit" covers two different guarantees, and only one of them is
untestable.** This distinction is normative for the audit:

| Shape | Example | Cross-user test |
|---|---|---|
| Session-scoped, **single row** addressed by `&user.id` | `POST /settings/wip-limit` | **Not constructible** — there is no other user's row to aim at |
| Session-scoped, **bulk** — an unbounded set selected by a predicate | `POST /inbox/mark-all-read` | **Required** |

The second was closed as "not constructible, same as `wip-limit`" during
`QA-007`, and a plant proved otherwise: deleting `user_id = ?1` from
`mark_all_read`'s `WHERE` marks **every** user's notifications read, and all
194 tests pass. A missing `user_id` in the path means impersonation is not
constructible. It says nothing about a predicate that can lose a term.

That is `§10.3`'s point from the other side: a session extractor plus an
unscoped query is one layer, not two.

**A bulk route needs both assertions.** Added 2026-08-25 from `QA-007` round
2's review, extended the same day from `QA-008`'s sweep.

The cross-user test asserts what the route must **not** do. On its own it is
satisfied by a route that does nothing at all: replacing `mark_all_read`'s
predicate with one that matches no rows leaves the button inert and **all 195
tests green**. So every bulk row needs a positive assertion — that the
operation affects the caller's own rows — or the negative one certifies an
empty feature.

**And the converse, which `QA-008`'s sweep found.** `silence-all` and
`resume` are in the opposite state: both have a genuine multi-row positive
assertion in `inbox_refinements`, and **neither has a cross-user test**. A
positive assertion alone cannot see a predicate that has lost its `user_id`
term — which is the exact defect `QA-007` planted on `mark-all-read`.

**A bulk row is not closed until both halves exist.** One half in either
direction reads as coverage and is not.

**Both rows were absent from this table entirely** until 2026-08-25. They were
found by reading `app.rs`, not by working these rows — the second time this
series has turned up a route the table never listed. A table that is itself
the audit's work-list cannot reveal what it omits.

### 2. Optimistic-lock audit

Symmetric table:

| Mutation | Lock check | Conflict test | Rollback UI |
|---|---|---|---|
| `POST /projects/{id}/issues/{iid}` (update) | yes | `optimistic_lock::issue_update_with_stale_timestamp_returns_409` | (D-1 + D-3) |
| `POST /projects/{id}/issues/{iid}/status` | yes | `..._status_change_..._returns_409` | (D-1) |
| `POST /projects/{id}` | yes | `..._project_update_..._returns_409` | n/a (HTML form, page reload) |
| `POST /teams/{slug}/sprints/{sid}/edit` | yes | `..._sprint_start_..._returns_409` (analogous; covers update path) | n/a |
| `POST /teams/{slug}/sprints/{sid}/start|complete|delete` | yes | covered above | n/a |
| `POST /settings/capacity/{id}` | yes | `..._capacity_period_edit_..._returns_409` | n/a |
| `POST /teams/{slug}/sprints/{sid}/plan/add|remove` | **no** (join-table, intentional) | n/a | n/a |
| `POST …/status/detail` \| `/status/list` | yes — shared `apply_status_change` | `status_control`, `optimistic_lock` | D-1 (0.26.0) |
| `POST /teams/{slug}/sprints/{sid}/delete` | **yes** | — | n/a |
| `POST /settings/capacity/{id}/delete` | **yes** | — | n/a |
| `POST /projects/{id}/delete` | **no** | — | n/a |
| `POST /projects/{id}/issues/{iid}/delete` | **no** | — | n/a |

*Reconciled 2026-08-25, and the last four rows are the finding.*

**Two of the four destructive deletes take a lock and two do not.** Verified by
reading the handlers: `sprints::delete_sprint` and `settings::delete_capacity`
call `check_optimistic_lock`; `projects::delete` and `issues::delete` take no
form at all — no `client_updated_at` reaches them.

**The mechanism for it exists and is used once.**
`render_delete_confirmation` takes a `hidden_fields: Vec<(String, String)>`
parameter. The sprint interstitial passes `client_updated_at` through it. The
project and issue interstitials pass `Vec::new()`.

**RFC 010 widened the window this protects.** A delete used to be one `POST`
from a page the user was looking at. It is now `GET` (render "Delete issue:
Fix login bug"), a pause while the user reads, then `POST`. Nothing binds the
`POST` to the state the `GET` displayed, so a user can confirm a deletion of an
entity that has changed underneath the sentence naming it — and for issues that
matters more than for most entities, because the delete **cascades**.

Whether a delete *should* lock is a real design question and this RFC does not
prejudge it: the row is gone either way, so a stale timestamp corrupts nothing.
What is not defensible is deciding it differently in four places without
recording a reason. The audit settles it once, in both directions.

**A copy finding from the same read, recorded here because it is the same
window.** `issues::delete_confirm` renders `ConfirmDeleteCannotBeUndoneNote` —
*"This cannot be undone."* — while `projects::delete_confirm` renders
*"All its issues will be deleted too. This cannot be undone."*
`issues.parent_issue_id` is declared `ON DELETE CASCADE` (`0015_sub_issues.sql`)
and the pool sets `foreign_keys(true)`, so **deleting a parent issue deletes its
sub-issues** and the screen naming what will be deleted does not say so. RFC 010
exists so a confirmation states its consequence; on the one route where the
consequence exceeds the entity named, it does not.

For mutations newly added in C-PR2/PR3/PR4 and Phase D, the
audit confirms each entry has a row.

### 3. Language audit

*Reconciled against the code 2026-08-25, before `QA-008` was written. **The
conversion this section describes is already done.** What is missing is the
guard that keeps it done.*

The original text: run a script scanning templates and Rust string literals for
Hiragana, Katakana and common Kanji; convert each to English; leave comments in
the language they were written in.

**Scanned.** Every occurrence of Japanese in the tracked tree is one of:

- **A comment citing a Japanese source document** — `peisear-core/src/lib.rs`,
  `components/me.rs`, `tests/auth_boundary.rs`, and others quote the V2.1 brief
  by section and phrase. §3 already exempts these, and they are load-bearing:
  a citation translated into English is no longer a citation.
- **`CHANGELOG.md` and `ROADMAP.md`**, same shape — quoted requirements.
- **One test input**: `escape_like_meta("ログイン")` in `storage/search.rs:216`,
  asserting that CJK survives `LIKE` escaping. That is a test fixture, and a
  good one.

**No user-visible Japanese string exists in the product.** RFC 006 did this
work at 0.21.0 as a side effect of moving all copy into the message table —
which is why nobody ever ran §3's script.

**What is live is the reverse direction, and it is not guarded.** Planted a
Japanese string into `en.rs`, the *English* renderer:

```rust
MessageKey::NewSubIssueLabel => "新しいサブ課題".to_string(),
```

`cargo test --workspace`: **195 passed, 0 failed.** `prose_scan` tests
`is_ascii_alphabetic`, so it does not see a non-Latin literal at all;
`find_violations` looks for prohibited English phrases and finds none in a
string with no English in it. The English locale can be given non-English copy
and every gate stays green.

**So §3 becomes a guard, not a conversion.** The rule: nothing the English
renderer produces may contain characters from a non-Latin script. Not "ASCII
only" — the shipped copy legitimately uses `—`, `←`, `✓`, `⚠`, and curly
quotes. Comments are out of scope, as they always were.

The spec stays Japanese; it is a separate document and not user-visible.

### 4. Colour contrast audit

*Reconciled against the code 2026-08-25. **The failures this section expects
to find are not where it is looking.***

The original text: run a checker over the documented colour pairs, and where a
Tailwind class fails AA, replace it with the next darker or lighter variant.

**We barely use named colour classes for body text.** The theme is DaisyUI's
`corporate` (`layout.rs:29`) and its tokens are not ours to audit — if
`base-content` on `base-100` failed AA, that is a theme choice, and swapping to
"the next darker variant" is not available for a semantic token.

**What is ours is the opacity modifier**, and it is applied 130 times:

| Class | Uses |
|---|---|
| `text-base-content/60` | 67 |
| `text-base-content/70` | 39 |
| `text-base-content/50` | 19 |
| `text-base-content/80` | 3 |
| `text-base-content/40` | 2 |
| `text-base-content/30` | 2 |
| `opacity-30` … `opacity-90` | 32 |

Every one of those **reduces** the contrast the theme provides, and the
reduction is a decision this project made, not one it inherited. A token that
passes AA at full strength can fail at 60% and will almost certainly fail at
30%. **That is the audit.**

`NFR-A11Y-005` is the requirement (AA, 4.5:1, P1) and it has read *Not
verified* since 0.19.1.

**Where the results go is an open question, not a decision.** The original text
named `docs/src/accessibility.md`. `docs/src/` contains only `assets/`, there
is no `book.toml`, and `DEC-020` — where this project's documents live — is
still unresolved. Do not create a file that presumes an answer to it.

---

#### The measurement, 2026-08-26 (`QA-012`, `QA-013`)

**This table expires.** It is true of **`daisyui@4.12.14`**'s `corporate`
theme and of nothing else. A DaisyUI upgrade or a theme change invalidates
every number below.

Tokens resolved from the pinned CDN build's `[data-theme=corporate]` block:

| Token | OKLCH | sRGB |
|---|---|---|
| `base-content` (`--bc`) | `22.3899% 0.031305 278.07229` | `#181A2A` |
| `base-100` (`--b1`) | `100% 0 0` | `#FFFFFF` |
| `base-200` (`--b2`) | `93% 0 0` | `#E8E8E8` |
| `base-300` (`--b3`) | `86% 0 0` | `#D1D1D1` |
| `primary` (`--p`) | `60.39% 0.228 269.1` | `#4D6EFF` |

Foreground composited against the background, then WCAG relative luminance.
Computed twice independently — by the implementer and by the reviewer, from
separate implementations — and agreeing to two decimals in all 24 cells.

| Modifier | `base-100` | `base-200` | `base-300` |
|---|---|---|---|
| solid | 17.21 | 14.01 | 11.25 |
| `/90` | 12.87 | 10.83 | 8.98 |
| `/80` | 9.08 | 7.95 | 6.85 |
| **`/70`** | **6.36** | **5.76** | **5.15** |
| `/60` | 4.54 | **4.23 ✗** | **3.89 ✗** |
| `/50` | **3.32 ✗** | **3.16 ✗** | **2.98 ✗** |
| `/40` | **2.50 ✗✗** | **2.42 ✗✗** | **2.32 ✗✗** |
| `/30` | **1.93 ✗✗** | **1.89 ✗✗** | **1.83 ✗✗** |

`✗` fails AA 4.5:1. `✗✗` fails 3:1 as well — no real background in this theme
where it could pass at any text size.

**`/70` is the floor**, and the reason is `/60`'s **0.04**. A muted tier that
passes by four hundredths is one theme adjustment from failing with nothing to
report it. 111 sites were identified; 108 were mechanical swaps and 3 needed
the arithmetic below.

#### `opacity` compounds; `text-base-content/N` does not

The handoff's own table assumed a single flat layer. That holds for a colour —
CSS `color` is one property and a nested value replaces an inherited one. It
does **not** hold for bare `opacity`, which composites the element's whole
rendering *including a colour that already carries alpha*. Found by the
implementer checking all 21 bare-opacity sites individually rather than
sampling:

| Site | Nesting | Effective | Ratio | Resolution |
|---|---|---|---|---|
| `projects.rs:71` | `opacity-60` in `/70` | 0.42 | **2.64** | drop the child modifier → 6.36 |
| `sprints.rs:561` | `opacity-60` in `/80` | 0.48 | **3.13** | `opacity-90` → 6.82 |
| `calendar.rs:85` | `opacity-60` over `bg-primary/10` | — | **4.35** | `opacity-70` → 6.00 |

`projects.rs:71` at 2.64 was **worse than the `/30` tier the rule bans
outright** — the rule as written would have left it standing. Forcing the
mechanical swap onto it would have given 0.49 → 3.24, still failing, while
satisfying every acceptance criterion and passing the new guard.

#### Every background in the tree, not only the base tokens

| Background | Composited | Foreground | Ratio |
|---|---|---|---|
| `bg-primary/10` on `base-100` | `#EDF1FF` | `opacity-70` | 6.00 |
| `bg-primary/10` on `base-200` | `#D8DCEA` | `opacity-70` | 5.45 |
| `bg-primary/15` on `base-100` | `#E4E9FF` | `opacity-70` | 5.82 |
| `bg-primary/15` on `base-200` | `#D0D5EB` | `opacity-70` | 5.29 |
| `bg-primary/25` **hover** on `base-100` | `#D3DBFF` | `opacity-70` | 5.44 |
| `bg-primary/25` **hover** on `base-200` | `#C1C9EE` | `opacity-70` | **4.97** |
| `bg-info/10` on `base-100` | `#E5F8FF` | solid | 15.72 |

`bg-info`/`bg-warning`/`bg-success` also appear, on `w-2 h-2` status dots that
carry no text.

**The tightest margin in the whole audit is 4.97:1, and it is a hover state.**
No screenshot and no static analysis would catch it, because it only exists
while a pointer is over the element. It is the first thing a theme change
breaks.

#### What is guarded, and what is merely true

`contrast_scan` bans `text-base-content/{10..60}` under
`crates/peisear-web/src/`. It says nothing about **which background** a passing
modifier sits over, and nothing about bare `opacity`.

Both were deliberate. `opacity` is not a text property — `calendar.rs`'s two
empty `<td>` cells are a legitimate non-text use — and distinguishing text from
non-text needs rendering, which `§10.15` records this project does not do.

So: **every tinted background is enumerated and passing, and no bare `opacity`
below the floor remains — but neither fact is guarded.** Both drift the moment
someone adds a `bg-*` container or an `opacity-*` on text. That is the position
to state; "contrast is handled" is not.

#### Hierarchy lost to the floor, reported and not fixed

Four steps of grey became two. Where a real distinction collapsed:

- **`sprints.rs`'s summary cards**, four repeated instances: a `/60` label and
  a `/50` sub-detail are now both `/70` and indistinguishable. The most
  repeated instance in the sweep, and the one worth fixing — **by size, not by
  contrast**.
- **`me.rs`'s burnout panel**: heading `/60` and sub-heading `/50` now render
  identically, flattening a two-level structure.
- **`issues.rs`'s issue-list table**: assignee `/70` and updated-date `/60`
  read as two tiers of metadata; now one.
- **`teams.rs` and `notification_preferences.rs`**, running the other way: a
  `/50` hint beside a bold label becomes *more* prominent at `/70`. The change
  made something louder, which nobody expected to find.

No fix applied. Carrying hierarchy by size, weight or placement is a design
change and not this RFC's.

### 5. Keyboard navigation

*Reconciled against the code 2026-08-25. **This section specified the wrong
requirement.***

The original text described `j`/`k` selection movement, a `?` shortcuts modal,
and a new `static/keynav.js`. That is **`NFR-A11Y-009`** — *"SHOULD provide
list navigation shortcuts"*, **P3**.

**`NFR-A11Y-001` is the requirement Phase E owes**: *"Every primary flow — list
to detail to back, edit and cancel, marking a notification read — MUST be
completable with the keyboard alone."* **P0**, and *Partial* since 0.19.1 with
"systematic audit is Phase E" written next to it.

A section titled "Keyboard navigation" that builds the P3 convenience and never
audits the P0 completeness is inverted. **The audit is the deliverable; the
shortcuts are not in Phase E at all.**

Three further reasons the original plan is now wrong:

1. **`DEC-021`.** `keynav.js` is JavaScript-only, so it can never *be* the
   keyboard path for anything — only a shortcut layer over one that already
   works. The original text did not say this, and a reader could have built it
   believing the requirement was being met.
2. **`§10.15`.** The shipped JavaScript is executed by no test. A fourth file
   with real behaviour widens that, and `QA-003` named exactly this residual:
   *"a fourth JavaScript file added later gets no reference test
   automatically."*
3. **`NFR-A11Y-008` is now in force.** It read "Deferred with Phase D"; D-1 and
   D-2 shipped at 0.25.0-0.26.0. It requires conflict notifications to use an
   **assertive** live region — and `board.js` and `dm.js` announce conflicts
   into `#status-announcements`, which is `role="status"`, a **polite** region
   (`components/issues.rs:117`). One region carries both *"Moved to Done."* and
   *"Another member changed this issue first."*, and those two want opposite
   politeness. Verified 2026-08-25.

So §5 becomes: **audit `NFR-A11Y-001` per primary flow, and fix `NFR-A11Y-008`.**
`NFR-A11Y-009` is deferred out of Phase E — a P3 SHOULD does not outrank the
accessibility axes that have been unverified for eight releases.

### 6. Mobile completion

*Reconciled against the code 2026-08-26. **Half of this section cannot be done
here, and the half it omits is the measurable one.***

The original text: manual QA against four flows, with mobile screenshots
recorded in `docs/src/mobile-checklist.md`.

**What cannot be done.** `QA-011` established that no browser is available in
this environment. Manual QA at narrow width and screenshots both need one, and
a markup assessment is not the same thing — that distinction was set in
`QA-011` and holds here. And `docs/src/` still has no `book.toml` and `DEC-020`
is still unresolved, so the checklist file cannot be created without answering
a question that is the owner's.

**What the original text omits, and what §6 becomes.** `NFR-A11Y-007` — 44 × 44
touch targets, P1 — appears in the original only as one clause of one bullet
("mark-read tap target ≥ 44 px"), as something to eyeball on one screen. **It
is measurable from source**, because DaisyUI's control heights are fixed values
in the pinned stylesheet:

| Class | Resolved height | Uses | vs 44 px |
|---|---|---|---|
| `btn-sm` | 2rem / 32 px | 64 | ✗ |
| `btn-xs` | 1.5rem / 24 px | 18 | ✗ |
| `input-sm` | 2rem / 32 px | 29 | ✗ |
| `select-sm` | 2rem / 32 px | 21 | ✗ |
| `input-xs` | 1.5rem / 24 px | 5 | ✗ |
| `select-xs` | 1.5rem / 24 px | 2 | ✗ |
| `checkbox` | 1.5rem square / 24 px | 10 | ✗ |
| `btn-md` (default) | 3rem / 48 px | 0 | ✓ |

Approximately **149 sites**, and **exactly one control in the product
complies**: `issues.rs:661`, the board card's status buttons, carrying
`min-h-11 min-w-11` from `DEV-002`.

**Nothing asserts even that one.** The requirements baseline listed
`board_keyboard` as verifying `NFR-A11Y-007`; it does not, and no test anywhere
in the suite asserts a touch-target dimension. Corrected in the baseline
2026-08-26.

**The requirement may be what needs revisiting.** `SPEC §33.2`'s 44 px is
stricter than WCAG 2.2's AA criterion (2.5.8, 24 × 24 with a spacing
exception); 44 px is 2.5.5, which is AAA. Raising 149 controls changes this
product's density fundamentally — a Kanban card whose status buttons are 44 px
tall is a different card, and this is a tool whose screens are dense on
purpose.

**So §6 splits.** The touch-target measurement is a source audit and can be
done now. The four flows' mobile behaviour needs a browser and stays open,
named as such rather than quietly satisfied by a markup pass. The decision
between raising 149 controls and amending `SPEC §33.2` is the owner's, and it
should be made against the table above.

### 7. Aggregate inferability check

*Reconciled against the code 2026-08-26. **All three surfaces this section
names are settled. The two that need the audit did not exist when it was
written.***

The original text proposed suppressing the workload chip at N < 2, and applying
the same logic to the sprint plan's capacity hint and a per-assignee rollup.

**All three are closed, and one of them was closed by doing and reverting
exactly what this section proposes:**

- **The workload chip.** A N < 2 suppression was added during `DEV-003` on the
  strength of `NFR-PRIV-007`, then **withdrawn at 0.20.0 as a
  misapplication**. A chip labelled with a person's name is not an aggregate;
  it is individual workload, governed by `NFR-PRIV-002`. Implementing §7 as
  written would re-introduce a defect this project already diagnosed and
  removed.
- **The capacity hint** was **withdrawn at 0.22.0** — the product's first
  genuine `NFR-PRIV-007` case. There is no capacity code in `sprint_plan.rs`.
- **The per-assignee mini-rollup** never landed in RFC 001.

#### What actually needs auditing

Two aggregates now exist that did not when this section was written, and the
0.27.0 baseline predicted them by name — *"when one is built, sprint charts and
health trends are the likely candidates."*

- **The sprint burndown** (`sprints.rs:641`, `render_burndown`) plots
  `cumulative_committed` and `cumulative_completed` **per day**.
- **The team velocity chart**, a median across recent completed sprints.

**The decisive fact, verified 2026-08-26**: `issue_events` — where completion
timestamps live — **is referenced nowhere in `peisear-web`**. No screen exposes
when an issue was finished. The issue list shows what is done; only the
burndown shows *when*, day by day.

So the burndown is materially different from the workload chip, and the
difference is the reason the 0.20.0 revert was right and this is not the same
question:

| | Workload chip | Burndown |
|---|---|---|
| Shows | a current snapshot | a day-by-day series |
| Reconstructible from other surfaces | **yes** — the issue list carries assignees and statuses | **no** — nothing else exposes completion timing |
| At one contributor, is | that person's current load, already visible | **that person's work-rate profile over time** |

On a product whose stated commitment is management-not-oversight (`DEC-019`),
a single-contributor burndown is a productivity graph of one named person,
assembled from data no other screen shows.

**Neither chart can currently express a suppression.** `BurndownPoint` is
`{ day, cumulative_committed, cumulative_completed }` — it does not know how
many people contributed, and nothing plumbs that through.

**§7 becomes**: audit the burndown and the velocity chart, establish what each
discloses beyond other surfaces, and report. **Suppression is not decided
here** — a one-person team losing its burndown entirely is a real cost, and
`NFR-PRIV-007` is a **SHOULD** at P2, not a MUST.

### 8. Phase A-D follow-up sweep

Final pass to confirm the original Phase A-D items still
satisfy ABDD/security after later PRs landed. Grep for:

- `// TODO` and `// FIXME` in handlers and components.
- `#[ignore]` in tests outside `tests/auth_boundary.rs`.
- `unimplemented!()` and `todo!()` in shipped code.

Each gets resolved: either fixed, ticketed for a future RFC,
or annotated with cause.

### 9. The test harness itself — pulled forward to 0.22.0

*Added 2026-08-11 from baseline `§10.13`, found while reviewing
`REL-0.21.0`.*

`TestApp::spawn` names its temporary database directory from
`SystemTime::now().as_nanos()` alone. Two tests entering it in
the same clock tick share a directory and a `test.db`;
`create_dir_all` succeeds on an existing directory, so nothing
signals the collision and the second arrival fails with
`SqliteError { code: 5, "database is locked" }`. Roughly one
`cargo test --workspace` run in two, on a different test each
time. Reproduced at `0.20.1` as well, so it is not new.

**Why it belongs to this RFC.** Phase E is where test debt is
paid, and this is test debt of the most consequential kind:
the harness that produces every gate result in this project.

**Why it does not wait for 0.24.0.** Every release from here
runs those gates. A suite that fails half the time on the
obvious command trains people to re-run rather than read, and
that habit is what a flaky test costs — not the minutes.

Two parts:

1. Make the suffix unique — process id and an atomic counter
   alongside the clock, or a crate that guarantees it. Prefer
   the crate: a hand-rolled unique-name scheme is what failed.
2. **Add a repeated full-workspace run to the gate set.**
   `DEC-007` mandates per-crate and per-target runs *for
   isolation*, and that procedure never triggers the
   collision. Every gate log this project has captured is
   honest and green, and the defect lived underneath all of
   them.

Item 2 is the finding, not item 1. **An isolation procedure
adopted to make results trustworthy hid a defect in the thing
producing them.** A gate set needs at least one run under the
conditions a contributor will actually use, or it measures
only the conditions it chose.

### 10. Three defects from CONF-001's review — pulled forward to 0.25.0

*Added 2026-08-16, from `.git-exclude/reviewed/CONF-001-review.md` §4–§6.*

Three independent defects surfaced while reviewing `CONF-001`. None is a
feature, none needs its own RFC, and all three are the kind of thing Phase E
exists to sweep up — so they arrive here rather than inventing a home, the same
way §9 did.

**They are pulled forward to 0.25.0** because two of them are reachable today
and the third makes a guard harder to use than it should be.

#### 10.1 An active sprint can be deleted

`handlers::sprints::delete_sprint` resolves membership, checks
`can_manage_team()` and the team match, verifies the optimistic lock, and
deletes — **for any status, `Active` included**.

The UI does not link delete for an active sprint, so this was read as a dead
path during `CONF-001`'s review. It is not: the route is live and destructive,
and `CONF-001`'s new confirmation `GET` will happily render "you are about to
delete *X*" for a team's running sprint.

**Owner decision, 2026-08-16: an active sprint may not be deleted.** At most one
sprint per team is active — `OtherSprintActiveInTeamMessage` exists because of
that — so the live one is not equivalent to a planned one, and deleting it
silently discards the state a team is currently working in.

The path out already exists: complete it, then delete it.

#### 10.2 ~~Project delete reports success to a non-owner~~ — **withdrawn, the defect does not exist**

*Withdrawn 2026-08-16, before implementation.* This entry claimed
`handlers::projects::delete` relied entirely on the storage layer's
`WHERE owner_id = ?2`, so that a non-owner deleted zero rows and was told the
project was deleted.

**False.** `peisear_storage::projects::delete` ends with
`if res.rows_affected() == 0 { return Err(StorageError::NotFound); }`, present
since v0.2. A non-owner's `POST` has always returned 404 with the project
intact.

The error was mine: I read that function through its `DELETE`, its `WHERE` and
its binds, and stopped three lines before the check that makes it correct. The
claim then travelled — a review, this section, and a dispatched handoff, each
citing the last. `QA-002`'s implementer checked empirically before implementing
and reported it.

**Kept rather than deleted**, because the register's own rule is that closed and
withdrawn items stay with their resolution. A section that quietly vanishes
teaches nothing; this one records that a defect can be manufactured by reading
most of a function.

Nothing to fix. The handler-level check `QA-002` added on this entry's
instruction is reverted — RFC 005's own "explicitly out" forbids refactoring
where the audit found no defect, and `rows_affected() == 0 → NotFound` **is**
the authorisation outcome, deliberately, not an implicit signal being abused.

#### 10.3 `prose_scan` scans comments as if they were code

A doc comment quoting attribute markup — `onsubmit="return confirm(...)"` in
prose — fails `prose_scan`. Reproduced in review.

**The sibling guard already fixed this.** `test_harness_scan`'s first iteration
false-positived against its own doc comment for exactly this reason, and
QA-001's round-1 correction was `strip_line_comments`. `prose_scan` strips only
`#[cfg(test)]` blocks and never received it.

Two guards, one false-positive class, one fixed and one not — the lesson stayed
local to the file that learned it, which is the more interesting half of this
entry.

### 11. Redirect construction — added 2026-08-16, from STATUS-001's review

Three handlers build a redirect by interpolating caller-supplied values straight
into a query string, unencoded:

- `handlers::sprints::plan_query_string` (`:594`) — PLAN-001, 0.22.0
- `handlers::issues::change_status_form_list` — STATUS-001, 0.25.0
- and the pattern is available to be repeated wherever a filtered view needs
  preserving across a POST

Meanwhile `percent_encode_query` exists in **two copies**
(`handlers/teams.rs:391`, `handlers/sprints.rs:829`), used for flash text and
not for these.

**Severity: low, and established rather than assumed.** axum 0.8.9's
`Redirect::into_response` does `HeaderValue::try_from` and returns a 500 with
the error string on failure — it does not panic and does not emit a split
header. A value containing `&` appends parameters to the redirect; the receiving
handlers read only known ones. There is no injection and no crash.

**What makes it audit material is the shape, not the risk.** Three construction
sites and two copies of the encoder they should be using is the
two-homes-for-one-fact pattern this project has now recorded five times, and a
redirect is a sink with different rules from the query parameter the value
arrived as — which is the reasoning STATUS-001's review request got slightly
wrong and is worth settling once.

§1's authorisation table has a natural sibling here: every place the application
constructs a redirect, and what encodes it.

### 12. Script tags nothing asserts

*Added 2026-08-25 from `REL-0.26.0`'s review, found by planting.*

`components/issues.rs:142` emits `<script src="/static/board.js" defer>`.
Delete that line and change nothing else, and **`cargo test --workspace`
still reports 178 passing**. The board ships with no drag-and-drop and no
undo, and every gate is green. Verified, not reasoned about.

`status_control.rs:485` states the opposite in a comment — that the board
"loads `board.js` instead — `boards_per_card_control_renders_unchanged`
above already pins that." That test asserts the board posts to
`/status/board` and does not pick up the two new routes. It never looks
for `board.js`.

Three files live in `static/` and exactly one of them is asserted
anywhere:

| File | Tag emitted at | Asserted by |
|---|---|---|
| `dm.js` | `components/issues.rs:564` | `status_control::dm_js_is_served_with_defer_on_both_surfaces` |
| `board.js` | `components/issues.rs:142` | **nothing** |
| `search.js` | `components/layout.rs:72` | **nothing** |

`search.js` is the worse of the two on reach — it sits in the app shell,
so it is on every page, and its tag disappearing takes search enhancement
with it everywhere at once.

**Why it belongs to this RFC rather than to a bug fix.** §8 is the Phase
A–D follow-up sweep, and this is precisely what that sweep is for: a
surface that grew across PRs and ended up with a dependency nothing
checks. It is also the second instance this project has found of *a
comment asserting what a test does* — the first was `RFC 003`'s
`global_acknowledged`, where a document and a test agreed with each other
and were both wrong. A comment is the one place with no guard.

**Not proposed: a scan.** A test that walks `static/` and asserts each
filename appears somewhere under `crates/peisear-web/src/` would extend
itself to a fourth file for free, but it would pass on a tag emitted in a
branch that never renders — which is most of what could go wrong here.
Three HTTP-level assertions for three files is complete today. The
residual gap is that a fourth file added later gets no assertion
automatically; that is named here rather than built for.

### 13. The `DEC-007` block omits a crate

*Added 2026-08-25 from `REL-0.26.0`'s review, found by the dev team in
their own gate table.*

`.github/CONTRIBUTING.md`'s `DEC-007` command block runs six of the seven
workspace members. `peisear`, the facade, is absent — not present with the
wrong flags, absent. Its single test is a doctest at
`crates/peisear/src/lib.rs:28`, so `cargo test -p peisear --lib` reports
`0` and only the bare `cargo test -p peisear` finds it.

**No coverage was ever missing.** `cargo test --workspace` runs doctests
and runs three times per release. What was wrong is provenance:
`REL-0.25.0`'s per-target table carried a `1` for that crate, and its own
`cold-gate-tests.log` contains zero `Doc-tests peisear` blocks. The number
was right; no command in the log produced it.

**The `--lib` shape is a trap, not a hole.** Three crates are invoked with
`--lib`, which would skip a doctest in any of them. There are none today —
the workspace contains exactly one doctest, the facade's — so nothing is
uncovered. It becomes a hole the day someone writes a documented example
in `peisear-core`.

**Why a guard and not just a line.** The block is a list of crate names
maintained by hand against a workspace that has grown to seven. That is
the shape that produced this, and it will produce it again at eight.

**What the guard does not catch, decided 2026-08-25 in `QA-004`'s round-2
review.** It asserts every member appears as `-p <name>` in the block. It
does **not** check that the flags on that line are right for the crate:

```bash
cargo test -p peisear --lib   # runs zero tests; the guard passes
```

That is this defect in a second shape — a facade line covering nothing.
Left open deliberately. Closing it means knowing which crates have
doctests, which means parsing fenced code blocks out of every crate's
source: a parser for one line of a contributing guide. And the block is
not the coverage boundary — `cargo test --workspace` runs three times
before every release and includes doctests, so flag drift here costs
developer feedback, not release coverage. The mitigation is prose under
the block saying why that line carries no `--lib`.

### 14. The four structural guards have no CI job

*Added 2026-08-25, found updating the requirements baseline to 0.26.0.*

The baseline's §9.1 has stated since 0.20.0: **a test crate without a CI job
does not exist.** `.github/workflows/test.yml` has a job for each of the twenty
`peisear-web` integration targets and for five of the seven crates. It has no
job running `cargo test -p peisear-web --lib`.

That is where every structural guard this project has built actually lives:

| Guard | Makes unconstructible | In CI |
|---|---|---|
| `prose_scan` | user-visible English authored in Rust (RFC 006) | **No** |
| `static_js_scan` | the same in `static/*.js` (`BOARD-001`) | **No** |
| `test_harness_scan` | §10.13's clock-derived temp paths (`QA-001`) | **No** |
| `dec_007_scan` | the `DEC-007` block drifting from the workspace (`QA-004`) | **No** |

`DEC-007`'s block in `.github/CONTRIBUTING.md` omits the same line, so a
contributor following the documented procedure does not run them either. They
execute only under `cargo test --workspace` — which **is** in the release gate,
three times, so no release has shipped without them. **The exposure is
per-pull-request, not per-release**: a change reintroducing any of those four
defect classes passes CI and is caught at the next release candidate, or by a
reviewer, or not at all.

**§13's limit, first live instance.** `dec_007_scan` asserts each member
appears as `-p <name>` but not that the flags are right for the crate.
`peisear-web` appears twenty times via `--test` lines, so the guard is
satisfied while the crate's library tests go unrun by the block. That was
recorded as a tolerable limit the same day; this is what it costs.

**Why this belongs to RFC 005 and not to a bug fix.** Phase E pays test debt,
and debt in the apparatus that enforces every other rule compounds faster than
debt in any single test. It is also the third entry in this RFC — after §12 and
§13 — where the project's verification reads as more complete than it is.

## Test plan

The Phase E test plan is largely the audit work itself; the
tests already exist in earlier crates and Phase E adds /
activates a few specific ones:

1. **Activate `cross_user_settings_post_returns_403`** when
   a user-scoped POST endpoint exists (or remove with cause).
2. **Add per-endpoint authorization tests** for any surface
   in the audit table that lacks one. Target: every row of
   the §40 audit table has a green test.
3. **Optimistic-lock conflict-rollback tests** in
   `tests/optimistic_lock.rs` for D-1/D-3 (add as those
   substeps land).
4. **Aggregate suppression tests**: in
   `tests/sprint_plan.rs` and `tests/projects.rs`, assert
   that workload chip / capacity hint render with N≥2 and
   suppress at N<2.
5. **Language audit script**: `scripts/audit-language.sh`
   shipped with the repo and run in CI as a non-blocking
   warning. (Blocking comes after the initial audit
   passes.)
6. **Contrast audit script**: same shape — `scripts/audit-
   contrast.sh` runs against documented colour pairs and
   warns on regressions.

## Security & privacy considerations

Phase E *is* the security pass. By construction:

- §11.5: the audit's purpose is to confirm the boundary
  holds across every endpoint. The audit table is the
  artefact; tests are the enforcement.
- §21.4: same — audit confirms every mutation's lock check
  and 409-rollback path.
- Audit log retention policy (deferred from V2.1): make a
  decision in this RFC's resolution. *Default: 30 days for
  audit log, 90 days for issue_events. Configurable via
  env var.*
- Aggregate inferability: section 7 above is the
  systematic answer.

## Out of scope

- Performance tuning. Phase E QA would surface a perf
  regression as a defect; perf *improvement* is its own
  separate work.
- Internationalisation infrastructure (i18n framework).
  Phase E unifies on English; future locale support gets a
  separate RFC.
- Replacing the audit doc format with a more structured
  tool (Excel, doc generator, etc.). Markdown tables are
  the format.

## Open questions

1. **Audit log retention** (raised in Sec & Privacy §). The
   spec leaves this open; this RFC's *Default* sets 30/90
   days. If a stakeholder wants different, raise here.
2. **`?` shortcut modal**: built with vanilla JS or Leptos
   island? *Default: vanilla — too small to justify
   hydration.*
3. **`j/k` collision with user input**: if focus is in a
   text input, `j`/`k` should type the letter, not navigate.
   *Default: implementation must check
   `document.activeElement` and bail when it's an
   editable element. Trivial in vanilla JS; mention in
   the implementation RFC if it gets one.*
4. **Mobile checklist screenshots**: where do they live?
   Repo (committed images) or external? Repo bloats the
   tarball; external introduces link rot. *Default: repo,
   under `docs/src/mobile-screenshots/`. Image size budget
   200 KB total.*

## References

- Spec §40 — Phase E plan
- Spec §11.5.5 — API authorization test suite
- Spec §21.4 — optimistic lock contract (the test surface
  this RFC sweeps over)
- Spec §30-34 — ABDD axes
- RFCs 0001-0004 — the surfaces this audit visits
