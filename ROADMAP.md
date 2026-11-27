# Roadmap

This document lays out where peisear is going, in three time
horizons. The Cargo workspace layout is deliberately designed so that
each roadmap item lands in one well-defined crate — see
[docs/architecture/crate-boundaries.md](docs/architecture/crate-boundaries.md)
for the mapping.

**How work is chosen today**: a handoff under `rfcs/handoffs/`, an
entry closing in the `§10.x` defect-class register, or the
specification sweep that reads every requirement's status and
acceptance each release. **The architect schedules from these and the
owner authorizes the result** — see **What ships next**, below. Not
from a release sequence fixed in advance. The Milestones and Release sequence
sections below were that fixed sequence, agreed 2026-08-01 and current
through `0.27.0` (shipped) — they are retained as history, like the
phase narrative that follows them immediately, and are superseded for
everything after. The Release cycle section's entry/exit criteria are
unaffected by this and remain how a release is cut today. See
**Unscheduled work**, below, for the kind of work this plan never had
a place for.

## Near-term

### v2.1 spec implementation (phases A–E — historical record)

The v2.1 feature specification (`SPEC`) defines a UI/UX overhaul (5-entry
navigation, sub-issue hierarchy, calendar, direct-manipulation surfaces, plus
the review-driven additions in §11.5 / §21.4 / §28.6). It rolls
out across five phases, one per minor release:

> **`SPEC` is provenance, not a published document**, and this paragraph
> linked to it as `docs/spec/peisear-feature-spec-v2.1.md` until 2026-11-03 —
> **a path that has never existed in this repository.**
> `docs/specification/README.md` says what `SPEC` is and that it is neither
> published nor a source of truth; `docs/specification/requirements.md` and
> `external-design.md` are. The link is removed rather than repointed, because
> there is nothing to point it at, and the fact it was asserting is stated once
> in the place that owns it. **Nothing caught this for the life of the file**:
> `mdbook-linkcheck2` checks the book, and `ROADMAP.md` is not in the book —
> see the candidate list under **What ships next**.

- **Phase A — Information architecture** *(0.17.0, shipped
  2026-05-03)*. 5-entry navigation (`/me` → `/today`,
  `/notifications` → `/inbox` with HTTP 308 permanent
  redirects — 308 over 301 preserves POST method on the two
  legacy POST endpoints), consolidated breadcrumb/back-link
  components, list filter+sort persistence (URL-primary,
  server view_state secondary), global search (LIKE % over
  project + open-issue scope, typeahead 8 / results 50),
  optimistic-lock contract for issue and project mutations
  (entities whose `updated_at` column predates 0.17.0) plus
  schema preparation (migration 0014) for the four
  remaining tables.
- **Phase B — Key screens** *(0.18.0, shipped 2026-05-03)*.
  `/today` panel collapsing + "what to read first" callout
  (B-1), project-health explainability (B-2), issue-detail
  edit-mode URL split with 308 legacy redirect (B-3), status
  segment UI on the detail page — display-only ahead of the
  Phase D direct-manipulation rollout (B-4). Personal-data
  API endpoints (`/api/users/{id}/burnout`,
  `/api/users/{id}/capacity`,
  `/api/users/{id}/notifications`) ship with `ApiAppError` /
  `ApiAuthUser` and §11.5 self-access enforcement (admin role
  does NOT bypass — verified by integration test). Closes the
  optimistic-lock rollout: `Sprint::updated_at`,
  `Team::updated_at`, `TeamMembership::updated_at`, and
  `CapacityRow::updated_at` are now plumbed through the
  storage SELECTs and into handler-level lock checks for
  sprint, team, membership, and capacity mutations. With
  Phase B done, every domain-entity mutation honours the
  §21.4 contract.
- **Phase C — Missing surfaces** *(in progress, 0.19.0+)*.
  - **PR1 — Sub-issue hierarchy** *(0.19.0, shipped 2026-
    05-04)*. parent_id approach per §8.3 / §8.4: nullable
    `parent_issue_id` column on `issues`, partial indices for
    top-level / parent-children query shapes, two triggers
    enforcing 1-level-only + same-project + no-self-reference
    + no-demotion-with-children. New routes
    `/projects/{id}/issues/{issue_id}/sub-issues/new` (GET
    form + POST create); detail page renders Sub-issues card
    on top-level issues with "+ Add sub-issue" affordance.
    Sprint follow-parent rule: sub-issues inherit the
    parent's sprint without a separate `sprint_issues` row;
    direct sprint assignment to sub-issues is rejected (400).
    Sprint detail listing filters to top-level only so effort
    isn't double-counted. 7 new integration tests cover the
    hierarchy, sprint-follow, and validation paths.
  - **PR2 — Sprint planning page** *(target: 0.20.0)*.
    `/teams/{slug}/sprints/{id}/plan` with backlog → sprint
    list-based assignment UI. (DnD lands in Phase D.)
    Detailed design: [`rfcs/001-sprint-planning-page.md`](./rfcs/done/001-sprint-planning-page.md).
  - **PR3 — Calendar surfaces** *(target: 0.21.0)*.
    `/today/calendar` (personal axis) and
    `/projects/{id}/calendar` (project axis). No team axis
    per §10.2. Read-only display in this PR; DnD is Phase D.
    Detailed design: [`rfcs/002-calendar-surfaces.md`](./rfcs/done/002-calendar-surfaces.md).
  - **PR4 — Inbox refinements** *(target: 0.22.0)*.
    Notification preferences UI, mark-all-read, snooze.
    Detailed design: [`rfcs/003-inbox-refinements.md`](./rfcs/done/003-inbox-refinements.md).
- **Phase D — Direct manipulation** *(target: 0.23.0)*. The
  five direct-manipulation surfaces (status click toggle,
  kanban DnD, calendar DnD, sprint-plan DnD, list reorder)
  rolled out in five sub-steps D-1 through D-5.
  Detailed design: [`rfcs/004-direct-manipulation.md`](./rfcs/done/004-direct-manipulation.md).
- **Phase E — Quality consolidation** *(now 0.27.0)*.
  ABDD QA + Security QA. The §11.5 authorization assertions
  and §21.4 optimistic-lock assertions reach full coverage on
  all relevant endpoints. WCAG AA contrast, mobile completion
  for the four key flows, language consistency.
  Detailed design: [`rfcs/005-quality-consolidation.md`](./rfcs/done/005-quality-consolidation.md).

*Phase target versions above are historical. The milestone and release
tables below were the plan from 2026-08-01 through `0.27.0` and are
retained as history too — see the note at the top of this document
for how work is chosen now.*

---

## Milestones (historical — superseded at `0.27.0`)

Releases are the unit of delivery; milestones are the unit of goal.

| ID | Milestone | Met when | Releases |
|---|---|---|---|
| **M1** | Trustworthy foundation | Nothing the documents claim is untrue, and a mechanism keeps it that way | 0.20.0, 0.21.0 |
| **M2** | Planned surfaces complete | The three designed-but-unbuilt surfaces ship | 0.22.0–0.24.0 |
| **M3** | Definition of Done reachable | SPEC §41 conditions 3 and 5 become achievable rather than blocked | 0.25.0 |
| **M4** | Direct manipulation | The five surfaces gain pointer affordances over working no-JS paths | 0.26.0 |
| **M5** | Audit consolidation | Accessibility and security verified across everything shipped | 0.27.0 |

M1 comes first because every later milestone adds surfaces that
M1's mechanisms then police — the vocabulary guard in particular.
Building M2 first would mean auditing it twice.

## Release sequence (historical — superseded at `0.27.0`)

| Version | Content | RFC | Milestone |
|---|---|---|---|
| **0.20.0** | Compliance pass — four P0/P1 corrections | [007](./rfcs/done/007-compliance-pass.md) | M1 |
| **0.21.0** | i18n architecture + vocabulary guard | [006](./rfcs/done/006-i18n-architecture.md) | M1 |
| **0.22.0** | Sprint planning page | [001](./rfcs/done/001-sprint-planning-page.md) | M2 |
| **0.23.0** | Calendar surfaces | [002](./rfcs/done/002-calendar-surfaces.md) | M2 |
| **0.24.0** | Inbox refinements | [003](./rfcs/done/003-inbox-refinements.md) | M2 |
| **0.25.0** | Explainability — indicator basis, chart alternatives | 008 *(to write)* | M3 |
| **0.26.0** | Direct manipulation | [004](./rfcs/done/004-direct-manipulation.md) | M4 |
| **0.27.0** | Quality consolidation | [005](./rfcs/done/005-quality-consolidation.md) | M5 |

Development continues in the 0.x series; no 1.0 transition is
scheduled. That is an owner decision and is not inferable from RFC
completion or architecture maturity.

**Sprint planning is fixed at 0.22.0.** It has slipped twice — behind
the compliance pass, then behind i18n. Each displacement was right on
its own; the pattern is not. Any further displacement is an explicit
decision, not a side effect.

## What ships next

**The architect maintains this table; the owner authorizes it.**
`0.64.0` was authorized 2026-11-17, on the proposal in its own row, the
same day it was proposed — **the row, not the cut** — and **cut
2026-11-18** on the owner's approval at exit criterion 7, the next day.
**Its row is removed here per exit criterion 8**: a shipped release is
history, and its record is `changelog/0.60-0.69.md`'s own `[0.64.0]` section and
Appendix C's baseline row, not this table. **What it decided is worth
carrying even though its row is not**: the release read sixty of
sixty-one smallest-text sites across three screens and then **declined
to write the rule it was scoped to consider**, closing `§10.60` on a
recorded decision that no requirement should govern text size — because
the size answer converged, because seven of its costs needed a reading
no class-token scan can do, and because 68 sites on seven unread screens
would otherwise have been governed by a rule written from three.
**`rule after design` survived its own first test by producing no
rule.** It answers the
**typography** half of the owner's own 2026-11-04 question, on the
condition the architect attached to it that day and which unit three met;
its measurements were re-derived at source rather than taken from that
note, and **the note's own figure was wrong by a third** — *"ninety
sites"* against a census of 120. **Every population figure in the row as first
written has since been corrected**, and the method was the fault rather
than the arithmetic. Two went on the day it was authorized, by
re-deriving rather than re-reading: the sites are across **thirteen**
component files, not fourteen, and the excluded remainder is an exact
figure, not an approximate one. **The rest went the same day, after
`TYPE-001` closed**: every census had been taken with a plain `grep -c`,
which counts a comment that *mentions* a class alongside a site that
*renders* it — so 126 became **120**, `/today`'s 23 became **21**, the
project screens' 35 became **34** (and **36** after that round raised
two more), 68 became **65**, and `text-sm`'s 157 became **151**. **A row
that names a population owes the command that measures it**, and the
first version of this one did not; the series' own `README.md` now
carries the command. **This row promises a read and
deliberately does not promise a floor**: whether a type requirement is
written at all is decided after its third round, which is written into
the row and into the series' own `README.md` so that a round cannot
quietly adopt one. `0.63.0` was authorized 2026-11-12, on the proposal in its own row, the
same day `0.62.0` was cut — **the row, not the cut** — and **cut
2026-11-15** on the owner's approval at exit criterion 7, three days
later, which is the gap those two decisions are meant to have. **Its
row is removed here per exit criterion 8**: a shipped release is
history, and its record is `changelog/0.60-0.69.md`'s own `[0.63.0]` section and
Appendix C's baseline row, not this table. **Its own scope was
re-scoped once, after authorization and before any of it shipped** —
`MARK-001` closed by finding the architect's census wrong (ten
markerless disclosures, not six) and the row was rewritten around the
lesson that produced, *make the property true by construction and keep
the check as a backstop*, rather than around four more repairs.
`0.62.0` was authorized 2026-11-10, on the proposal recorded below the
table — the row, not the cut — and **re-authorized 2026-11-11** when its
own reading produced two decisions outside the scope first authorized.
**A row that changes after authorization is re-authorized, not quietly
edited**, which is the rule this table already carried; that was its
first use. **`0.62.0` was cut 2026-11-12** on the owner's approval at
exit criterion 7, and **its row is removed here per exit criterion 8**:
a shipped release is history, and its record is `changelog/0.60-0.69.md`'s own
`[0.62.0]` section and Appendix C's baseline row, not this table.
Authorized by the owner 2026-10-14, as it stood then; `0.58.1`
authorized 2026-10-28, on the four-part proposal recorded in its own
row. `0.59.0` authorized 2026-10-29, on the proposal recorded in its
own row; `0.60.0` authorized 2026-10-31, on the **revised** proposal in
its own row, the first version having been withheld and the
architect's own rule reversed before it was accepted, and **cut
2026-11-03** — **including the architect's recommended direction for
PostgreSQL, which is to make it real rather than end it.**
**`0.61.0` had no row between 2026-11-03 and 2026-11-07**: the architect
withdrew that day's proposal after finding its headline content — a
published promise the published documents do not make — did not exist,
and the recommendation that replaced it, **no number until there is
something an installed user would notice**, was **accepted by the owner
the same day**. **`0.61.0` was authorized 2026-11-07 and
cut 2026-11-10**, on the fourth proposal, two of whose own directions
the architect reversed between proposing and authorizing — see the note
below the table. **Its row is removed here, per exit criterion 8**: a
shipped release is history, and its record is `changelog/0.60-0.69.md`'s own
`[0.61.0]` section and Appendix C's baseline row, not this table.
What satisfies the 2026-11-03 rule is a defect in the shipped product's
own navigation, measured in a browser rather than argued (`§10.52`).
**The authorization is of the row. The cut is a separate decision, at
Release cycle exit criterion 7, and nothing below has been approved for
release.**
**A row marked *Proposed* carries no authorization** — it is the
architect's recommendation, in the one place release numbers live,
waiting for a yes or a no. It is
reconciled before every release is cut (Release cycle, exit criterion
8), which is what stops it becoming the fixed sequence retired above —
and a row that changes after authorization is re-authorized, not
quietly edited.

**Authorizing a row is not approving a cut, and this is the one place that
says so.** They are two decisions, days or weeks apart, and until 2026-11-03
each was named once in a different document with nothing relating them: the
owner **authorizes a row** here — this scope, under this number, is the work
to do — and the owner **approves the cut** separately, against a release
recommendation, at **Release cycle, exit criterion 7**. The second is not
implied by the first; a candidate can be built, reviewed and still not cut.
**Written down because the architect conflated them in this file**: a reply
authorizing the `0.61.0` *row* was recorded as an authorization of the
*release*, held as a credit *"returned unspent"* and *"available"*, which is
not a thing either decision produces — corrected above on the owner's
clarification the same day. `DEC-056`'s four legs run only after criterion 7,
never on a row's authorization.

**Release numbers live here and nowhere else.** An RFC states its
phases and their order; which release a phase lands in is this table's
to say. Two documents carrying the same number is how the retired
sequence above went twenty minor versions stale.

| Release | Theme | Governing RFC | State |
|---|---|---|---|
| **`0.72.0`** — *proposed 2026-11-27, re-scoped the same day* | **The password a user cannot change.** **Reviewed against the owner's own philosophy on 2026-11-27 and re-scoped from the ground up; the first scoping of this number is withdrawn and the reason is recorded below the table.** **`FR-AUTH-003` is now genuinely met: a user can end every session on every device.** And the release that delivered it had to say, in its own notes, that this is **a partial remedy and not the missing one** — because whoever knows the password signs straight back in. **Measured 2026-11-27, not assumed**: `requirements.md` specifies password change or reset **nowhere** — not `Implemented`, not `Divergent`, not `Deferred`, in neither `§11` nor any phase; the word *password* appears **13** times in that document and never once as a requirement to change or reset one. The product agrees: **0** hits for `password` in `app.rs`'s **68** `.route(` registrations and **0** in `handlers/settings.rs`. **So this is the one candidate where a user has no action available at all**, which is the second sentence of the owner's philosophy — *APIs and UI/UX for users not to be confused or misunderstand* — failing in its hardest form: not a confusing screen, an absent one. **Recommended scope: the change half now, the reset half accepted as a requirement and recorded `Deferred`** — and **this reverses what this cell said hours earlier**, which was *both halves, with reset as the half that justifies the release*. `RFC 025` `§6` keeps the withdrawn version as a rejected alternative rather than dropping it. **Three measurements reversed it.** **SMTP is optional in this product**: `peisear-notify`'s own config logs *“the email channel will fail at send time; in-app delivery continues to work”* and carries on, which is right for a notification and **not survivable for a credential recovery path** — a user told *“check your email”* on a mail-less instance waits for something that will never arrive, which fails the philosophy's second sentence **worse than the absence this release is written to fix**. **Nothing in this product rate-limits anything** — 0 hits for `rate_limit`, `RateLimit` or `governor` across every crate's `src` — and a reset endpoint without one is a mail cannon any stranger can point at any address in the instance; a rate limiter is its own design, not a line inside a password feature. **And hiding the affordance correctly, where mail is unconfigured, leaves a locked-out user needing an operator action that does not exist**: there is no administrative CLI and `§2.2`'s administrator boundary is a management role, not an account one. Named and excluded as a third feature rather than smuggled in. **The change half, by contrast, needs nothing new**: the existing Argon2 hasher, the current password re-asked — without which an unattended tab is an account takeover — and `session_generation + 1` in the same transaction, which **`0.71.0` already built**, since a change that leaves the old password's sessions alive is not a change. **Amended on the owner's philosophy review, 2026-11-27, and the amendment is in the normative sentence**: the RFC first required every session to die *including the acting one*, dropping the user at `/login` for doing the right thing. **That inverts its own step 2** — an attacker holding a stolen cookie cannot supply the current password, which is why it is asked, so the acting session is the one that must survive. Corrected to *every **other** session dies, this one is re-issued under the new generation* — the two lines `login_submit` already runs — which is also **one fewer screen, one fewer copy string, and `§5.3`'s newest flash kind left meaning one thing**. **No migration; `0025` stays reserved for the reset half.** | **`RFC 025` — [The password a user can change](rfcs/accepted/025-the-password-a-user-can-change.md), proposed 2026-11-27, not accepted.** **Correcting this cell's own earlier claim**: it read *an RFC cannot propose a mechanism for a requirement that does not exist*, and that was wrong — measured against the register's own `*Source*` fields, **`RFC 001` originated three requirements** and two more came from the owner directly. So an RFC is exactly the vehicle, and the architect was not blocked. **The RFC proposes two requirements and the mechanism for one**: `FR-AUTH-006` (change) to build, `FR-AUTH-007` (reset) accepted as a requirement and recorded `Deferred` with three measured prerequisites. **Entry criterion 1 is met: `RFC 025` was accepted by the owner 2026-11-27** and is in `rfcs/accepted/`. **Two series running, two series meeting it** — after six consecutive that did not. The two requirements it proposes are written into `requirements.md`: **`FR-AUTH-006` `Specified`** and **`FR-AUTH-007` `Deferred`**, with Appendix A's Authentication row and functional total moved to match (5 → 7 entries, 100 → 102 functional) and `spec_scan`'s own population guard renamed `requirement_entries_is_173_on_the_real_tree` — **which went red before Appendix A's rule b did, the third consecutive time that order has held**. `requirements.md` `FR-AUTH-*`, `NFR-SEC-003`, `external-design.md` `§9`/`SCR-22`/a new screen, `peisear-notify`, `crates/peisear-storage/src/users.rs` | **Row authorized 2026-11-28.** **`RFC 025` accepted 2026-11-27** — the owner's answer to its `§8` was the recommendation as scoped, both requirements — so entry criterion 1 is **met**, and criterion 3 is met by `rfcs/handoffs/070-the-password-a-user-can-change/`. **`PWD-001` is cleared to start.** **Three decisions, taken separately and on two days**, which is this file's own rule working rather than a formality: the RFC's acceptance (2026-11-27), this row's authorization (2026-11-28), and **the cut, which is not authorized** — exit criterion 7 is a separate decision against a release recommendation nobody has written. **This row is removed at the cut, per exit criterion 8.** **The architect's recommendation changed during the writing and the RFC records it as a rejected alternative rather than dropping it**: it was *both halves, with reset as the half that justifies the release*, and measuring three things reversed it — SMTP is optional in this product, nothing here rate-limits anything (0 hits across every crate), and a correctly-hidden reset affordance leaves a locked-out user on a mail-less instance needing an operator action that does not exist. **A reset that silently cannot deliver is worse than no reset.** On a yes, the requirement entry and `RFC 025` follow, in that order, and the row becomes authorizable. On a no, this number is free and the guard work below is the honest alternative. **The cut remains a separate decision** at exit criterion 7. |
| — | **`0.71.0` was cut 2026-11-27 and its row is removed here, per exit criterion 8** — proposed 2026-11-26, **row authorized the same day**, its **mechanism amended and the amendment authorized** the same day, `RFC 024` **accepted** the same day (entry criterion 1, met for the first time in seven series), and **approved for release 2026-11-27 against `REL-0.71.0`**. **What shipped is what was authorized**, with three things the row did not anticipate, all the architect's and all found at the cut: `external-design.md` was owed its first amendment since `0.62.0` and got four sections of it; that sweep found **two normative rows already wrong** — `§5.3`'s `Role` column and `§3.1`'s flash-region obligation both naming `role="status"` for a carrier no `?flash=` renderer has ever had (`§10.36` instance 11); and the candidate's own criterion-2 evidence row named one of two push-triggered workflows and was read as naming the tree, twice and in opposite directions (`§10.36` instance 12). **The security property was verified four independent ways**: `SES-001`'s own tests, the full 42-command `DEC-007` block (703 passed, 0 failed), the architect's walk replaying this row's own published reproduction against the binary extracted from the release tarball — `/projects` **303**, `/today` **303**, `/api/search` **401**, landing on `/login?flash=…` — and an independent re-verification's walk against a separately built binary. **The row's own scope, kept as the record of how it was scoped:** **The session that cannot be ended.** **`FR-AUTH-003` (P0) says a user MUST be able to end their session, *"after which protected routes are inaccessible with the prior credentials"*. Its status reads `Implemented`. It is not.** Measured 2026-11-26 against a debug build of `cfa45d9`, the `0.70.0` release commit, on `127.0.0.1`: register → `303` with `Set-Cookie: it_session=<231 chars>`; `GET /projects` with that token → **200**; `POST /logout` carrying it → `303` and `Set-Cookie: it_session=; Max-Age=0`; then the **same token replayed** → `/projects` **200**, `/today` **200**, `/api/search` **200**. `logout` (`handlers/auth.rs:121`) builds an empty cookie and removes it from the jar, and that is all it does — **there is no `jti`, no session table, no denylist, nothing server-side to end** (grep over `crates/*/src`: the only `blocklist` hits are two comments in `config.rs` about a configuration value). So the credential stays valid for the rest of `SESSION_TTL_SECS`, which `jwt.rs:28` sets to **7 days**, and neither the user nor the operator has any way to shorten that. **The product has no password change or reset at all** — 0 hits for `password` in the router and 0 in `handlers/settings.rs` — so a user who believes their password is compromised has no action available either. **Why it survived five releases of accessibility sweeps and three of guard-hardening**: the acceptance citation, `logout_clears_session_and_redirects` (`smoke.rs:158`), drives a cookie-jar client — it logs out, then asks for `/projects` **with no cookie**, and asserts a redirect. **It verifies that the client forgot the credential, never that the credential stopped working**, which is `§10.36`'s class in its narrower half, the one `§10.36` instance 9 named at the `0.70.0` cut: a check whose reach is narrower than its name. This makes **four** instances of that half in two days, and the first with a **P0 product defect** behind it rather than a tooling gap. **The second item, same family, same class, one test**: `FR-AUTH-001`'s acceptance has two clauses and **nothing in the repository names the second** — *"no plaintext password is persisted or logged"* has zero matches for `plaintext` across `tests/` and `peisear-auth`. **Measured rather than assumed, and the product is clean**: registering with a distinctive marker password under `RUST_LOG=debug`, that string appears **0 times** in the SQLite file, 0 times in the log and 0 times anywhere under the data directory, and the ≥ 8-character rule holds (7 characters → `400`, 8 → `303`). **True, and evidenced by nobody** — which is the distinction worth shipping a test for, not a fix. **What the fix costs, measured rather than estimated, because this decides the design.** `AuthUser` (`extractors.rs:60`) **already reads the user row on every request**, with a comment saying exactly why — *"so deleted or altered accounts are immediately invalidated rather than waiting for the JWT to expire"* — so the project already bought a per-request read for this very purpose. Census by handler signature across `handlers/*.rs`: **33 take `AuthUser` only, 29 take both, 10 take `Requester` only, 10 take neither** (public). So **62 of the 72 authenticated handlers already make a read of that row** — but that only makes the check free if the check lives *inside* `AuthUser`, and it must not: it belongs in `verified_requester_id` (`extractors.rs:22`), the single function all five extractors funnel through and the one `PRIV-002`/`DEC-058`'s unforgeability already rests on, so that a sixth extractor written next year cannot go around it. **Put it there, hand that row to `AuthUser` in place of its own second `find_by_id`, and memoise it in `parts.extensions` for the 29 handlers that extract both** (which already decode the JWT twice today), and the per-request arithmetic is: 33 unchanged at one read, 29 unchanged at one, **ten going from none to one**, ten public routes untouched. The ten, named, because they are the whole cost: `notification_preferences::{save_preferences,silence_all}`, `notifications::{resume,email_opt_in,mark_read,mark_all_read}`, `settings::{insert_capacity,update_capacity,delete_capacity,close_capacity}`. **Every one of the ten mutates the requester's own personal data**, which is precisely where a revoked session must not be honoured, so the ten are the argument for the check rather than against it. **What is traded is `Requester`'s documented no-I/O property, not its safety property**: `PRIV-002`/`DEC-058` makes `RequesterId` unforgeable because only `jwt::verify` can construct one, and adding a revocation read does not touch that; the doc comment's *"no DB round-trip, unlike `AuthUser`"* is a performance note, and it stops being true for those ten. **The mechanism, amended 2026-11-26 after the owner asked for the three open decisions to be reviewed against his philosophy — and the review changed it**: a single `users` column holding a **session generation**, `INTEGER NOT NULL DEFAULT 0`, advanced by `logout`; the generation the token was issued under travels in `Claims` (read with `#[serde(default)]`, so tokens minted by `0.70.0` keep working instead of logging every user out at upgrade); a request whose generation differs from the row's is refused. **Nothing is compared but two integers.** **What was withdrawn, and why it matters more than the field it saved**: the first recommendation reused the token's `iat` against a stored instant — no new claim, which read as the smaller change. `iat` is whole seconds (`jwt.rs:71`), and **this project has already paid a migration for exactly that comparison**: `0021_updated_at_millisecond_precision.sql` exists because SQLite's whole-second `CURRENT_TIMESTAMP` left *"two writes to the same row inside one second"* admitting a stale optimistic-lock token, red-then-green against `optimistic_lock.rs`'s `issue_edited_twice_in_the_same_second_with_the_same_token_must_409`. Reused here it gives a choice of three defects: a replay inside the logout's own second admitted; or, compared finer, **a user who logs out and immediately logs back in refused and sent back to the login page** — the owner's second sentence broken by the fix; or either way, an NTP step backwards refusing new credentials for the length of the step. **A counter has no resolution, no timezone, no `DATETIME`-versus-`TIMESTAMPTZ` difference between the two backends, and no boundary to test.** One column, one migration, no new table, nothing to prune — **zero continuous maintenance**, which is the owner's stated axis, and one fewer thing that can be wrong, which is ahead of it. The alternative, a `sessions` table keyed by a `jti`, buys per-device revocation and costs a growing table that must be swept forever, **for a precision the product has no way to express**: there is one Log out button and no device list. **Its UX consequence must be stated in the product's own words rather than discovered**: pressing Log out ends that user's sessions everywhere. For a self-hosted tracker that is both the safer default and the less surprising one — the surprising behaviour is the current one, where logging out leaves you logged in on the machine you walked away from. **And it does not foreclose the table**: a generation check and a later per-device `jti` compose, so the cheap mechanism now is not a decision against the precise one later. **`jwt`'s own module boundary moves by exactly one field, deliberately.** `Claims` is private and its doc comment records that nothing outside ever read anything but `.sub`; revocation needs the generation, so `verify` returns one more value — a value, not the struct, keeping `DEC-058`'s shape. **And the product must say what it does, in two places, not one**: on the Log out control (*it ends your sessions everywhere*) and **on the other device**, which today gets a bare `303` to `/login` with nothing to distinguish a revocation from an expiry or from never having signed in — `login_page` already takes `?flash=` (`auth.rs:26`) and `one_encoder_scan` already governs how copy reaches it, so this is wording and a redirect target, not machinery. **No new `FR`/`NFR` number is spent**: `FR-AUTH-003`'s normative text already requires this, so what widens is its **acceptance**, which is the same discipline as `NFR-A11Y-005`'s four widenings and `NFR-A11Y-010`'s two. **`§10.66` opens** (the gap, and `FR-AUTH-003` recorded `Divergent` until it closes — exit criterion 3 requires the entry for exactly that), and **`§10.36` gains instance 10**, the cause rather than the defect. **The 7-day TTL stays and the row says so on purpose**: without revocation it was the exposure, with revocation it is a policy, and changing both in one release would make neither measurable. **Deliberately not in this row**: password change and reset, e-mail verification, a device or session list, an operator revocation command, and any `sui-id`/OIDC delegation — each is new user-facing scope the owner has not asked for, and the first two need an e-mail flow this release would have to invent. R1 of `sui-id`'s own RP contract asks an embedder for exactly this revocation property, so the work is not wasted if that question ever reopens, but **this row is peisear's own defect and is scoped as such, not as preparation for an upstream that is not scheduled**. **Trigger**: the rule's second one — a defect in the shipped product, measured rather than argued, and the first one this project has found against a **P0** | — | **No number assigned — awaiting new user-facing scope from the owner, or a defect in the shipped product** |


**`0.72.0`'s first scoping is withdrawn, 2026-11-27, and this paragraph is the
record of why rather than a quiet edit.** The owner asked for the scope to be
reviewed against his own philosophy. **Reviewing it found three flaws, all of
them the architect's, and the honest outcome was again reversing his own
recommendation** — the third time that prompt has produced that result.

**The first scoping was seven units of guard work** — entry criterion 1's own
scan, `changelog_scan` rules 4 and 6 with an 18-link repair, the flash-role
check, a missing population floor in a test, a cookie-capture correction, and
an apt-scoping fix in CI. Every figure in it was measured and three carried
figures were corrected by it; **that work is real and is kept, below, in
*Unscheduled work*.** What was wrong was proposing it as a release.

**Flaw one: not one of those seven units changes anything a user can see,
while two things fail the philosophy's second sentence today.** The published
documentation site describes a product two releases old, and a user whose
password may have been seen has no action available at all. A release that
ships only guards, while those stand, has the ordering backwards.

**Flaw two: the row's own justification was a cost argument wearing a design
argument's clothes.** It read *"nothing in this row needs a new requirement or
a new owner decision beyond authorizing the row itself, which is why it is
proposed as one release"*. That is a sentence about what is **easy to
approve**, and the philosophy is explicit that design and user confusion are
judged **before** cost. This is the same mistake that produced the philosophy
note in the first place, made again.

**Flaw three: "less is more" fails outright.** `0.70.0` was one mechanism;
`0.71.0` was one property. The withdrawn row was a governance scan, two
changelog rules, a document sentence, a link repair, a browser check, a test
assertion, a test refactor and a workflow fix, with a title stretched over
them doing the work that coherence should do. **And it had a concrete cost**:
rule 6's new clause carries a named residual — between a tag and the next
version bump it would permit a premature rollover of the live series — which
deserves its own attention and would not have received it as item (c) of
seven.

**So the guards are not a release theme. They are three small rounds that ride
along with releases whose themes are user-facing**, split by what they
actually are rather than by when they were decided: `changelog_scan` and
`DEC-055` finished (rules 4 and 6, the 18-link repair, the one new sentence);
entry criterion 1's own scan; and the flash-role check. **Two of the seven are
not rounds at all** — a one-line population floor and a two-site cookie
capture, both in `smoke.rs`, which belong in the first round that touches that
file; and the apt-scoping fix is one line in a workflow.

**GitHub Pages stays out of every row, and the case for its urgency collapsed
when the owner asked what it was for.** The first scoping argued it must not
wait for a release and then filed it where things wait — a contradiction
inside one proposal. **Asked for the goal, the honest answer was that the harm
did not survive measurement**: `docs/getting-started/` is byte-identical
between the published site and `HEAD`, so no reader is misled; `NFR-REL-011`,
which that row cited, is about CI exercising install paths and not about a
published site at all; and `README.md` points readers at `docs/README.md` on
`main` rather than at the site. **The real trigger — a stuck deployment
blocking the link check for three days — was fixed on 2026-11-21**, and what
was carried into two release candidates afterwards was the residue of a solved
problem, restated with borrowed urgency. The row now says that, keeps the four
eliminated causes because those are durable, and claims nothing more.

**`0.65.0` was cut 2026-11-21**, on the owner's approval at exit criterion 7
against `REL-0.65.0`; **its row is removed from the table above per exit
criterion 8**, and the paragraphs below are kept as the record of how it was
scoped. **What shipped matched what was authorized**, with one thing the row
did not anticipate: the cut's own `DEC-028` sweep found three defects in the
two specifications' own headers and change history, all the architect's, and
repaired them in the release (`§10.36` instance 8).

**`0.65.0`'s scope, measured rather than asserted.** Proposed 2026-11-18,
the same day `0.64.0` was cut, on the owner's own instruction to scope it.
**The trigger was that instruction, not a defect** — stated plainly rather
than dressed up as one. Every **P1** requirement remains Met or Implemented
except `NFR-REL-004` (§9.4's evidence limitation), and the architect's own
search for a user-facing defect in what `0.64.0` shipped found none. What
qualifies this row under the 2026-11-03 rule is its **content**: a reader
sees larger labels and hints on five screens, and the product stops
carrying two sizes for one job.

**The census, with the command that measures it** — a row that names a
population owes the command, which `0.64.0` learned by publishing five
inflated figures taken with a plain `grep -c`:

```bash
grep -rn 'text-xs' crates/ --include='*.rs' | grep -vE ':[[:space:]]*//' | wc -l
```

**68 render sites remain**, across ten component files. **In scope: 47** —
`sprints.rs` 20, `teams.rs` 10, `sprint_plan.rs` 5, `settings.rs` 8,
`auth.rs` 4. **Out of scope: 21** — `calendar.rs` 11,
`notification_preferences.rs` 4, `notifications.rs` 3, `search.rs` 2,
`layout.rs` 1.

**The finding that makes this a row rather than a chore.** The product has
two sizes for the same job, and **the line between them is not a design
decision — it is which screens have been read.** Measured product-wide,
render sites only: `label-text text-sm` **54** against `label-text
text-xs` **8**; `label-text-alt text-sm` **5** against `label-text-alt
text-xs` **7**. All five larger hints are in `issues.rs` and
`projects.rs` — the screens `TYPE-003` read. All seven smaller ones are on
screens nobody has read (`teams.rs` 3, `sprints.rs` 2, `settings.rs` 1,
`auth.rs` 1), and so are all eight small form labels (`settings.rs` 5,
`sprint_plan.rs` 3). **`0.64.0` did not leave a rule; it left a
majority** — and carrying that boundary another release makes the
inconsistency **authored rather than inherited**.

**And for the hints, `0.64.0` did not inherit the split — it created
it.** Measured against the `0.63.0` tag rather than remembered
(`git grep -c` at the tag, so those figures count comment lines too and
are an upper bound): `label-text-alt text-xs` **12 → 7** and
`label-text-alt text-sm` **0 → 5**. **Before `0.64.0` the hint class was
uniform**; the five sites that round read moved up, the seven it did not
read stayed, and the product has had two sizes for one hint ever since.
The form labels are the gentler case — the majority already stood at
**51 to 11** before that release, and raising the three in its path left
**54 to 8** with every remaining small one on a screen nobody has read.
**This row's content is `0.64.0`'s own unfinished edge**, which is the
most honest thing that can be said for it and the reason it is proposed
rather than left as a candidate.

**No new type rule, and the one condition that would reopen the question.**
`§10.60` closed on a recorded decision that no requirement should govern
text size, because the size answer converged. This row applies the
existing majority and writes nothing new. **If a round finds the right
answer differs site by site on these screens**, that is evidence the other
way: the round says so in its own report rather than adopting a floor
quietly, and `§10.60` is reopened by a recorded decision or not at all.

**Eight sites whose contrast nothing has ever checked.** In scope,
`text-xs` meets a bare `opacity-*` at eight sites — `sprints.rs:531`,
`sprints.rs:1163`, `teams.rs:169`, `teams.rs:184`, `teams.rs:538`,
`settings.rs:81`, `settings.rs:205`, `auth.rs:92`. `NFR-A11Y-005`'s own
entry states the limit in terms: `contrast_scan` **does not cover bare
`opacity` at all**. `0.64.0` measured four such sites and all four passed;
these are eight, at the smallest size in the product, and the rounds
measure them rather than reason about them.

**Two traps in this row's own path — and a correction to this table's own
candidate list.** `sprints.rs:436` and `sprints.rs:983` each state `py-4 =
32px + text-xs's own 16px line-height = 48px` in a comment nothing reads,
and raising those screens invalidates both. **The 44 px floor itself is not
at risk, and the candidate list said otherwise:** `grow_summary()` composes
`TOUCH_TARGET` (`min-h-11 min-w-11` = `2.75rem` = 44 px) and
`alignment_scan` enforces that every real `<summary>` reaches that one
helper — with one bare `<summary>` excluded by its exact literal tag
(`issues.rs:650`, outside this row). So these are **explanatory comments
that go stale, not a requirement held together by arithmetic**, and the
candidate entry framing them as `061`'s principle left unfinished was
wrong about which mechanism holds the property. It also listed **five**
locations as four, and three of the five (`me.rs:347`, `me.rs:409`,
`issues.rs:385`) were re-derived to `text-sm` at `0.64.0` and are correct
today. **Two remain, both in `sprints.rs`, both in this row's path.**

The second trap is real and has already fired once.
`browser-checks/overflow-gate.mjs:538` validates that the board fixture
rendered a workload chip by quoting the literal `text-sm font-medium` —
a literal `0.64.0` broke on a round that never touched the gate, and a
class that now matches **seven** sites in `issues.rs` rather than one, so
it is both coupled and less specific than when it was written.
**Recommended direction: delete the coupling rather than check it** — give
the chip (`issues.rs:717`) a named hook and have the gate match that.
`css_scan` already tracks named JavaScript hooks, so the hook is checkable,
no size round can break it, and **no new scan is added**. That answers the
candidate list's three undecided shapes by choosing none of them: the
arithmetic is explanatory and needs correcting rather than deriving, and
the one real coupling is removed rather than guarded.

**The rounds, and how the 47 partition.** Four rounds, one handoff series,
opened on authorization and not before:

| round | what it reads | sites |
|---|---|---|
| `LBL-001` | both label conventions, product-wide, decided once — the four `settings.rs` points-table column headings measured **first**, a table being the one place a step up can widen rather than grow | 15 |
| `SIZE-001` | `sprints.rs`'s remaining body and meta text, and its two stale comment sums | 18 |
| `SIZE-002` | `teams.rs`'s remaining 7 and `sprint_plan.rs`'s remaining 2, and the gate coupling above | 9 |
| `SIZE-003` | `settings.rs`'s remaining 2 and `auth.rs`'s remaining 3 — including the sign-in page's own flash message, the front door at 12 px | 5 |

15 + 18 + 9 + 5 = **47**, which is the in-scope census above. Method
unchanged from `062`: every cost **measured at 390 px and 1280 px** rather
than assumed, **removal is on the table in every round**, and a site may be
left where raising it would not fix what is actually wrong with it —
`0.64.0` raised sixty of sixty-one, and **this row does not promise a
uniform raise either**.

**Deliberately not in this row, each with its reason.** The **21 remaining
sites** (`calendar.rs`, both notification surfaces, `search.rs`,
`layout.rs`): non-form text whose costs are calendar density and
navigation chrome rather than convention, decided after this row shows what
the read costs — `062`'s own stopping discipline. **`/today`'s content**,
whether the first screen carries the day's planned work above its metrics:
the owner's open question, and **not a defect** — `NFR-A11Y-012` has been
Met for all five destinations since `0.62.0`, when `NAME-001` moved the
navigation word to **Me** rather than renaming the page. It is new scope
that reopens a **P0** (`FR-PER-001`) against a **P1** (`FR-PER-007`), so it
deserves its own row after the owner's decision, not a place inside someone
else's theme. **`§10.32`'s unrendered fixture branches**: that entry's own
stopping rule says an unrendered branch is recorded in
`browser-checks/README.md` and **not chased** unless a defect turns up
behind it, so scheduling it would run the campaign the entry resists.
**`§10.57`'s remaining two surfaces**: the remedy shipped — all four undo
surfaces carry the restore ladder (`dm.js`, `board.js`, `calendar.js`,
`plan.js`, each guarded by `hadFocus`) — and what is open is gate coverage
on two of the four, which is internal and carries no number.
**`proc-macro-error2`**: see **Unscheduled work**, whose remedy sentence is
corrected there today.

**The `0.70.0` record moved here from the table at the `0.71.0` cut**, per the
practice that keeps exactly one cut row in *What ships next*. Nothing is
abridged; it reads as it did in that cell.

**`0.70.0` was cut 2026-11-26 and its row is removed here, per exit criterion 8** — proposed 2026-11-25, authorized the same day, **revised and re-authorized 2026-11-26** after the owner asked for the architect's own scope to be reviewed against his philosophy, approved for release 2026-11-26 against `REL-0.70.0`. **What it decided is worth carrying even though its row is not.** `contrast-gate.mjs` renders **24 pages × 5 widths = 120 cells** of the product's own fixture, paints each background layer and then the foreground on a 1×1 canvas, and **reads the pixel back** — so the *renderer* performs every colour conversion and composite, which is the only route of four that works. It examined **4,813** text-bearing elements and **217** elements in the families `0.68.0` repaired, **focused every one of them**, and finished with **0 unacknowledged failures and 0 unpaintable** in about 16 seconds. **It found one real defect that no source mechanism can see**: `join-item btn btn-sm btn-active btn-primary`, the selected segment of a project's board/list control, at **3.87:1** against 4.5 — opaque text on an opaque fill, two tokens applied by two different rules, while `contrast_scan` was 8/8 on that same tree. **The repair removes a class rather than adding a colour**: `btn-active` darkens `btn-primary` toward black while its own foreground token is already near-black, so dropping it restores the highlighted-tab look the product already uses elsewhere, at **4.83:1**. **The gate's hardest requirements come from the architect's own failures, not his success** — three prototypes each returned a confident wrong answer: `examined=0, failures=0` from a regex colour parser that matched nothing (*which reads exactly like the product passes*), then **1,906 of 1,906** because Chrome round-trips `oklch()` through canvas `fillStyle` unchanged, then the pixel route. So the gate asserts **its own population floor, `unpaintable == 0`, a known-good control pair, and treats a 0% or 100% failure rate as an error in itself**, and each of the four was planted and watched failing. **The two checks now have a stated authority rule**: when the rendered check and a source mechanism disagree, the rendered one governs, written into `NFR-A11Y-005` rather than left to whoever reads the two outputs — and the exclusion list both of them read has **one home**, `browser-checks/contrast-exclusions.json`, instead of two copies. **`NFR-A11Y-005` and `NFR-A11Y-010` were widened a fourth and a second time rather than given siblings**, `§10.65` opened and closed, and **no new register number was spent**. **And the scope was wrong when the owner authorized it, and he caught it by asking rather than by reading**: *"review them carefully with my fundamental philosophy"* found three flaws, all in the architect's shape rather than in his measurement — a fifth source mechanism with a sixth piece of machinery to police a two-entry list, **no stated answer to which check is right when two disagree**, and a text/non-text split that paid for one browser harness twice. **What shipped is one round instead of three.** **Two of the architect's own predictions were wrong again and both were caught by measuring before either reached a document**: a checked `.checkbox`'s fill, predicted at about 4.2:1 and measured at **17.21:1**, and which doc links rustdoc would flag. That second one leaves a new candidate below — **CI's `rustdoc-links` job cannot see private items' doc links, and two are broken on the tree**. **The previous cut's record follows, on the same footing**: **`0.69.0` was cut 2026-11-25 and its row is removed here, per exit criterion 8** — proposed, authorized and approved for release the same day, against `REL-0.69.0`. **What it decided is worth carrying even though its row is not.** `disclosure-marker-gate.mjs` guards thirteen `<summary>` controls' own marker and 44×44 target — properties `touch_target_scan` cannot see — and **`git log -S` found no commit that had ever wired it**, while two source comments, five register passages and eight handoffs cited it as a required re-run. It now runs in CI and passes on the runner, with a guard whose population is deliberately **not** a `*-gate.mjs` glob, because `undo-mousedown-trap.mjs` is a wired gate that does not match one. `image.yml` gained an **anonymous** post-publish pullability check, since a push log proves a push and not that a stranger can pull. `§0.6` now states the register's own reading rule — **an entry's last `*Status*:` line governs**, which `spec_scan` had implemented by `rfind` for releases and no sentence had ever said — the three real multi-status entries mark their earlier line inline, and `§10.64` opened and closed. **And the architect was wrong four times in one release, three of them in this file.** He reported eight registry tags (there are eighteen; his command ended `jq '.tags[-8:]'` and he read his own slice as the list), `NFR-REL-010` as `Divergent` (its governing status read `Implemented` throughout; his parser took the **first** `*Status*:` line), eight statuses asserting an absence (**one**), and `NFR-CMP-007` as carrying seven status lines (**one**; its span ends at a `##` heading his regex ignored). **`rule_b` was green the whole time** — the project's own checker was stating the right value while the proposal was written over it. **Two of the release's three rounds exist only because of those errors, and all three were things the project needed.** `§10.64` names all four instances, the architect's own shell included, because the pattern is the finding: reading the shape of one's own output as the shape of the world. **This row's standing rule is unchanged** and the candidate list below is what `0.69.0`'s own proposal was chosen from. **The previous cut's record follows, on the same footing**: **`0.68.0` was cut 2026-11-24 and its row removed here per the same criterion** — proposed 2026-11-23, authorized and approved for release 2026-11-24, against `REL-0.68.0`. A shipped release is history, and its record is `changelog/0.60-0.69.md`'s own `[0.68.0]` section and both specifications' Appendix C baseline rows, not this table. **What it decided is worth carrying even though its row is not**: three releases had taught the contrast check to read *text* colour, and **WCAG 1.4.11 governs the rest** — **67 focus rings and 70 control boundaries all drew at `base-content/20`, 1.52:1 against 3:1**, so a keyboard reader could not see where they were and, on a card, a line at one and a half to one was the only thing saying a field existed. Rings raised to **full `base-content`** (17.21:1), boundaries to **`/70`** (6.42:1 / 5.24:1), **both tiers the product already had**. `contrast_scan` gained a **fourth mechanism**, planted twice and watched failing both ways, which **deliberately does not compute ratios** — and **every one of its four parts exists because a colour escaped a guard whose requirement's words covered it** (`§10.63` carries that table). **`NFR-A11Y-010` was widened rather than duplicated**: it had named *"the boundaries of interactive controls"* since `0.42.0` and had only ever been measured for chart series. **Two of the architect's own instructions in this series were load-bearing and wrong** — a `/50` tier that reads **2.73:1** because a border composites over the control's own fill, and a recommended new requirement that already existed — **the first of his errors in four series that would have changed what shipped**, both caught by a round reading the thing itself rather than the sentence handed to it. And the `rfcs/` citation count that reddened `main` three times at `0.67.0` now shares `docs_scan` rule 5's own exclusion and asserts zero. **This row's standing rule is unchanged** and the candidate list below is what `0.68.0`'s own proposal was chosen from; **its trigger is the rule's second one — a defect in the shipped product**, measured rather than argued, and the first one this project has found against a **non-text** criterion. **`0.67.0` was cut 2026-11-23 and its row is removed here, per exit criterion 8** — proposed 2026-11-22, authorized 2026-11-23 (**the row**), approved for release the same day against `REL-0.67.0` (**the cut**). A shipped release is history, and its record is `changelog/0.60-0.69.md`'s own `[0.67.0]` section and both specifications' Appendix C baseline rows, not this table. **What it decided is worth carrying even though its row is not**: `0.66.0` taught the contrast check to read all four of the theme's colour families, **and it reads only the classes a component writes** — so **eighteen shipped sites** took their colour from the served stylesheets instead and nothing could see them: seventeen placeholders at **2.54:1** and the account menu's own e-mail line at **1.85:1**, the worst text in the product, where two fades multiplied. **Eight table headers at 4.53:1 were re-floored with them** — a pass by 0.03 on the `/60` tier this requirement itself rejected; *the row as authorized claimed they failed outright, and the round measured otherwise.* **Every repair removes or re-floors a colour and none adds one**; the theme is untouched because those same tokens are what make 37 `btn-primary` sites and every solid badge pass. `contrast_scan` gained a **third mechanism** — sheet-applied colours, each acknowledged with what beats it or exempt with its reason — **planted twice and watched failing both ways**, and a tenth below-floor rule nobody had named (`input::-moz-placeholder`) was found by enumerating the sheets rather than trusting the three families the row was scoped on, and fixed although this environment cannot render it. **And three red `main`s on the way, all the architect's**, all in one counted population over `rfcs/`: the first unwatched, the second caught by the watcher, the third caused by the sentence explaining the second. The content was never affected; the structural cause is the candidate recorded below. **This row's standing rule is unchanged** and the candidate list below is what `0.67.0`'s own proposal was chosen from; **its trigger is the rule's second one — a defect in the shipped product**, measured rather than argued, and the second instance in two releases of the same class one layer down. **`0.66.0` was cut 2026-11-22 and its row is removed here, per exit criterion 8** — proposed and authorized 2026-11-21 on the owner's own instruction to scope it (**the row**), approved for release the next day against `REL-0.66.0` (**the cut**). A shipped release is history, and its record is `changelog/0.60-0.69.md`'s own `[0.66.0]` section and both specifications' Appendix C baseline rows, not this table. **What it decided is worth carrying even though its row is not**: **fifteen shipped sites rendered text below WCAG AA while `NFR-A11Y-005` (P1) stood recorded `Met`** — every destructive action in the product among them — because that entry's acceptance check read **one** of the four colour families the shipped theme defines, and its own closing sentence (*"the theme's own tokens were never the problem"*) was true of `base-content` and read as true of all of them. **All fifteen were repaired by removing a colour rather than adding one**, the theme left untouched because those same tokens are what make 37 `btn-primary` sites and every solid badge pass, and **the one red that passes was left exactly where it was**: the confirm button on the step that deletes, 6.83:1. `contrast_scan` now reads all four families with three exclusions held **by exact class value and by exact count**, both guards planted and watched failing, **and both plants re-run independently at the candidate**. **Five of the architect's own figures or instructions were corrected by the rounds** — a ratio asserted against the wrong background, an after-figure that ignored ambient opacity, a hover measurement this environment cannot produce, *thirteen* repairs where the total is fifteen, and a stale `DEC-007` figure his own review had let through — **and two more he caught in the cut's own sweep**, including two invented section numbers in a specification header. **This row's standing rule is unchanged** and the candidate list below is what `0.66.0`'s own proposal was chosen from; this row's standing rule is unchanged, and **`0.66.0`'s own trigger is the rule's second one — a defect in the shipped product**, measured rather than argued. **`0.65.0` was cut 2026-11-21 and its row is removed here, per exit criterion 8** — proposed and authorized 2026-11-18 on the owner's own instruction to scope it (**the row**), approved for release 2026-11-21 against `REL-0.65.0` (**the cut**, a separate decision, three days later). A shipped release is history and its record is `changelog/0.60-0.69.md`'s own `[0.65.0]` section and both specifications' Appendix C baseline rows, not this table. **What it decided is worth carrying even though its row is not**: series `063` read the five screens `0.64.0` left, raised **30 of 47** in-scope sites, left **17** with a stated reason each, and **wrote no type rule** — `§10.60` stayed closed through four rounds that each had the standing to reopen it. Its own finding was that **`0.64.0` created the split it closes**: measured against that tag, the hint class had been uniform until three screens were read and seven were not, so the inconsistency was authored rather than inherited. Two sets with no counterpart elsewhere in the product (eight stat captions, five chart captions) were **decided as sets and recorded**, reopened only if another screen grows the same pattern. **Six recorded reasons in the series were wrong and corrected in place, three of them the architect's**, and the cut's own `DEC-028` sweep found three more in the two specifications' own headers and change history — all three the architect's, now `§10.36` instance 8, including three releases of Appendix C rows that **were not rows**, placed above their own table header and rendered as paragraph text in the published book. **This row's standing rule is unchanged** and the candidate list below is what `0.65.0`'s own proposal was chosen from. Otherwise: **No release number is assigned (`0.64.0` was cut 2026-11-18 and its row removed per exit criterion 8; `0.63.0` the same, 2026-11-15), by the architect's recommendation of 2026-11-03 which the owner accepted the same day** — *it held for four days, was resolved by the `0.61.0` row (authorized 2026-11-07, cut 2026-11-10, removed from this table by exit criterion 8), applied again from `0.62.0` on, and was resolved a second time by the `0.63.0` row — three defects in the shipped product, which is the rule's own second trigger rather than its first*: the next number waits for something an installed user would notice — new user-facing scope from the owner, or a defect in the shipped product. Every **P1** requirement is Met or Implemented except `NFR-REL-004`, whose gap is §9.4's evidence limitation, so the user-facing obligations written down are discharged and a further one is the owner's to set. **This is a stated position, not an empty table.** **Outside this table, and deliberately**: `rfcs/handoffs/054-the-front-door/` was authorized with **no release number** and **closed 2026-11-03, three rounds against one scoped** — each of the last two found by the round before it, `QS-001` absorbed by `0.60.0`'s own sweep and `QS-002`/`QS-003` sitting under `[Unreleased]` for whichever release is cut next — `README.md` on `main` is what GitHub serves the moment it merges, so the Quick Start fix does not wait for a cut and does not get one of its own. It lands under `[Unreleased]` and whichever release is cut next picks it up in its own `DEC-028` sweep. Raised by the owner 2026-10-31: the Quick Start offered two paths and both needed Rust, while `install.md` had offered two toolchain-free ones since `0.53.0`. **`056`'s three rounds (closed), `057`'s two, the `spec_scan` baseline rule, the changelog section-date check and the link check outside the book carry no release number**, on `054`'s own precedent of 2026-10-31: they are internal, `main` is what a reader gets, so they land under `[Unreleased]` and whichever release is cut next picks them up in its own `DEC-028` sweep. They are candidates for *work*, not for a *number*. Remaining candidates, in no order: **nothing checks a release tag's own message, and the documented shape has been unmet for two releases — both the architect's** — `changelog-and-releases.md` §*The tag carries the link* requires the subject `peisear X.Y.Z`, a blank line and `Release notes: https://github.com/nabbisen/peisear/blob/X.Y.Z/CHANGELOG.md`, **pinned to the tag so it survives a `DEC-055` archive move**. Measured across all tags 2026-11-26: **`0.68.0` and `0.69.0` have empty bodies** — no pointer at all, the rule simply not followed at either cut — and `0.61.0`/`0.62.0` carry a different shape (*"Full section: CHANGELOG.md [0.61.0]."*, a path and not a pinned URL). Every tag from `0.40.0` to `0.67.0` except those four is correct, so this is a regression and not an unimplemented rule. **They are not rewritten**, on that document's own rule: the crates are published, and `0.40.0`'s force-pushed message is the recorded precedent for why rewriting one is *"not something to do on purpose"*. **`0.70.0`'s own tag is written to the documented shape**, and the gap is that the only reader of the rule is whoever remembers it — `changelog_scan` already shells out to `git tag` for its rule 7, so reading each tag's message is the same call it already makes, and the class is `§10.36`'s: a rule whose evidence is a person following it. **Note the dependency**: the link is pinned to the tag precisely so that an archive move cannot break it, which is the candidate below one layer down; **a relative link inside a changelog section breaks when `DEC-055` moves that section into an archive, and nothing checks it** — the link was written to resolve from `CHANGELOG.md` at the repository root, and the archive sits one directory deeper. **Re-measured across all seven archives 2026-11-26, and the first figure was wrong**: the candidate read *four broken in `changelog/0.60-0.69.md`* and *three in `changelog/0.50-0.59.md`* — **seven** — because it looked only at the two newest archives. **The real count is 17, and the shape matters more than the count.** Of 24 relative links under `changelog/`, **7 are each archive's own `../CHANGELOG.md` back-link**, written in place by the header template and fine — and **every one of the other 17, each of them moved text, is dead**. Twelve resolve from the repository root but not from `changelog/` (five in `0.10-0.19.md` to `docs/operations/*`, three in `0.50-0.59.md`, four in `0.60-0.69.md`), and **five resolve from nowhere at all**: `docs/spec/peisear-feature-spec-v2.1.md`, cited five times in `0.10-0.19.md`, a path `git log --all` shows **this repository has never contained** — those were dead the day they were written, which no move can explain. **So the move breaks 100% of the relative links it carries**: not drift, deterministic. **`changelog_scan` rule 4 checks that every link in `CHANGELOG.md` resolves and does not check an archive's links at all**, and rule 5 only reports a *fragment* that no longer exists in a changelog file, so a path that resolves nowhere passes both. **Settled by the owner 2026-11-26, and not by narrowing *verbatim*, which was the architect's first recommendation and is withdrawn**: what is wrong is not the sentence but **location-dependent paths in a document that moves**. The rule is that **a link in a changelog file is either a sibling under `changelog/` or an absolute URL pinned to a tag** — the same reasoning the tag-message rule above already uses, *pinned to the tag so it survives a `DEC-055` archive move* — after which *verbatim* and *written once* both hold literally and permanently, and no future rollover can break anything. **One-time repair**: the twelve pinned to the tag of the release whose entry cites them (a historical record should cite the tree it describes), the five de-linked to plain text with the citation kept, since a path that never existed cannot be pinned. **`DEC-055` gains exactly one sentence** — an archive may be corrected only to make a link resolve or to remove one that never could, and that commit's diff shows nothing but link targets. **Then rule 4 widens to archives**, which closes this class for good and is checkable offline: `changelog_scan` already shells out to `git tag`, so a pinned URL's version can be verified against a real tag without the network. **CI's `rustdoc-links` job cannot see private items' doc links** — `test.yml:91` runs `cargo doc --workspace --no-deps --keep-going`, which renders the **public** half of the workspace, so a doc comment written **on a private item** is never rendered and its links are never resolved — and **two are genuinely broken on the tree today**, one on a private `static` (`static_assets.rs:41`) and one on a private `fn` (`calendar.rs:128`): `crate::static_js_scan` and `super::super::handlers::issues::parse_planned_datetime`. Found 2026-11-26 while reviewing `SWP-001`, by the architect predicting rustdoc would flag two stale references it replaced and then measuring that it does not. **Pre-existing and not that round's**, and the class is `§10.36`'s in a new place — **a gate whose reach is narrower than its name**: it is named for doc links and checks the public half of them. The widening is one flag, `--document-private-items`, and the decision is what the widened job then reports across the whole workspace, which nobody has measured. **It is not the lint that job already un-denies on purpose**: `rustdoc::private_intra_doc_links` covers a *public* comment pointing at a private item, which eight comments here do deliberately, and the job's own header says so. This is the other direction, and nothing covers it; **a size change propagates into arithmetic written in comments and into literal class strings held inside checks, and nothing reads either** — `0.64.0`'s own finding, and the one thing that recurred in all three of its rounds. **Measured again 2026-11-18 while scoping `0.65.0`, and this entry was wrong twice over.** It said *four* `<summary>` touch-target sums and listed *five* locations; and of those five, three (`me.rs:347`, `me.rs:409`, `issues.rs:385` — the lines `me.rs:332`, `me.rs:373` and `issues.rs:350` have since moved) were re-derived to `text-sm` at `0.64.0` and are correct today. **Two stale sums remain, both in `sprints.rs`** (`436`, `983`), both still keyed to `text-xs`. It also named the wrong mechanism: **the 44 px floor is not held by the arithmetic at all** — `grow_summary()` composes `TOUCH_TARGET` (`min-h-11 min-w-11` = `2.75rem` = 44 px) and `alignment_scan` enforces that every real `<summary>` reaches that one helper, one bare `<summary>` excepted by its exact literal tag (`issues.rs:650`). So this is not `061`'s principle left unfinished: **the property is already true by construction**, and what goes stale is prose. What is left is **two comments to correct, one real coupling to delete** (`overflow-gate.mjs:538`, which quotes `text-sm font-medium` after `0.64.0` moved it, a class now matching seven sites in `issues.rs`; the fix is a named hook on `issues.rs:717`, not a new scan), and one `min-h-[2rem]` (`projects.rs:97`) whose expired reason is already recorded in its own comment. **All three are inside `0.65.0`'s proposed row**, so this ceases to be a standing candidate if that row is authorized; **the `xs` control tier** — **18** `btn-xs` render sites and **2** `select-xs` remain while `input-xs` is **0**. *This entry read **19** and **3**, and re-measuring at 2026-11-26 shows those two figures were **not decay but wrong when written**: the same command (`grep -rn` over `components/`, comment-only lines excluded) returns 18 and 2 both at `HEAD` and at `f8be759`, `LBL-001`'s own commit, and `git log -S` finds no commit between that removed one of either. `input-xs` genuinely moved, 5 → 0, exactly as recorded. **A source comment in `settings.rs` still asserts `btn-xs` has 19 render sites**, and another beside it asserts 3 — which is `0.64.0`'s own standing finding, arithmetic written in comments that nothing reads, caught here by the figure it got wrong* (`LBL-001`, 2026-11-18, which raised the five `settings.rs` inputs as clusters and left the buttons beside them at 12 px, with the reason stated in both cards rather than left implicit). Raising the tier is a product-wide control-density decision across screens series `063` does not read, so **it is deliberately not that series'**; a set is decided once, after the screens are read; **a counted population over `rfcs/` that every handoff moves** — `spec_scan`'s `rfcs_citations_to_the_four_numbers_are_counted_not_enforced` counts citations of four dangling register numbers across `rfcs/`, **including `rfcs/handoffs/`**, while `docs_scan` rule 5 excludes that directory by name as *execution records, not documentation*. So a handoff or release candidate that mentions one of those four numbers moves a counted population and turns CI red: **it happened twice in two commits at `0.67.0`, both the architect's** (`PLC-003`'s own handoff and then the release candidate, each citing `§10.23` for the same historical fact). Each time the honest fix was to re-derive the population by hand and move the number with its reason — which is what the test's own message asks for — but **the recurrence is structural, not careless**: whether this count should share rule 5's own exclusion is a decision rather than a cleanup. Small, and the cost is a red `main` each time someone writes a document; **nine links with no underline at rest** — `link link-hover` removes the underline and shows it on hover, so those links are distinguished from their surroundings by position alone until pointed at (`teams.rs:512`, `sprints.rs:1129`, `search.rs:183`/`:192`, `issues.rs:762` and four more). Found 2026-11-21 while measuring `0.66.0`'s own links, and **deliberately not in that series**: it is a use-of-colour and affordance question rather than a contrast one, and **WCAG F73 only bites for a link inside a block of text** — several of these are standalone card titles and pagination controls, where it does not apply. The work is to read each one in place and decide per site, which is a set, not a sweep; **the two spellings of a faded foreground** — `text-base-content/70` at **149** render sites against bare `opacity-70` at **22**, measured in `components/` 2026-11-18. `NFR-A11Y-005`'s guard reads the first and **cannot read the second at all**, so a measured-once `opacity` site is unchecked at the next change — `§10.60` moved all four of its own sites to the guarded spelling on measured evidence that the two composite identically at single nesting depth, and `LBL-001` round 2 moves six more. **The remaining sixteen are a set, not a sweep**, and they now carry **two** conditions per site rather than one. The first is `§10.60`'s single-level check (bare `opacity` fades an element *and its subtree*; the `/70` colour fades text only). **The second was measured 2026-11-21 by `SIZE-003` and corrects a sentence three rounds and both architect handoffs had carried forward as general**: does the element's colour come from a **component token**? At `settings.rs:81` it did — `alert-warning` sets its own `--wac` foreground — and the spellings were **not** equivalent there: bare `opacity-80` measured **9.01:1** against `text-base-content/80`'s **6.61:1**, a 2.4-point drop and a hue change, both still clearing AA. **That conversion was reverted** and the exception recorded at the site. So where a component token supplies the colour, the guarded spelling is a **colour change, not a spelling change**, and it has to be argued on appearance and contrast rather than waved through as equivalent; **the two boundaries `0.63.0`'s own guards name in `§10.59` and `§10.55`** — a count-bearing field that only `counted()` can render, which would close the agreement class by construction rather than by a check that sees two words past the number, and an ancestor/descendant exclusion in the occlusion gate for the day the site header first nests one interactive control inside another; both are small, neither is urgent, and both are written into the closed entries so a later release meets them as decisions rather than as oversights; **`QS-002`** — the `docs_scan` rule that would enforce `NFR-REL-007`'s new clause (the Quick Start must offer a toolchain-free path, said in the line that introduces any path that needs one); written and open, and `0.60.0` ships without it, the clause enforced by a reader rather than a check for one release (`§10.36`'s class); **`SMTP_TLS_MODE`'s seven spellings for two modes** — `implicit`/`implicit_tls`/`smtps`/`tls` and `starttls`/`start_tls`/`submission`, found reviewing `CFG-002`; the variable was documented for the first time at 0.59.0 naming only `implicit` and `starttls`, so five are undocumented and were never promised. Not narrowed in 0.60.0, on `RFC 022` §5's own constraint that a set keeps accepting what works today — breaking a working deployment in the refusing direction is worse than the untidiness. Deprecating five aliases is a decision with its own release, and it is untidiness with a compatibility cost rather than a defect, so no register entry; **`§10.32`** — `BROWSER-001` sweeps nineteen pages at five widths and its fixture cannot render four branches those pages contain (a completed sprint with no issues, a team with no projects, no assignee badge, no calendar blocks); three of the four hid real overflows, one of them 441 px on a 1280 px desktop, and each was found by changing the fixture for another reason rather than by looking, so the gate reported green cells for eleven releases on a page it could not have failed on; **`§10.51`** — **closed at `0.60.0`**, `SMTP_PORT` and `SMTP_TLS_MODE` no longer discard a malformed value silently, guarded against reintroduction by `docs_scan` rule 8; `RUST_LOG`, fixed by the same release but not guarded the same way, recorded in that entry's own closing note rather than carried here as open; **a `spec_scan` rule reading both specifications' headers and both Appendix C baseline markers against the version being cut** — `§10.36` instance 7, which went three releases undetected because the evidence of a `DEC-028` sweep is a document edit and no edit reads the same as a sweep that found nothing to edit; **no clippy step under any feature combination** — **CLOSED, and this entry was stale when `0.70.0` shipped carrying it.** It read: *"`test.yml:39` is the only clippy invocation and it runs default features, so clippy's own lints never reach the `postgres`-only code paths … and two lines to fix"*. Measured 2026-11-26 while scoping `0.71.0`: **`test.yml` holds three clippy invocations**, and the one this candidate asked for exists — `test.yml:1063`, `cargo clippy --workspace --all-targets --no-default-features --features postgres -- -D warnings`, landed by `DIA-004` inside the postgres job, with a comment saying in so many words that it is the only invocation that compiles those bodies. The third is `test.yml:65`, `clippy-windows`. **Removed rather than carried**, and the lapse is the architect's: nothing re-read this line for the releases between; **a `changelog_scan` rule comparing a released version's section date to its own tag's date** — `0.60.0` is the first release whose two disagree (section 2026-11-01, tag 2026-11-03, because the owner's approval came two days after the candidate was written and the architect did not re-date the heading before tagging); the scan already reads real `git tag`s for its "every released version has a section" rule, so the date comparison is the same read, and `docs/development/changelog-and-releases.md` now carries the rule by hand and names it unchecked; **the status census** — every `*Status*` clause in `requirements.md` that asserts an absence, checked against the runner rather than read; raised by four corrections in two days (`§10.34` instances 8, 9 and 10, plus the inverse recorded in `FR-API-006`), three of them found by hand while measuring a release proposal and one of them a **P1**. Bounded and checkable: the population is one file's status lines, the oracle is `cargo test --list` and the suite itself, and the output is a correction or a confirmation per clause. **This is the one piece of the withdrawn `0.61.0` that survived both reviews**, and it is internal; **a filed defect left unscheduled is this entry's own warning** — `§10.16` filed `FR-PLAN-004` correctly in 0.40.0, called the fix unscheduled, and the entry then read as a mandatory thing outstanding for twenty-two releases and was twice carried as candidate release work on that reading; **nothing checks a relative link outside the book** — `mdbook-linkcheck2` runs over `docs/` via `docs/book.toml`, and `README.md`, `ROADMAP.md`, `CHANGELOG.md` and `TERMS_OF_USE.md` are not in `docs/SUMMARY.md`, so their links have never been checked by anything. Measured 2026-11-03 across all four files (relative, non-anchor targets only): **one dead link, and it was `ROADMAP.md`'s link to the `SPEC` document the whole Near-term section is built on** — a path that never existed here, because `SPEC` is provenance held outside the repository. Fixed in place; the check that would have caught it is four files added to a linkcheck step, and `054` is the precedent for treating the front door as read by strangers; **`cp .env.example .env` is documented on two pages and nothing exercises it automatically** — found verifying `QS-003`, which added the second page. `.github/workflows/env-example-walk.yml` genuinely asserts the documented copy-and-run path (non-zero exit if it does not reach a listening server, since `0.53.0` round 2), but it is **`on: workflow_dispatch:` only, with a required `ref`**, and its entire history is **three manual dispatches, the last 2026-10-16** — before `0.57.0`, `0.58.x`, `0.59.0` and `0.60.0`. So the project's own tooling cannot read `.env.example` by the owner's permission rule, and the one job that walks it has not run in four releases: **the command is asserted by a gate that is not running**, which is `§10.36`'s class in a new place — not a check that examines nothing, but a check that examines nothing *because nobody triggered it*. `§10.40` describes the job accurately; **the architect's `QS-003` handoff said it ran on every push, and that was corrected in place.** The fix is a trigger, not a new job — a `push` on paths touching `.env.example` or the two pages that document it, or a schedule — and the reason it is a decision rather than two lines is that `NFR-REL-011` deliberately excludes this reader, so widening what runs automatically touches a scope that entry already settled. **Corrected 2026-11-18: this candidate was stale when it was last carried, and is now closed.** `WLK-001` landed exactly that trigger — `push`, scoped to the paths that can break it, with the workflow's own header recording that it does not widen `NFR-REL-011` — and the walk has run on `main` since, most recently run `43006627465` on 2026-11-11, **green**. It is removed from this list rather than carried; **`spec_scan` rule 3's plant is anchored on a hardcoded sentence from `requirements.md`**, and statuses are the most-amended lines in that document — the architect's own `FR-NAV-001` amendment invalidated it within a day of its landing. **It failed loudly and by name** (`the plant's own anchor must be unique in the real file`, `left: 0`), which is `REQ-006` `§3`'s property working, so this is a maintenance cost rather than a hole: re-anchoring is two lines. Deriving the anchor from the document instead of quoting it would end the recurrence; two lines against a few, and not urgent while the failure is this legible; the `file:line` citation convention (37 occurrences, the class demonstrated twice during 0.57.0); an attended start unproven on Windows; and `§10.50` — **whose statement here was stale and is corrected 2026-11-26**: it read *"no CI job compiling for Windows outside the release tag"*, and `PLT-001` took the Windows half on 2026-11-03. `test.yml:56`'s `clippy-windows` job runs `cargo clippy -p peisear -- -D warnings` on `windows-latest`, which is the remedy that entry names for itself. **What is open is macOS**, deliberately, on that entry's own reasoning about runner minutes — and `§10.50` in `requirements.md` has read **open, narrowed** since the day it moved, so the register was right and this line was the drift

**`0.70.0` was cut 2026-11-26**, on the owner's approval at exit criterion 7
against `REL-0.70.0`; **its row is removed from the table above per exit
criterion 8**, and the paragraphs below are kept as the record of how it was
scoped — **including the shape the owner's philosophy review found wrong in
them**, which was revised where it stands rather than deleted.

**`0.70.0`'s scope, measured by building it rather than by predicting it.**
Proposed 2026-11-25, **authorized the same day and revised and re-authorized
2026-11-26 — the row, not the cut** (`Release cycle`, exit criterion 7 is a
separate decision, against a release recommendation, and it was taken
2026-11-26). **Every figure below came from
running a prototype against the real product today**, not from reasoning
about what a sweep would find — because the last release's proposal was
wrong in two of its three parts, and both errors were the architect reading
his own tooling's output as the world.

**1. What the four source mechanisms cannot do, stated structurally.**
`contrast_scan` reads a muted class token (`§10.59`), a semantic family used
as text (`§10.61`), a colour a stylesheet applies with no class involved
(`§10.62`), and a boundary or focus property (`§10.63`). **All four read a
declaration. None resolves a cascade**, and `EDG-003`'s own guard was
deliberately told *not* to compute ratios, because that arithmetic needs
`background-clip` semantics, the element's own fill, and which side of a
border the adjacent colour sits on. `§10.52` is the precedent: a `<nav>`
that read as visible in the file computed `display: none` at four widths in
a real browser.

**2. The prototype, and what it measured.**

```
cells=120 (24 pages x 5 widths)   examined=4813   failures=130
skips: noText=8310 invisible=1547 disabled=0 unpaintable=0
self-check: <h1> 14.05:1, fg [24,26,42] (--bc #181a2a) on bg [232,232,232] (--b2 #e8e8e8)
runtime: 16.5s   (the five-gate browser job currently takes 3m42s)
```

**130 failures, two distinct shapes:**

| ratio | need | site | instances |
|---|---|---|---|
| **3.87:1** | 4.5 | `join-item btn btn-sm btn-active btn-primary` — `issues.rs:58`/`:65`, the **selected** segment of the board/list control, 14px | 10, on 2 pages |
| **4.21:1** | 4.5 | `<span class="text-primary">●</span>` — `auth.rs:15`, `layout.rs:211`, the brand dot, 18px | 120, on all 24 pages |

**The first is a real defect that no source mechanism can see.** It is
opaque-on-opaque — `primary-content` text on a `primary` fill, two tokens
applied by two different rules. `§10.61` bans a semantic family used as a
*text* colour; this is a button's own background. `§10.62` reads sheet
colours **below the muted floor**; these are fully opaque. **`contrast_scan`
is 8/8 on the very tree where this reads 3.87:1.**

**The second validates the prototype rather than accusing the product.** The
brand dot is already in `NAMED_EXCLUSION_CLASS_VALUES` with an exact live
count, and that list's own doc comment states **4.21:1** in those words. An
independent route — rendered pixels, not source — reproduced the
document's own number. **A gate shipped without the same exclusion fails on
its first run for a reason the project has already decided.**

**3. The prototype's three failures are the design specification.**

| attempt | what it reported | why it was wrong |
|---|---|---|
| 1 | `examined=0, failures=0` | a regex colour parser matched nothing. **Zero examined reads exactly like "everything passes."** |
| 2 | `examined=1906, failures=1906` | **Chrome round-trips `oklch()` through canvas `fillStyle` unchanged** — the normalisation trick does not convert, so every colour parsed as three meaningless numbers |
| 3 | `examined=4813, failures=130` | correct: paint each background layer, then the text colour, on a 1×1 canvas and **read the pixel** — the renderer does every conversion and every alpha composite |

**Each wrong attempt produced a confident number.** A rendered sweep that
cannot tell *nothing wrong* from *nothing examined* is `§10.36` with a
browser attached. **Recommendation**: the gate asserts its own population
the way `overflow-gate` asserts 120 cells and `css_scan` asserts `(287, 3)`
— a floor on elements examined, `unpaintable == 0`, a known-good control
pair, and **a 0% or 100% failure rate as a hard error in itself**.

**4. The anti-drift design, reusing what exists.** The gate needs the same
named exclusions `contrast_scan` carries, and two hand-kept lists drift.
**`css_scan::browser_check_files()` already walks `browser-checks/` and is
already `pub(crate)`** — `GATE-001` made it so for exactly this kind of
cross-module reuse. **Recommendation**: a Rust test asserting the gate's own
exclusion list and `NAMED_EXCLUSION_CLASS_VALUES` name the same class
values, read from the `.mjs` file rather than duplicated by hand.

**5. What this release is not.** **Text only — WCAG 1.4.3.** A rendered
**non-text** sweep (1.4.11: borders, outlines, focus rings as the browser
composites them) is the obvious sibling and **is not in this scope**: it
needs its own measurement, and `EDG-001`/`EDG-002` already repaired the
control boundaries by hand-measurement. Naming it here as a follow-on with
its own prototype is the opposite of what this file did to the rendered
text sweep for eleven mentions.

**~~Rounds, proposed. Three~~ — REVISED 2026-11-26, and re-authorized the
same day.** The owner authorized the row on 2026-11-25 and asked for it to be
reviewed *"carefully with my fundamental philosophy."* **Three flaws, all in
the architect's own shape rather than in the measurement:**

1. **A fifth mechanism, and a sixth piece of machinery to police it.** The
   existing exclusion state is **two entries with two exact counts**
   (`NAMED_EXCLUSION_CLASS_VALUES`, `NAMED_EXCLUSION_COUNTS`), in one file
   that has changed in **seven commits** across its whole life. It is cheap
   *because* it is small and single-homed. The proposal gave the gate its own
   list in `.mjs` and then a Rust test to keep the two equal — **three places
   to write a future exclusion, to guard two entries.**
2. **The sweep cannot be the authority, and the proposal never said what it
   is.** The fixture builds **24 URLs**; `app.rs` carries **68 `.route(`
   registrations** (the share of those that render a page is unmeasured —
   this is a bound, not a ratio). The four source mechanisms read every class
   token in every component, rendered or not. So **source is total and
   approximate; rendered is exact and fixture-bounded** — and calling the
   sweep "a fifth mechanism" would have left five checks with no written
   answer to *which is right when they disagree*. That is not hypothetical:
   the brand dot is a **named exclusion** in one and a **4.21:1 failure** in
   the other.
3. **The text/non-text split builds the harness twice.** Fixture, composite
   and colour conversion are shared; the criteria differ only in which CSS
   property is read. And `EDG-003`'s guard was deliberately built **not to
   compute ratios** because source cannot — deferring 1.4.11 leaves it
   approximating for another release and pays the integration cost twice.

**The revised scope: one round, both criteria, one exclusion list with two
readers.**

- **One gate**, `browser-checks/contrast-gate.mjs`, covering **WCAG 1.4.3
  (text) and 1.4.11 (non-text) together**, importing `overflow-gate`'s own
  `WIDTHS`/`createFixtures`/`buildPages` rather than a second copy — the
  `occlusion-gate` precedent.
- **One exclusion home**: a small JSON file both readers read — `contrast_scan`
  in Rust (`serde_json` is already a dependency of `peisear-web`) and the gate
  in Node. **Not** two lists plus a comparison test. The honest cost, stated
  rather than hidden: this **moves a working, shipped, two-entry list**, which
  touches a guard that currently passes.
- **The authority rule written once**, in `NFR-A11Y-005`: source is total and
  approximate, rendered is exact and fixture-bounded; where they disagree the
  renderer is right about what it reached and silent about the rest.
- **The one real defect repaired**: `join-item btn btn-sm btn-active
  btn-primary` — `components/issues.rs`'s own selected segment of the
  Board/List control — **3.87:1** at 14px.

**What the architect's own three broken prototypes specify**, and it is the
part that matters most: `examined=0, failures=0` from a colour parser that
matched nothing; `1906/1906` because **Chrome round-trips `oklch()` through
canvas `fillStyle` unchanged**; and the correct route, **painting each layer
on a 1×1 canvas and reading the pixel**, so the renderer performs every
conversion and every composite. **The gate must assert its own population and
sanity — an `examined` floor, `unpaintable == 0`, a known-good control pair,
and a 0% or 100% failure rate as a hard error in itself.**

**Deliberately not in this row**: the 17 in-scope small-text sites, the 19
`opacity` sites, the nine `link link-hover` sites, the `xs` tier,
`FR-HLT-005`'s unverified status, `proc-macro-error2`, and the wedged Pages
deployment.

---

**`0.69.0` was cut 2026-11-25**, on the owner's approval at exit criterion 7
against `REL-0.69.0`; **its row is removed from the table above per exit
criterion 8**, and the paragraphs below are kept as the record of how it was
scoped — **including the two claims in them that were false**, corrected in
place where they stand rather than deleted.

**`0.69.0`'s scope, measured rather than asserted.** Proposed 2026-11-25,
**authorized the same day — the row, not the cut** (`Release cycle`, exit
criterion 7 is a separate decision, against a release recommendation).
Every figure below was taken at the tree on that date,
with the command that produces it, because four of this file's own carried
candidates were wrong when `0.65.0` was scoped and **three of four were
wrong again here** (below).

**The trigger is the rule's second one — a defect in the shipped product —
and this time the defect is in what the project says about itself.** Three
releases hardened what the guards *read*. This one is about two things that
were never executed at all.

**1. A gate that has never run.**
`browser-checks/disclosure-marker-gate.mjs` enforces `§10.58` over
**thirteen** `<summary>` controls — `display: list-item` and ≥44×44 at two
widths — which `touch_target_scan` **cannot** see, by its own documented
limit: it reads which helper a `<summary>` was given, never the style the
browser computes from it.

```bash
git log -S 'disclosure-marker-gate' -- .github/workflows/test.yml   # no commits
for f in $(git ls-files | grep -E '\.(mjs|sh|py)$' | grep -v '^crates/'); do
  printf '%-45s %s\n' "$f" "$(grep -rl "$(basename $f)" .github/workflows/)"
done
```

**No commit ever wired it**, and the second command — every tracked script
outside `crates/`, not a sample — shows it is the **only** gate absent from
every workflow. `doc-screenshots.mjs` is absent *deliberately*, and
`browser-checks/README.md` says so in its own words; `cdp.mjs` is the shared
library.

**It is cited as authority by things that cannot invoke it**: two
source-code comments (`components/me.rs:358`,
`components/issues.rs:386`) telling a future editor it re-runs when those
lines change, five places in the requirements register, and eight handoffs
across series 061–065 listing it as a required re-run. **A human typing its
name is the only thing that has ever run it.**

**Run at this proposal — exit 0**, every one of the thirteen passing at both
widths (`[disclosure-marker-gate] all thirteen checkable summaries … >=44x44,
at both widths`), alongside `overflow-gate` (120 cells, 0 failing) and
`occlusion-gate` (120 cells, 0 failing) as a control. **So this wires a
passing gate, not a failing one** — the cheap moment, and the honest one.
Its port is `4233`, distinct from the four in CI (`4173`, `4174`, `4175`,
`4177`), so it joins that job without the collision the README warns about —
and that README's own sentence, *"the four scripts default to different
ports"*, is now incomplete.

**The machinery to stop this recurring already exists and nothing joins it
up**: `dec_007_ci_scan::test_yml_run_lines()` already parses `test.yml`'s
`run:` lines, and `css_scan` already walks `browser-checks/` and panics if
it finds no `*.mjs` at all. **Recommendation**: one test asserting every
`browser-checks/*-gate.mjs` appears in a `test.yml` run line, with a named
exclusion list carrying its reason — the shape `PLC-003` and `EDG-003`
already established for acknowledgements.

**2. ~~A status that asserts an absence, and has been false for eight
releases.~~ — CORRECTED 2026-11-25 by `IMG-001`, which re-measured it.**

This proposal claimed: *"`NFR-REL-010 — Published container image` reads
`*Status*: **Divergent** — no image has ever been published`. The image is
published and publicly pullable, and has been for eight releases … It is the
only genuinely `Divergent` entry of the 171."*

**Two errors, both the architect's own, and both from reading a tool's
output instead of the thing it describes.**

1. **"Eight releases."** The registry holds **eighteen** tags — `0.53.0`
   through `0.68.0`, plus `latest`. The command in this proposal ended
   `jq -c '.tags[-8:]? // .'`, **a slice he wrote himself and then read as
   the whole list**. `IMG-001` returned the full list twice and could not
   explain the disagreement from the registry, the workflow source or the
   run history — correctly, because the explanation was in the architect's
   shell, not in any of them.
2. **"`Divergent`."** `NFR-REL-010` carries **two** `*Status*:` lines: a
   superseded opening, and the governing `**Implemented** (0.53.0)`.
   **`spec_scan::governing_status_token` resolves an entry by
   `body.rfind("*Status*:")` — the last line, by design.** The architect's
   own parser took the first. `rule_b` cross-checks Appendix A against
   governing statuses and **was green throughout**; the project's own
   scanner was stating the correct value while this row was written over
   it. Appendix A's `| Release |` row contains **no `Divergent` at all** —
   `NFR-REL-004` is `Partial`, `NFR-REL-005` is `Not defined`, and that
   row needed no change.

**What was genuinely missing, and what `IMG-001` built instead**: nothing
ever asked the registry. `image.yml` proves a push at the moment it ran; the
entry's property is that the image is *publicly pullable*. The round added
an **anonymous** post-publish check — no job credentials, since a
credentialed call would pass on a private package and prove the wrong thing
— with the registry's full `Accept` set, asserting both manifest `200` and
membership in `tags/list`, planted failing on a nonexistent tag and passing
on a real one. **Re-run by the architect at review**: `0.68.0` → `200` and
present; `0.999.99-does-not-exist` → `404` and absent.

**3. ~~The population behind both: eight of the 171 statuses assert an
absence.~~ — CORRECTED 2026-11-25.** That table came from the **same
first-match parse**. Re-derived against **governing** statuses it is
**one**: `FR-HLT-005`, *Implemented, unverified*. The other seven were
superseded openings the document keeps on the record deliberately.

**What is actually there, measured**: **four** of 171 entries carry more
than one `*Status*:` line — `FR-NAV-003`, `NFR-REL-009`, `NFR-REL-010` and
`NFR-CMP-007` (seven lines) — and only **one** of the four marks its earlier
status as superseded near the line itself. **The rule that the last one
governs is stated nowhere in the document; it exists only in `spec_scan`'s
Rust.** A reader reading top-down meets the superseded value first, which is
precisely what happened here, twice, to the person who wrote the row.

**Recommendation**: make the document state its own reading rule, mark the
three unmarked earlier statuses as superseded, verify `FR-HLT-005`'s one
real absence claim, and open a register entry for *a document whose reading
rule lived only in its checker*. Small, and it closes the trap that cost
this release two thirds of its scope.

**Three of four carried candidates were wrong again, and one of them
shipped in a changelog.**

- **"The 21 remaining smallest-text sites"** — the `063` census's **21 is
  its *out-of-scope* column** (`calendar.rs` 11,
  `notification_preferences.rs` 4, `notifications.rs` 3, `search.rs` 2,
  `layout.rs` 1). **The architect carried the excluded population as the
  remaining work** — through `0.67.0`'s candidate, `0.68.0`'s candidate,
  this file's own row, and `changelog/0.60-0.69.md`'s shipped `[0.68.0]` section
  (*"Twenty-one pieces of small print still read at 12px"*). Re-derived with
  the census's **own** command (`grep -rn 'text-xs' crates/ --include='*.rs'
  | grep -vE ':[[:space:]]*//' | wc -l`): **39** render sites, of which
  **in scope: 17** (`sprints.rs` 13, `teams.rs` 2, `sprint_plan.rs` 2) —
  and **one hit is not a render site at all**, `contrast_scan.rs:718`'s own
  acknowledgement literal. The census's own standing instruction said this
  would happen: *"if a figure here disagrees with the tree, the tree is
  right and this file is wrong."*
- **"Fifteen bare-`opacity` sites"** — **19** by the same method
  (`sprints.rs` 5, `calendar.rs` 5, `issues.rs` 4, `notifications.rs` 3,
  `settings.rs` 1, `projects.rs` 1). **No derivation for "fifteen" exists
  anywhere in the repository** — searched; the figure has no source.
- **The `xs` control tier, "18/2"** — now `btn-xs` **23**, `badge-xs`
  **11**, `input-xs` **3**, `select-xs` **3**.
- **"Nine `link link-hover` sites"** — **9**. The only one that held.

**`proc-macro-error2`, re-measured**: three direct parents today —
`leptos_macro 0.8.16`, `reactive_stores_macro 0.4.2`, and `syn_derive 0.2.0`
via `rstml 0.12.1`. `leptos` **0.8.23** is available against the pinned
`0.8.19`, and `syn_derive 0.2.0` **is** its own latest. So an upgrade can
remove at most one of three paths and **cannot clear the warning** — the
candidate's old remedy stays wrong, now with the version numbers beside it.

**The browser-rendered contrast sweep is named here for the fifth time, and
that is the finding about this file.** `PLC-001` wanted it, `PLC-003` wanted
it, `EDG-003`'s row wanted it, `0.68.0`'s candidate wanted it — and the
architect's own `/50` error, the one that would have shipped a failing fix,
is exactly what it would have caught. **A candidate deferred five times is
not a candidate; it is a decision nobody has made.** **Recommendation**: it
is not a line in another release's *deliberately not in this row* list but
its own release, scoped on its own terms, and the architect will bring that
scope as `0.70.0`'s proposal rather than name it a sixth time.

**Rounds, proposed.** Three, one handoff series, opened on authorization:
the gate and the guard that keeps gates wired; `NFR-REL-010` and the check
that asks the registry; the eight absence statuses and `§10.36`'s next
instance. **Deliberately not in this row**: the 17 in-scope small-text
sites, the 19 `opacity` sites, the `xs` tier, the nine `link link-hover`
sites — all text or type questions, and all now carrying a figure that was
re-derived rather than carried.

---

**`0.68.0` was cut 2026-11-24**, on the owner's approval at exit criterion 7
against `REL-0.68.0`; **its row is removed from the table above per exit
criterion 8**, and the paragraphs below are kept as the record of how it was
scoped — including the recommended tier the rounds proved wrong.

**`0.68.0`'s scope, measured rather than asserted.** Proposed 2026-11-23,
the same day `0.67.0` was cut, on the owner's own instruction to scope it.
**Its trigger is a defect in the shipped product**, and it is the first this
project has measured against a **non-text** criterion: three releases of
contrast work all read *text*, and `contrast_scan`'s own doc comment says so
in terms.

**What was measured.** Tokens resolved from the pinned sheets, composited
over the product's own backgrounds, the same method the last two releases
used:

| what the sheet draws | resolved | on `base-100` | on `base-200` |
|---|---|---|---|
| `.input-bordered`/`.select-bordered`/`.textarea-bordered`/`.checkbox` border — `base-content/20` | `#d1d1d4` | **1.52:1** | **1.50:1** |
| `.input:focus`/`.select:focus`/`.textarea:focus` **outline** — the same `/20` | `#d1d1d4` | **1.52:1** | **1.50:1** |
| *for comparison*, `base-content/30` | `#bababf` | 1.93:1 | 1.89:1 |
| **`base-content/50`** — the first tier that passes | `#8c8c94` | **3.34:1** | **3.16:1** |
| `.checkbox:focus-visible` outline — full `base-content` | `#181a2a` | **17.21:1** | 14.01:1 |

**WCAG 1.4.11 asks 3:1** for the visual information needed to identify a
control and its states. A focus indicator is such information.

**And the fill does not rescue the boundary**: these controls paint
`base-100`, so a field on a white card is **1.00:1** against its own
surround and **1.23:1** on the grey page body. On a card, **the 1.52:1
border is the only thing that says a field is there.**

**The populations, counted**: `input-bordered` **35**, `select-bordered`
**23**, `textarea-bordered` **9**, `checkbox` **3** — **70 bordered
controls**. Of those, **67** (every input, select and textarea) also carry
the `/20` focus ring; the three checkboxes already have the strong one.

**The product contains the remedy twice over, which is why this row adds no
new colour.** The vendored sheet is internally inconsistent: a checkbox's
focus ring is full `base-content` while a text field's is `/20`, and
`.btn`/`a` rings use `currentColor` — so the fix is the product's own
stronger pattern applied to the controls that lack it. Borders land on
`/50`, the first tier that clears 3:1 on both backgrounds; **the ring
matters more than the border** and should land stronger still, which is a
decision for the round with a measurement rather than a preference.

**The cascade, and it is `PLC-002`'s direction rather than `PLC-001`'s.**
These rules live in `daisyui.min.css`, served **first**, so an
equal-specificity rule in `app.css` wins on order — and `.input:focus`
carries a pseudo-class, so an override must match its specificity rather
than merely its selector. **Verify in a browser; `§10.52` is why neither
direction is believed from the file.**

**The guard would be a fourth mechanism, and it is harder than the third.**
The three in place all ask *what colour is this text*. A non-text check must
also know *which property* (border, outline, background) and *which adjacent
colour* the comparison is against — and `PLC-001` already showed that only
the rendered tree answers the second reliably. **Recommendation**: extend
`PLC-003`'s acknowledgement machinery to boundary and focus colours for
control classes the product writes, with the same *acknowledged with what
beats it* shape; and **name the browser-rendered sweep again** — it is now
the second mechanism that would be better served by it, which strengthens
that candidate rather than this row.

**Also folded in, because it has cost three red `main`s**: `spec_scan`'s
counted population of dangling-number citations across `rfcs/` **includes
`rfcs/handoffs/`**, which `docs_scan` rule 5 excludes by name as *execution
records, not documentation*. **Recommendation: exclude it there too**, matching
rule 5, and re-derive the constant — one line plus a number, and it ends a
recurring tax on writing a document. The architect broke `main` three times
on it at `0.67.0`.

**Deliberately not in this row.** The **21** remaining smallest-text sites,
the **nine** `link link-hover` sites, the **fifteen** bare-`opacity` sites,
the **`xs` control tier** — recorded candidates, all text or type questions.
**Card and alert borders** (`border-base-300` at ~1.3:1, `border-error/30`,
`hover:border-primary`) — measured and out: each is decoration beside a
shadow or a filled surface, not the sole identifier of a control, which is
the test 1.4.11 actually applies. **`/today`'s content** remains the owner's
own open question.

**Rounds, proposed.** Three, one handoff series, opened on authorization:
`EDG-001` the 67 focus rings (the half a keyboard reader feels first);
`EDG-002` the 70 boundaries; `EDG-003` the guard, the specification work,
one register entry, and the `rfcs/` exclusion above.

**`0.67.0` was cut 2026-11-23**, on the owner's approval at exit criterion 7
against `REL-0.67.0`; **its row is removed from the table above per exit
criterion 8**, and the paragraphs below are kept as the record of how it was
scoped — including the one claim in them the rounds corrected.

**`0.67.0`'s scope, measured rather than asserted.** Proposed 2026-11-22,
the same day `0.66.0` was cut, on the owner's own instruction to scope it.
**Its trigger is a defect in the shipped product** — the rule's own second
one — and it is `0.66.0`'s class one layer down: that release taught the
contrast check to read all four of the theme's colour families, **and the
check reads only the classes a component writes.** It cannot see a colour
the stylesheet applies by element or bakes into a component class.

**What was measured.** Tokens resolved from the pinned sheets, composited
over the product's own backgrounds, the conversion checked against daisyUI's
published hexes (the same method `0.66.0` used):

| what | where it comes from | resolved | on `base-100` | on `base-200` |
|---|---|---|---|---|
| form placeholders | `input::placeholder` in **both** served sheets | `#9ca3af` | **2.54:1** | **2.07:1** |
| the account menu's email line | `.menu-title` (`base-content/40`) **plus** that site's own `opacity-70` | `#bebfc3` | **1.84:1** | — |
| every table header | `.table :where(thead, tfoot)` (`base-content/60`) | `#74767f` | **4.53:1** | **4.25:1** |
| *the floor this project chose* | `base-content/70` | `#5d5f6a` | 6.34:1 | — |

**AA is 4.5:1.** So: 17 placeholder sites fail outright; the account menu's
own email line is the **worst text in the product at 1.84:1**; and every
table header sits on the `/60` tier that **`NFR-A11Y-005`'s own entry
rejected** when it chose the floor — *"`/60` measured 4.54:1 on `base-100`
— a pass by 0.04 — while failing at 4.23:1 on `base-200`"* — which is
exactly what the sheet applies to `thead`, and on `base-200` it fails by
measurement rather than by argument.

**The populations, counted rather than estimated**: placeholders at **17**
sites across seven files (`issues.rs` 4, `teams.rs` 4, `settings.rs` 3,
`projects.rs` 2, `sprints.rs` 2, `search.rs` 1, `layout.rs` 1);
`.menu-title` at **1** site (`layout.rs:383`); `<thead>` at **8** sites
across 11 tables. In the two served sheets, **109 selectors** set a text
colour below the `/70` floor; the product reaches **three** of those
families today, which is why this row is three families and not 109.

**The repairs, and the one trap that decides how they are written.** No new
colour: every repair lands on `base-content/70`, the floor this project
already measured and chose, so `contrast_scan`'s existing rule and this
one agree by construction. **But the cascade is a trap, and it is the same
shape as `§10.52`**: the sheets are served `daisyui.min.css` → `app.css` →
**`tailwind.css` last**, and `tailwind.css` carries the bare
`input::placeholder{color:#9ca3af}`. A rule added to `app.css` with the same
selector **loses** — later sheet, equal specificity. The override must
out-specify it (`.input::placeholder`, a class the product writes at 35
input and 9 textarea sites), and **the round must verify it in a browser
rather than reasoning from the file**, which is what `§10.52` exists to
insist on.

**The guard, and the decision it needs.** The class is now stated twice over:
*a check that reads what components write cannot see what the stylesheet
applies.* The bounded shape — and the one this row recommends — is a scan
over the three served sheets for text-colour declarations resolving below
`/70`, **restricted to selectors the components can actually reach** (an
element or pseudo-element selector, or a class token a component writes),
which is the intersection `css_scan` already computes for its own
population. 109 selectors become the three families above, and a fourth
would fail the gate on the day a component first writes the class that
carries it. **Deliberately excluded, with the reason**: the disabled tiers
(`.input:disabled` and friends at `/40`, `.menu li.disabled` at `/30`) —
WCAG 1.4.3 exempts text that is part of an inactive user-interface
component, so they are named in the scan's own exclusions rather than
silently skipped.

**`NFR-A11Y-005` is amended a second time in two releases**, and that is
worth saying plainly rather than hiding in a status line: at `0.66.0` its
acceptance learned to read four colour families; here it learns that a
colour can arrive without a class at all. One register entry opens for the
class and closes with the repair.

**Also found while measuring, and small enough to fold in**: `app.css`'s own
header comment says *"Tailwind's play CDN covers the bulk of styling"*. The
product stopped loading CSS from a CDN at `0.32.0` (`§10.23`), and that file
is the first thing anyone reads before editing the project's only own
stylesheet. Corrected in the round that touches it.

**Deliberately not in this row.** The **21** remaining smallest-text sites,
the **nine** `link link-hover` sites, the **fifteen** bare-`opacity` sites
and the **`xs` control tier** — all recorded candidates, none of them a
contrast failure, and this row is about one defect class rather than a
sweep of everything adjacent. `/today`'s content remains the owner's own
open question.

**Rounds, proposed.** Three, one handoff series, opened on authorization:
`PLC-001` the 17 placeholders and the cascade trap; `PLC-002` the account
menu's email line and the eight table headers; `PLC-003` the guard, the
amendment, the entry, and `app.css`'s own stale comment.

**`0.66.0` was cut 2026-11-22**, on the owner's approval at exit criterion 7
against `REL-0.66.0`; **its row is removed from the table above per exit
criterion 8**, and the paragraphs below are kept as the record of how it was
scoped. **What shipped matched what was authorized**, down to the site count:
fifteen failing sites found by scoping, fifteen repaired.

**`0.66.0`'s scope, measured rather than asserted.** Proposed 2026-11-21,
the same day `0.65.0` was cut, on the owner's own instruction to scope it.
**Unlike `0.65.0`, this row's trigger is the rule's own second one**: a
defect in the shipped product, and a **P1** requirement recorded `Met` that
is not met.

**What was measured, and how.** The product serves one theme —
`data-theme="corporate"`, hardcoded at `layout.rs:63` — so its tokens are
the product's reality rather than one configuration among many. Resolving
them from the pinned `static/daisyui.min.css` and converting OKLCh → sRGB →
relative luminance (conversion checked against daisyUI's own published
hexes: `#00a96e`, `#ff5861`):

| foreground | on `base-100` | on `base-200` | on `base-300` |
|---|---|---|---|
| `success` `#00a96e` | **3.03:1** | 2.47:1 | 1.98:1 |
| `error` `#ff5861` | **3.08:1** | 2.50:1 | 2.01:1 |
| `primary` `#4d6eff` | **4.21:1** | 3.42:1 | 2.75:1 |
| `base-content` `#181a2a` | 17.21:1 | 14.01:1 | 11.25:1 |

**AA needs 4.5:1** for normal text, 3:1 for large text (≥24 px, or ≥18.66 px
bold). So every use of a semantic colour *as a text colour* on a base
background fails, and `base-content` — the only family
`NFR-A11Y-005`'s own check and its `QA-012`/`QA-013` measurement ever
looked at — passes comfortably. **The entry's sentence *"the theme's own
tokens were never the problem"* is true of one token and false of three.**

**The fifteen failing sites, by family** (census by every spelling that puts
a semantic colour on text, not by one grep — `text-*`, `link-*`, and
`btn-outline` paired with a colour):

| family | sites | measured |
|---|---|---|
| destructive buttons — `btn-ghost btn-sm text-error` ×4, `btn-ghost btn-xs text-error` ×2, `btn-error btn-outline btn-sm` ×1 | **7** | 3.08:1 |
| `link-primary` — the sign-in ↔ register cross-links, a notifications settings link, a notification target, an issue's basis link | **5** | 4.21:1 |
| `text-error` headings — `projects.rs:286` (16 px medium), `confirmation.rs:56` (18 px semibold, just under the bold large-text threshold) | **2** | 3.08:1 |
| `text-xs text-success` — the e-mail opt-in status line | **1** | 3.03:1 |

**Three sites use these colours and pass**, and they are named so the round
does not "fix" them: `error_page.rs:18`'s 48 px status number (large text,
3:1 → 3.08:1), and the two `text-primary` brand bullets, which are
decorative glyphs rather than text.

**Two narrow passes worth recording rather than leaving to be rediscovered.**
`btn-primary` — the product's main action, **37 sites** — measures
**4.83:1**: a pass by 0.33, on exactly the reasoning `NFR-A11Y-005` used to
choose a `/70` floor over `/60` (*"a muted tier that passes by four
hundredths is one theme adjustment from failing with nothing to report
it"*). And daisyUI's solid variants pass properly: `error-content` on
`error` **6.83:1**, `success-content` on `success` **6.93:1**.

**The repairs this row recommends, and why they add no new colour.** The
product already contains an AA-safe pattern for each family, so the fix is
the majority applied rather than a new colour language — the same shape
`0.65.0` used, and `app.css` stays as it is (16 lines, two rules, no colours
of its own):

- **destructive buttons** → solid `btn-error` (`confirmation.rs:64` already
  does this, 6.83:1), or keep the ghost and drop the colour, measured both
  ways;
- **`link-primary`** → plain `link`, which underlines and inherits
  `base-content` (17.21:1); the affordance stays, the colour goes;
- **`text-error` headings** → the round decides between `base-content` with
  the card's own `border-error/30` carrying the warning, and making the
  heading large enough for the 3:1 rule (`confirmation.rs:56` at 20 px bold
  qualifies and would pass at 3.08:1);
- **the success status** → `badge badge-success` (6.93:1), a pattern already
  in the product.

**The guard, and the decision it needs.** `contrast_scan` bans
`text-base-content/{10..60}` and nothing else. Two shapes were considered:
**ban a semantic colour in a text position** (`text-*`, `link-*`,
`btn-outline` + a colour), with a named exclusion list for the three
measured passes; or **compute each site's ratio from the pinned tokens**.
**Recommendation: the ban**, because it is this project's own precedent (a
tier is banned rather than measured per site) and because the hard half of
computing — *which background does this element actually sit on* — is a
limitation `NFR-A11Y-005`'s entry already records for the `/70` floor. The
measured ratios go into the entry so a theme change has something to
re-measure against.

**`NFR-A11Y-005` is amended by this row**, not merely cited: its acceptance
names a check that cannot see three of the four token families, and its
*"never the problem"* note is wrong as written. One register entry opens
for the class — *a guard measured one family and the requirement generalised
to all of them* — and closes with the repair.

**Deliberately not in this row.** The **21** remaining smallest-text sites
(`calendar.rs` 11, the two notification surfaces 7, `search.rs` 2,
`layout.rs` 1) — a type question, not a colour one, and the row that reads
them should read them as a set. The **`xs` control tier** — and its recorded
figures are corrected here: **18** `btn-xs` and **2** `select-xs` render
sites, not 19 and 3; the earlier count included `touch_target_scan.rs`'s own
ban-list strings, which is this table's own decay again. Note that two of
this row's destructive buttons are `btn-xs`: **their size is not this row's
business, only their colour.** The **fifteen bare-`opacity` sites**, whose
two conditions `0.65.0` recorded. And **semantic borders** —
`border-error/30`, `border-info/40`, `hover:border-primary` — measured and
out of scope: each is card or alert decoration, or a hover affordance that
comes with a shadow, never the sole indicator of a control's boundary, so
WCAG 1.4.11's 3:1 is not implicated the way a form field's only border would
be.

**Rounds, proposed.** Three, one handoff series, opened on authorization:
`CLR-001` the seven destructive buttons (the family with the worst ratio and
the clearest existing precedent); `CLR-002` the five links and the success
status; `CLR-003` the two headings, the guard, the amendment and the entry.

**Found while opening the series, 2026-11-18 — the scope is unchanged, so
this is a note rather than a re-authorization.** Reading the two vendored
stylesheets instead of reasoning from the class names: daisyUI's own
`.label-text` is **`.875rem/1.25rem`** — **identical to `.text-sm`** — and
its `.label-text-alt` is **`.75rem/1rem`**, **identical to `.text-xs`**
(`static/daisyui.min.css`; `static/tailwind.css` is served last, so at equal
specificity its rule wins — the cascade `layout.rs:34` already depends on).
So **three classes are doing the work of one**: the eight small form labels
carry a `text-xs` that *overrides a component class which already says
14 px*, and the seven small hints carry a `text-xs` that is simply
**redundant** with `.label-text-alt`'s own 12 px. For the labels the fix is
therefore a **deletion**, not a replacement; for the hints the decision is
genuine — 14 px on all twelve (what `0.64.0` did to the five it read, by
overriding) or 12 px on all twelve (delete the redundant token and revert
those five). **Removal is on the table in that round in both directions**,
and `LBL-001` owns the decision. **Scope, screens and site count are
untouched by this**, which is why the row above is not re-authorized; a
change to any of those would be.

**One thing this release deliberately does not clean up, and it is new.**
The **54** sites that read `label-text text-sm` carry a token that is
**redundant with `.label-text`'s own size** — they render identically with
or without it. Deleting it would change nothing a reader can see, would
touch screens this row does not read, and the question it raises (whether a
size class should be written beside a component class that already sets the
size) is a convention worth deciding once rather than as a 54-site drive-by.
**Recorded as a candidate, not scheduled**, and named here so the rounds do
not take it on their own initiative.

**Why a number now, when the rule above says to wait.** That rule asks for
*something an installed user would notice*, and the architect's own answer on
2026-11-03 was that nothing qualified. What qualifies now is already on `main`
and measured in a browser rather than argued: at 1920 px a reader saw
`● peisear … Menu   Demo Account` and nothing else — no five destinations, no
search box, no unread badge — because the committed stylesheet had no rule for
the classes `NAV-001` shipped (`§10.52`, fixed in `838fb24`). After that fix
the same reader sees five named destinations and a search box at every width
above `md`. `059` adds three rounds in the same register: an undo that stops
expiring after five seconds, destinations that answer to their own names, and
the guard that makes the stylesheet checkable.


**`0.62.0`'s scope, and what reading the specification did to it.** The
owner's sequence puts **each view** third, after the base layout and the
workflows, and the obvious proposal was *"make `/today` show the day's
work"* — the open question `059` left. **That proposal does not
survive its own requirement.** `FR-PER-001` is a **P0** and says `/today`
shows *"the authenticated user's own current load, work rhythm, and
sustainability signals"*; `FR-PER-007` (**P1**) says in terms that the
rhythm panel **MUST** be collapsed by default and the sustainability
panel **MUST** self-expand only at a watch threshold, sourced to the
**Minimal-by-Default pillar** and Definition of Done `§41.4`. The
collapsed panels `059` reported as *the home screen's substance hidden*
are **specified behaviour**, and adding the day's work to that page is a
scope change to a P0, not a fix.

**So the `/today` question changes shape rather than going away**, and it
is the owner's: the name promises the day and the page delivers the
day's *state*. Three ways out — rename the heading to **Today** and let
the word mean *how your work is going today*; change what `/today`
shows, which reopens a P0; or keep two names for one place.
**Recommendation: the first**, on `FR-PER-001`'s own words. It also
unblocks `FLOW-002`'s held-back half, which `page_title_scan` currently
pins as pending.

**What this row is scoped to instead, measured rather than asserted.**
There are **18 `<details>` across six views**, two of them open by
default, and **exactly one requirement governs any of it** —
`FR-PER-007`, for `/today` alone. The other thirteen are ad-hoc. And on
the project board at 1280×900, in the screenshot that ships in the
README, **the first column header sits about 610 px down and the first
issue card's title about 650 px** — a reader sees roughly **seventy per
cent of the viewport** before the first piece of work, all of it
breadcrumbs and health analysis. **Nothing governs that order**:
`external-design §5.9` states the order of items *within* lists, and
`FR-HLT-008` the composite's weight *within* the health block; neither
reaches the vertical order of a screen's own sections. Checked in both
documents rather than assumed — twice, after `/today` taught the lesson
an hour earlier.

**Deliberately not in this row**: the thirteen ungoverned disclosures as
a census (they are a set, and a set is decided once, after two views
have shown what the rule should be); `/today` itself, which waits on the
question above; and the demoted contrast-and-size floor, which is
adopted **after** the views are decided, never before — rule after
design, the correction the owner made on 2026-11-04.

**Two of this row's own directions were reversed before it was
authorized, by the architect's re-reading at the owner's request on
2026-11-07.** `FLOW-001` had said *delete the undo's 5-second timer, it
is less code than pausing it* — false, and measurably so: each surface
builds its own fixed-position toast in the same corner, nothing clears
another element's, and two of them print over each other with the
earlier **Undo** covered and unreachable. The timer is the only thing
keeping that rare, so the round now pauses it on hover and on focus and
does nothing else. `FLOW-002` had said rename `/today`'s heading to
*Today* — held back, because `components/me.rs` renders no issue at
all, and a page named for the day that shows two metric chips makes a
promise where it used to make a mismatch. **Both corrections make the
round smaller**, and both came from asking what the result is rather
than what the diff looks like.

**This is the fourth `0.61.0` proposal, and the first whose headline was
measured before it was written.** The three before it were withdrawn — two on
2026-11-03 for leading with claims the cited documents disproved, and the
earlier pair for assembling a release by defect *shape* rather than by subject.
The difference here is not confidence: it is that the headline was read off a
running browser at four widths and a screenshot, and the fix is already pushed.

**What this row does not include, named rather than implied**: whether
`/today` should carry the day's planned work above its metrics (an open
question for the owner, in `059`'s own README — the data is already computed
for the calendar); unit three of the accessibility sequence (each view), which
is not written; and the view-level reading `059`'s README records so it is not
lost — a home screen whose substance sits behind three closed disclosures in
the smallest, faintest text on the page.

**A `0.61.0` proposal written and withdrawn the same day — 2026-11-03.** The
first one was *"a check that cannot fail is not a check"*: `QS-002`, `§10.32`,
a `spec_scan` baseline rule and two one-line CI gaps, assembled because all
four share `§10.36`'s failure *shape*. The owner asked for it to be reviewed
against this project's fundamental philosophy and it did not survive, on three
counts, all the architect's:

1. **Its `§10.32` round is the campaign `§10.32` itself forbids.** The four
   branches it named were added to the fixture at `0.40.0` (`GATE-001`–
   `GATE-003` and `LAYOUT-011`'s); the entry stays open for a different reason
   — nothing tells you *from the artefact* that a swept page rendered what it
   was swept for, and that remedy is *"not designed, not scheduled"* — and
   from `GATE-004` an unrendered branch is **"recorded in
   `browser-checks/README.md` and not chased"** unless a defect appears behind
   it. **The round was proposed from this table's own candidate sentence, which
   had been stale for twenty releases, rather than from the entry.**
2. **Two of its four rounds should carry no release number at all**, on the
   `054` precedent three days older than the proposal — see the note above the
   candidate list.
3. **Nothing in it was for a user.** The last release whose subject is a screen
   a user of this tracker uses is **`0.46.0`** (a team page's breadcrumb and a
   working back link); the fourteen since have been install paths, backends,
   configuration, documentation and the project's own checks — all defensible,
   none of them the second half of the owner's philosophy. Assembling a release
   by defect *shape* produced one with no subject a reader can read.

**Recorded rather than quietly replaced**, on this project's own convention for
its mistakes: the first proposal is the fourth time the architect has brought a
recommendation that reversed on reading the entry it rested on, which is why
the row above names what each of its own claims was measured against.

**And the replacement was withdrawn the same day, 2026-11-03, because its
headline did not exist — after the owner had authorized its row.** *The
architect first recorded that as an authorization of the release, "returned
unspent" and "available"; the owner's clarification the same day was that the
**cut** was never what had been authorized, and the two-decision rule is now
written above the table rather than assumed. Attributing a decision to the
owner that the owner did not make is the worst-placed instance of this day's
three claims-without-a-source, because the other two misread a document and
this one misread a person.* The second proposal — *"where the specification
and the product disagree"* — led with
`FR-API-006`: two API paths *"named in `SPEC Appendix E.1`, published in this
repository's own specification"* and routed nowhere, **so a consumer reading
our own documentation gets a 404.** Both halves are false, and the documents
that disprove them are the ones being cited:

- **`SPEC` is not published and is not a source of truth.**
  `docs/specification/README.md:46` says so in one line, of `SPEC` together
  with `KICK`, `BRIEF`, `V3` and `GUI`: provenance, named so a reader can see
  where a rule came from, held outside this repository. No consumer can read
  `Appendix E.1`. **The architect's own link to that document, at the head of
  this file's Near-term section, was dead** — to a path that has never
  existed here.
- **What is published says the right thing, twice.** `external-design.md`
  §4.4 is headed **Reserved routes** and lists both paths; §8.2 lists the four
  endpoints that exist and then **`Reserved: … → FR-API-006`**. The
  requirement itself reads *Specified — not implemented*. **The product and
  every published document agree.** The disagreement was in the proposal.

**So the release's stated content was a defect that does not exist, and
count 3 above stands against the second proposal as well** — with the
outward half void, nothing in it was for a user either. **Twice in one day a
release was assembled from the register rather than from the product, and the
register cannot say what a user is missing, because it is a record of what we
noticed.**

**What the measurement did find is the reason not to assign a number at all.**
Every **P1** requirement in `requirements.md` is Met or Implemented except
`NFR-REL-004`, whose gap is §9.4's evidence limitation — a process
requirement, not a screen. *(Bounded claim: a scripted pass over the file's
`*Priority*: P1` entries and their recorded statuses, which reads the statuses
as written — and three of those statuses were wrong as written this week, so
the census under the candidate list is the thing that would make this number
trustworthy.)* On that reading **the user-facing obligations this project has
written down are discharged**, the remaining candidates are internal or
deliberately deferred, and the next release for a *user* needs a **new
requirement** — which is the owner's to set, not the architect's to invent
from a defect register.

**The architect's recommendation, therefore: no `0.61.0` yet — and the owner
accepted it on 2026-11-03.** The status census and the link check join the
no-number queue under `[Unreleased]`, where `054`'s precedent already puts
internal work, and **the next number is assigned when there is something an
installed user would notice** — either new scope from the owner or a defect
found in the shipped product. A release cut to honour a number is the opposite
of *less is more*.

**What this accepted recommendation commits the architect to, stated so it is
not re-litigated from memory.** The no-number queue is *work*, not a holding
pen: it is scheduled as rounds under `[Unreleased]` and reconciled at exit
criterion 8 of whichever release is eventually cut, so none of it waits on a
number. And **the next release for a user starts with the owner, not with this
file** — the register records what we noticed, and twice in one day it was
mistaken for a backlog of what users are missing. When the owner sets new
user-facing scope, or a defect is found in what is installed, the architect
brings a numbered row for it; until then the number stays unassigned and this
section says so plainly rather than looking empty.

**Two candidates named in the second proposal did not survive their own surface
reads either, and both reads were owed before they became rounds**:
`FR-PLAN-004` is a **status correction, not work** — its **MUST** limb ships
at `components/sprint_plan.rs:149` and is pinned by
`committed_total_matches_sum_of_effort`, and its **MAY** limb is a choice, not
a gap (`§10.34` instance 10; `§10.16` had filed it correctly in 0.40.0 and
called the fix unscheduled). And **`FR-HLT-007`'s history limb is a deliberate
deferral with a privacy rationale, not drift**: an indicator's history is a
time series, and for a project with one active contributor it is that person's
history — the shape `NFR-PRIV-007` suppressed at 0.28.0 — so it needs
`QA-017`'s contributor predicate and an audit of its own. It is removed from
the candidate list; a scoped deferral is not a disagreement between the
specification and the product.

**What ships next for the dev team — authorized by the owner 2026-11-03 with
no release number, as `rfcs/handoffs/056-checked-against-the-thing-it-describes/`
(`DIA-004`, `REQ-005`, `REQ-006`, one in flight at a time).** The three were
**revised the same day** after the owner asked for them to be reviewed against
this project's philosophy: they did not survive as proposed, and the lead item
is new.**
The first proposal was: the status census, `FR-TEAM-005`'s test, the clippy
feature gap. What measuring them found:

**The clippy item fixes nothing, and measuring it was the whole value.**
`cargo clippy --workspace --all-targets --no-default-features --features
postgres -- -D warnings` is **already green — exit 0, zero diagnostics**,
run here rather than assumed. So the two lines are a regression guard, not
a repair, and calling it *"the one item about the product's compiled code"*
oversold it. **But running it surfaced a false claim in published
rustdoc.** `dialect.rs`'s own module comment says *"**No `#[cfg(feature =
"postgres")]` anywhere outside this module and `pool.rs`**"*, naming five
files that touch dialect-dependent types and asserting *"None of the
fourteen occurrences carries a `#[cfg]`."* Measured across
`crates/peisear-storage/src` (grep for `feature = "postgres"`, attribute
and comment lines alike): **thirteen sites in six files**, and they are
**production code, not tests** — `user_burnout.rs` (3),
`personal_metrics.rs` (3), `metrics_snapshots.rs` (1) each carry a
**second, postgres-only SQL body** (`const QUERY`, or a `format!`ed `sql`)
for the health and sustainability queries; `optimistic_lock_guards.rs` (2),
`sprints.rs` (2), `issue_events.rs` (1) and `lib.rs` (1) carry more, and
four of those six files are not among the five the comment names. **A
contributor who trusts that comment will not look for the second query to
keep in step** — two SQL bodies per query, in the product's own domain
logic, with a published comment saying they do not exist. That is the
owner's *maintenance cost (continuous) and serious risk* clause, not a
tidiness item, and it is recorded nowhere.

**Revised, in this order:**

1. **The dialect seam's own claim.** Correct `dialect.rs`'s comment against
   what the code holds, and — the substantive half — decide whether seven
   postgres-only query bodies living outside the two files said to own them
   are the intended design or drift that accumulated behind a comment
   nobody re-read. **The clippy two-liner lands here as this round's
   regression guard**, with its *green today* stated so nobody reads it as
   a fix.
2. **`FR-TEAM-005`'s missing screen test** — and it is the **only one of
   the three that is for a user**: the footnote tells a reader that admin
   is a management role rather than an oversight one, which is the *not to
   be confused or misunderstand* clause in as plain a form as this product
   has. A **P1** whose own *Acceptance* has named the assertion all along.
   Two bounds: it must be written to `§10.17`'s standard, since eleven
   assertions in this suite once passed on markup other than their own
   subject; and **it is one of 28** — `requirements.md` has 51
   `*Acceptance*` clauses and 28 name no test identifier at all. I picked
   this one because I stumbled on it, and **the other 27 are unaudited,
   not cleared.**
3. **Pin the absence claims, instead of sweeping 185 of them.** The census
   as first proposed was a one-time read of every `*Status*` clause — a
   snapshot that goes stale the next time a status is written, which is the
   shape this project rejects on its own doctrine: *the defect is not the
   instances, it is a rule that was never written down.* The durable form
   is the one `docs_scan` rules 8 and 9 already use and this project
   already trusts: **an asserted enumeration** of the status clauses that
   claim an absence, so adding one becomes a deliberate act and an
   existing one cannot be forgotten. A one-time read is still owed, but to
   **calibrate what the pattern misses**, not as the deliverable.
   *(Considered and rejected: a rule routing coverage claims into
   `*Acceptance*`, where CI already checks citations. It dies on
   measurement — only 51 of 185 entries have an `*Acceptance*` clause, so
   that "clean rule" is a 134-entry migration wearing a rule's clothes.)*

**What the review says about the proposer, not the proposal.** Two of the
original three were *a check for something already fixed by hand* — the
exact shape I rejected by name three days ago when withdrawing the first
`0.61.0`, and reintroduced here under a different heading. The one item
that was not is the one I ranked second and called small. **The lead item
exists only because the owner asked, and because asking made me run a
command instead of repeating a sentence I had written twice.**

**Proposed 2026-11-03, after `056` closed and the dev team reported —
correctly — that nothing was open.** Verified: **zero** rounds marked open
in any series. Of the seven open register entries, one closes on reading
(`§10.42`, below) and one is clearly the strongest piece of work this
project has outstanding.

**`057` — a gate that is not watching. Two rounds, no release number — **accepted by the owner 2026-11-03** and written as `rfcs/handoffs/057-a-gate-that-is-not-watching/` (`PLT-001`, `WLK-001`, one in flight at a time). **No RFC and no requirement amendment**: the RFC numbering is at `022` and `057` is a handoff series; `§10.50`/`§10.36` already name the class, and `NFR-REL-009` already promises the artefact a user cares about — what is wrong here is *when we find out it cannot be built*, a cost and a risk rather than a promise, so folding it into an entry whose own text says *"the check is the artefact being there"* would blur the distinction that entry exists to draw.**

1. **`§10.50`: a `windows-latest` job in `test.yml`.** All **47** of that
   workflow's jobs run on `ubuntu-latest`; **not one** runs on Windows or
   macOS. The only workflow that compiles any other platform is
   `release.yml`, which fires on a tag — **so every platform but Linux is
   first compiled at the moment the release archive is being produced**,
   the latest point at which a finding can still be acted on without
   re-cutting. **This is not hypothetical twice over**: `0.58.0` lost a
   published platform to this class, and in one series `release.yml` found
   two defects `test.yml` structurally could not — a helper with no
   `#[cfg]`, dead on Windows, which `-D warnings` would have rejected had
   any job compiled it there; and `script: illegal option -- f`, because
   BSD `script` is not GNU `script`. **The entry names the cheap remedy
   itself** — a `windows-latest` job running `cargo clippy -p peisear`
   closes most of it, cross-compiling from Linux will not because
   `libsqlite3-sys` builds C, and macOS costs runner minutes Linux does
   not — and it says in terms that **whether to pay, and when, is this
   table's to say.** It is worth paying: five archives ship to users,
   `.zip` among them, and the only thing standing between a Windows user
   and a broken artefact today is that the tag build happens to be read.
   **Windows first, macOS deliberately not yet** — the recorded incident
   was Windows, and one platform bought cheaply beats two argued about.
2. **The `env-example-walk` trigger — the architect's decision, now
   made.** `.github/workflows/env-example-walk.yml` genuinely asserts the
   documented copy-and-run path (non-zero exit if it does not reach a
   listening server, since `0.53.0` round 2), but it is
   **`on: workflow_dispatch:` only**, with a required `ref`, and its whole
   history is **three manual dispatches, the last 2026-10-16** — before
   `0.57.0`, `0.58.x`, `0.59.0` and `0.60.0`. Meanwhile `QS-003` made
   `cp .env.example .env` documented on a **second** page. **Decision: give
   it a `push` trigger scoped to the paths that can break it** —
   `.env.example`, the configuration sources, and the two pages that
   document the command — rather than every push or a schedule. It is a
   real gate that is simply not running; a path filter makes it run exactly
   when its own inputs change. **This does not widen `NFR-REL-011`**, whose
   `0.53.0` round 2 decision excluded this reader and said in terms that
   the walk *"stands on its own merit as a regression guard… it does not
   need this entry's authority to justify existing."*

**Why this is not the shape the owner's review removed on 2026-11-03.**
That proposal bundled four items because they shared `§10.36`'s failure
*shape*, and three of them were checks for defects already fixed by hand.
**Neither of these is.** Both are live gaps on **shipped artefacts**: a
platform axis that has already cost a published archive, and an assertion
that exists and is not running. Nothing here is a check for something
already repaired.

**Closed on reading rather than scheduled: `§10.42`**, the entry the owner
raised on 2026-10-15 asking whether relic rules were holding the project
back. Its scheduled remedy — *a `spec_scan` over this document's own
checkable properties* — shipped at `0.57.0` and now runs four rules, 39
tests and eight CI steps, and **its stated bound held**: `REQ-006`'s rule 3
shortlists and judges nothing, on `REQ-003`'s own evidence. The entry
stayed open only because nobody re-read it after the thing it scheduled had
shipped. **The answer to its question is yes, a relic was holding us
back** — `REQ-003`'s conclusion over-applied — **and it has been narrowed
to the class it proved.** The five-pass sweep remains the only guard for
status *semantics*, deliberately.

**Still not recommended**, and still on *less is more* rather than merit:
the `spec_scan` baseline rule, the `changelog_scan` section-date rule and
the link check outside the book. Each is a check for something already
fixed by hand.

**One item in the no-number queue is owned and not optional**:
`§10.54`, the browser gate's first unstable result —
`google-chrome-stable did not expose a page target` on a
documentation-only commit, `DEC-048` condition 3, **the architect's** by
that condition's own words. The repair is `cdp.mjs`'s `launch()`: wait
on the signal for longer, and on failure report the browser's own
stderr instead of a conclusion the harness cannot support. **No retry,
at any level.** The gate is deliberately **not** quarantined — see the
entry for why, and for what to do if it recurs before the repair lands.

**One candidate added at `0.61.0`'s cut, on the same footing**: a
`changelog_scan` rule that a release section may not carry the same
`###` heading twice. `[Unreleased]` carried **two `### Fixed` sections**
for several days before the cut merged them, and nothing noticed —
`changelog_scan` checks where versions live and whether links resolve,
not the shape of a section. **Not recommended either**, for the same
reason as the three above: it is a check for something already fixed by
hand, and the thing that caught it was assembling a release.

**The accessibility work, reconsidered 2026-11-04 on the owner's second
reading — and the architect's own lead recommendation is withdrawn.** The
owner asked whether layout, typography and colour could be kinder to **elder
people**, listing **view layout first**. The first proposal answered **colour**
first, typography second, and put the layout read **last, labelled
unmeasured** — because colour is the part a scan can compute. *"Good trial on
direction. However, we have to consider carefully one by one with time we
need… App base layout(s), user workflows, and each view."*

**Three things were wrong, and the second is the serious one.**

1. **The owner's first bullet was ranked last.** The ranking followed what was
   measurable, not what was asked.
2. **The recommended rule points the wrong way.** It would have forbidden the
   muted tier below 14 px — and the effect of that, applied to the ninety
   sites, is to **enlarge ninety pieces of currently-quiet secondary
   information**, twenty of them on `/today`, the first screen a user sees
   after signing in. **That makes the busiest screens denser.** *Less is more*
   points at **removing** what the reader does not need, not at making it
   bigger. **A rule cannot decide whether information belongs on a screen; only
   a read of that screen can.** The rule was a mechanical remedy offered for a
   design question, which is the shape the owner's review removed once already
   on 2026-11-03.
3. **It was batched into three rounds** of one release. The owner asked for one
   at a time, with the time each needs.

**What the base layout actually does, measured from source — and it is the
answer to the first bullet.** Three shells in `components/layout.rs`:
**`Base`** (the document), **`AppShell`** (authenticated), **`PublicShell`**
(login, register, error — no navbar). **68 routes** inherit one of them, so a
decision here changes every screen at once. `AppShell` renders
`SkipLink → Navbar → main[FlashBar + children]`. And inside `Navbar`:

| element | visible by default? |
|---|---|
| brand link → **`/projects`** | yes, with text |
| search input | **`hidden sm:block`** — not below the `sm` breakpoint |
| bell → `/inbox` | yes, **icon-only**: an 18 px SVG with an `aria-label` and no visible word |
| `/today`, `/teams`, `/inbox`, `/settings`, sign out | **no — all five sit inside one `dropdown-content`**, behind the user's own name |

**So "what is visible by default" has a specific answer: the primary
navigation is not.** Phase A's *5-entry navigation* exists — and four of its
five destinations are one interaction behind a menu whose label is the
reader's own username, which is the least predictable place to look for
"where do I go". The only always-visible destination is the brand, and it
leads to `/projects`, which is not one of the five. The one control that is
always visible and *is* a destination carries **no word at all**.

**Which corrects what *less is more* means here.** The base layout has already
applied "less" — by **hiding**. That buys a quieter bar and charges a two-step
interaction plus a memory burden. For a reader whose acuity and short-term
recall are both declining, **less to decode** is the goal, and a visible row of
labelled links decodes faster than a dropdown even though it is *more on
screen*. **Hiding is not simplifying.** That is a design judgement for the
owner, and it is the first one worth making.

**The sequence, in the owner's own three terms and in the owner's order. One
unit per round, no batch, no release number yet.**

1. **App base layout(s) — authorized 2026-11-04 and written as
   `rfcs/handoffs/058-where-do-i-go/` (`NAV-001`, one round).** The three
   shells and the navbar, because 68 routes inherit them and this is where one
   decision reaches every screen. **And the finding sharpened on reading the
   requirement**: `FR-NAV-001` is a **P1** stating that the primary navigation
   **MUST expose exactly five entry points — Today, Inbox, Projects, Teams,
   Search** — its status is `Implemented`, and its *Acceptance* is the
   sentence *"navigation renders five entries"*, which `§10.35`'s own table
   already records as **a restatement nobody can check**. So nothing has ever
   verified the five. Measured against the source: **none of the five is
   visibly named at every width**; `Inbox` is an unlabelled 18 px glyph,
   `Projects` is reachable only through the brand link, `Today` and `Teams`
   are in the dropdown, `Search` is `hidden sm:block`, and **`Settings` is
   exposed although it is not one of the five.**

   **The architect's design decision, which the previous entry wrongly handed
   up**: *expose* means **visible and named, without an interaction**. Design
   authority is the architect's; the owner authorizes scope. The five become a
   labelled row, the bell keeps its badge and gains its word, `Projects` stops
   depending on a logo, `Settings` moves to the user dropdown with Sign out,
   and below the collapse threshold the five sit behind one control labelled
   with a **word** rather than the reader's username. The round also writes the
   **locatable** acceptance the requirement has never had.
2. **User workflows.** The paths across those views — register → first project
   → board, and the daily loop through `/today`. **Not measured.** `§10.15`
   records that this project does not execute rendering; `browser-checks`
   already sweeps 19 pages at 5 widths, so the fixture exists and the reading
   does not.
3. **Each view, in the order the evidence gives.** The ninety sites are not a
   defect list — they are a **map of where secondary information has
   accumulated**: `me.rs` **20**, `sprints.rs` **18**, `issues.rs` **18**,
   `teams.rs` **10**, then a tail of ten files with four or fewer. **Fifty-six
   of the ninety are in three files**, and the densest is the screen a user
   opens first. In each view the question is *does the reader need this at
   all* — and only then, if it stays, how it should read.

**The conjunction rule is not discarded; it is demoted.** `text-xs` with the
muted tier is still the least legible thing this product renders, and the
measurement stands (**90** sites at 12 px and 5.15–6.36:1, against a
`NFR-A11Y-005` bar of AA that the project already exceeds to **AAA** on touch
targets). But it belongs **after** the views are decided, as a floor that stops
the next drift — **rule after design, not instead of it.** Adopting it first
would have frozen ninety decisions nobody had made.

**This is order, not dates.** A row's number changes when something
lands ahead of it, and the change is recorded here rather than argued
from memory.

**Shipped and removed from this table**: `0.52.0` (the guards
themselves) on 2026-10-14. A row leaves when the release is **cut**;
what it contained is in `CHANGELOG.md` and in the specification's own
baseline rows, which is where release history belongs.

**Shipped 2026-11-03**: `0.60.0` (what the product
does with what the operator typed) — `RFC 022`, three work rounds
against three scoped (`CFG-001`–`CFG-003`, two of them sent back a
round: an `unsafe` block round 1 introduced, and a guard that could
not fail on the shape it existed for). One rule, stated once and
applied to the whole configuration surface: absence degrades; a
malformed present value refuses startup, by name, naming what would
have been accepted — generalising the shape `PEISEAR_SETUP`,
`JWT_SECRET` and `BIND_ADDR` already had to the four that had not,
`COOKIE_SECURE`, `RUST_LOG`, `SMTP_PORT`, `SMTP_TLS_MODE`. `§10.51`
closes with it, on a plant rather than a feeling. **`README.md`'s own
front door**, authorized separately and with no release number of
its own (`054`), lands under this cut since it merged first and had
nowhere else to go. **`DEC-055`'s own archive rollover falls due**:
ten versions move to `changelog/0.50-0.59.md`, the first move since
`0.50.0`. **What it does not fix**: `NFR-REL-007`'s new clause ships
enforced by a reader, not a check — `QS-002` is open and this
release does not wait for it; `RUST_LOG` fixed but not guarded (no
literal `env::var` for `docs_scan` rule 8 to anchor on); `§10.50`;
`§10.32`; an attended start unproven on Windows; `SMTP_TLS_MODE`'s
five undocumented aliases, deliberately not narrowed.
`changelog/0.60-0.69.md`'s own `[0.60.0]` section carries this in full. **Cut
2026-11-03**, on the owner's approval: tag `0.60.0` at `66741ae`,
signed (`gpg: Good signature`, the same key as `0.59.0`), its message
carrying the tag-pinned release-notes link — and that link read back
from the API rather than assumed, `CHANGELOG.md` at `ref=0.60.0`
resolving with `[0.60.0]` as the first dated section in it; **seven
crates on crates.io** at `0.60.0`, none yanked, each confirmed by its
own per-crate registry query — six `Published` lines again covered
seven crates, `peisear-auth` and `peisear-i18n` batched onto one, so
the line count would have read as six; **five per-platform archives
on the GitHub Release**, read from the asset list and compared **as a
set** against `install.md`'s own target table, none extra and none
missing, the `x86_64-pc-windows-msvc` `.zip` among them; the Release
body diffed against the section as it stands in the *tagged*
`CHANGELOG.md`, identical across 114 lines, with the extraction rule
itself validated first by reproducing `0.59.0`'s live body byte for
byte; `ghcr.io/nabbisen/peisear` at `0.60.0` and `latest` under one
digest (`sha256:56eb9d21`), `latest` moved off `0.59.0`'s, and
`0.59.0` (`sha256:0a5a7474`) and `0.58.1` (`sha256:1057c422`)
unchanged. All four `DEC-056` legs verified against the registries
themselves rather than from their workflows' own exit codes.

**What this cut itself got wrong**: the section is dated **2026-11-01**
and the tag is **2026-11-03** — the first divergence in at least nine
releases, where section date, tagger date and commit date had been the
same day every time. The architect wrote the date into the candidate
and did not re-date the heading when approval came two days later,
finding it only after the tag was pushed and while the crates were
publishing. **Left as tagged**, on this project's own rule that a tag
moves only before its crates are published, and because re-dating
`main` alone would leave the tagged copy and `main` disagreeing about
the one fact the heading carries. The rule it should have followed,
and the unchecked row that admits nothing enforces it, are now in
`docs/development/changelog-and-releases.md`; a `changelog_scan` rule
that would catch it is a candidate above.

**Shipped 2026-10-31**: `0.59.0` (configurations the product actually
supports) — `RFC 021`, in four work rounds (`SUP-001`–`SUP-004`).
PostgreSQL is a configuration peisear says it supports, and now means
it: `§10.48` closed (the feature forwards the driver it names, not
only the `cfg`), all 33 `peisear-web` integration binaries run
against a real `postgres:17` in CI unfiltered (nine real defects
found and fixed, none of the 33 genuinely SQLite-specific), one
documented install path (`cargo install
--no-default-features --features postgres`), one declared version
(`postgres:17`). All seven `SMTP_*` variables documented where
configuration is documented; the startup warning no longer teaches a
subset of four. **`NFR-CMP-007` is added**, on the owner's
decision of 2026-10-31 — `NFR-CMP-003` reserved that question for the
owner on the trigger this release fired, and without it 0.59.0 would
have shipped a supported configuration whose persistent state no
requirement governed. **This release's own second subject**: both
specifications' record of themselves had drifted — `external-design.md`
unamended since `0.57.0`, both Appendix C baselines stale, three
different answers to which release each document covers — found
while assembling this cut and repaired in it, backfilled rows marked
as backfilled rather than written as though produced at their own
cuts (`§10.36` instance 7). `external-design.md §1.1` no longer
describes a single-backend product, the condition `0.55.0`'s own row
stated for that description having been removed by this release's own
install path. **What it does not fix**: `§10.51` (opened by this
release — `SMTP_PORT`/`SMTP_TLS_MODE` discard a malformed value
silently where `PEISEAR_SETUP` refuses one by name); `§10.50` (no CI
job compiles for Windows outside the release tag); an attended start
unproven on Windows; PostgreSQL's one-version range; the wizard
writing a SQLite `.env` only; the container image's wizard unproven
inside the image itself. `CHANGELOG.md`'s own `[0.59.0]` section
carries this in full. **Cut 2026-10-31**: tag `0.59.0` at `602ae80`, signed
(`gpg: Good signature`), its message carrying the tag-pinned
release-notes link; **seven crates on crates.io** at `0.59.0`, none
yanked, each confirmed by its own per-crate registry query rather
than from the publish command's exit code — which mattered here,
because `peisear-auth` and `peisear-i18n` published on one line
together, so six `Published` lines cover seven crates and the line
count would have read as five; **five per-platform archives on the
GitHub Release**, read from the asset list and matched against
`install.md`'s own target table with no extra and none missing, the
`x86_64-pc-windows-msvc` `.zip` among them; `ghcr.io/nabbisen/peisear`
at `0.59.0` and `latest` under one digest (`sha256:0a5a7474`), with
`0.58.1`'s own digest unchanged (`sha256:1057c422`). All four
`DEC-056` legs verified against the registries themselves rather
than from their workflows' own exit codes, and the archive count read
from the asset list — the check `0.58.0` taught, whose own record was
one sentence away from claiming five when the Release carried four.

**Shipped 2026-10-29**: `0.58.1` (the terminal nobody is reading) —
`RFC 020` §7 (amended), `§10.49`, in two series rounds against two
scoped (`TTY-001`, `TTY-002`; `TTY-002` earning a round of its own
after its first review found the fix proven by hand but not yet
guarded). `should_run_wizard`'s own predicate asked whether a
terminal was attached, not whether anyone could answer it:
`stdin_is_terminal` replaced with `can_prompt` — `PEISEAR_SETUP`
first, then a terminal, then, on Unix, the kernel's own
foreground-process-group test (`tcgetpgrp` against `getpgrp`, the
same comparison it makes before raising `SIGTTIN`). The same cause
cost `0.58.0` its own Windows archive (`Start-Process`'s console is a
terminal nobody will ever type into); the release walkthrough is now
two cases named for the property each proves — an unattended start
serves, an attended start prompts — and `test.yml` gained case 7, the
first automated check for this entry's own defining case, seen red on
a planted predicate and green at the commit that shipped.
**`0.58.0`'s own `CHANGELOG` claim was wrong in a third shape**,
corrected forward in `[0.58.1]` rather than rewritten. **Nothing a
configured instance runs moved** — the predicate change is reached
only where no configuration exists, the same cases `WIZ-001`'s own
suite already proved untouched. **What it does not deliver**: an
attended start prompting stays unproven on Windows, five diagnosed
attempts named in `release.yml`'s own comment; no CI job in this
repository compiles for Windows outside the release tag, carried as a
finding for the architect's own next proposal. `CHANGELOG.md`'s own
`[0.58.1]` section carries this in full. **Cut 2026-10-29**: tag `0.58.1` at `60a0684`, signed
(`gpg: Good signature`); seven crates on crates.io, none yanked;
**five per-platform archives on the GitHub Release**, one per target
in `install.md`'s own table, the `x86_64-pc-windows-msvc` `.zip`
among them; `ghcr.io/nabbisen/peisear` at `0.58.1` and `latest`
under one digest (`sha256:1057c422`), with `0.58.0`'s own digest
unchanged. All four `DEC-056` legs verified against the registries
themselves rather than from their workflows' own exit codes — and
the archive count read from the asset list, which is the check
`0.58.0` taught: that release's record was one sentence away from
claiming five when the Release carried four.

**Shipped 2026-10-28**: `0.58.0` (the first thing it says) — `RFC 020`,
in five rounds against three scoped (`WIZ-001`–`WIZ-005`, two of them
added under the standing exception after reviewing the shipped wizard
against the owner's own philosophy, each time finding a real defect).
A first run with no configuration asks two questions with working
defaults — where the database lives, what address to listen on — and
makes three statements — the generated signing key and where it is,
the cookie flag and how to turn it on over TLS, the `.env` written and
where — then writes the file it describes, so the file and the
explanation cannot disagree. Skipped where a prompt would be an
outage **in the two shapes it was tested in** -- no terminal, or
configuration already exists -- proven by a CI walk including a real
pty. **It is not skipped in a third shape, and that is a defect this
release shipped**: see the cut record below. **`RFC 020` §2's own test of itself,
answered honestly**: one of the three defects that motivated this RFC
is prevented — `§10.40`, a copied `.env.example` placeholder, cannot
happen on a path that never copies that file — and that defect was
already closed in 0.53.0, so what this release adds is that the copy
step is no longer needed, not a fix for something open. The other two
are untouched: documentation drifting from the binary's own output is
a prose/code gap a wizard does not read or write, and an install
producing no assets is a packaging defect a wizard runs too late to
see. **Nothing a configured instance runs moved** — the piped and
terminal/configured CI cases assert byte-identical startup output to
before this release, and `WIZ-001`'s own predicate and `WIZ-004`'s
`listening`-wait were each seen to fail on a planted regression at
least once this series. **What it does not deliver**: seven `SMTP_*`
variables remain documented nowhere operator-facing, only in
`peisear-notify`'s own rustdoc; the wizard is proven on a GitHub
runner's pty, not inside the published container image, which is the
one place `WIZ-001` said the difference matters most; the two
questions cover two of the four variables `Config::from_env` reads —
`JWT_SECRET` is generated, `COOKIE_SECURE` a commented statement, both
deliberately not configured by a question. `CHANGELOG.md`'s own
`[0.58.0]` section carries this in full.
**Cut 2026-10-28, incompletely**: tag `0.58.0` at `54512ba`, signed;
seven crates on crates.io; `ghcr.io/nabbisen/peisear` at `0.58.0` and
`latest` (one digest, `sha256:4bd0dcde`, under both tags). All four
`DEC-056` legs verified against the registries themselves rather than
from their workflows' own exit codes. **Four per-platform archives,
not five.** `docs/getting-started/install.md`'s own table offers
`peisear-<version>-x86_64-pc-windows-msvc.zip`; the `0.58.0` Release
does not carry it, and `0.56.0` and `0.57.0` both did. This is
the class `§10.41` names — an install path the artefact cannot
satisfy — recurring in a new shape, and a platform regression. (That
entry is **closed** and stays closed; it was already ruled
not-reopened for an earlier recurrence. The wording here first said
"`§10.41` ... live", which cited a closed entry as open — corrected
2026-10-29.)

*Why it is missing, and the defect behind it.* The `Release` workflow's
Windows job built and packaged the archive, then failed its own
walkthrough: `Start-Process` gives the child a fresh console, so
`stdin` is a terminal, so the wizard ran, so the process sat at *Where
should the database live?* until the health check gave up after thirty
seconds. The upload and attach steps never ran. **The cause is not
Windows.** Measured twice on Linux at this tag, in an empty directory
with no `.env` and none above it, started the way an operator starts a
server -- `./peisear > out.log 2>&1 &` from an interactive shell:
`STATE=T`, `WCHAN=do_signal_stop` (stopped by `SIGTTIN`), nothing
listening on 3000, no `.env` written, and the only trace a question in
a log file. `should_run_wizard` is `stdin_is_terminal && !configuration_exists`,
and `stdin_is_terminal` answers *is a terminal attached*, not *can a
human answer now*. A background job, and a detached Windows console,
are both terminals nobody is reading. There is no escape hatch: no
flag, no environment variable -- the binary parses no arguments at all
-- so an operator who meets this has no documented way past it but to
answer in the foreground or hand-write `.env`, and nothing on screen
says so. Registered as `§10.49`; `0.58.1` is proposed above.

*The commit that first wrote this paragraph's place in the file is
titled "0.58.0 released", which at the time it was not; the title is
pushed history and is corrected here rather than rewritten.*

**Shipped 2026-10-24**: `0.57.0` (the specification checks itself) —
`§10.42`'s own argument, acted on, in five rounds (`SPEC-001`–`SPEC-005`,
two of them added after the series was scoped as three). `spec_scan`
reads `requirements.md` and `external-design.md` mechanically for the
first time: the two citation rules, Appendix A checked against its own
rows and against the 168 real entries, the status vocabulary closed,
and eight test-count citations held to the runner. **The first reading
found more wrong than expected**: thirteen dangling `§10.N` citations,
nine stale Appendix A areas, a vocabulary with two spellings of one
status and no column for another, and three wrong test counts of
eight — all four found and fixed in this release. **Nothing a user
runs moved** — every file this release touches is the guard module
itself, its own tests, `test.yml`, and the two specification documents.
**Cut 2026-10-24**: tag `0.57.0` at `12c94a2`, signed; seven crates on
crates.io; five per-platform archives on the GitHub Release;
`ghcr.io/nabbisen/peisear` at `0.57.0` and `latest`. All four
`DEC-056` legs verified against the registries themselves rather than
from their workflows' own exit codes.

**Shipped 2026-10-17**: `0.53.0` (the first five minutes), all four
`DEC-056` legs, seven crates on crates.io, five archives on the
Release and `ghcr.io/nabbisen/peisear` at `0.53.0` and `latest`.

**Shipped 2026-10-22**: `0.56.0` (the conformance suite and the
operations story) — `RFC 018` P4, in six rounds (`CON-001`, `CON-002`,
`CON-005`, `CON-006`, `CON-003`, `CON-004`; scoped as four, two more
earned rounds of their own after production defects the series' own
widening surfaced). Both backends now green on `peisear-storage`'s own
suite and the 37 lock tests, in CI, on every push; `NFR-CMP-003`
restated for the SQLite backend. **Nothing an existing SQLite operator
runs moved** — `DEC-007` at 515 unchanged across the whole series,
`migrations/` byte-identical to `0.55.0`, walked from a real `0.55.0`
install. **What it does not deliver**: PostgreSQL is still not a
supported configuration — 298 of `peisear-web`'s 335 integration tests
have never run against it, among them the only tests proving two
**P0** privacy controls; `CHANGELOG.md`'s own `[0.56.0]` section states
this plainly, as the release whose subject is a second backend should.
**Cut 2026-10-22**: tag `0.56.0` at `d259dcc`, signed; seven crates on
crates.io; five per-platform archives on the GitHub Release;
`ghcr.io/nabbisen/peisear` at `0.56.0` and `latest`. All four
`DEC-056` legs verified against the registries themselves rather than
from their workflows' own exit codes.

**Shipped 2026-10-20**: `0.55.0` (a second backend that runs) —
`RFC 018` P3 and the queries its own phase table had no home for, in
four rounds. All four `DEC-056` legs: seven crates on crates.io, five
archives on the Release, `ghcr.io/nabbisen/peisear` at `0.55.0`.
**Nothing an existing SQLite operator runs moved** — the migration
count was 22 before the candidate and 22 after, walked from a real
0.54.0 install. **What it does not deliver**: `peisear` has no
`[features]` section, so every archive, the image and
`cargo install peisear` build SQLite only; the backend is real code
reachable by building `peisear-storage` yourself, and `RFC 018` P4 is
where the conformance suite and the operations story land.

**Shipped 2026-10-18**: `0.54.0` (the dialect seam), `RFC 018` P1 + P2
— coded trigger errors (`0022`, matched by bracketed code rather than
prose), the `postgres` feature publishing for the first time (no
implementation behind it yet; `RFC 018` P3 is where one lands), and
`docs_scan` rule 7. All four `DEC-056` legs: seven crates on crates.io,
five archives on the Release, `ghcr.io/nabbisen/peisear` at `0.54.0`
and `latest`. **The first release this project has ever asked an
existing database to upgrade** — `0022`, walked from a real 0.53.0
install from crates.io before the tag and again independently at
review, with the original session's cookie still authenticating after.

*This sentence was written by the candidate, before the legs ran, and
is true only because they then succeeded.* `0.53.0`'s note two
paragraphs down records the same thing going the other way. **A row
leaves, and a "shipped" line lands, when the release is cut** — and
this is the second release where the candidate did it at the
candidate. The next `REL-` handoff states it as an exit condition
rather than a warning in prose.

**Its row was removed on 2026-10-15, restored on 2026-10-17, and
removed again when the release was actually cut.** The candidate removed it under exit criterion 8, which
says *as of the tag* — and legs 1 and 2 did run, so the tag exists.
But `DEC-056` is four legs and leg 3 failed (`§10.44`): nothing was
published and no Release was created. **A row leaves when the release
is cut, not when the tag is written**, and criterion 8 is reconciled
at the cut rather than at the first leg. Recorded rather than quietly
restored, because for two days this table said a release had shipped
that had not — and the second removal, above, is the one that was
earned.

**Why `RFC 017` moved off 0.52.0, kept for the record.** `RFC 018 §8` was written when the
answer was *"0.51.0 is documentation accuracy and 0.52.0 is `RFC
0017`"*. 0.51.0 then found two things it could not absorb: a helper
five guards had copied naively, so that `touch_target_scan` — a guard
`NFR-A11Y-007` depends on — reported clean on five files it could not
read, and a rule enforcing the spelling of what it bans rather than the
thing. Both are guard infrastructure, both were named in 0.51.0's own
changelog as carried, and **a guard that cannot see its subject should
not queue behind a release about installation.** The cost is named
rather than hidden: every row below moves one release later, and
`RFC 018`'s P1 now waits three releases longer than it needs to rather
than two.

## Release cycle

### Entry — before implementation starts

1. Governing RFC is in `rfcs/accepted/`. **An RFC may be as short as its
   decision** — nothing here requires the length of a large one, and a short
   RFC in `accepted/` meets this criterion exactly as a long one does.
2. Its open questions are resolved, or defaulted and recorded.
3. Handoffs exist under `rfcs/handoffs/NNN-slug/`.
4. The previous release's requirements amendments have been applied.

**Criterion 1 was unmet and unmentioned for six consecutive series** —
`063`–`068`, the releases `0.65.0` through `0.70.0`, which cite no RFC
anywhere in their directories and nowhere waive the criterion. All six were
the architect's. **And the practice is older and wider than those six, which
is the part worth being accurate about**: `rfcs/README.md` labels **19 of the
41 handoff series its index lists "Not RFC-governed"**, openly, going back to
series `013` — and that index **stops at `036` while 75 series directories
exist**, so it describes well under half of them and the real proportion is
unmeasured. So what was new in `063`–`068` was the **silence**, not the practice —
and the ruling below ends the practice itself from series `069` on, which is
why the check is floored there rather than applied backwards. **The owner settled the question it raised on 2026-11-26:
an RFC is required; a register entry may not govern a series.** The reasoning
is the gate's purpose — criterion 7 is the owner's gate *after* the work, and
criterion 1 is his only gate *before* it, so admitting a document the
architect writes himself alongside the work would leave him his own authority
before the fact. *What ships next*'s column header, which read **Governing
document** and was the entire surface of the disagreement, now reads
**Governing RFC**.

**And the criterion is to become machine-checked rather than remembered**:
a series directory's `README.md` must name a path under `rfcs/accepted/` that
exists, **floored at series `069`** so that `063`–`068` stay on the record
rather than being papered over by a baseline. **Scheduled, not yet written** —
see *Unscheduled work*. This follows the same ruling as the withdrawn
criterion 9 below: a guarantee belongs in a job that runs itself.

### Exit — before the release is cut

1. Every handoff reviewed and accepted, and committed as **one commit per
   handoff, in dependency order**. Splitting review units without splitting
   commits leaves the split notional — the reviewer cannot tell one unit's
   work from another's.
2. `cargo fmt --check`, `cargo clippy -D warnings`, and the full
   per-crate test suite pass, **with captured logs**.
3. Requirements amendments applied; nothing left recorded
   `Divergent` without a compliance-gap entry.
4. `CHANGELOG.md` records the *why*, not only the what.
5. Affected documentation updated in the same release.
6. Tarball built and extraction verified.
7. Release recommendation issued; **owner approves**. **This is the cut
   approval, and it is not the same decision as authorizing the row in
   What ships next** — see the note above that table.
8. **What ships next** reconciled — the shipped row removed, the
   remaining rows' numbers and blockers correct as of the tag.
*There is no criterion 9.* One was drafted on 2026-10-15 — *walk the
documented start before every tag* — and **withdrawn before first use,
on the owner's reading of this project's own philosophy**. The property
it wanted is real and is now `NFR-REL-011`, enforced by CI rather than
by a person: a reader following a written instruction must reach a
working instance. **A guarantee belongs in the specification and in a
job that runs itself, not in a checklist someone remembers.** The
reasoning, including what the first draft got wrong, is in that
entry.

Criterion 2 is the one that has historically failed. It is a gate,
not an aspiration.

## Unscheduled work

Known rough edges on things already shipped — not a handoff yet, and
not a deferred future feature (`§11` of the requirements specification
covers those: features accepted in principle but not built, a
different thing from a known imperfection in what already ships).
Each line: what, why it isn't scheduled yet, and whose call it is.

| What | Why not now | Whose call |
|---|---|---|
| **POST landing on long pages — the calendar's half only.** A mutation that redirects to a page the acted-on subject has **left** lands the reader at the top with the subject nowhere in view. **Measured** (`POST-001`, 0.48.0): the calendar Move loses 231 px of a 1289 px view **and the block itself**. *The sprint-plan half is fixed* — `NAV-002` (0.48.0) appends a fragment to the row's own id, so after a move **the row you acted on is on screen** (measured: `scrollY` 1292 → 2177, the browser clamping to the page end with the row in view). **That is a different promise from *your scroll position is preserved*, and deliberately so**: restoring 1292 would return the reader to where the row no longer is. | **A fragment cannot fix the calendar case** — it targets an element, and the subject is not on the landing page. `CAL-006`'s flash naming the new day is already that case's remedy, so what remains is only the **residual** scroll loss when a calendar mutation leaves its subject in view — narrower than what was measured, and unconfirmed. | Owner, whether the residual is worth anything at all. |
| **A dependency contains code a future Rust will reject, and nothing fails on it.** `cargo` reports `proc-macro-error2 v2.0.1` as *"code that will be rejected by a future version of Rust"* on every build, including both clippy invocations CI runs. It reaches this project **transitively through `leptos 0.8.19`** — `leptos_macro` and `reactive_stores_macro` both depend on it — so it is not a direct dependency and not removable here. **Found 2026-11-18 while running `0.64.0`'s own criterion-2 gates and reading the log rather than the exit code**, and recorded in no document before that. **`-D warnings` cannot catch it**: it is emitted by `cargo`, not by `rustc` as a lint, so both clippy commands exit 0 with the warning present — a warning whose guard's name suggests it would be caught and is not, which is `§10.36`'s own class on a build tool rather than a check. | **Not ours to fix, and the remedy named here was wrong — measured 2026-11-18 against crates.io rather than assumed.** `leptos_macro` **did** drop the dependency, at **0.8.20** (shipped in `leptos 0.8.23`, 2026-10-31: 22 normal dependencies became 21, the one removed being exactly this crate). **It still reaches the graph by two other paths**: `reactive_stores_macro 0.4.2`, and `syn_derive 0.2.0` — which is `syn_derive`'s **own latest published version** — so **no `leptos` upgrade silences it** and nothing in this project can. Nothing breaks today and the rejection is a future Rust's, not the pinned `rust-version = "1.88.0"`. Worth watching rather than scheduling: the honest action now is that it is written down, since the thing that made it invisible for an unknown number of releases was that nobody had read a passing gate's log. `cargo report future-incompatibilities` names the detail on demand. | Architect's, to raise again if a `leptos` upgrade lands or a Rust release sets a date. |
| **117 requirement entries citing no mechanism at all.** `REQ-004` (0.46.0) audited the 48 of 165 entries that cite a mechanism; the other 117 were its denominator, not its subject. Whether any of the 117 hide a `§10.35`-shaped gap — acceptance narrower than the requirement — is unaudited, not cleared. | Deliberately not commissioned: `REQ-004` was scoped to the entries it could audit without a second investigation's worth of work. | Owner, to decide whether and when to commission the audit. |
| **Two call-site counts with no stated population.** `NFR-A11Y-007`'s own 0.52.0 amendment says `grow(` has **171** call sites and `grow_text(` **41**, as the measured bound on `alignment_scan`'s coverage. `DISC-001` moved three sites between them 2026-11-12, and re-deriving the figures showed the real problem: **three measurements of the same quantity disagree** — 41 (the entry), 36 (grepped over `src/components/`), 49 (over `src/`) — because the entry never records which files it counted. Decrementing by three would make a wrong number differently wrong. | The fix is to state the population and then re-derive, which is a round with a guard rather than an edit; nothing reads these figures today, so they are historical rather than load-bearing | The architect's |
| **Three rules the owner settled on 2026-11-26 whose checks are not written yet.** (a) **Entry criterion 1** — a series directory's `README.md` must name a path under `rfcs/accepted/` that exists, floored at series `069`; unmet and unmentioned for six consecutive series before anyone read it. (b) **`changelog_scan` rule 4 widened to archives**, plus the one-time repair of the **17** dead links in `changelog/` (12 pinned to the tag whose entry cites them, 5 de-linked because the path never existed) and `DEC-055`'s one new sentence permitting a link-only correction. (c) **`changelog_scan` rule 6 widened by one clause** — the archive may hold the workspace version's dated section **when that version is already released, a real `git tag`** — which is the whole of what made *"in its own commit"* impossible to keep green. **The decisions are made; what is missing is the code.** | **Each needs a plant, and this project does not ship a rule it has not seen fail.** Three scan rules with their planted tests, a 17-link repair and two document edits are a round, not an edit — and `069` is open on a P0 authentication defect, which is not the series to widen. Nothing is blocked meanwhile: (a) is met by `069` itself once `RFC 024` is accepted, and (b) and (c) only bite at the next rollover, roughly ten releases away. | **Architect's, to bring as its own series** — the rules are the owner's and already given. **Scoped, re-measured and then deliberately *not* made a release**, 2026-11-27: the figures here were re-derived at source and the carried 17-link count corrected to **18** (12 root-relative, **6** naming a path the repository never had), but the `0.72.0` row that held them is withdrawn on the owner's philosophy review — see the paragraphs under *What ships next*. **Three rounds, split by what they are**: `changelog_scan` and `DEC-055` finished (rules 4 and 6, the repair, the one new sentence) is one; entry criterion 1's scan is another, and it must assert its own population floor because **the population at the `069` floor is 1** (of 75 series READMEs, exactly 2 name an accepted RFC). Each rides along with a release whose theme is user-facing. |
| **Two normative rows about the flash carrier that no check reads, and both were wrong.** `external-design.md` `§5.3`'s `Role` column read `role="status"` for `?flash=`, and `§3.1`'s region-obligation row read *`role="status"` or `role="alert"`, `alert` reserved for immediate errors*. **Neither describes either renderer**: the authenticated shell's flash is `role="alert"` with `alert alert-info` — on an ordinary success flash, which that reservation forbids — and the public shell's, which `0.71.0` now routes a security notice through, carries no role at all. Found by `0.71.0`'s own `DEC-028` sweep, 2026-11-27, and **corrected there**; `§10.36` instance 11 is the durable record. | **The rows are true now; what is absent is anything that would catch the next drift.** A check has to read a *rendered* role out of two components, which puts it in `contrast-gate.mjs`'s family rather than a source scan — and this project does not ship a rule it has not seen fail, so it needs a plant. That is a round, not an edit at a cut, which is the same reason the three rules above are waiting. Nothing is wrong today. | **The architect's**, as its **own** small round rather than folded into the changelog rules above — a rendered role is `contrast-gate.mjs`'s family, not a source scan, and bundling it with changelog rules was one of the three flaws the `0.72.0` philosophy review found. |
| **The Pages deployment has not succeeded since 2026-11-18, and the harm the architect claimed for it does not exist — measured 2026-11-27 after the owner asked what it was for.** **The site is up**: `https://nabbisen.github.io/peisear/` serves **HTTP 200**, content frozen at `bb79b15`, 2026-11-18T07:26:34Z. **What is actually stale, measured as a diff rather than inferred from a date**: three files — `changelog-and-releases.md` (62 lines), `external-design.md` (147) and `requirements.md` (992, almost all of it this architect's own Appendix C rows and register entries). **`docs/getting-started/` is byte-identical to `HEAD`'s**, so **nobody following the published install instructions is getting anything wrong**. **Every `github-pages` deployment since reaches state `error`** with an empty description within about two seconds — six consecutive ones read individually, `c42171d` through the `0.71.0` release commit `f235c61` — and `docs.yml`'s own recorded diagnosis no longer describes that: it documents a deployment stuck in `waiting`, and these do not wait. **Measured 2026-11-27**: `https://nabbisen.github.io/peisear/` serves **HTTP 200**, so the site is up — and its content is frozen at the last successful Pages deployment, **`bb79b15`, 2026-11-18T07:26:34Z**, which predates both `0.70.0` and `0.71.0`. **Every `github-pages` deployment since has reached state `error`**, with an **empty description**, within roughly two seconds of creation — six consecutive ones read individually, `c42171d` through the `0.71.0` release commit `f235c61`. **What is not the cause, each checked rather than assumed**: Pages' source is correctly `build_type: workflow`; the `github-pages` environment's branch policy allows `main` (one custom policy, `main`, type `branch`); and `mdbook build (with link check)` succeeds on every commit, uploading its artifact. **And `docs.yml`'s own recorded diagnosis no longer describes what happens.** Its comment documents a deployment stuck in `waiting` — the condition that made the lock move off workflow level on 2026-11-21. These deployments do not wait; they error. | **Low, and the first write-up of this row overstated it three ways; this is the correction.** **(1) The trigger was real and is already fixed.** A deployment stuck in `waiting` blocked the **link check** on every later commit for three days, which is why the `pages` lock moved off workflow level on **2026-11-21**. That change worked: `mdbook build (with link check)` has been green on every commit since. **What was carried into two release candidates afterwards is the residue of a solved problem.** **(2) `NFR-REL-011` was the wrong citation.** That entry is explicitly about CI exercising the documented install paths — *“the check is a job doing it, not a person walking it before a release”* — and says nothing about a published site. **(3) The site is not the reader's route.** `README.md`'s own Documentation section points at `docs/README.md` **on `main`**, which is current by construction; three files in the repository mention the Pages URL at all, two of them this row and the handoff that built the site. **And `DOCS-001`'s own exit condition shows where the value sits**: *serves the documentation tree, built from `main` by a workflow that fails on a broken internal link*. **The second half is the load-bearing one and it works.** So: worth repairing when convenient and worth telling GitHub about; **not urgent, not user-facing, and not a release item**. **The diagnosis was then done rather than filed, 2026-11-27, and four causes are eliminated by measurement** — recorded here because the eliminations are the durable part and whoever looks next should not repeat them. **Not the Pages source**: `build_type` is `workflow`, which is correct. **Not the environment's branch policy**: one custom policy, `main`, type `branch`, and these deployments come from `main`. **Not the build**: `mdbook build (with link check)` succeeds on every commit and uploads its artifact. **And not a deployment stuck holding the `pages` concurrency group**, which is what `docs.yml`'s own comment documents and what the architect expected to find: all **30** most recent deployments were read individually and **every one reaches a terminal state on its own** — `error` — so nothing is holding the lock. **The deploy job never runs at all**: `gh run view --job --log` returns *log not found* for every one of the last seven, because the job is cancelled while still queued, so the error is upstream of any step this repository controls. **What remains is not reachable from the API or the workflow**, so it is the repository's Pages settings or GitHub itself. | **Split, and stated rather than assumed.** The architect's: read the deploy job's own logs on a run where it was not superseded, and re-derive `docs.yml`'s comment against what is actually happening. **The owner's**: anything inside the repository's Pages settings or a GitHub support request — settings are his, and the architect does not change them. |
| **A CI setup step whose failure surface is far wider than its purpose.** `test.yml`'s *Install a pg_dump/pg_restore matching the service container's own major version* runs `sudo apt-get update` under `shell: bash -e` to add the PGDG repository, so **any** apt source on the runner image failing kills the job before a test runs. **Measured 2026-11-27 on `e50769c`**: `packages.microsoft.com/ubuntu/26.04/prod` — a source **pre-installed on the GitHub runner image, which this project never adds** — returned **403 Forbidden**, apt read it as *“no longer signed”*, `apt-get update` exited **100**, and the `peisear-web and peisear-storage (postgres feature, real server)` job reported `failure` having executed **zero** tests. 49 of 50 jobs were green; re-running that one job passed, so the 403 was transient. **The step needs exactly one repository — the one it adds itself** — and today depends on every source the image ships. **A second instance of the same shape, same day, different job — recorded here rather than given its own row, because one occurrence each is not yet a pattern and two rows would imply one.** `wizard skip/run detection (WIZ-001)` failed on `2a81117`, a **markdown-only** commit, with `Error: pool timed out while waiting for an open connection` while starting the binary backgrounded with a terminal attached (case 7, exit code 7). **Classified by measurement rather than called flaky on one data point**: re-running that same job on the same tree passed — 50 jobs, 0 not-success — and the next commit, `45d6a88`, passed the same job first time. So the tree was never wrong and the job said it was. **Both of today's failures are a CI job reporting failure having proven nothing about the product**, one from a third-party apt source and one from a SQLite pool timing out under a start shape only CI uses. | **Not a `§10.36` instance, and the difference matters**: this check does not report success having examined nothing, it reports **failure having examined nothing**. It is loud, not silent, so it cannot be mistaken for a pass — which is why it waits rather than blocks. The remedy is one line, scoping the update to the file the step wrote (`-o Dir::Etc::sourcelist=sources.list.d/pgdg.list -o Dir::Etc::sourceparts=- -o APT::Get::List-Cleanup=0`), so the step's reach equals its purpose. **But it is a gate edit**, which by this project's own rule must be pushed and watched rather than reasoned about — and a push now would stale `REL-0.71.0`'s measured tree, which is the one thing a candidate cannot survive. | **The architect's**, one line in a workflow, to fold into the next round that touches CI. Not a round of its own and not a release item. |

Add a line here when a quality gap on already-shipped work surfaces
and has nowhere else to live. Not everything belongs in this table —
a gap that already has a clear fix should become a handoff instead of
waiting here.

---

## Historical detail and deferred notes

Shipped-feature histories, and options considered and deliberately
not scheduled. Kept because the reasoning is worth more than the
list — an option rejected for a recorded reason does not need
re-deriving later.

#### Test infrastructure (preparation, in 0.16.0 → 0.17.0)

- e2e basecost: `axum-test 20` workspace dev-dep, shared
  helpers in `peisear-web/tests/common/` (Rust convention),
  `tests/smoke.rs` covering the existing happy paths,
  `tests/auth_boundary.rs` and `tests/optimistic_lock.rs`
  with `#[ignore]`d test inventory that gets unblocked Phase
  by Phase as endpoints land.
- CI: `.github/workflows/test.yml` runs `cargo fmt`, clippy
  with `-D warnings`, each test crate in a separate runner
  job (the combined link step OOMs the default 7 GB runner),
  and `cargo build --workspace`.

#### List filter/sort future enhancements (post-Phase A)

Phase A Step 3 (0.17.0) ships a deliberately small filter/sort
vocabulary on the project-detail issue list: status, assignee,
and sort by priority / created / updated. The schema-less
`user_view_states.state_json` shape is designed to grow without
migrations — these are the candidates a future release should
consider:

- **Priority filter** — pair with the existing priority sort,
  e.g. "show only high+ priority items". Requires only a new
  query parameter and a `retain` predicate.
- **Due-date filter** — depends on Phase C adding the
  `due_date` field to issues (per spec §8.1). Once that lands,
  filters like "due this week" / "overdue" / "no due date"
  become natural.
- **Sub-issue visibility toggle** — Phase C introduces the
  parent/child issue hierarchy. The list view will need a
  toggle: "show top-level only" vs "flatten with indentation".
- **Multi-select filters** — e.g. "in_progress AND open" as
  one filter view. Requires the URL param to accept comma-
  separated values; the `apply_filter_and_sort` predicate
  becomes `contains` instead of `==`.
- **Saved query presets** — let a user name and re-select
  multiple filter combinations ("My this-week view"). This is
  a UI affordance over the same `user_view_states` storage,
  with the key extended from `project_issues:{id}` to
  `project_issues:{id}:preset:{name}`.
- **Sort direction toggle** — currently sort is one-way per
  key. Adding `?dir=asc|desc` is a small enhancement that
  doubles the available orderings.
- **Reset semantics revisit** — the current Reset link
  inherits the saved default rather than wiping it (rationale
  in 0.17.0 changelog). If user testing surfaces confusion,
  consider an explicit "Clear my saved default" affordance
  next to Reset.
- **Apply other lists**: extend the same scheme to
  `/teams/{slug}/sprints/{id}` issue table (Phase B/C scope).

#### Global search future enhancements (post-Phase A)

Phase A Step 4 (0.17.0) ships a deliberately simple global
search: LIKE `%q%` over project names and open issue titles,
fixed scope, no ranking. The endpoint surface is stable enough
that future enhancements can land underneath without breaking
the typeahead's JSON shape:

- **Result ranking** — currently results are ordered by
  `updated_at DESC`. A future revision could score by name
  match position (prefix > substring > fuzzy) and recency,
  but only after user testing surfaces a real ordering
  problem at scale.
- **Search descriptions / comments** — currently only project
  names and issue titles are searched. Description-body
  matches surface a long tail of false positives in early
  testing; revisit when issue templates settle and
  descriptions are more uniformly structured.
- **Search closed (done) issues** — Phase A excludes
  `status = done` from typeahead. A separate "Search archive"
  surface or a "Show completed too" checkbox on /search could
  open this up without polluting the live-work typeahead.
- **Search teams and sprints** — Phase A scope is
  project + open issue. Once Phase C lands sub-issues and
  the calendar, broaden search to those too.
- **FTS5 migration** — at higher data volumes (10k+ issues),
  LIKE goes from "milliseconds" to "noticeable". Migration
  to SQLite FTS5 with the trigram tokenizer (handles
  Japanese without a morphological analyser) is the natural
  upgrade. Plan the migration so the storage-layer API
  (`projects_by_name`, `open_issues_by_title`) stays the
  same — handlers and components shouldn't have to change.
- **`/` keyboard shortcut** to focus the navbar search input
  globally. A small UX win; deferred to Phase E (ABDD QA)
  where keyboard shortcuts are reviewed holistically.
- **Recent searches** — server-side per-user history with a
  small dropdown on focus. Useful for quickly re-running a
  filter; the storage shape (key/value JSON) is already
  there in `user_view_states`.
- **Search on mobile** — the navbar input is hidden below
  the `sm` breakpoint in 0.17.0 because it competes with
  the user menu for space. Phase E mobile QA will rework
  this (likely a drawer-mounted search).

#### Optimistic-lock future enhancements (post-Phase A/B)

0.17.0 ships optimistic-lock for issue and project
mutations using `updated_at` as the version field. 0.18.0
extends it to sprint / team / team-membership / capacity.
Beyond that:

- **Conflict-resolution UI** — today, a 409 surfaces an
  error page asking the user to refresh and re-apply.
  A future revision could render a side-by-side diff
  ("their version" / "your edit") and let the user merge.
  Useful for long-form fields like issue descriptions; less
  useful for status/priority where the right move is
  usually "their change wins, mine moves on".
- **Toast notification for conflict on direct-manipulation
  surfaces** — Phase D's drag-and-drop and inline edits
  produce 409s the user can't reasonably anticipate. A
  toast with "Refresh — someone changed this" is a better
  UX than an error page when the user didn't even click
  Save.
- **`/api/*` JSON conflict response** — the `IntoResponse`
  for `AppError::OptimisticLockConflict` currently always
  emits HTML. The structured shape from spec appendix
  E.3.3 (`{error, message, current_updated_at,
  entity_type, entity_id}`) is wired into the type already;
  Phase B/D will plumb an `ApiAppError` sibling type for
  `/api/*` mutation handlers so conflicts there return JSON
  for client-side handling.
- **Same-second race window** — SQLite's `CURRENT_TIMESTAMP`
  is whole-second precision, so two writes inside the same
  second land with equal `updated_at` values and the lock
  can't tell them apart. This is rare in practice but not
  zero. Mitigation options: (1) switch to `strftime('%s%f')`
  for sub-second precision in the trigger, (2) add an
  explicit `version BIGINT` column that increments on each
  UPDATE. The integration tests use a 1.1s sleep
  (`ensure_distinct_timestamp`) to side-step the issue;
  production traffic almost never sees it. Revisit if user
  reports surface concrete instances.
- **Lock for delete operations** — 0.17.0 doesn't gate
  DELETE endpoints (issue delete, project delete) on
  `client_updated_at`. Their contention model differs:
  the user clicked Delete deliberately, and a stale read
  on a row about to be deleted just raises NotFound on the
  next mutation rather than overwriting work. Adding the
  lock here would convert a NotFound into a Conflict, which
  is arguably more accurate but doesn't add user-visible
  value. Revisit if there's evidence that delete races
  cause real surprise.

### Workload fairness

A cluster of features that together let a team distribute work
without one person silently drowning.

- **Per-issue effort estimates** — *shipped in 0.3.0.* Storage migration
  `0002_issue_effort.sql` adds `effort INTEGER NULL CHECK (effort > 0)`
  on `issues`; `core::Issue` carries an `Option<i64>` field; web
  renders a `<select>` of Fibonacci-scale presets (1, 2, 3, 5, 8, 13)
  with `—` as "not estimated" on the new/edit forms, plus a compact
  badge on board cards and a column in list view.
- **Per-issue assignee** — *shipped in 0.4.0.* Storage migration
  `0003_issue_assignee.sql` adds `assignee_id TEXT NULL` with
  `ON DELETE SET NULL` against `users(id)`; new storage query
  `list_assignee_candidates` returns the eligible users for a project
  (today: the project owner; when team support lands, all team members);
  `core::Issue` carries an `Option<String>`; web renders an assignee
  selector on new/edit forms, a ghost badge on cards, and an
  "Assignee" column in list view.
- **Per-period capacity limits per assignee** — *capacity primitive
  shipped in 0.5.0; period support shipped in 0.12.0.* The
  `user_capacities` table with optional `period_start` /
  `period_end` is now the source of truth; see the 0.12.0
  CHANGELOG entry for the migration story.
- **Project-health score** — *shipped in 0.6.0.* Three indicators
  (Throughput, Oldest in-flight, Activity-14d) rendered as a strip
  on the project detail page, sharing a `HealthIndicator` palette
  (`Good` / `Watch` / `Concern`) with the workload chips. Pure
  SQL aggregates over `issues`, no migration. The deliberately
  *non*-shipped item is a single composite 0–100 score: forming
  one would require uncalibrated weights, and shown numbers tend
  to be optimised against rather than understood.

### Health & burnout extension (active)

Per the V2.1 brief, this is the current concentrated investment.
Phase 1 ships in **0.7.0** (composite project-health score, three
new health indicators, personal dashboard, WIP limits,
accessibility uplift). Phase 2 and beyond:

- **`issue_events` table** for precise transition history.
  *Shipped in 0.8.0.* Append-only log written transactionally
  alongside every issue mutation. Long-stale detection and
  personal estimation skew now use event-based dwell time
  with graceful fallback to the 0.7.0 calendar-time
  approximation for legacy issues.
- **`metrics_snapshots` table + nightly aggregation** to give
  trends. *Shipped in 0.9.0.* Background tokio task runs every
  six hours, captures one snapshot per active project. Trend
  uses the median of recent (7-14 day) snapshots as the past
  baseline. `Trend::Up { delta } / Down { delta } / Flat /
  Unavailable` rendered next to the composite score.
- **`user_burnout` storage + core module** sibling to
  `personal_metrics`, sharing the `HealthIndicator` palette.
  *Phase 1 shipped in 0.10.0; Phase 2 shipped in 0.11.0.* Four
  indicators live: sustained-overload streak, stalled-assigned
  streak, estimation drift trend (median dwell-per-point recent
  vs. older half of a 28-day window), and cognitive switching
  pattern (median pickups per active day). The first two have a
  `Watch` palette ceiling; the latter two render in neutral
  palette because they are descriptive rhythm, not warnings.
  No `Concern` branch exists in any classifier.
- **Period-scoped capacity.** *Shipped in 0.12.0.* New
  `user_capacities` table replaces `users.capacity_points`; rows
  have optional `period_start` / `period_end`. Periods may not
  overlap (application-layer enforced). `/settings` UI provides
  CRUD with a "Close on date" helper for closing open-ended
  rows when adding new periods. Today's effective capacity
  resolves through `user_capacities::effective_for_user`. The
  Load chip on `/me` shows a "(this period)" hint when the
  active row has period bounds. Migration is destructive: the
  old column is dropped after data is moved into the new table.
- **Sprints**: a `sprints` table with `(project_id, name,
  starts_at, ends_at)`, prerequisite for velocity-stddev,
  burndown lines, and per-sprint completion ratios. Now that
  period-scoped capacity (0.12.0) is in place, sprint rows can
  link to capacity rows by date range and the per-sprint
  capacity is naturally computed.
- **Roles**: manager / neutral-third-party scopes with their own
  `/me/{user_id}` (or aggregated dashboard) views. These arrive
  alongside the planned Team / organisation feature so the
  permission model lands once.
- **Notification surfaces**. *Shipped in 0.13.0 (Phase 1).
  Real email delivery shipped in 0.16.0.*
  `notifications` and `notification_preferences` tables back
  an inbox at `/notifications` (with the navbar bell + unread
  badge) and a preferences page at `/settings/notifications`
  (with smart defaults and a one-question first-login email
  prompt). Three channels through a `Channel`-shaped
  abstraction: in-app via audit row; **email via the
  wasm-smtp 0.9 family (real delivery as of 0.16.0)**;
  webhook still a log stub awaiting per-user URL UI.
  Edge-triggered detection with a 24-hour cooldown.
  Architecture: snapshot loop and dispatch loop are
  independent tokio tasks connected by
  `mpsc::channel<DispatchEvent>`. The dispatch pipeline now
  lives in its own crate, `peisear-notify`, so transport
  dependencies (SMTP today, webhook HTTP client tomorrow,
  AI digests later) don't bleed into the web crate's
  compile graph.

  **Email integration**: *Shipped in 0.16.0.* Real SMTP
  delivery via the wasm-smtp 0.9 family (`wasm-smtp` core,
  `wasm-smtp-tokio` Transport adapter, `wasm-smtp-cloudflare`
  kept in tree as future option, plus `mail-builder` for
  RFC 5322 / MIME composition). Both implicit TLS (port 465)
  and STARTTLS (port 587) are supported. SMTP credentials
  read from environment (operator territory). Graceful
  degradation when unconfigured: in-app channel continues
  working; email send attempts fail with a logged warning
  and the audit row records `dispatched_via` without
  `email`.

  **Phase 2 notification candidates (deferred):**
  - HTML email (`multipart/alternative`). `mail-builder`
    makes adding HTML straightforward when a use case
    appears.
  - Connection pooling (multi-message sessions on a single
    SMTP connection — `wasm-smtp` already supports this in
    its API; we'd refactor our send path).
  - Digest mode (bundle multiple notifications per day or
    per week into one email).
  - Per-team `From:` address override.
  - Webhook channel real implementation (per-user URL UI
    + outbound HTTP client).
  - `project_trend_decline` notification kind detection.
  - Per-page nav unread badge coverage (today the badge is
    in the navbar bell only).

The foundation laid in 0.3.0–0.7.0 oriented toward this
trajectory; Phase 2 (events 0.8.0 → snapshots 0.9.0 → user
burnout 0.10.0 → drift & switching 0.11.0 → period-scoped
capacity 0.12.0 → notification surfaces 0.13.0) realises the
core of it. The remaining items above (sprints, roles,
AI assistance) are now a sequence of thin layers on top of
the foundation rather than further structural work.

### AI assistant per user

A per-user helper that can summarise issues, suggest labels, and
draft responses. Lands as a new `peisear-ai` crate sitting
alongside the existing four and depending on `peisear-core` plus an
async HTTP client. The web crate wires it in via a toggleable panel.
Provider-agnostic by design (Anthropic, OpenAI, local models via
OpenAI-compatible endpoint).

### Inline editing and optimistic updates

*Deferred.* Two technical paths exist:

1. **Leptos hydration**: full reactive client. Requires a wasm32
   build target, a client-side bundle, and a substantial change to
   the way `peisear-web` ships assets. Highest fidelity, highest
   complexity. See [docs/guides/hydration-upgrade.md](docs/guides/hydration-upgrade.md)
   for the migration path that was sketched.
2. **HTMX-style hypermedia partials**: server returns small HTML
   fragments on edit. Single ~12KB JS dependency, no WASM toolchain,
   straightforward additive endpoints. Lower fidelity (no shared
   client state across cells), much lower complexity. Future-compatible
   with path 1 since the markup stays HTML-attribute-driven.

The decision between the two is postponed in favour of the
**Health & burnout extension** above, which has been raised to the
top of the work queue per the user's direction. Inline editing
returns to the queue once health/burnout is in users' hands.

## Medium-term

Bigger moves that require more scaffolding but are well-scoped.

### Search refinement (future revisit)

The Phase A search shipping in 0.17.0 is deliberately the simplest
working version: SQL `LIKE %query%` against `projects.name`,
`projects.description`, and open issues' `title` / `description`
columns; typeahead returning 8 hits, results page returning 50
with pagination. This is enough to make peisear navigable while
the project's scale remains in the "individual contributor +
small team" regime the product is designed for.

Three threads to revisit when the product or its scale changes:

- **Search engine**: SQLite **FTS5** is the obvious next step
  if `LIKE %…%` becomes too slow or misses too much. Adopting
  it adds three things: a virtual-table schema mirror per
  searchable entity, INSERT / UPDATE / DELETE triggers to keep
  the mirror in sync, and a tokenizer choice. The tokenizer is
  the real decision: the default whitespace tokenizer doesn't
  index Japanese (no spaces, so an entire issue body becomes
  one token), so Japanese support requires either the built-in
  `trigram` tokenizer or an external morphological analyzer
  (mecab / lindera). `trigram` is the lower-cost path and is
  what to plan for first, with the constraint that queries
  shorter than 3 characters won't hit the index. We also need
  to confirm Cloudflare D1's FTS5 support if peisear ever runs
  on D1 — sqlite extension support there has been a moving
  target. Not a near-term need; revisit when issue counts cross
  the order of 10⁵ per project, or when users start asking for
  ranked relevance instead of substring matching.

- **Search scope**: Phase A intentionally limits search to
  projects and *open* issues, not closed issues, sub-issues,
  sprints, or teams. The scope was chosen to keep the
  typeahead responsive and to match the most common query —
  "what was that issue about X". Revisit as user feedback
  accumulates: closed-issue search is the most-likely-asked
  extension; sprint and team search would matter once a single
  account has dozens of either. Sub-issue search becomes
  meaningful after Phase C lands the sub-issue hierarchy.

- **Result-count defaults & UI/UX**: typeahead 8 / results
  page 50 is a starting point chosen by gut, not measurement.
  Things to revisit once we have real usage signal: whether
  typeahead 8 is a useful preview or just truncation noise,
  whether the results page benefits from facets (project,
  status, assignee), whether keyboard navigation + recent
  history needs surface real estate beyond the current
  dropdown, and whether mobile typeahead deserves its own
  treatment rather than the desktop layout shrunk down. None
  of this is urgent; flagged so it isn't forgotten under
  "we shipped search, done".

### PostgreSQL backend

**`RFC 018`, accepted 2026-10-11, is the assessment this section used to
defer to a future RFC.** This is now the single place that assessment is
stated; `README.md` and `docs/operations/scaling.md` each carry one
sentence pointing here rather than repeating it.

**The shape: a feature flag, not a sibling crate.** A `backend` feature
flag on `peisear-storage`, swapping the `Pool` type alias between
`SqlitePool` and `PgPool`, with the dialect-specific SQL confined to one
module per backend. A sibling `peisear-storage-postgres` crate was the
other option on the table and is rejected: it would carry a second copy
of 137 query function signatures to keep in step with nothing but
discipline holding them together, the same shape this project has
already ruled against twice elsewhere.

**The reason is server stability and security first, extensibility
second** (owner's own ordering) — **not** scale. `NFR-PERF-001`'s 5–30
users on a single node is unchanged and PostgreSQL does not widen it.

- **What PostgreSQL genuinely buys**: point-in-time recovery and
  replication (SQLite has neither, at any configuration — the strongest
  item), failure isolation (the database is a separate process; an
  application crash, upgrade or replacement never puts the data file in
  the blast radius), and concurrent writers via MVCC (SQLite admits one
  writer at a time).
- **What it does not buy, and is available sooner without it**: crash
  safety (already there — WAL plus `foreign_keys`) and durability on
  power loss (`synchronous = FULL`, landed at 0.50.0, P0b below — no
  backend needed for either).
- **Security runs the other way on one box.** For a single-node
  self-hosted install, SQLite has no network surface, no credentials and
  no second daemon to patch; PostgreSQL adds all three. What it buys
  instead is *separation* — the data stops being a file the
  application's own user can copy wholesale — which matters once the
  database runs somewhere the application's host cannot read, and is
  largely theoretical on one box with both services and one `.env`.
  **The documentation must say separation and recoverability, never
  "more secure."** Encryption at rest is not a differentiator either way
  — neither engine has it; volume encryption is the control on both,
  before and after this lands (`SEC-001`, 0.50.0).
- **Extensibility, uncontested**: full-text search without `LIKE
  '%…%'`, `LISTEN`/`NOTIFY` for the notification dispatcher instead of
  in-process polling, richer types, read replicas.

**The contracts underneath the type boundary are implemented as of
0.55.0, and the triggers were the real work:**

- **Eleven `CREATE TRIGGER` statements** across four migrations carry
  two requirement families — `NFR-CONC-003` (`updated_at` has exactly
  one authority, seven triggers) and `FR-SUB-001..003` (sub-issue
  invariants enforced at the database layer, not only in application
  code, two triggers; two more validate planned dates). `FR-SUB-001` is
  explicit that application-level enforcement is not sufficient, so
  these were **ported, not replaced** (`PG-003`, 0.55.0) — two
  implementations of one invariant, on two engines, each verified by
  its own real-server tests; `P4`'s conformance suite is where the same
  tests run against both in CI as one suite, not yet done.
- `issues::translate_trigger_error` used to string-match SQLite `RAISE`
  message text to produce validation errors, which would not have
  survived a port in any form. Replaced by coded errors (`PG-003`) —
  `ERRCODE` on PostgreSQL, a bracketed prefix on SQLite — raised by
  both backends and translated on the code, not the prose.
- SQLite's `CURRENT_TIMESTAMP` **was** whole-second, which would have
  given the two backends different concurrency characteristics on the
  same code. **Closed already**: `0021` (`STORE-001`, 0.50.0) moved the
  lock token to millisecond precision before this backend work begins,
  so the divergence this bullet used to warn about does not carry
  forward.
- `NFR-CMP-003` requires state in a single file so that backup is a
  file copy. PostgreSQL does not remove that guarantee for SQLite
  users, but it does create a second supported topology with a
  different operational story — restated to say *for the SQLite
  backend* once a second one ships.

**Schedule**: P0 and P0b (the lock-token fix and the `synchronous`
measurement, both scheduled whether or not this backend ever ships)
landed at **0.50.0**. P1 (coded errors) and P2 (the dialect seam, no
PostgreSQL code yet) were originally slated for 0.53.0, then moved one
release later by the 0.51.0 guard-infrastructure delay recorded above
— P1/P2 **shipped at 0.54.0**; P3 plus the queries gap scoping found
(`PG-001`–`PG-004`) **shipped at 0.55.0**. P4 (the conformance suite,
both backends green in CI, the operations story rewritten) is
**0.56.0**. **SQLite stays the default and loses nothing at every
phase** — a condition of the acceptance, enforced at each gate.

**P4 does not make PostgreSQL a supported deployment, and that is
deliberate.** It makes the backend selectable at build time, proves
the storage crate's own suite and the optimistic-locking tests
against it, and writes the operations story. It leaves 298 of
`peisear-web`'s 335 integration tests never having run against
PostgreSQL — among them `aggregate_privacy` and `workload_privacy`,
which are the only tests anywhere proving `NFR-PRIV-007`'s
contributor floor and `NFR-PRIV-001`'s (**P0**) capacity
visibility, both enforced by SQL that `PG-004` rewrote per backend.
No release documents an install path while that is true, because
`NFR-REL-011` would then require CI to walk it and the walk would
certify an unverified P0 control.

**What would earn it — a P5, not scheduled**: the full
`peisear-web` integration suite against both backends, after which
a documented and walked `cargo install` path, and only then the
question of a second archive set and container tag. Writing it down
here so the step is taken deliberately rather than reached by drift
from a passthrough that already exists.

**P5 also carries the dead driver.** `peisear-storage`'s
`sqlite = []` is an empty feature: it gates `#[cfg(feature =
"sqlite")]` and not the dependency, because `sqlx`'s own `sqlite`
feature is unconditional in `[workspace.dependencies]`. Measured at
`f0cd461`, a `--no-default-features --features postgres` build of
`peisear` links the bundled SQLite C library — 387 distinct
`sqlite3_*` symbols in the binary — and needs a C toolchain to
compile it, on a build that will never open a SQLite file. It is
pre-existing and documented in `peisear-storage/Cargo.toml`'s own
comment; `CON-001` made it reachable rather than causing it, and it
harms nobody while no install path exists. **It stops being
cosmetic the day P5 documents one**, so it is a condition of P5 and
not a loose end.

### OIDC / IDaaS integration

Land the OIDC verifier alongside the JWT code in `peisear-auth`
behind a feature flag. The web crate grows an OIDC callback
handler; the rest of the architecture is unchanged. Supports
discovery, PKCE, and refresh flows.

**Requires a security review, not only a feature flag.** This
modifies the authentication boundary, and the authorization
boundary above it (`NFR-PRIV-003` — no administrator read path to
member personal data) is the project's central guarantee. Any
identity-provider integration must demonstrably preserve it,
including where the provider supplies group or role claims that
might be mistaken for authorization.

### Team model (Phase 1: flat teams)

The 0.14.0 design adopts the **Linear-style flat team** approach
discussed in design notes: optional teams that small users can
ignore entirely. `team_id` on projects is nullable (a project
without a team is a *personal project*, owned by its creator
just as today). Users can join multiple teams; each membership
carries a `role` (`admin` / `member` / `viewer`).

Role extensibility (future):

- The 3-tier `admin / member / viewer` set is what 0.14.0
  ships. The schema reserves `role TEXT` (not `enum`) so
  future fixed roles (e.g. `billing` for paid hosted
  scenarios, `security_manager` for compliance) can be added
  without migration.
- Custom roles (per-team named roles with selectable
  capability sets) are an open design question — Linear has
  no equivalent today; GitHub Enterprise does. We'd add
  `team_roles` + `team_role_capabilities` tables only if
  operator feedback justifies the complexity. Until then,
  the three fixed values cover the design intent.

Phase 2 candidates (deferred):

- **Sub-teams** (one-level parent / nested teams). Linear
  added this in 2025; we'd consider it after observing
  operator usage of flat teams. The data model already
  reserves `team_id` for this — adding `parent_team_id`
  later is non-breaking.
- **Per-team configuration**: cycle / sprint / label
  defaults inherited by projects in the team.

### Privacy & access control evolution

V2.1 §2.5 ("集計と個別を混同しない") sets the boundary that
0.14.0 ships: project-level metrics are visible to all team
members; per-user signals (burnout panel, personal dashboard)
are visible to that user only. The team admin role is
**political** (manage members, configure team settings), not
**surveilling** (read other people's burnout / dwell time /
streak data).

Future considerations on this surface:

- **Per-team privacy policy**. A team could opt for a stricter
  posture ("burnout panels are silent across the team")
  without needing user-by-user opt-out. Default is the 0.14.0
  posture (member workload visible, individual signals
  private); admins of a team might restrict further but never
  loosen below this floor.
- **Per-user privacy controls**. A user could opt to hide
  their per-user workload chip from team members (the
  workload section on a project page would render as
  "Hidden by user" rather than the load number). The
  in-flight points are still computed and the user still
  sees their own; what's suppressed is team-mates' visibility.
- **Anonymous aggregation surface**. A project's overall
  health composes individual workload signals; a future view
  could show only the *aggregate* without naming specific
  users (median/p90 instead of per-user chips). Useful where
  the team is large enough that individual chips create
  pressure.
- **Deletion + retention semantics**. Deleting a user
  currently `ON DELETE CASCADE`s their snapshots and
  notifications. We should document and surface this as the
  user's privacy guarantee: leaving a team is a clean
  forget. Today's behaviour is correct; the surfacing is
  what's missing.
- **Audit trail visibility**. `issue_events` records actor_id
  for every event; admins should not be able to retroactively
  reconstruct who did what beyond what the issue's
  history-pane already shows. A privacy-conscious
  admin-tooling layer would expose less, not more.

These are not 0.14.0 work — the Phase 1 floor (project
metrics public to team, individual private) is the design
this release ships against. They are **the privacy decisions
we will need to make as the team feature matures**, recorded
here so they don't get lost when the team feature does start
demanding answers.

### Exports and imports

CSV, JSON, and GitHub-compatible Markdown. Lands in `peisear-web` as
a cluster of new handlers; the heavy lifting (SQL → struct → format)
is storage + core.

### Server-migration support

A planned and supported way to move an entire peisear installation
from one host to another. Distinct from *Exports and imports*
above (which targets per-project data leaving the system in
neutral formats); this is about the operator picking up the whole
state and putting it down somewhere else.

The current story is "stop the process, scp the SQLite file and
the env vars to the new box, start the process there". That works
but is undocumented as a supported path and has sharp edges
around session continuity (JWT secret) and asset paths. Concrete
work to make this a first-class feature:

- **Bundled export.** A `peisear export <path>` subcommand that
  produces a single archive (`.tar.zst` is the obvious choice)
  containing: a hot-copy of the SQLite database via
  `sqlite3 ".backup"`; a manifest with the version, schema
  revision, and a checksum; and the contents of `static/` if it
  has been customised. Specifically excludes secrets — those
  must be redeployed out-of-band, see below.
- **Bundled import.** Mirrored `peisear import <path>` that
  validates the manifest version against the running binary,
  refuses to overwrite a non-empty database without a force flag,
  and applies the archive transactionally (move the existing
  file aside, write the new one, run pending migrations, swap
  back on success).
- **Secrets-out-of-band by design.** `JWT_SECRET` is *not* in the
  bundle. Migrating the secret moves all live sessions across
  with it; rotating it forces every user to re-authenticate.
  Both are defensible, neither is universally correct. The
  documentation surfaces the choice and the operator decides.
  Argon2 password hashes are in the database itself, so the
  password file moves with the export — that's correct.
- **Encrypted archive option.** For environments where the
  archive lives on intermediate storage (S3, a desktop, a
  thumb drive), an `--encrypt-with` flag taking a public key
  produces an age-encrypted tar so the archive is not readable
  in transit. Plain-text export remains the default for
  trusted-network migrations where encryption is overhead.
- **Cross-version compatibility window.** The manifest's schema
  revision lets the new binary know whether it can run pending
  migrations on the imported database (yes within the
  forward-compatible window; refuse with a clear message if
  someone tries to import a 0.12.0 archive into a 0.11.0 binary).
- **Documentation.** A new `docs/operations/migrate-host.md`
  walks through the full sequence including the secrets
  decision, with rollback notes consistent with
  [upgrade-runbook.md](docs/operations/upgrade-runbook.md).

The deliberate alternative to a bundle command is to keep saying
"the database file is the migration unit, here's how to copy it
safely". That's documented in `backup.md` today and works. The
case for shipping a dedicated tool is mostly that it removes the
foot-guns (forgot to stop the process; copied a partial WAL;
missed the static directory). Which path we pick depends on
whether operator feedback says "the manual way is fine" or "we
keep getting bitten". Currently leaning toward the bundle
command, but holding off until at least one operator has made a
real cross-host migration and reports back.

### Deployment guide expansion in `docs/`

*Shipped in 0.10.1.* `docs/operations/` now includes
`background-jobs.md`, `data-retention.md`, `upgrade-runbook.md`,
`observability.md`, and `scaling.md`. The operations README is
organised into "Day-one" (deployment, backup, Tailwind
self-hosting) and "Day-two" (the new five) sections.

## Long-term / vision

Directional commitments, not promises.

### CI/CD and IaC support

The `infra/` directory is the staging ground. Eventually:

- **`infra/docker/`** — scratch-based container image built in CI.
- **`infra/compose.yaml`** — end-to-end local environment including
  PostgreSQL once that backend lands.
- **`infra/terraform/`** — minimal IaC for a single-node VM
  deployment with TLS, backups, and health probes.
- **`infra/github/`** — GitHub Actions workflows for test, lint,
  build, and release.
- **`infra/k8s/`** — Helm chart or kustomize overlay for clusters.

### Pluggable backends beyond relational

Object storage for attachments, a search index (Meilisearch or
PostgreSQL full-text), maybe a message queue for outbound
notifications. Each would be a new crate alongside `peisear-storage`.

### Plugin interface

A thin stable contract — probably WebAssembly component model —
letting operators add custom fields, validations, or sidebar panels
without forking peisear itself.

## Out of scope

A few things peisear has deliberately decided not to become:

- **A swiss-army project suite.** No wikis, no calendars, no chat.
  If you need them, integrate them.
- **A SaaS.** peisear is self-hosted. There will be no
  peisear.cloud.
- **A mobile app.** The web UI is responsive; a native app isn't in
  scope.

## Contributing to the roadmap

If a feature you want isn't here, or if a priority seems wrong,
please open a discussion issue. Roadmap changes are public by
design — see [.github/CONTRIBUTING.md](.github/CONTRIBUTING.md).
