# peisear — External Design Specification

**Document type**: External design (basic design)
**Document status**: Baseline
**Covers release**: `0.38.0` (implementation through `0.38.0`)
**Supersedes**: [`history/peisear-0.19.1-external-design-en.md`](./history/peisear-0.19.1-external-design-en.md),
retained unedited as the record of that release
**Language**: English (normative)
**Prepared**: 2026-07-27 · **Amended**: 2026-09-24 (0.38.0)
**Location**: `docs/specification/external-design.md`. **Normative**, English
only, placed 2026-09-24 (`DEC-020`, closed). See
[the directory README](./README.md) for how its citations resolve

> **Amendment note (0.21.0).** This document was three releases stale. 0.20.0,
> 0.20.1 and 0.21.0 all changed externally observable behaviour and none of
> them amended it, so the gaps in §17 described defects that had since been
> fixed while §10.3 quoted copy the product no longer shipped.
>
> That is worth naming rather than quietly correcting. **An external design
> document that lags the product stops being a specification and becomes a
> second, unreliable description of it** — and this one was cited as normative
> in three handoffs while §10.3's "MUST NOT be paraphrased" text had already
> been paraphrased in shipping code, with a byte-identity test asserting the
> new wording.
>
> The lag is mine. Amending this document is now an exit criterion of the
> release cycle (`DEC-028`), not a task that waits for a quiet week.

> **Amendment note (0.28.0).** Amended at the release, the **third**
> consecutive time. `DEC-028` has now been met three times running after being
> missed at 0.22.0, 0.23.0 and 0.24.0.
>
> **§5.2 and §14 change for every screen**: the muted text tier's floor moved
> from `/60` to `/70`. Measurement, not preference — 111 sites were below WCAG
> AA, and `/60` passed on the card background by **0.04** while failing on the
> page background. The cost is stated where the design is: the muted tier has
> **two** visible steps of grey where it had four, so a caption that used the
> lightest step now reads like one that used a middle one.
>
> **§15 gains a second live region.** Conflict and success announcements shared
> one polite region, so an edit conflict competed with routine confirmations on
> equal footing. Two regions now, chosen by outcome — and `settings.rs`'s
> conflict block deliberately keeps `aria-live="polite"`, because it renders on
> a fresh page load and nothing ever mutates it.
>
> **§6's sprint screens lose a chart below two contributors** — the burndown
> entirely, the completed-work chart's median line only. The totals stay.
> **Nothing on the page explains why**, and that is the design: naming the
> reason would disclose exactly what the suppression withholds.
>
> **What this document must not be read as saying.** Contrast now meets AA.
> **Touch targets do not** — 139 controls remain below this project's 44 px
> target, measured. The design pass is 0.30.0's, and it is a design pass rather
> than a resize because raising them changes the density of a tool whose screens
> are dense on purpose.

> **Amendment note (0.27.0).** Amended at the release, the **second**
> consecutive time — `DEC-028` has now been met twice running after being
> missed at 0.22.0, 0.23.0 and 0.24.0.
>
> **§15 gains the four destructive deletes**, and the reason belongs to this
> document rather than to the requirements: **§17.4's own fix created the
> window.** A delete used to be one `POST` from a page the user was looking at.
> This document specified a `GET` that names the consequence, a pause while the
> user reads, then a `POST` — and nothing bound the second to the state the
> first displayed. Two of the four deletes had no lock; they do now.
>
> That is worth stating plainly: a screen this document designed to make a
> destructive action safer opened a concurrency gap it did not mention. The gap
> was small and is closed. The lesson is that adding a step to a flow changes
> what the flow can observe, and this document is where that should have been
> noticed.
>
> **§4.2 records a narrowed request contract** — the two delete `POST`s now
> require `client_updated_at`. **§10.3 gains the cascade note**: the issue
> interstitial names the sub-issues that go with the issue, which
> `ON DELETE CASCADE` has removed since migration `0015` without the screen
> saying so.

> **Amendment note (0.25.0-0.26.0).** Amended at the release. `DEC-028` made
> this an exit criterion of the release cycle at 0.21.0; it was missed at
> 0.22.0, 0.23.0 and 0.24.0, and named as overdue at each. This is the first
> amendment that did not have to open by apologising for its own lag.
>
> **§17.4 closes, and the position this document took on it was wrong in two
> ways.** It proposed the interstitial for all nine destructive actions; four
> got it, because the other five are reversible through the interface and a
> vanishing dialog costs nothing that can be undone. And it proposed retaining
> `confirm()` as an enhancement that skips a round trip; that was dropped,
> because two confirmation mechanisms for one action — with the safe one
> reachable only when the other fails — is the shape that produced the gap.
>
> Both corrections came from implementation, not from re-reading this
> document. An external design that is never contradicted by what ships is
> not being read closely enough.

> **Amendment note (0.24.0), and the criterion above was missed three more
> times.** 0.22.0, 0.23.0 and 0.24.0 each shipped without amending this
> document. I named it as overdue at every one of those releases and did the
> release work first each time.
>
> That is worth stating plainly rather than quietly catching up: **an exit
> criterion that is announced and then skipped is not a criterion, it is an
> intention.** It survived four releases as one. Whether it should be enforced,
> dropped, or replaced by something cheaper is the owner's call; what it should
> not be is asserted again unchanged.
>
> This amendment covers all three releases at once. Six reserved routes became
> real, one route was removed, two design gaps closed, and one "planned" fixed
> text is now shipped copy.

---

## 0. Document control

### 0.1 Position in the development workflow

The project workflow is:

```
Requirement definition (planning, RFC)
  → External design (basic design)          ← this document
    → Internal design (detailed design)
      → Program design
        → Implementation
          → Testing
```

This document defines **what the system presents to the outside world**:
screens, routes, forms, messages, interface contracts, and externally
observable behaviour. It does not describe internal structure — crate
layout, module decomposition, algorithms, and persistence mechanics
belong to internal design.

The test for whether something belongs here: *could a user, an
integrator, or a black-box tester observe it?* If yes, it is external
design. If it is only visible by reading the source, it is internal.

### 0.2 Relationship to other documents

| Document | Relationship |
|---|---|
| `docs/src/requirements.md` | **Upstream.** Every design element here traces to one or more requirement identifiers (`FR-*`, `NFR-*`). |
| `docs/spec/peisear-feature-spec-v2.1.md` | **Upstream, canonical.** Screen intent and rationale. Cited as `SPEC §N`. |
| GUI External Specification v0.1 | **Superseded by this document.** See §18 for the specific points where it has gone stale. |
| `rfcs/0001`–`0005` | **Downstream for unbuilt work.** Screens marked *Planned* here are specified in detail there. |

### 0.3 Conventions

- Screens carry identifiers `SCR-NN`. Planned screens use `SCR-P-NN`.
- Requirement references appear as `→ FR-ISS-004`.
- Normative keywords (MUST / SHOULD / MAY) follow RFC 2119.
- **Implemented** means present in `0.19.1`. **Planned** means specified
  and accepted but not built.
- Route parameters are snake_case, matching `GUI §8`.

### 0.4 Scope of this document

**In scope**: system boundary, actors, screen inventory and transitions,
URL design, per-screen external specification, form contracts, the JSON
API contract, HTTP status semantics, message and vocabulary catalogue,
externally observable background behaviour, accessibility obligations
per screen, responsive behaviour, and externally visible concurrency
behaviour.

**Out of scope**: crate and module structure, database schema, query
design, algorithms for indicator computation, rendering internals,
test implementation.

---

## 1. System context

### 1.1 Boundary

peisear is a single self-hosted web application. It has no
service-to-service dependencies at runtime beyond an optional outbound
SMTP relay.

```
        ┌──────────────────────────────────────────────┐
        │                                              │
  ┌─────┴──────┐   HTML over HTTP    ┌──────────────┐  │
  │  Browser   │◀───────────────────▶│              │  │
  │ (desktop / │   JSON over HTTP    │   peisear    │  │
  │  tablet /  │◀───────────────────▶│  (single     │  │
  │  mobile)   │                     │   process)   │  │
  └────────────┘                     │              │  │
                                     └──┬────────┬──┘  │
  ┌────────────┐   outbound SMTP        │        │     │
  │ SMTP relay │◀───────────────────────┘        │     │
  │ (optional) │                                 │     │
  └────────────┘                        ┌────────▼──┐  │
                                        │  SQLite   │  │
                                        │  (single  │  │
                                        │   file)   │  │
                                        └───────────┘  │
        │                                              │
        └────────────── deployment host ───────────────┘
```

| External entity | Direction | Protocol | Notes |
|---|---|---|---|
| Browser (human user) | in / out | HTTP, HTML | Primary interface. Server-rendered. |
| Browser (script) | in / out | HTTP, JSON | Type-ahead search; personal-data reads. |
| SMTP relay | out | SMTP | Optional. Absence degrades gracefully → `FR-NTF-008`. |
| SQLite file | out | file I/O | Single file; backup is a file copy → `NFR-CMP-003`. |

### 1.2 Runtime context

The system runs as one process serving all routes. There is no separate
API server, no background worker process, and no message broker.
Periodic work (metric snapshots, notification dispatch) runs inside the
same process and is observable only through its effects — new
notifications and updated indicator values.

### 1.3 Client contexts

| Context | Support level | Design consequence |
|---|---|---|
| Desktop browser | Full | Primary target. All screens and flows. |
| Tablet browser | Full | Same layout, wider touch targets. |
| Mobile browser | Confirmation-first | Four flows MUST complete on a phone → `NFR-A11Y-006`. Planning-style screens degrade to a read view. |
| No JavaScript | Degraded but functional | Every flow MUST work without scripting. Scripting only enhances (type-ahead dropdown). → §6.8 |

The no-JavaScript position is a deliberate external commitment, not an
accident of the rendering approach. The search box submits as a plain
form; the dropdown is an enhancement layered on top.

---

## 2. Actors and permission model

### 2.1 Actors

| Actor | Description |
|---|---|
| **Anonymous** | Unauthenticated visitor. |
| **User** | Authenticated account holder, acting for themselves. |
| **Project owner** | User who created a personal project. |
| **Team viewer** | Team member with read access. |
| **Team member** | Team member with write access to team content. |
| **Team admin** | Team member with management rights over the team. |

### 2.2 The administrator boundary

The single most important external behaviour in this system:

> A team administrator has authority over **team membership and sprint
> lifecycle**. They have **no** authority to view another member's
> personal sustainability data. No screen, no route, and no API
> response exposes it.

This is enforced at the interface contract level and returns `403` —
identically for "not permitted" and "target does not exist", so that
probing cannot distinguish them. → `NFR-PRIV-003`, `NFR-PRIV-006`,
`FR-API-003`.

### 2.3 Permission summary by data class

| Data class | Anonymous | Self | Other member | Team admin |
|---|---|---|---|---|
| Own personal sustainability data | ✗ | ✓ | ✗ | ✗ |
| Own settings | ✗ | ✓ | ✗ | ✗ |
| Own notifications | ✗ | ✓ | ✗ | ✗ |
| Project issues (accessible project) | ✗ | ✓ | ✓ | ✓ |
| Project health aggregates | ✗ | ✓ | ✓ | ✓ |
| Team membership list | ✗ | ✓ (if member) | ✓ | ✓ |
| Team membership mutation | ✗ | ✗ | ✗ | ✓ |
| Sprint lifecycle transition | ✗ | ✗ | ✗ | ✓ |

---

## 3. Screen architecture

### 3.1 Global layout regions

Every authenticated screen renders inside a common shell:

```
┌────────────────────────────────────────────────────────────┐
│ [skip link — first focusable element]                      │
├────────────────────────────────────────────────────────────┤
│ HEADER (banner)                                            │
│   brand · [search box: role="search"] · inbox · account    │
├──────────────┬─────────────────────────────────────────────┤
│ NAV          │ MAIN (main, id="main-content")              │
│  Today       │   ┌─────────────────────────────────────┐   │
│  Inbox       │   │ breadcrumb trail                    │   │
│  Projects    │   ├─────────────────────────────────────┤   │
│  Teams       │   │ page title · primary action         │   │
│  Settings    │   ├─────────────────────────────────────┤   │
│              │   │ flash / error region                 │   │
│              │   ├─────────────────────────────────────┤   │
│              │   │ page content                        │   │
│              │   └─────────────────────────────────────┘   │
└──────────────┴─────────────────────────────────────────────┘
```

Region obligations:

| Region | Element | Obligation |
|---|---|---|
| Skip link | `<a>` | MUST be the first focusable element and move focus to `#main-content`. |
| Header | `<header>` | Persistent. Carries search and unread count. **The row MUST fit the viewport at every width**: the account control shows the signed-in name and gives it up with an ellipsis only when the row cannot otherwise fit; nothing else in the row yields, and the control keeps its 44 px target (`LAYOUT-006`, `§17.8`). |
| User-supplied text | any | Titles, names and descriptions MUST NOT widen the page, however long an unbroken run they carry. Three shapes, three remedies — a flex or grid item carrying such text opts out of its content minimum *and* the text carries a break opportunity; an ordinary block needs the break opportunity only; a container that sizes itself to its text needs `overflow-wrap: anywhere` — recorded once in `components.rs` and observed by `§17.8`'s gate with a 64-character unbroken run in every user-text field of its fixture. |
| Search box | `<form role="search">` | MUST function without scripting (submits to `/search`). |
| Unread count | text | MUST be exposed as text, never colour alone → `NFR-A11Y-004`. |
| Page title row | `<h1>` beside the page's primary actions | MUST wrap rather than squeeze the title: when the row cannot fit both, the actions move below and the title keeps the full content width. No breakpoint — the row wraps when its content does not fit and not otherwise, and every control keeps its 44 px target (`LAYOUT-009`, 0.34.0). |
| Nav | `<nav>` | Current item marked `aria-current="page"`. |
| Main | `<main>` | Exactly one per page. |
| Breadcrumb | `<nav aria-label="Breadcrumb">` + ordered list | Present on all detail screens → `FR-NAV-003`. |
| Flash region | `role="status"` or `role="alert"` | `alert` reserved for immediate errors. |

### 3.2 Navigation model

`SPEC §4.2` calls for five primary entry points: Today, Inbox,
Projects, Teams, Search. These are realised as **four navigation links
plus a persistent search affordance in the header**, with Settings
additionally present as a navigation link.

| Entry | Realisation | Responsibility |
|---|---|---|
| Today | nav link → `/today` | "What should I look at right now, about myself?" |
| Inbox | nav link → `/inbox` | "What has changed that I should know about?" |
| Projects | nav link → `/projects` | "Where is the work?" |
| Teams | nav link → `/teams` | "Who am I working with, and on what cycle?" |
| Search | header search box → `/search` | "Take me to a specific thing." |
| Settings | nav link → `/settings` | Self-configuration. Not a primary entry in `SPEC` terms. |

*Design note.* Search is a box rather than a link because its purpose
is to accept input immediately; routing the user to a search page first
would add a step to every lookup. Settings appears in navigation
because it is the only route to capacity and notification preferences,
and burying it would make personal configuration hard to find — a poor
outcome for a product whose personal data is deliberately self-managed.

### 3.3 Screen inventory

| ID | Screen | Route | Status | Requirements |
|---|---|---|---|---|
| SCR-01 | Landing / root | `/` | Implemented | `FR-AUTH-004` |
| SCR-02 | Login | `/login` | Implemented | `FR-AUTH-002` |
| SCR-03 | Register | `/register` | Implemented | `FR-AUTH-001` |
| SCR-04 | Today (personal dashboard) | `/today` | Implemented | `FR-PER-001/006/007` |
| SCR-05 | Inbox | `/inbox` | Implemented | `FR-NTF-004` |
| SCR-06 | Project list | `/projects` | Implemented | `FR-PROJ-001` |
| SCR-07 | Project create | `/projects/new` | Implemented | `FR-PROJ-001` |
| SCR-08 | Project detail | `/projects/{id}` | Implemented | `FR-PROJ-004`, `FR-HLT-005` |
| SCR-09 | Project edit | `/projects/{id}/edit` | Implemented | `FR-PROJ-001` |
| SCR-10 | Issue create | `/projects/{id}/issues/new` | Implemented | `FR-ISS-001` |
| SCR-11 | Issue detail (read) | `/projects/{id}/issues/{issue_id}` | Implemented | `FR-ISS-004/005` |
| SCR-12 | Issue edit | `/projects/{id}/issues/{issue_id}/edit` | Implemented | `FR-ISS-004` |
| SCR-13 | Sub-issue create | `…/issues/{issue_id}/sub-issues/new` | Implemented | `FR-SUB-004` |
| SCR-14 | Team list | `/teams` | Implemented | `FR-TEAM-001` |
| SCR-15 | Team create | `/teams/new` | Implemented | `FR-TEAM-001` |
| SCR-16 | Team detail | `/teams/{slug}` | Implemented | `FR-TEAM-001/005` |
| SCR-17 | Team edit | `/teams/{slug}/edit` | Implemented | `FR-TEAM-001` |
| SCR-18 | Sprint list | `/teams/{slug}/sprints` | Implemented | `FR-SPR-001` |
| SCR-19 | Sprint create | `/teams/{slug}/sprints/new` | Implemented | `FR-SPR-001` |
| SCR-20 | Sprint detail | `/teams/{slug}/sprints/{sprint_id}` | Implemented | `FR-SPR-001/003` |
| SCR-21 | Sprint edit | `/teams/{slug}/sprints/{sprint_id}/edit` | Implemented | `FR-SPR-001` |
| SCR-22 | Settings index | `/settings` | Implemented | `FR-SET-001` |
| SCR-23 | Notification preferences | `/settings/notifications` | Implemented | `FR-SET-003` |
| SCR-24 | Search results | `/search` | Implemented | `FR-SCH-001` |
| SCR-25 | Error page | (any) | Implemented | §10 |
| SCR-26 | Sprint plan | `/teams/{slug}/sprints/{sprint_id}/plan` | Implemented (0.22.0) | `FR-PLAN-001..005` |
| SCR-27 | Personal calendar | `/today/calendar` | Implemented (0.23.0) | `FR-CAL-001` |
| SCR-28 | Project calendar | `/projects/{id}/calendar` | Implemented (0.23.0) | `FR-CAL-002` |
| SCR-29 | Delete project — confirm | `/projects/{id}/delete` (GET) | Implemented (0.25.0) | RFC 010, §17.4 |
| SCR-30 | Delete issue — confirm | `…/issues/{issue_id}/delete` (GET) | Implemented (0.25.0) | RFC 010, §17.4 |
| SCR-31 | Delete sprint — confirm | `…/sprints/{sprint_id}/delete` (GET) | Implemented (0.25.0) | RFC 010, §17.4, `FR-SPR-002` |
| SCR-32 | Indicator basis — the issues behind one indicator | `/projects/{id}/health/{indicator}/basis` | Implemented (0.29.0) | RFC 008, `FR-HLT-007`, §17.2 |

*Renumbered at 0.24.0.* These carried provisional `SCR-P-*` identifiers while
planned. They now hold permanent ones, and the `SCR-P-*` forms are retired
rather than reused — a provisional identifier that outlives its provisional
state is a second name for one screen.

Capacity and WIP-limit management are sections within SCR-22 rather
than separate screens; their mutation endpoints are listed in §8.

**SCR-32 is one screen serving the indicators that have an issue-shaped
basis** (0.29.0, RFC 008). It lists the issues an indicator actually counted,
obtained from the computation that produced the count rather than re-derived
from a filter, so the number on SCR-08 and the list on SCR-32 **cannot**
disagree.

**The link and the route agree on one predicate**, which is the point of the
design: SCR-08 renders a basis link exactly when the indicator's basis set
exists and is non-empty, and the route answers 404 in exactly the cases where
no link would have been rendered. There is no reachable link that 404s and no
silently-working route with no link to it. `wip_compliance` is excluded
**structurally** — it returns no basis set at all, rather than being filtered
out at the link or checked for by name at the route → `NFR-PRIV-007`, §17.2.

**SCR-29 to SCR-31 are three screens serving four flows** (0.25.0, RFC 010).
The sprint interstitial covers both planned and completed sprints on one
route, and refuses an `Active` sprint outright rather than confirming it.
Each names the specific project, issue or sprint — a confirmation that says
"Are you sure?" without saying what is about to be deleted is a keystroke
collector, not a confirmation.

**They are screens, not dialogs, and that is the point.** §17.4's defect was
that the previous confirmation was a JavaScript dialog: without JavaScript
none ran and the delete proceeded. A server-rendered `GET` cannot fail that
way. The five **reversible** confirmations — leave team, remove member,
detach project, remove capacity row, silence all — keep their dialogs and
gain no screen, because each is undoable through the interface.

### 3.4 Screen transition map

```
                          ┌──────────┐
                          │ SCR-01 / │
                          └────┬─────┘
                 unauthenticated│authenticated
                   ┌────────────┴────────────┐
                   ▼                         ▼
            ┌────────────┐            ┌────────────┐
            │ SCR-02     │◀──────────▶│ SCR-04     │
            │ Login      │            │ Today      │
            └─────┬──────┘            └─────┬──────┘
                  │                          │
            ┌─────▼──────┐      ┌────────────┼───────────┬──────────┐
            │ SCR-03     │      ▼            ▼           ▼          ▼
            │ Register   │  ┌────────┐  ┌────────┐  ┌────────┐ ┌────────┐
            └────────────┘  │ SCR-05 │  │ SCR-06 │  │ SCR-14 │ │ SCR-22 │
                            │ Inbox  │  │Projects│  │ Teams  │ │Settings│
                            └────────┘  └───┬────┘  └───┬────┘ └───┬────┘
                                            │           │          │
                                   ┌────────▼───┐  ┌────▼─────┐ ┌──▼──────┐
                                   │ SCR-08     │  │ SCR-16   │ │ SCR-23  │
                                   │ Project    │  │ Team     │ │ Notif.  │
                                   │ detail     │  │ detail   │ │ prefs   │
                                   └──┬───┬─────┘  └────┬─────┘ └─────────┘
                          ┌───────────┘   └──────┐      │
                          ▼                      ▼      ▼
                    ┌──────────┐          ┌──────────┐ ┌──────────┐
                    │ SCR-10   │          │ SCR-11   │ │ SCR-18   │
                    │ Issue    │─────────▶│ Issue    │ │ Sprint   │
                    │ create   │          │ detail   │ │ list     │
                    └──────────┘          └──┬────┬──┘ └────┬─────┘
                                             │    │         │
                                     ┌───────▼─┐ ┌▼───────┐ ▼
                                     │ SCR-12  │ │SCR-13  │ ┌──────────┐
                                     │ Issue   │ │Sub-iss.│ │ SCR-20   │
                                     │ edit    │ │ create │ │ Sprint   │
                                     └─────────┘ └────────┘ │ detail   │
                                                            └────┬─────┘
                                                                 │ (planned)
                                                            ┌────▼─────┐
                                                            │  SCR-26  │
                                                            │Sprint plan│
                                                            └──────────┘

  Search box (header, all screens) ──────▶ SCR-24 Search results
                                              │
                                              ├──▶ SCR-08 Project detail
                                              └──▶ SCR-11 Issue detail
```

Transition rules:

1. Any authenticated screen MUST reach any primary entry in one action
   (navigation is persistent).
2. Every detail screen MUST offer a breadcrumb path back to its parent
   and a back link → `FR-NAV-003`.
3. Creating an entity redirects to that entity's detail screen.
4. Mutating an entity redirects back to the screen the action began on,
   preserving filter and sort context → `FR-NAV-005`.
5. Unauthenticated access to a protected screen redirects to SCR-02
   with HTTP 303 → `FR-AUTH-004`.

---

## 4. URL design

### 4.1 Conventions

| Rule | Statement |
|---|---|
| Resource nesting | Paths mirror ownership: `/projects/{id}/issues/{issue_id}`. |
| Verbs as sub-paths | Mutations use a trailing verb segment: `/delete`, `/edit`, `/start`. |
| Mode as path, not query | Edit mode is `/edit`, never `?edit=1` → `FR-ISS-004`. |
| Filter and sort as query | View state is query parameters, so URLs remain shareable → `FR-NAV-005`. |
| Identifiers | Projects, issues, sprints use opaque ids; teams use human-readable slugs. |
| API namespace | JSON endpoints live under `/api/`. |

*Design note on mode-as-path.* A query parameter makes edit mode a
hidden state: refreshing, using browser-back, or opening in a new tab
all behave inconsistently. Making it a path segment means the URL fully
describes what the user is looking at.

### 4.2 HTML route table (implemented)

| Method | Route | Screen / action | Access |
|---|---|---|---|
| GET | `/` | SCR-01 landing / redirect | Any |
| GET | `/health` | Liveness probe (no UI) | Any |
| GET, POST | `/login` | SCR-02 | Anonymous |
| GET, POST | `/register` | SCR-03 | Anonymous |
| POST | `/logout` | End session | User |
| GET | `/today` | SCR-04 | Self |
| GET | `/inbox` | SCR-05 | Self |
| POST | `/inbox/mark-all-read` | Mark all read | Self |
| POST | `/inbox/{id}/read` | Mark one read | Recipient |
| POST | `/inbox/resume` | Resume notifications (0.24.0) | Self |
| POST | `/inbox/email-opt-in` | Answer the email opt-in (0.24.0) | Self |
| GET | `/today/calendar` | SCR-27 (0.23.0) | Self |
| GET | `/projects/{id}/calendar` | SCR-28 (0.23.0) | Project access |
| GET | `/projects` | SCR-06 | User |
| GET, POST | `/projects/new` | SCR-07 | User |
| GET | `/projects/{id}` | SCR-08 | Project access |
| GET | `/projects/{id}/health/{indicator}/basis` | SCR-32 — the issues behind one indicator (0.29.0). The slug parses for all six; **`wip_compliance` has no issue-shaped basis and answers 404** → §17.2 | Project access |
| GET, POST | `/projects/{id}/edit` | SCR-09 | Owner / admin |
| GET, POST | `/projects/{id}/delete` | SCR-29 (GET, 0.25.0), delete (POST); **POST requires `client_updated_at`** (0.27.0) | Owner / admin |
| GET, POST | `/projects/{id}/issues/new` | SCR-10 | Write access |
| GET, POST | `/projects/{id}/issues/{issue_id}` | SCR-11 (GET), update (POST) | Read / write |
| GET | `/projects/{id}/issues/{issue_id}/edit` | SCR-12 | Write access |
| POST | `/projects/{id}/issues/{issue_id}/status` | Status transition (edit form) | Write access |
| POST | `…/issues/{issue_id}/status/board` | Status change from the board | Write access |
| POST | `…/issues/{issue_id}/status/detail` | Status change from SCR-11 (0.25.0) | Write access |
| POST | `…/issues/{issue_id}/status/list` | Status change from SCR-08's list view (0.25.0) | Write access |
| GET, POST | `/projects/{id}/issues/{issue_id}/delete` | SCR-30 (GET, 0.25.0), delete (POST); **POST requires `client_updated_at`** (0.27.0) | Write access |
| GET, POST | `/projects/{id}/issues/{issue_id}/sub-issues/new` | SCR-13 | Write access |
| POST | `/projects/{project_id}/issues/{issue_id}/sprint` | Sprint assignment | Write access |
| GET | `/teams` | SCR-14 | User |
| GET, POST | `/teams/new` | SCR-15 | User |
| GET | `/teams/{slug}` | SCR-16 | Member |
| GET, POST | `/teams/{slug}/edit` | SCR-17 | Admin |
| POST | `/teams/{slug}/members` | Add member | Admin |
| POST | `/teams/{slug}/members/{user_id}/role` | Change role | Admin |
| POST | `/teams/{slug}/members/{user_id}/remove` | Remove member | Admin |
| POST | `/teams/{slug}/projects/{project_id}/unassign` | Detach project | Admin |
| GET | `/teams/{slug}/sprints` | SCR-18 | Member |
| GET, POST | `/teams/{slug}/sprints/new` | SCR-19 | Admin |
| GET | `/teams/{slug}/sprints/{sprint_id}` | SCR-20 | Member |
| GET, POST | `/teams/{slug}/sprints/{sprint_id}/edit` | SCR-21 | Admin |
| POST | `/teams/{slug}/sprints/{sprint_id}/start` | Begin sprint | Admin |
| POST | `/teams/{slug}/sprints/{sprint_id}/complete` | Complete sprint | Admin |
| GET, POST | `/teams/{slug}/sprints/{sprint_id}/delete` | SCR-31 (GET, 0.25.0), delete (POST); refuses an `Active` sprint | Admin |
| GET | `/teams/{slug}/sprints/{sprint_id}/plan` | SCR-26 (0.22.0) | Member, incl. `viewer` |
| POST | `/teams/{slug}/sprints/{sprint_id}/plan/add` | Move issue into sprint (0.22.0) | `admin` / `member` |
| POST | `/teams/{slug}/sprints/{sprint_id}/plan/remove` | Move issue out of sprint (0.22.0) | `admin` / `member` |
| GET | `/settings` | SCR-22 | Self |
| POST | `/settings/wip-limit` | Set WIP limit | Self |
| POST | `/settings/capacity` | Create capacity period | Self |
| POST | `/settings/capacity/{id}` | Update capacity period | Self |
| POST | `/settings/capacity/{id}/close` | Close open-ended period | Self |
| POST | `/settings/capacity/{id}/delete` | Delete capacity period | Self |
| GET, POST | `/settings/notifications` | SCR-23 | Self |
| POST | `/settings/notifications/silence-all` | Silence all | Self |
| GET | `/search` | SCR-24 | User |
| GET | `/static/*` | Static assets | Any |

**On the four status routes.** One route per surface, not one shared route:
each surface's no-JavaScript fallback redirects to its own page, and a shared
route would have to guess which. `/status/board` predates the other two.
All four accept `client_updated_at` and answer `409` on a mismatch (§15).

**Each returns `200` with the new `updated_at`** where `/status/board`
formerly returned `204` (0.26.0). A client that applies the change in place
instead of reloading needs that value, or its next request carries a stale
`client_updated_at` and conflicts with itself.

**On the three `GET` delete rows.** The `GET` renders SCR-29 to SCR-31; the
`POST` is unchanged from before 0.25.0. A `GET` that deletes nothing and a
`POST` that deletes is the split §17.4 asked for.

**The two `POST`s narrowed at 0.27.0**, and this is the release's one
compatibility note. Both now require a `client_updated_at` form field:

| Request | Response |
|---|---|
| Form body carrying a matching `client_updated_at` | the delete proceeds |
| Form body carrying a stale value | `409` (§15) |
| Form body **omitting** the field | `400`, with a readable message |
| **No body at all** | `415`, from the extractor, **before any handler runs** |

The value is rendered as a hidden field on the confirmation screen, which is
where a caller adapting a script reads it from. The sprint delete route has
required it since before 0.25.0.

**A caller who scripted a delete against either route will find it stops
working.** That audience is probably empty — these are HTML form endpoints with
no documented contract — but §4.5 records a removed route and its reason for
the same class of reader, and a narrowed contract is the same obligation.

### 4.3 Redirect map

All redirects use **HTTP 308**, preserving the request method
→ `FR-NAV-002`.

| Legacy | Target | Introduced |
|---|---|---|
| `/me` | `/today` | Phase A |
| `/notifications` | `/inbox` | Phase A |
| `/notifications/mark-all-read` | `/inbox/mark-all-read` | Phase A |
| `/notifications/{id}/read` | `/inbox/{id}/read` | Phase A |
| `/projects/{id}/issues/{issue_id}?edit=1` | `…/{issue_id}/edit` | Phase B |

*Design note.* 301 was rejected because user agents historically
downgrade the method to GET on 301, which would silently convert a
bookmarked POST into a read. 308 has no such licence.

### 4.4 Reserved routes

| Route | Screen | Source |
|---|---|---|
| `/api/users/{user_id}/wip-limit` | API | `SPEC E.1` |
| `/api/users/{user_id}/calendar` | API | `SPEC E.1` |

*Emptied of HTML routes at 0.24.0.* The seven entries previously here all
shipped across 0.22.0–0.24.0 and moved to §4.2. One changed on the way:
RFC 0003 reserved `/inbox/silence/resume`; it shipped as **`/inbox/resume`**,
matching `/inbox/mark-all-read`'s shape. The reservation is not preserved as a
redirect — a reserved route that was never live has no callers to keep.

### 4.5 Removed routes

| Route | Removed | Reason |
|---|---|---|
| POST `/settings/notifications/ack-global` | 0.24.0 | The email opt-in prompt moved to `/inbox`; the form this served no longer renders |

**No redirect, deliberately.** It was POST-only and reachable only from that
form, so nothing external can hold a link to it — §4.3's 308 rule exists for
addresses users may have bookmarked, and a POST endpoint is not one. Recorded
here rather than omitted, because a route disappearing from §4.2 with no
explanation reads as an error in this document.

---

## 5. Common elements and behaviour

### 5.1 Progressive disclosure

Secondary content is placed inside a native disclosure element,
collapsed by default → `FR-PER-007`, `NFR-A11Y-001`.

| Property | Specification |
|---|---|
| Mechanism | Native `<details>` / `<summary>`. No scripting. |
| Default state | Collapsed, unless a threshold condition applies. |
| Summary text | Describes what opening reveals, not "more". |
| Accessibility | Keyboard-operable by default; state exposed by the element. |

*Design note.* A native disclosure element was chosen over a scripted
accordion because it keeps the no-JavaScript commitment, is already
keyboard-operable, and is announced correctly by assistive technology
without extra attributes.

### 5.2 State badges and chips

| Family | Values | Presentation |
|---|---|---|
| Issue status | Open · In Progress · Done | Label + colour |
| Health state | Insufficient · Good · Watch | Label + glyph + colour → `NFR-LANG-002` |
| Trend | trending higher · stable · trending lower | Arrow glyph + text, neutral colour |
| Ownership | Personal · Team | Label |
| Sprint lifecycle | Planned · Active · Completed | Label |

Every badge MUST carry a text label. Colour is never the sole carrier
of meaning → `NFR-A11Y-004`.

**Severity ceiling.** No user-visible label may exceed `Watch`. The
words `Concern`, `danger`, and `failing` MUST NOT appear, and danger
colouring MUST NOT represent health state → `NFR-LANG-002`.
*Current implementation diverges; see §17.1.*

### 5.3 Flash and error presentation

| Kind | Carrier | Role | Persistence |
|---|---|---|---|
| Success flash | `?flash=` query parameter | `role="status"` | Until next navigation |
| Recoverable error | `?error=` query parameter or inline | `role="alert"` | Until corrected |
| Field validation | Inline, adjacent to field | `aria-describedby` | Until corrected |

Errors MUST preserve the user's input → `GUI §9`. A rejected form
re-renders with the submitted values intact.

*Design note.* Flash state travels in the URL rather than a session
cookie so that a redirect-after-post is idempotent, the resulting page
is bookmarkable without a phantom message, and private-browsing
sessions behave identically.

### 5.4 Empty states

Empty states MUST explain what the area is for and what the user could
do next, without implying that emptiness is a deficiency
→ `NFR-LANG-001`, `GUI §9`.

| Screen | Empty message intent |
|---|---|
| Project list | Explain that projects hold issues; offer creation. |
| Issue list | Explain the list will fill as work is recorded. |
| Sub-issues | Explain that breaking work down is optional and what it enables. |
| Inbox | State that nothing needs attention. No praise, no urging. |
| Health strip | State that indicators appear once there is enough activity. |

### 5.5 Focus behaviour

| Situation | Focus destination |
|---|---|
| Page load | Document start; skip link first focusable. |
| Skip link activated | `#main-content`. |
| Form validation failure | First field in error. |
| Disclosure opened | Remains on the summary element. |
| Post-action redirect | Page start; flash region announced. |

### 5.6 Keyboard operation

Every flow MUST complete with the keyboard alone → `NFR-A11Y-001`.
Interactive controls are real interactive elements — links for
navigation, buttons for actions — so that native keyboard behaviour
applies without additional handling.

The status segment on SCR-11 is deliberately non-interactive in this
release; it is a display of state, and the edit route is the mutation
path → `FR-ISS-006`.

### 5.7 Touch targets

All interactive elements MUST present at least 44 × 44 CSS pixels of
**target** area → `NFR-A11Y-007`. This applies especially to status badges
in lists, disclosure toggles, and inbox rows.

*Amended at 0.30.0 by `DEC-049` (RFC 012).* Two clauses were implicit and are
now explicit, because their absence is what left this section unenforceable:

- **Target ≠ visible bounds.** The target is the area that responds to a pointer
  or touch. A control may satisfy this by **expanded hit area** rather than by
  growing. Three controls in the product satisfy it by growing
  (`confirmation.rs:53`/`:58`, `issues.rs:825`, all `min-h-11 min-w-11` over a
  small size class); both mechanisms conform, and which one a surface takes is a
  layout decision with no accessibility consequence.
- **Targets MUST NOT overlap.** Expanding two adjacent controls into each other
  produces a region where a tap resolves to whichever element is stacked above —
  **worse than the small target it replaced**, because it is wrong rather than
  merely difficult. A size floor without this clause manufactures the defect it
  was adopted to prevent.

**Status at 0.31.0: conforming for class-carrying controls. Amended 2026-09-08
(`DEC-050`).** All 139 such controls reach a 44 px target and
`touch_target_scan` enforces it with no exception list.

**The "unassessed, not passing" limit has been measured and removed.** It was
not narrow: an account menu on every page, `<summary>` disclosure toggles at
16 px — which this very section names — breadcrumbs, and the indicator basis
links. The rule now covers **every** interactive element, with **one declared
exception**: a link inside a block of running text, marked as such in the markup.
Across ten pages exactly one control qualified.

**Target still means target, not visible size** — these reach 44 px by padding
the interactive row, not by enlarging type. See `§17.7`.

### 5.8 Behaviour without JavaScript

| Feature | With script | Without script |
|---|---|---|
| Search | Type-ahead dropdown (`search.js`) | Form submit to `/search` |
| Kanban board | Drag a card between columns (`board.js`) | Per-card status buttons that post and reload |
| Status change, all three surfaces | Applied in place, 5-second undo (`dm.js`, `board.js`) | Form posts, page reloads at the new status |
| Destructive deletes (4) | Identical — a server-rendered confirm screen | Identical |
| Reversible confirmations (5) | `confirm()` dialog | No dialog; the action proceeds and is undoable |
| All other forms | Identical | Identical |
| All navigation | Identical | Identical |

No flow depends on scripting.

**That sentence was false for four releases and is true again from 0.25.0.**
Destructive deletes confirmed through `onsubmit="return confirm(…)"`. Without
JavaScript the handler never ran and the delete proceeded **unconfirmed** —
the no-script path was not degraded, it was more dangerous. See §17.4.

**The ordering rule, and where it now shows.** `DEC-021` permits JavaScript
only as an enhancement over a working no-JavaScript path, and requires that
path to ship **first**. 0.25.0 shipped the status control as plain forms on
the issue list and issue detail; 0.26.0 layered the in-place update over it.
So the row above is not a fallback retro-fitted to an enhancement — it is the
thing that shipped, with the enhancement added a release later.

**Where the fallback stops.** Falling back to a native form submit is correct
**before** the server has applied the change and wrong after: re-submitting an
applied change would send it twice. Past the point where the response confirms
the change landed, a client-side failure is announced rather than retried.

**What the automated suite does and does not cover here.** The right-hand
column is tested — the harness drives HTTP and every no-script path is
asserted. The left-hand column is not: `dm.js`, `board.js` and `search.js` are
never executed by any test. Recorded in the requirements baseline as `§10.15`.
The mitigation is the structure of this table rather than a test: a total
failure of those files degrades to the tested column.

---

### 5.9 List order

**Every list-bearing screen states the order it presents its items in.** Added
at 0.38.0; see `§17.9` for why it was absent and what that cost.

Stated **once here** rather than repeated per screen, because the four defects
`§10.29` records all came from an order being expressed in more than one place
or in none.

| Surface | Order | Obligation |
|---|---|---|
| Issue list (`SCR-08`), default | **Newest first.** No status grouping | The board is the status-grouped view; a second grouping rule on the list would be redundant, and `status` is TEXT so ordering by it was alphabetical |
| Issue list, explicit sort | `priority` (urgent → low), `created`, `updated` | Severity comes from `Priority::severity_rank`, never from the stored string. Ties keep the default order |
| Board columns (`SCR-08`) | Columns **Open, In progress, Done**; within a column, **newest first** | Column order is the lifecycle, from `IssueStatus::all()`, never from SQL |
| Sprint issue list (`SCR-20`, and the plan's sprint column) | **Open, In progress, Done**; within each, **assignment order, oldest first** | Grouped, unlike the issue list, because a sprint has no board beside it and *what is left* is the question the screen answers. Status order comes from `IssueStatus::lifecycle_rank` |
| Sprint-plan backlog | **Project name, then severity urgent → low, then newest first** | |
| Inbox (`SCR-05`) | **Newest first** | |
| Project list (`SCR-06`), search results | **Most recently updated first** | |
| Sub-issues (`SCR-11`) | **Creation order, oldest first** | The order they were defined in |
| Capacity history (`SCR-22`) | **Oldest first** | |

**Two obligations that apply to all of them.**

1. **An order must be total.** Every ordering above resolves ties to a single
   arrangement — timestamps carry a `rowid` tiebreak in the matching direction,
   because the stored resolution is one second and a burst would otherwise be
   arranged by whatever the query plan happened to do.
2. **An order must not be carried by a value's spelling.** Severity and
   lifecycle are ranks, expressed once each in `peisear-core`. Ordering a TEXT
   column and relying on its alphabetical order is what `§10.29` records.


## 6. Screen specifications

Each screen is specified as: purpose · entry points · access · layout ·
elements · states · actions · messages · privacy · accessibility.

---

### SCR-02 · Login

**Purpose.** Authenticate an existing account holder.
**Entry.** SCR-01 when unauthenticated; any protected route (303
redirect); link from SCR-03.
**Access.** Anonymous.

**Layout.**

```
┌────────────────────────────────┐
│  Sign in                       │
│  ┌──────────────────────────┐  │
│  │ Email        [_________] │  │
│  │ Password     [_________] │  │
│  │            [ Sign in ]   │  │
│  └──────────────────────────┘  │
│  Need an account? Register     │
└────────────────────────────────┘
```

**Elements.**

| Element | Type | Obligation |
|---|---|---|
| Email | `input[type=email]` | Required; explicit `<label>`, not placeholder-only. |
| Password | `input[type=password]` | Required. |
| Submit | `button[type=submit]` | Primary action. |
| Register link | `<a>` | To SCR-03. |

**States.** Default · validation error · authentication failure.

**Messages.** Authentication failure MUST NOT disclose which field was
wrong → `FR-AUTH-002`. A single neutral message covers both cases.

**Accessibility.** Labels associated with inputs; error linked by
`aria-describedby`; submit reachable by keyboard.

---

### SCR-03 · Register

**Purpose.** Create an account.
**Access.** Anonymous.

**Elements.** Email (required) · display name (required) · password
(required, minimum 8 characters) · submit · link to SCR-02.

**Validation.** → §8. Password length is enforced server-side and
stated up front, not only on failure.

**Post-condition.** Successful registration establishes a session and
redirects to SCR-04.

**Design note.** The email opt-in prompt is deliberately **not** shown
here. It appears after the user's first notification, so the decision
is made with knowledge of what a notification looks like
→ `FR-NTF-007`. *Planned; RFC 0003.*

---

### SCR-04 · Today (personal dashboard)

**Purpose.** Answer, for the signed-in user only: *is my current
workload sustainable, and is there anything I should look at first?*
**Entry.** Navigation; post-login redirect; `/me` (308).
**Access.** Self only. No parameterised form of this screen exists;
there is no route by which one user requests another's dashboard
→ `NFR-PRIV-001`.

**Layout.**

```
┌──────────────────────────────────────────────────────┐
│ My dashboard                                         │
│ Personal metrics for {name}. Visible only to you.    │
├──────────────────────────────────────────────────────┤
│ ┌── what to read first (0 or 1) ──────────────────┐  │
│ │ {title}                                          │  │
│ │ {one supporting sentence}                        │  │
│ └──────────────────────────────────────────────────┘  │
├──────────────────────────────────────────────────────┤
│ RIGHT NOW                            (always open)   │
│   WIP  [3 / 5]      Load  [12 pt]                    │
├──────────────────────────────────────────────────────┤
│ ▸ Rhythm                            (collapsed)      │
│     Throughput · Long-stale · Pace                   │
├──────────────────────────────────────────────────────┤
│ ▸ Sustainability                (opens on signal)    │
│     Overload streak · Stalled · Drift · Switching    │
├──────────────────────────────────────────────────────┤
│ ▸ What do these mean?               (collapsed)      │
└──────────────────────────────────────────────────────┘
```

**Panel obligations.**

| Panel | Default | Rationale |
|---|---|---|
| What to read first | Rendered only when a condition matches | At most one; none when nothing applies → `FR-PER-006` |
| Right now | Always visible | Most actionable, smallest surface → `FR-PER-007` |
| Rhythm | Collapsed | Reference material, not first-glance |
| Sustainability | Self-opening at watch threshold | Surfaces itself precisely when relevant |
| What do these mean? | Collapsed | Explanation on demand |

**Callout precedence.** Exactly one, chosen in strict order:

1. Sustained burnout signal (overload streak or stalled assignment at
   or beyond its watch threshold).
2. WIP strictly greater than the effective limit.
3. One or more long-stale assigned issues.
4. Otherwise: **no callout is rendered.**

*Design note.* Two callouts compete for the same attention and dilute
each other; a callout manufactured for a healthy state teaches the user
that the slot is noise. Silence in the healthy case is what makes the
slot trustworthy in the unhealthy one.

**Privacy.** The subtitle states "Visible only to you." No aggregate
across users appears anywhere on this screen.

**Accessibility.** Callout is `role="note"` with an accessible name;
panels are native disclosures; every metric carries a text label
alongside its badge.

---

### SCR-05 · Inbox

**Purpose.** Present notifications addressed to the signed-in user.
**Access.** Self only; individual notification actions are restricted
to the recipient → `FR-NTF-009`.

**Layout.**

```
┌──────────────────────────────────────────────────────┐
│ Inbox                            [ Mark all read ]   │  ← planned
├──────────────────────────────────────────────────────┤
│ ┌ Notifications are silenced.   [ Resume ] ┐         │  ← planned
│ └──────────────────────────────────────────┘         │
├──────────────────────────────────────────────────────┤
│ ● {kind}   {factual body}                {relative}  │
│ ○ {kind}   {factual body}                {relative}  │
└──────────────────────────────────────────────────────┘
```

**Elements.**

| Element | Status | Notes |
|---|---|---|
| Notification row | Implemented | Unread marked by glyph + text, not colour alone |
| Mark one read | Implemented | POST, recipient only |
| Mark all read | Planned (RFC 0003) | Hidden when nothing unread → `FR-NTF-005` |
| Silence-resume banner | Planned (RFC 0003) | Shown only while silenced → `FR-NTF-006` |
| Email opt-in prompt | Planned (RFC 0003) | After first notification → `FR-NTF-007` |

**Messages.** Notification bodies are factual descriptions. Severity is
limited to Info and Watch → `FR-NTF-003`.

**Design note on the resume banner.** The setting to un-silence already
exists under notification preferences, but a user who silenced
notifications weeks ago is unlikely to remember where that lives. The
banner puts the reversal where the consequence is felt.

---

### SCR-06 · Project list

**Purpose.** Enumerate accessible projects and provide entry to each.
**Access.** Projects the user owns or that belong to their teams.

**Elements.** Ownership filter (all / personal / team) · project rows
(name, ownership badge, compact health, issue count, updated) · create
action.

**States.** Populated · empty (supportive message + creation
affordance).

---

### SCR-08 · Project detail

**Purpose.** Support one judgement: *what is the state of this project,
and what should I open next?* → `FR-PROJ-004`.

**Layout.**

```
┌──────────────────────────────────────────────────────┐
│ Projects › {project}                                 │
│ {project name}     [Personal|Team]        [ Edit ]   │
├──────────────────────────────────────────────────────┤
│ HEALTH                                               │
│   [Score badge] {summary sentence}   [trend]         │  ← see §17.1
│   ▸ Indicators                                       │
│       • {plain-language explanation}                 │
│       • {plain-language explanation}                 │
│       [chip] [chip] [chip] [chip] [chip] [chip]      │
├──────────────────────────────────────────────────────┤
│ ISSUES        [filter] [sort]        [ New issue ]   │
│   ○ {title}         {status} {priority} {assignee}   │
│   ○ {title}         {status} {priority} {assignee}   │
└──────────────────────────────────────────────────────┘
```

**Health section obligations.**

| Element | Obligation |
|---|---|
| Indicator set | Six indicators covering completion trend, staleness, activity, concentration, long-stale work, WIP compliance → `FR-HLT-001` |
| Explanations | One plain-language sentence per non-healthy indicator; none for healthy or insufficient → `FR-HLT-005` |
| Explanation tone | Descriptive only; no evaluation, no directive → `FR-HLT-006` |
| Trend | Direction only, neutral colour → `FR-HLT-004` |
| Composite | Equal weight beside the individual indicators; no headline score → `FR-HLT-008` *(diverges; §17.1)* |
| Basis link | Route from each indicator to the issues it counted, plus its threshold stated inline → `FR-HLT-007`. **Five of six indicators; `wip_compliance` is excepted** and offers no link, because its basis is one person's assignment load → `NFR-PRIV-007`. **History is deferred** for the same reason → §17.2 |

**Issue list obligations.**

- Only top-level issues appear → `FR-SUB-006`.
- Filter and sort persist across navigation and are reflected in the
  URL → `FR-NAV-005`.
- The issue list remains the visually dominant region; analysis must
  not crowd it → `GUI §9`.

---

### SCR-11 · Issue detail (read mode)

**Purpose.** Present one issue for reading and provide routes to act on
it. This screen never mutates.

**Layout.**

```
┌──────────────────────────────────────────────────────┐
│ Projects › {project} › {issue}                       │
│  ── or, for a sub-issue ──                           │
│ Projects › {project} › {parent} › {sub-issue}        │
├──────────────────────────────────────────────────────┤
│ {issue title}                    [ Edit ] [ Delete ] │
│                                                      │
│ ┌ Open ┬ In Progress ┬ Done ┐   ← display only       │
│ └──────┴─────────────┴──────┘                        │
│                                                      │
│ {priority} {effort} {assignee} · created · updated   │
├──────────────────────────────────────────────────────┤
│ {description}                                        │
├──────────────────────────────────────────────────────┤
│ SUB-ISSUES                    [ + Add sub-issue ]    │  ← top-level only
│   {status} {title}                                   │
│   {status} {title}                                   │
├──────────────────────────────────────────────────────┤
│ SPRINT   [ select ▾ ]  [ Save ]                      │  ← top-level, team only
└──────────────────────────────────────────────────────┘
```

**Status segment.**

| Property | Specification |
|---|---|
| Content | All three statuses, always all shown |
| Active state | `aria-pressed="true"` on exactly one segment |
| Interaction | **None.** Display only in this release → `FR-ISS-006` |
| Mutation route | The Edit action → SCR-12 |
| Grouping | `role="group"` with an accessible name |

*Design note.* Showing all three statuses rather than only the current
one lets the reader see the whole lifecycle and where this issue sits
in it. The affordance is deliberately inert until the
direct-manipulation phase; shipping the layout early lets it settle
without promising an interaction that does not yet exist.

**Sub-issues section.**

| Condition | Presentation |
|---|---|
| Top-level, has children | Listed with status badge and link |
| Top-level, no children | Explanatory text plus add action |
| Is itself a sub-issue | **Section absent entirely** → `FR-SUB-001` |

**Sprint section.**

| Condition | Presentation |
|---|---|
| Top-level issue in a team project | Sprint selector with save |
| Sub-issue | **Selector absent.** Sprint follows the parent → `FR-SUB-007` |
| Personal project | Absent; sprints are a team feature → `FR-PROJ-002` |

*Design note.* Offering a sprint selector on a sub-issue would imply an
independence it does not have. The parent's sprint governs; changing it
there propagates.

**Breadcrumb.** A sub-issue's trail includes its parent as a link
→ `FR-NAV-004`.

**Privacy.** No personal sustainability data appears here, including
for the assignee.

---

### SCR-12 · Issue edit

**Purpose.** Modify one issue.
**Access.** Write access to the project.

**Elements.** Title · description · status (select) · priority ·
effort · assignee · concurrency token (hidden) · save · cancel.

**Concurrency.** The form carries the value the client observed when
rendering. On submit the server compares it; a mismatch is rejected
with a conflict response rather than silently overwriting
→ `NFR-CONC-001`, §11.

**Design note.** The status control here is a select, not the segment
from SCR-11. The segment is a read affordance; converting it into the
edit control would blur the read/write separation the URL split
establishes → `FR-ISS-006`.

---

### SCR-13 · Sub-issue create

**Purpose.** Create a child issue under a parent.
**Access.** Write access; the parent MUST be top-level.

**Elements.** Title · description · status · priority · effort ·
assignee · submit · cancel (returns to parent).

**Deliberate omission.** No sprint selector. The sub-issue inherits its
parent's sprint → `FR-SUB-007`.

**Guard.** Requesting this screen for a parent that is itself a
sub-issue is refused with a validation message explaining the
one-level rule and suggesting a sibling instead → `FR-SUB-001`.

**Post-condition.** Redirects to the **parent's** detail screen, so the
user sees the new child in place.

**Explanatory copy.** "This sub-issue follows its parent's sprint. You
can give it its own assignee, status, priority, and effort." — states
both the constraint and the freedom.

---

### SCR-16 · Team detail

**Purpose.** Show membership, team projects, and sprint entry.
**Access.** Members only. Non-members MUST NOT learn whether the team
exists → `FR-TEAM-004`.

**Sections.** Header (name, own role, edit if admin) · members (table;
invite and role controls for admins) · team projects · sprints link ·
**privacy footnote**.

**Privacy footnote (fixed text, MUST appear).**

> Project trends and workload distribution are visible to all members.
> Personal sustainability data (burnout panel, /today) is visible only
> to the individual concerned. Admin is a management role, not an
> oversight role.

→ `FR-TEAM-005`. This wording is normative and MUST NOT be paraphrased.

---

### SCR-20 · Sprint detail

**Purpose.** Show one sprint's scope, progress, and lifecycle controls.

**Sections.** Header (name, lifecycle badge, admin actions) · summary
(committed, completed, in flight, carried over) · burndown · issue
list.

**Chart rules.**

| Rule | Rationale |
|---|---|
| No ideal line | An ideal line converts a plan into a standard to fall short of |
| No prediction line | Prediction becomes an expectation |
| No completion-rate emphasis | → `NFR-LANG-001` |
| Caption states what the chart does *and does not* show | → `FR-HLT-007` family |
| Tabular equivalent | Both charts carry a data table and a textual summary (0.29.0) → `NFR-A11Y-003`. The summary **inherits `NFR-PRIV-007`'s suppression** rather than restating it: where the chart is suppressed, the table and summary are absent too, not populated from the same data by another path |

**Vocabulary.** Unfinished work is "carried over" — a factual
description, never shortfall → `FR-SPR-003`.

**Lifecycle.** Transitions occur only by explicit administrator action
→ `FR-ISS-007`, `FR-SPR-001`. Completed sprints preserve their
historical values.

**Issue list.** Top-level issues only, so each unit of work is counted
once → `FR-SUB-008`.

---

### SCR-22 · Settings

**Purpose.** Self-configuration. Every value here belongs to the
signed-in user and is visible to nobody else → `FR-SET-001`.

**Sections.**

| Section | Contents |
|---|---|
| Profile | Display name, email |
| WIP limit | Personal concurrent-work limit; default and effect explained |
| Capacity | Period rows: points, optional start and end, note; add / edit / close / delete |
| Notifications | Link to SCR-23 |

**Capacity rules.** Periods MUST NOT overlap; a rejected submission
explains the overlap in text, not only visually. Open-ended periods can
be closed on a chosen date rather than deleted, preserving history.

**Concurrency.** Capacity mutations carry a concurrency token
→ `NFR-CONC-001`.

---

### SCR-24 · Search results

**Purpose.** Locate a project or open issue by name.
**Access.** Results restricted to the user's accessible scope
→ `FR-SCH-002`.

**Behaviour.** Blank query renders guidance rather than an error.
Results group projects and issues. Completed issues are excluded from
type-ahead but appear in full results → `FR-SCH-003`.

**Planned.** Sub-issue results should show parent context
→ `FR-SUB-010` (RFC 0003).

---

### SCR-26 · Sprint plan (0.22.0)

**Purpose.** Assign many issues to a sprint in one session
→ `FR-PLAN-001`.

```
┌───────────────────────────┬──────────────────────────┐
│ BACKLOG                   │ SPRINT ITEMS             │
│ [project▾][priority▾][…]  │ committed: 24 pt         │
│                           │ capacity hint: ~30       │
│ {title}  5pt  [→ Sprint]  │ {title}  8pt [← Backlog] │
│ {title}  3pt  [→ Sprint]  │ {title}  5pt [← Backlog] │
└───────────────────────────┴──────────────────────────┘
```

Obligations: button-driven moves, **and since 0.35.0 a drag beside them**
→ `FR-PLAN-002`, whose two halves are now both shipped; filters in
the URL → `FR-PLAN-003`; capacity hint advisory only → `FR-PLAN-004`;
team-scoped backlog → `FR-PLAN-005`; top-level issues only
→ `FR-SUB-006`; completed sprints render read-only → `FR-SPR-004`.

---

### SCR-27 / SCR-28 · Calendar (0.23.0)

**Purpose.** Place issues on a time axis — personal axis for one's own
assigned work, project axis for a project's work → `FR-CAL-001/002`.

| Property | Personal axis | Project axis |
|---|---|---|
| Content | Own assigned issues, all projects | Project's top-level issues |
| Sprint band | Absent | Active sprints overlapping the window |
| Privacy footnote | "Private to you" | Fixed calendar note → `FR-CAL-008` |
| Views | Day · week · month, in the URL | Same |

**Prohibitions (normative).** No occupancy rate, no week-over-week
comparison, no free-hours total, no derived efficiency figure of any
kind → `FR-CAL-007`. Crowding MAY be indicated quietly; emptiness MUST
NOT be marked at all → `FR-CAL-005`.

**Direct manipulation (0.36.0).** On the **day view only**, a block may be
dragged to reschedule the appointment: both planned timestamps move by one
delta, so the duration — and therefore the block's height — is unchanged. It
moves, it does not resize. The week and month views are unchanged, and the
calendar carries **no on-page control** for this: the plain path is the issue's
own edit form, which is also the path on a phone, since drag-and-drop does not
fire for touch (RFC 0004 cross-cutting requirement 10).

**Permanent exclusion.** No team-axis calendar exists or will
→ `FR-CAL-006`.

*Design note.* The asymmetry between crowding and emptiness is the
design. Flagging a full day helps someone notice over-commitment.
Flagging an empty day tells them their time is being audited — which is
the oversight posture the product refuses.

---

## 7. Form contracts

| Form | Route | Method | Required | Validation | Success |
|---|---|---|---|---|---|
| Login | `/login` | POST | email, password | Valid email; password present | → SCR-04 |
| Register | `/register` | POST | email, display name, password | Password ≥ 8 | → SCR-04 |
| Project create | `/projects/new` | POST | name | Name present; team eligibility | → SCR-08 |
| Project edit | `/projects/{id}/edit` | POST | name, token | Edit permission | → SCR-08 |
| Issue create | `…/issues/new` | POST | title | Title 1–200; description ≤ 10 000 | → SCR-11 |
| Issue edit | `…/issues/{iid}` | POST | title, token | As create; concurrency | → SCR-11 |
| Issue status | `…/issues/{iid}/status` | POST | status, token | One of open / in_progress / done | Event appended; refresh |
| Sub-issue create | `…/sub-issues/new` | POST | title | As create; parent must be top-level | → parent SCR-11 |
| Sprint assignment | `…/issues/{iid}/sprint` | POST | sprint_id (optional) | Completed sprint rejected; sub-issue rejected | Refresh SCR-11 |
| Capacity create | `/settings/capacity` | POST | points, dates | No overlap; valid range | Refresh SCR-22 |
| Capacity edit | `/settings/capacity/{id}` | POST | points, token | As create; concurrency | Refresh SCR-22 |
| Capacity close | `…/{id}/close` | POST | date, token | Date required | Refresh SCR-22 |
| WIP limit | `/settings/wip-limit` | POST | limit | Positive integer | Refresh SCR-22 |
| Team create | `/teams/new` | POST | name | Slug collision handled | → SCR-16 |
| Member add | `/teams/{slug}/members` | POST | email, role | Existing user; valid role | Refresh SCR-16 |
| Role change | `…/members/{uid}/role` | POST | role | Last-admin guard | Refresh SCR-16 |
| Sprint create | `/teams/{slug}/sprints/new` | POST | name, start | Valid range | → SCR-20 |
| Sprint start | `…/{sid}/start` | POST | token | Only one active per team | Becomes active |
| Sprint complete | `…/{sid}/complete` | POST | token | Must be active | Becomes completed |
| Mark read | `/inbox/{id}/read` | POST | — | Recipient only | Row becomes read |
| Notification prefs | `/settings/notifications` | POST | preferences | Valid channel and severity | Refresh SCR-23 |

**Universal form rules.**

1. Mutations of owned entities carry a concurrency token; omission is
   rejected → `NFR-CONC-005`.
2. Rejected submissions re-render with input preserved.
3. Validation messages are factual and non-directive → `NFR-LANG-001`.
4. Destructive actions require confirmation.
5. Successful mutation redirects rather than re-rendering, so refresh
   does not resubmit.

---

## 8. External API design

### 8.1 Conventions

| Property | Specification |
|---|---|
| Namespace | `/api/` |
| Format | JSON request and response |
| Methods | GET only in this release (read-only) |
| Authentication | Session cookie, as for HTML |
| Unauthenticated | `401` with JSON body — never an HTML redirect → `FR-AUTH-005` |
| Cross-user | `403`, including for administrators and for non-existent targets → `FR-API-003` |
| Field naming | snake_case; stable machine-readable codes → `FR-API-004` |
| Shape stability | Response types are defined independently of internal types → `FR-API-005` |

*Design note on 401 versus redirect.* A JSON client receiving an HTML
login page cannot parse it and cannot distinguish authentication
failure from corruption. Separating the two error surfaces is what
makes the API usable from a script.

*Design note on 403 for missing targets.* Returning 404 for a
non-existent user would let an unauthenticated prober enumerate
accounts by status code. All refusals present identically.

### 8.2 Endpoints

| Method | Endpoint | Returns | Access |
|---|---|---|---|
| GET | `/api/users/{user_id}/burnout` | Burnout signals | Self only |
| GET | `/api/users/{user_id}/capacity` | Capacity rows and effective value | Self only |
| GET | `/api/users/{user_id}/notifications` | Recent notifications and unread count | Self only |
| GET | `/api/search?q=` | Type-ahead matches | User scope |

Reserved: `/api/users/{user_id}/wip-limit`, `/api/users/{user_id}/calendar`
→ `FR-API-006`.

### 8.3 Response shapes

**Burnout** — signals appear only when meaningfully raised; each
carries a stable `code` so clients need not parse human labels.

```json
{
  "user_id": "…",
  "indicator": "watch",
  "signals": [
    { "code": "overload_streak",     "value": 9,  "label": "…" },
    { "code": "stalled_assigned",    "value": 21, "label": "…" },
    { "code": "estimation_drift",    "value": "…", "label": "…" },
    { "code": "cognitive_switching", "value": "…", "label": "…" }
  ],
  "computed_at": "…"
}
```

`indicator` observes the severity ceiling → `NFR-LANG-002`.

**Capacity**

```json
{
  "user_id": "…",
  "effective_today": 20,
  "rows": [
    { "id": "…", "points": 20, "period_start": null,
      "period_end": null, "note": null, "updated_at": "…" }
  ]
}
```

**Notifications**

```json
{
  "user_id": "…",
  "unread_count": 3,
  "items": [
    { "id": "…", "kind": "burnout_overload", "severity": "watch",
      "body": "…", "read": false, "created_at": "…" }
  ]
}
```

### 8.4 Error bodies

```json
{ "error": "unauthorized", "message": "…" }
{ "error": "forbidden",    "message": "…" }
{ "error": "validation",   "message": "…" }
{ "error": "conflict",     "message": "…",
  "entity_type": "issue", "entity_id": "…",
  "current_updated_at": "…" }
```

The `error` value is a stable code. The `message` is human-readable and
subject to the vocabulary rules in §10.

---

## 9. HTTP status semantics

| Situation | Status | Body | Requirement |
|---|---|---|---|
| Success (read) | 200 | Content | — |
| Success (mutation) | 303 | Redirect | §7 rule 5 |
| Renamed route | 308 | Redirect, method preserved | `FR-NAV-002` |
| Unauthenticated, HTML | 303 → `/login` | Redirect | `FR-AUTH-004` |
| Unauthenticated, API | 401 | JSON | `FR-AUTH-005` |
| Cross-user personal data, **named by a user id** | 403 | JSON / page | `FR-API-003` |
| Cross-user personal data, **named by a resource id** | 404 | Page | Identical whether the row is absent or another user's, so it enumerates neither accounts nor rows — `GUI §3.4`'s access-safe denial. Measured 2026-09-23 |
| Non-existent user's data | 403 | Identical to above | `NFR-PRIV-006` |
| Non-member team request | 404 | Page | `FR-TEAM-004` |
| Validation failure | 400 | Re-rendered form / JSON | §7 |
| Concurrency conflict | 409 | Structured | `NFR-CONC-006` |
| Unexpected failure | 500 | Neutral page | §10 |

*Note on 403 versus 404.* Personal data uses 403 uniformly so that
existence is not disclosed. Team membership uses 404 so that a
non-member cannot confirm a team exists. The two rules differ because
what is being concealed differs: in the first case the resource's
existence is implied by the request itself; in the second it is not.

---

## 10. Message and vocabulary catalogue

### 10.1 Prohibited vocabulary (normative)

*Amended at 0.21.0.* This table is enforced, not merely stated. Every
user-visible string renders from `peisear-i18n`'s message table, and a test
walks every entry against this list. Two clarifications now govern how the
list is read — `requirements.md` §1.7.1 (a term is prohibited in every
inflection and casing) and §1.7.2 (prohibited in **use**, permitted in
**mention**: this document names these terms in order to prohibit them, which
is not a violation).

MUST NOT appear in any user-visible string:

| Category | Examples |
|---|---|
| Evaluation | "good progress", "bad pace", "performance is increasing/decreasing" |
| Judgement | "concerning trend", "underperforming", "failing to meet" |
| Direction | "you should…", "you must…" |
| Industry evaluative | "velocity" |
| Ranking | "ranking", "top performer" |
| Achievement pressure | completion-rate or attainment-rate emphasis |
| Failure framing | "Failed to update", "Error: outdated version" |

### 10.2 Preferred vocabulary

| Instead of | Use |
|---|---|
| velocity | completed work this period |
| behind / incomplete | carried over |
| active work | in flight |
| performance up/down | trending higher / trending lower |
| (admin capability) | this is a management role, not an oversight role |
| (personal data) | private to you |

### 10.3 Fixed texts

**Team detail footnote** — normative, MUST NOT be paraphrased:

> Privacy note: project trends and workload distribution are visible to all
> team members. Personal sustainability data (your burnout panel, your
> dashboard) remains visible to you only — admin role is a management role,
> not an oversight role.

*Amended at 0.21.0.* The text above is what ships. The 0.19.1 wording of this
clause was different, was marked MUST NOT be paraphrased, and had already been
paraphrased in code before this document declared it fixed.

**The document was corrected to the code, not the reverse**, on the merits:
the shipped text addresses the reader in the second person, which is the
product's voice everywhere else, and names the surface a user recognises
("your dashboard") instead of a URL path (`/today`) that appears in no
sentence a user reads. The 0.19.1 phrasing — "the individual concerned" — is
the register of a policy document, not of a product that claims this data is
private *to you*.

`FR-TEAM-005` binds this string, and `peisear-i18n`'s
`team_privacy_footnote_renders_byte_identically` asserts it verbatim. **A
change to this text is a change to a test and to a normative requirement, in
that order.**

**Project calendar footnote** — shipped at 0.23.0, no longer planned:

> Calendar note: this view shows planned issue work for this project.
> Personal schedules are not aggregated here. Each member's individual
> calendar is private to that person.

*Amended at 0.23.0.* RFC 0002 carried a **different** "do not paraphrase"
version of this same string — one sentence shorter, dropping the clause that
actually states the guarantee. This document's text won, and a byte-identity
test in `peisear-i18n` now holds it, as `FR-TEAM-005`'s footnote has.

That is the second time two of our documents carried conflicting normative
versions of one string. The first (the team footnote) was found *after* the
code had already diverged; this one was found before anything was built, which
is the only difference that matters — and it is the reason a fixed text without
a test is only a wish.

**The claim has to survive the implementation, not just the document.** A
project calendar labelling each block with a person would make this footnote
false while leaving it on the page, so `CAL-002` forbids the assignee on any
block and a test asserts it.

**Concurrency conflict** — shipped at 0.20.0, no longer planned. States that
another member changed the item first and that the latest state is now shown.
Neutral tone, neutral colour, no failure vocabulary → `FR-DM-005`.

*Amended at 0.21.0.* Two defects in this surface are worth recording, because
both were in the mechanism rather than the wording:

- **0.20.0** — the message rendered as `"validation failed: {message}"`. The
  error type's `Display` impl, written for logs, was reachable from the user
  path through a fallthrough.
- **0.21.0** — the same defect on the sibling `Conflict` variant, rendering
  `"conflict: {message}"` to every user who hit a conflict, including
  duplicate-email registration. Found while converting copy, not while
  reviewing errors.

The fix was structural: every variant now has an explicit user-facing arm and
there is no catch-all, so a future variant cannot inherit log wording by
default. **Copy can be correct in the table and wrong on the screen**; §10.4's
tone rules constrain what is written, not what a fallthrough appends to it.

*Amended at 0.26.0.* The board's three status-change sentences — its conflict
notice, its stale-page notice and its unavailable notice — had been authored
**inside `static/board.js`** since before this project had a vocabulary guard.
They now live in `peisear-i18n`'s message table with the rest of the copy,
moved byte-exact, and they passed §10.1's check without a reword.

**They were never excluded from that check. The check had only ever read
Rust.** `prose_scan` covers `crates/**`; nothing looked at `static/*.js` until
0.26.0, when `static_js_scan` was added. Two named limits on it: `search.js`
is excluded pending a rendering mechanism, and it does not catch a single word
standing alone — it looks for two or more.

Copy that had never been checked turning out to be fine is the true and
slightly anticlimactic outcome, and it is recorded that way rather than as a
correction, because the finding is about the guard's reach and not about the
words.

**Undo's two failure modes are distinct copy** (0.26.0). A `409` inside the
5-second window says another member changed the issue; any other failure says
the change could not be completed. Conflating them told users someone else had
changed the issue when nobody had — `STATUS-002` round 1, corrected before
release. `board.js` had drawn this distinction since 0.20.0; the new surfaces
initially did not.

**Delete confirmation consequences** — three texts, one per shape, added
0.25.0 and completed 0.27.0:

> Project: *"All its issues will be deleted too. This cannot be undone."*
> Issue with sub-issues: *"This issue has N sub-issues. Deleting it deletes
> all of them too. This cannot be undone."* (singular branch for one)
> Issue without, and both sprint cases: *"This cannot be undone."*

*Amended at 0.27.0.* The issue interstitial shipped at 0.25.0 with the bare
*"This cannot be undone."* while `issues.parent_issue_id` had been
`ON DELETE CASCADE` since migration `0015`. The project screen named its
cascade from the first day; the issue screen did not, on a route that also
cascades. **A confirmation that names the entity but not its children names
the smaller half** — and this document's §17.4 position was that the
interstitial exists to state what will be deleted, so the omission contradicted
the reason the screen was built.

**The count is what makes this a §15 concern.** A number is only true when it
is read, so the sentence is trustworthy only if nothing can add or remove a
sub-issue between the `GET` that renders it and the `POST` that acts on it.
That is why the issue delete gained an optimistic lock in the same change, not
a release later.

**The singular branch is this project's first pluralised count in prose.**
`PointsValue`, `NavBellCount` and `UnreadOfTotalStatus` all sidestep the
problem by not putting a number inside a sentence. Both branches are exercised
by `MessageKey::all()`, so the vocabulary guard reads each.

### 10.4 Message tone rules

1. State facts; leave interpretation to the reader.
2. Never attribute a state to a person's quality.
3. Empty is not a deficiency.
4. Errors describe what happened and what would resolve it.
5. No celebration on completion → `FR-DM-007`.

---

## 11. Externally observable background behaviour

### 11.1 Notification generation

```
periodic snapshot
      │
      ▼
 threshold crossed from below?  ──no──▶ nothing
      │yes
      ▼
 cooldown active for this user × kind?  ──yes──▶ nothing
      │no
      ▼
 record notification  ──▶  deliver in-app
                      └─▶  deliver by email (if configured and opted in)
```

| Property | Specification | Requirement |
|---|---|---|
| Trigger | Edge only — the moment of crossing, not the duration | `FR-NTF-001` |
| Cooldown | 24 hours per user × kind | `FR-NTF-002` |
| Severity | Info and Watch only | `FR-NTF-003` |
| Channels | In-app always; email when configured and opted in | `FR-NTF-008` |
| Kinds | Personal overload; personal stalled work | `SPEC §29.2` |

*Design note.* Edge triggering is what makes the product quiet. A
level-triggered design would re-notify while a condition persists,
which trains the user to ignore notifications — and a notification
stream that is ignored provides no protection at all.

### 11.2 Degradation without SMTP

| Condition | Externally observable behaviour |
|---|---|
| SMTP unconfigured | Warning at startup; in-app delivery continues normally |
| SMTP configured, unreachable | Send failure logged; in-app delivery unaffected |

No user-facing error is raised for mail failure; the in-app channel is
authoritative.

---

## 12. External data view

Entities as the user perceives them. Physical schema is internal
design.

| Entity | User-visible attributes | Externally visible rules |
|---|---|---|
| Project | Name, description, ownership, health, issues | Personal or team-scoped; sprints only for team projects |
| Issue | Title, description, status, priority, effort, assignee, parent, timestamps | One level of nesting; sub-issue shares the parent's project and sprint |
| Team | Name, slug, members, projects, sprints | Cannot lose its last administrator |
| Sprint | Name, goal, dates, lifecycle, issues | One active per team; transitions are explicit; completed is immutable |
| Capacity period | Points, optional range, note | Periods must not overlap |
| Notification | Kind, severity, body, read state, time | Visible only to its recipient |

**Sub-issue rules as the user experiences them:**

1. Any top-level issue may be broken into sub-issues.
2. A sub-issue cannot itself be broken down.
3. A sub-issue belongs to its parent's project.
4. A sub-issue has its own assignee, status, priority, and effort.
5. A sub-issue is scheduled with its parent, not separately.
6. Completing every sub-issue does **not** complete the parent
   → `FR-SUB-005`.
7. Deleting a parent deletes its sub-issues.

*Design note on rule 6.* The parent usually carries integration or
review work that no child represents. Automatic completion would close
work that has not been done, and would take a decision away from the
person best placed to make it.

---

## 13. Accessibility obligations by screen

Every screen is accepted against six axes → `NFR-A11Y-001..008`.

| Screen | Keyboard | Focus | Screen reader | Colour | Mobile | Live |
|---|---|---|---|---|---|---|
| SCR-02/03 Auth | Required | First error | Labels, error association | n/a | Required | n/a |
| SCR-04 Today | Required | Panel summaries | Callout named; metrics labelled | Badge + text | **Required** | n/a |
| SCR-05 Inbox | Required | Row order | Unread stated as text | Glyph + text | **Required** | On mark-read |
| SCR-06 Projects | Required | Row order | Row purpose clear | Badge + text | Required | n/a |
| SCR-08 Project | Required | Disclosure | Indicator state as text; basis link and inline threshold (0.29.0) | Badge + glyph | Required | n/a |
| SCR-11 Issue | Required | Action order | Segment `aria-pressed`; group named | Badge + text | **Required** | n/a |
| SCR-12 Issue edit | Required | First error | Field labels; error association | n/a | Required | n/a |
| SCR-16 Team | Required | Table order | Footnote read in order | Badge + text | Required | n/a |
| SCR-20 Sprint | Required | Chart alternative | Caption + tabular data + summary (0.29.0) | Pattern + colour | Required | n/a |
| SCR-22 Settings | Required | First error | Overlap explained in text | n/a | Required | n/a |
| SCR-24 Search | Required | Result order | Result count announced | n/a | Required | On type-ahead |

**Required** in the mobile column marks the four flows that MUST
complete on a phone → `NFR-A11Y-006`.

---

## 14. Responsive behaviour

| Breakpoint | Layout |
|---|---|
| ≥ 1024 px | Full shell; multi-column where specified |
| 640–1023 px | Navigation collapses to a toggle; content single-column |
| < 640 px | Single column; tables become stacked rows; planning-style screens degrade to read-only |

| Screen | Mobile behaviour |
|---|---|
| SCR-04 Today | Panels stack; callout stays first; disclosures remain usable |
| SCR-05 Inbox | Full function; row targets ≥ 44 px |
| SCR-08 Project | Health strip wraps; issue list becomes stacked rows |
| SCR-11 Issue | Full function including status display and sub-issue list |
| SCR-26 Sprint plan | Columns stack; move actions remain, drag does not |
| SCR-27/28 Calendar | Day view is the mobile default; month becomes a chronological list |

---

## 15. Externally visible concurrency behaviour

| Step | Behaviour |
|---|---|
| 1 | The rendered form carries the value observed at render time. |
| 2 | On submit, the server compares it with the stored value. |
| 3 | Match → the mutation proceeds; the stored value advances. |
| 4 | Mismatch → `409`; **the entity is not modified**. |
| 5 | The user is told another member changed it first, and the current state is shown. |
| 6 | No automatic retry. No override option exists → `NFR-CONC-004`. |

Applies to: issue update and status change, project update, sprint
update and lifecycle transitions, capacity period mutations, and — from
**0.27.0** — **all four destructive deletes** (project, issue, planned
sprint, completed sprint).

**On the deletes specifically.** Step 1 is the confirmation screen's hidden
`client_updated_at` field rather than an edit form's, and step 4's "the entity
is not modified" means the entity is **not deleted**: the user returns to a
confirmation that has gone stale and re-reads what they are about to remove.

The window this closes is one **this document created**. Before `§17.4`'s fix a
delete was a single `POST` from a page in front of the user. The interstitial
introduced a `GET`, a pause, and a `POST` — and for two of the four routes
nothing bound the third step to what the first displayed. It matters most on
the issue route, where the confirmation names a **count** of sub-issues
(§10.3): a count is only true at the moment it is read, and the lock is what
carries that truth to the moment it is acted on.

Does not apply to: pure association changes such as sprint membership,
where convergent last-write-wins yields a coherent result either way
→ `NFR-CONC-007`.

*Design note on the absence of override.* An override would let one
member discard another's edit without seeing it. Requiring re-read
before re-submission is a simpler invariant and a safer one, and it
avoids handing administrators a privileged overwrite that would sit
badly with the management-not-oversight principle.

---

## 16. Traceability

| Requirement area | Realising sections |
|---|---|
| `FR-AUTH-*` | §6 SCR-02/03, §7, §9 |
| `FR-NAV-*` | §3.2, §3.4, §4.3, §5.3 |
| `FR-PROJ-*` | §6 SCR-06/08/09 |
| `FR-ISS-*` | §6 SCR-10/11/12, §7 |
| `FR-SUB-*` | §6 SCR-11/13, §12 |
| `FR-TEAM-*` | §6 SCR-16, §10.3 |
| `FR-SPR-*` | §6 SCR-20 |
| `FR-PLAN-*` | §6 SCR-26 |
| `FR-HLT-*` | §6 SCR-08 |
| `FR-PER-*` | §6 SCR-04 |
| `FR-NTF-*` | §6 SCR-05, §11 |
| `FR-SCH-*` | §3.2, §6 SCR-24 |
| `FR-CAL-*` | §6 SCR-27/28 |
| `FR-API-*` | §8 |
| `NFR-PRIV-*` | §2.2, §2.3, §6 SCR-04, §9 |
| `NFR-CONC-*` | §15 |
| `NFR-A11Y-*` | §5.5–5.7, §13 |
| `NFR-LANG-*` | §5.2, §10 |

---

## 17. Known design gaps

Carried forward from `requirements.md` §10, restated as external-design
consequences.

### 17.1 Health presentation exceeds the severity ceiling — **closed at 0.20.1**

The project detail screen rendered a headline score ("Score N / 100"), exposed
a severity above `Watch`, and used danger colouring for it. All three
contradicted `SPEC §28.2`, `§28.4`, `§28.5` and `GUI §9`, and were recorded as
violations of `FR-HLT-008` and `NFR-LANG-002`.

**The position held: the implementation was brought to the specification, and
the specification was not relaxed.** SCR-08 now renders the composite at equal
weight with the other indicators, with no 0–100 figure, no severity label above
`Watch`, and no danger colour.

*Closed across two releases, which is the part worth recording.* 0.20.0 fixed
the badges and glyphs by attaching the clamp to the type they render from.
0.20.1 fixed the **summary sentence directly beneath them**, which selected
from the internal four-state model and had never passed through that type — a
third instance of the same violation, on the same screen, missed by a release
whose purpose was to correct the first two.

The closing fix is structural rather than local: the two message-table entries
capable of naming an unclamped severity were deleted, so no caller in any
crate can construct such a sentence. **A ceiling enforced on a render path
holds for that path; a ceiling enforced on the vocabulary holds everywhere.**

### 17.2 Explainability affordances incomplete — **closed, partially**, at 0.29.0

Two obligations in this document were unimplemented:

- **Indicator basis route** (SCR-08): no path from an indicator to its
  underlying issues, calculation, and history → `FR-HLT-007`.
- **Chart alternatives** (SCR-20): no tabular equivalent or textual
  summary → `NFR-A11Y-003`.

**`NFR-A11Y-003` is met in full.** Both sprint charts carry a table and a
summary, and both inherit the suppression rather than reimplementing it.

**`FR-HLT-007` is met on two limbs of three.** Basis and calculation ship;
**history does not**, and will not until an indicator's time series can be
shown without showing one contributor's history with it — the same ground
`NFR-PRIV-007` suppressed the sprint chart on at 0.28.0.

**And one indicator is excepted rather than served.** `wip_compliance` has no
basis route: the issues behind it are one person's assignment load, and a link
labelled "see the issues behind this" that discloses exactly what
`NFR-PRIV-007` withholds would be a privacy hole wearing an accessibility
label. The route returns 404 for it. That exception is the owner's amendment
and is recorded in the requirement itself, not only here.

**This entry is closed and the condition it blocks is not fully met.** The
Definition of Done's "Explainable" item moves to *"Met, with `FR-HLT-007`'s
history limb outstanding"* — not to *"Met"*.

*Worth recording: the first design for the basis route was wrong.* It was to
be query filters over the existing issue list; **three of the six indicators
could not be expressed as a filter**, and building the missing filters would
have put the staleness clock in two places. The shipped design returns each
indicator's membership from the computation that produced its count, so a
count and its basis cannot disagree. It was caught by the implementer
reproducing the architect's table before building on it.

### 17.3 Inbox affordances pending — **closed at 0.24.0**

Mark-all-read, the silence-resume banner, and the deferred email opt-in
were specified in §6 SCR-05 and RFC 0003 but not built.

*Closed at 0.24.0* (RFC 0003 as rewritten, handoff `INBOX-001`) — with a
correction to this entry's own premise. **Mark-all-read was already built**,
at 0.9.0–0.16.0, hide-when-zero included. It was listed as pending here and in
RFC 0003, and in both cases nobody checked. The other two shipped.

The email opt-in is the one worth recording: it did not move because the inbox
was a nicer home. The settings-page prompt could be answered **before the user
had received any notification**, which is precisely the state `FR-NTF-007`
exists to prevent, so the old surface was a standing violation and was removed
rather than kept alongside the new one.

---

### 17.4 Destructive actions lose their confirmation without JavaScript — **closed at 0.25.0**

*Added at 0.21.0.*

Nine destructive actions — project delete, issue delete, team leave, member
removal, project detach, two sprint deletes, capacity-row removal, and
silence-all — confirm via `onsubmit="return confirm('…')"`. With JavaScript
unavailable the handler never runs and **the action proceeds without any
confirmation at all**.

This contradicts §7 rule 4 (destructive actions are confirmed) and inverts
`DEC-021`, which permits JavaScript only as progressive enhancement over a
working no-JS path. Here the no-JS path is not degraded; it is more dangerous
than the enhanced one.

**External design position.** Confirmation belongs in a server-rendered
interstitial — a `GET` that states what will be deleted and a `POST` that does
it — with the `confirm()` dialog retained only as an enhancement that skips a
round trip. That shape needs an owner decision before it is specified here,
because it adds a screen to five flows.

Recorded rather than fixed in 0.21.0: the nine strings are copy, and this is
not a copy defect. They are allowlisted in `prose_scan.rs` with that reasoning
attached, so the exclusion is visible in the code rather than only here.

*Closed at 0.25.0* (RFC 010, handoff `CONF-001`), and the shape shipped is
**not** the one proposed above.

**Four of the nine, not nine of nine.** The interstitial went to project
delete, issue delete, planned-sprint delete and completed-sprint delete —
three screens, SCR-29 to SCR-31, since the two sprint cases share a route.
The other five — leave team, remove member, detach project, remove capacity
row, silence all — are **reversible through the interface**, so a dialog that
vanishes without JavaScript costs nothing that cannot be undone. Confirming
them on a server-rendered screen would have added a round trip to five flows
to protect against an outcome the user can reverse in one click.

**The `confirm()` dialog was not retained as a skip-the-round-trip
enhancement** on the four. Keeping it would mean two confirmation mechanisms
for one action, with the safe one reachable only when the other fails — which
is the shape that produced this gap. The four confirm on a screen, always.

**One behaviour beyond confirmation.** The sprint route refuses an `Active`
sprint outright rather than confirming it, and says to complete the sprint
first (`FR-SPR-002`). The route had accepted any status, so a team's running
sprint could be deleted with one confirmation. That was found while building
the interstitial, not by the design that asked for it.

*Verification*: `confirmation` suite, 11 tests — four interstitials, the
active-sprint refusal, and assertions that the five reversible cases were
left alone.

### 17.5 Assignment is limited to the project owner — **closed at 0.22.0**

*Added at 0.21.0.*

In a team-owned project, the assignee control offers exactly one person — the
project owner — and submitting any other user is rejected as a validation
error. Team members cannot be assigned work in their own team's projects.

Externally this looks like a nearly-empty dropdown rather than an error, so it
reads as "nobody else is available" rather than as a defect. The per-user
workload strip on the same screen shows one chip for the same reason, and
every non-owner's personal sustainability surfaces are permanently empty
because nothing can be assigned to them.

**External design position.** The assignee set for a team-owned project is its
team's membership plus the owner; for a personal project, the owner alone.

*Closed at 0.22.0* (RFC 009, handoff `TEAM-001`), with `viewer` excluded —
read-only on team projects by the team schema's own design.

**The privacy question this entry raised was withdrawn, not answered.** RFC 009
held the per-user workload strip behind an owner decision on whether more than
one row should be visible to all team members. `NFR-PRIV-002` already permits
sharing workload distribution, and `ISSUE-003` had already ruled it holds
regardless of how many members a surface lists. There was no gate to defer
behind, and the deferral was additionally incoherent: the strip iterates
whatever the query returns, so "make the two queries agree" and "do not widen
the consumers" could not both hold.

### 17.6 The enhanced path is executed by no test — open

*Added at 0.26.0.* §5.8's left-hand column — type-ahead, drag, in-place status
change, the undo toast, the client's `409` handling — lives in `static/*.js`,
and the test harness drives HTTP without executing scripts. Every behaviour in
that column is verified by reading and by hand.

The right-hand column is tested in full, and that is the mitigation rather
than a consolation: `DEC-021` makes every entry in the left column an
enhancement over a tested entry in the right, so a total failure of those
files degrades to asserted behaviour rather than to nothing.

**Recorded here rather than left implicit** because this document's readers
infer coverage from the absence of a caveat, and 0.26.0 is the first release
where user-visible behaviour ships in code no gate executes. Requirements
baseline `§10.15`. Not scheduled: closing it means a headless browser in CI.

---

### 17.7 Touch targets do not conform — **closed with a named population**, 0.32.0

**139 interactive elements do not present a 44 × 44 px target** → `§5.7`,
`NFR-A11Y-007`. Open since 0.19.1; the oldest condition in the Definition of
Done and the last unmet limb of *"Reach"*.

*Amended rather than merely re-stated at 0.30.0 (`DEC-049`, RFC 012).* The
requirement was unverifiable in one direction and misleading in another, and any
pass run against its previous wording would have inherited both faults:

- It said *"touch target"* while reading as visible size, so conformance looked
  like a density decision it never was.
- It stated a bare size floor with no adjacency clause — which, applied
  literally by expanding hit areas, **creates overlapping targets**, a worse
  defect than the one it fixes.

`§5.7` now states both clauses. **Nothing about the 139 has changed**; what
changed is that the rule they fail is now one a guard can enforce.

**Exactly three controls conform** — `confirmation.rs:53`, `confirmation.rs:58`
(raised by `QA-015` because the pair sat 32 px apart with one of them
irreversible) and `issues.rs:825`, the board card's status buttons. That last
one matters to the record: the board is the densest surface in the product, it
has carried 44 px targets for releases, and it was cited in this RFC's own first
draft as the reason a uniform floor could not work.

**Closed in 0.31.0.** All 139 controls reach a 44 px target — 136 by growing,
three checkboxes by a `<label>` wrap that keeps each box at its native 24 px.
The value has one home in Rust, and a structural guard makes a control without
it unconstructible, with no exception list.

**The named limit was measured at 0.32.0 and removed** (`DEC-050`, `TT-004`). It
was not narrow: an account menu on every page, `<summary>` disclosure toggles at
16 px — which `§5.7` names by hand — breadcrumbs, and the indicator basis links.
The rule now covers **every** `<a>`, `<button>` and `<summary>`, with **one
declared exception** (a link inside running text, marked in the markup; exactly
one site) and **three call sites excluded by design**: the calendar's event chips,
whose block height is proportional to an appointment's duration and would
misrepresent the schedule at a 44 px floor, and the per-indicator "why" toggle,
which at 44 px would dominate the health data it annotates.

**Clause (2) has no open gap for anything now in the tree**: `Grow` inside a
positive `gap` is structurally safe, and the single `Expand` — the checkbox
label — participates in layout, so it keeps the same protection. Full
verification of the clause in general still wants rendered geometry, which is
`§17.6`'s question and does not arrive before 0.32.0.

*What this entry cost to close, recorded because the outcome hides it.* Two
architect recommendations were reversed by evidence already in the tree — the
density objection, and the mechanism preference — and three of the four review
rounds across `TT-002` and `TT-003` corrected gaps in the handoffs rather than
in the work.

---

### 17.8 The rendered layout is observed by one assertion — **one property gated**, 0.33.0; the rest open

**`§17.6` says the enhanced path is executed by no test. This is its
counterpart, and nobody had written it down**: no test in this project observes
what the browser actually lays out.

The suite drives HTTP and reads markup. It can assert that a class is present in
a response; it cannot assert that the element is 44 px, that the page does not
scroll sideways, or that two controls do not overlap.

**Three defects at 0.32.0 came from this gap**, found in three days by one
inspection with a headless browser — and a fourth entry that turned out to be
the inspection's own error:

| | |
|---|---|
| `§10.18` | every authenticated page scrolled sideways when the signed-in email was long |
| `§10.20` | the project toolbar could not wrap below 390 px |
| `§10.19` | the adjacency guarantee was narrower than `DEC-049` claimed |
| ~~`§10.21`~~ | *withdrawn 2026-09-10 — **not a defect**, a measurement artefact of my own sweep: unrendered content inside a closed `<details>` still reports layout geometry* |
| `§10.24`, `§10.25` | thirteen surfaces overflowed on unbreakable user text, in three shapes with non-interchangeable remedies — found by the gate's own fixture as each field gained an unbroken run, and by pages as they joined its list |
| `§10.26` | every page overflowed a 320 px phone for a 21-character display name — found by the gate's first run at 320 |

**None was reachable by any discipline already in use.** The markup is correct in
every case; the defects live in what the layout engine does with it.

**It also cleared a worry**, which is worth recording because a gap that only
ever produces bad news gets treated as an alarm: the vertical-centring concern
across 57 inputs and selects was measured and dismissed.

**One property is gated, and only one.** `BROWSER-001` (0.33.0, RFC 011 step 4,
`DEC-048`) asserts `scrollWidth <= clientWidth` on **eighteen rendered pages at
five widths** (320, 390, 768, 1280, 1920), reporting every failing cell rather
than the first. It is **a CI job, not a `cargo test`**: a CDP-driven Rust test
would have put Chromium on every contributor's `cargo test --workspace`,
including `DEC-007`'s three-run gate before every release. It is a gate on the
footing `fmt` and `clippy` already have, and it is **not counted** in the
project's test inventory. It was **watched red before it was trusted**: four
planted defects — `§10.18`'s cause, `§10.20`'s, `§10.24`'s and `§10.26`'s —
each fail it, each restored and rebuilt before the next.

**What the gate sees is exactly its fixture and its page list, and 0.33.0 showed
this four times.** The gate was green on a tree with three overflow defects
because its fixture title had a space at every point; green on two more pages
because they were not in its list; green on two subtitles because its fixture
*display name* had a space at every point; green on the sprint pages because it
had no sprint. Each correction was to the fixture or the list, watched red first. The fixture now carries a long unbreakable email, a
190-character title, and a 64-character unbroken run in the title, project name,
team name, display name, sprint name and sprint goal. **A reader of a green
run should read the coverage line beside it**, not the word "gate".

**Deliberately one assertion.** Rendered target size is step 4's second-ranked
candidate and is more sensitive to browser version. **Overlap is last, and as
of 2026-09-16 it is decided rather than deferred: it does not become a second
assertion.** `TT-005` measured it instead — every interactive pair, four
widths, `<details>` closed and open, on two trees three releases apart — and
found **one** overlap class in the product: `join`-grouped segmented buttons
sharing a 1 px column, which is the collapsed border that makes them look like
one control. A gate assertion for overlap would therefore ship with a `join`
exception on its first day, and an exception list in a gate is the weakening
pattern `touch_target_scan`'s own doc names. The requirement carries the carve-
out instead (`NFR-A11Y-007`, amended), where it can be read.

**Overlap measurement has now produced three artefacts and one fact**, which is
the other half of the reason: a closed `<details>` reporting geometry it does
not occupy (`§10.21`), a dropdown's close-animation `scale(0.95)` reading
41.8 px for a 44 px control, and the two `<summary>` pairs `§10.19` recorded and
`TT-005` withdrew. The fact is the `join` seam. **A measurement whose false
positives outnumber its findings three to one is not a gate candidate**, and
saying so is worth more than a fourth attempt.

One assertion proved stable before a second was considered; the second is now
declined on evidence. Nothing here observes overlap, size as rendered, or
anything visual.

`ASSET-001` is what made any of it possible: `DEC-048` condition 1 forbids a gate
that can fail because a CDN is slow, and until 0.32.0 every page fetched CSS from
two of them.

**The mitigating property**: every finding so far has been a degradation, not a
failure. Pages remained usable, links worked, forms submitted. That is `DEC-021`'s
posture holding, and it is why this is recorded rather than treated as urgent.

## 18. Supersession of the prior GUI specification

The GUI External Specification v0.1 remains valuable for component
naming (`GUI §8`), element roles (`GUI §4`), and acceptance criteria
(`GUI §9`). It has gone stale in four respects, all resolved in favour
of this document:

| Point | GUI v0.1 | This document | Reason |
|---|---|---|---|
| Personal dashboard route | `/me` | `/today` | Renamed in Phase A; `/me` 308-redirects |
| Inbox route | `/notifications` | `/inbox` | Renamed in Phase A; legacy 308-redirects |
| Navigation entries | Dashboard, Projects, Teams, Notifications, Settings | Today, Inbox, Projects, Teams, Search (+ Settings) | `SPEC §4.2` five-entry model |
| Cross-user personal data | "Return 404 or access-safe denial" (`GUI §3.4`) | **403 where the request names a user; 404 where it names a resource** | `SPEC Appendix E.2`. *Corrected 2026-09-23: this row read "403, uniformly", and "uniformly" was wrong — the capacity-row mutations return 404. The enumeration argument that justifies 403 applies to a **user id** in the path, where 403-versus-404 would distinguish a real account from a fake one; it does not apply to a **row** id, where 404 is returned identically for absent and not-yours and so reveals nothing. The behaviour is right in both shapes; the word was the defect* |

Additionally, the GUI specification predates and therefore does not
cover: the sub-issue hierarchy (SCR-13, §12), the read/edit URL split
(SCR-11/12), the status segment (SCR-11), the Today panel structure
(SCR-04), search screens (SCR-24), and the calendar and sprint-plan
surfaces (SCR-26..28).

---

## Appendix A — Screen-to-requirement index

| Screen | Primary requirements |
|---|---|
| SCR-02 Login | `FR-AUTH-002` |
| SCR-03 Register | `FR-AUTH-001`, `FR-NTF-007` |
| SCR-04 Today | `FR-PER-001`, `FR-PER-006`, `FR-PER-007`, `NFR-PRIV-001` |
| SCR-05 Inbox | `FR-NTF-004..007`, `FR-NTF-009` |
| SCR-06 Projects | `FR-PROJ-001`, `FR-PROJ-003` |
| SCR-08 Project detail | `FR-PROJ-004`, `FR-HLT-001..009`, `FR-SUB-006`, `FR-NAV-005` |
| SCR-11 Issue detail | `FR-ISS-004..007`, `FR-SUB-001`, `FR-SUB-007`, `FR-NAV-004` |
| SCR-12 Issue edit | `FR-ISS-004`, `NFR-CONC-001` |
| SCR-13 Sub-issue create | `FR-SUB-001..004`, `FR-SUB-007` |
| SCR-16 Team detail | `FR-TEAM-001..005`, `NFR-PRIV-003` |
| SCR-20 Sprint detail | `FR-SPR-001..003`, `FR-SUB-008`, `NFR-A11Y-003` |
| SCR-22 Settings | `FR-SET-001`, `FR-PER-002`, `FR-PER-003` |
| SCR-24 Search | `FR-SCH-001..004` |
| SCR-26 Sprint plan | `FR-PLAN-001..005` |
| SCR-27/28 Calendar | `FR-CAL-001..008` |
| SCR-29 Delete project — confirm | `FR-PROJ-001`, §7 rule 4 |
| SCR-30 Delete issue — confirm | `FR-ISS-001`, §7 rule 4 |
| SCR-31 Delete sprint — confirm | `FR-SPR-001`, `FR-SPR-002`, §7 rule 4 |

*Corrected at 0.26.0.* These three rows read `SCR-P-01` and `SCR-P-02/03`
until now. §3.3 retired those provisional identifiers at **0.24.0**, saying in
the same breath that "a provisional identifier that outlives its provisional
state is a second name for one screen" — and the second name then survived in
**eight** other places: this appendix, §3.4's transition map, both §6 screen
headings (still marked *(planned)* for screens that shipped at 0.22.0 and
0.23.0), two §14 rows, two §16 traceability rows, and §18. All corrected here.

Renumbering a table is not finished until every index into it has been
followed. The 0.24.0 amendment renamed the definition and left the references,
which is the same defect as a route table that no longer matches the router —
and it survived a release whose own note was about this document being stale.

## Appendix B — Route-to-screen index

Implemented routes are listed in §4.2; reserved routes in §4.4. Every
route in §4.2 maps to exactly one screen in §3.3 or to a mutation
action documented in §7.

### 17.9 The order a list presents its items in was specified nowhere — **opened and closed at 0.38.0**

This document is the record of what the product presents. Through 0.37.0 it
stated, for every list-bearing screen, **what the list contains and not the
order it is in.**

Four orderings were wrong at once (`§10.29` in the requirements): the issue
list led with Done issues, the sprint-plan backlog put `high` below `low`, a
sprint's issues led with Done, and 22 timestamp orderings carried no tiebreak.
**None of them was a divergence from this document**, because this document
said nothing for them to diverge from — so a reader checking the implementation
against the external design would have found it conformant on every one.

That is the same shape as `§17.8` and `§10.27` in a third place: **a property
nobody stated cannot be checked by anything**, whether the checker is a gate,
a test or a person reading the specification.

**Closed in the same release it opened.** `§5.9` now states the order of every
list-bearing screen, as an obligation rather than a description, so the next
wrong ordering is a divergence a reader can find. It is **one table rather than
a line per screen**, deliberately: all four defects came from an order being
expressed in more than one place or in none, and a per-screen line would have
been nine places to disagree. `§5.9` also carries the two rules underneath the
four — that an order must be total, and that it must not be carried by a
value's spelling.

**What this does not close**: nothing verifies these lines. They are
obligations a reader can check, in a document a reader can read — which is one
more than existed before and fewer than a test. `§17.6` and `§17.8` remain the
entries about what is executed by nothing.

## Appendix C — Change history

| Version | Change |
|---|---|
| GUI v0.1 | Original GUI external specification, based on `0.16.0` |
| **0.38.0 (this baseline)** | **Six surfaces change the order in which they present their items, and none of them changes what it contains.** The issue list no longer leads with Done issues — it is newest first; each board column reads newest first where it read oldest first; the sprint-plan backlog reads urgent, high, medium, low, where `high` sorted **last, below low**; a sprint's issues read Open, In progress, Done, where Done led; lists of items created in the same second read newest first instead of backwards; and `sort=priority`'s tie order follows the new storage order. **`§17.9` opens and closes in the same release**: this document stated the *contents* of every list and the *order* of none, which is why four wrong orderings were invisible to anyone checking the implementation against it. §6's list-bearing screens now carry an order line. **No screen, route, status code or copy string changed**, and the one behaviour change that is not presentation is the capacity overlap check, which now refuses concurrent overlapping saves rather than storing both |
| 0.37.0 | No externally observable behaviour changed. The status table's cross-user rows, split at 0.36.0 into *named by a user id* (403) and *named by a resource id* (404), are now **asserted** rather than only measured: three tests make the cross-user attempt on the capacity-row mutations and one covers the unauthenticated case for two more endpoints |
| 0.36.0 | **`SCR-27`/`SCR-28`'s day view gains a drag to reschedule** — one action, one view: the day view is the only one whose vertical axis is time, so a drag distance means a duration. The block's markup becomes a wrapper carrying identity and position around the same `<a>`, which keeps its `href` and its `bg-primary/15` — the latter because that class is the key `touch_target_scan` uses to recognise `DEC-050`'s duration-proportional exclusion. **No new interactive element**, so the exclusion is not extended. A reschedule can conflict, and the page reloads showing the current state rather than overwriting the other writer's change |
| 0.35.0 | **`SCR-26`'s obligation line changes for the first time since 0.22.0**: button-driven moves *and* drag, rather than "not drag", now that `FR-PLAN-002`'s deferred half has shipped. The screen's responsive row — *"columns stack; move actions remain, drag does not"* — was written before the drag existed and is **correct as written**, which is worth recording: it anticipated the touch limit that RFC 0004 has now made a cross-cutting requirement. Nothing else about the screen's externally observable behaviour changed: the same two endpoints, the same permissions, the same filters in the URL |
| 0.34.0 | **§3.1's page-title-row obligation is now met on every page that has one** (`LAYOUT-009`): a header pairing a title with actions wraps rather than squeezing the title, and the actions move below only when the row cannot fit. Three headers; a sweep of every `justify-between` row found no fourth. **§17.8 gains a decision rather than a deferral**: overlap does not become a second gate assertion, because measuring it has produced three artefacts to one fact, and the one fact — a `join` group's 1 px seam — is deliberate, so the assertion would ship with an exception on its first day. The requirement carries the carve-out instead, where a reader can find it |
| 0.33.0 | **The rendered layout is observed by a check for the first time** (`§17.8`): one assertion, eighteen pages, five widths, a CI job outside the suite. **§3.1 gains two obligations the product already had to meet and never wrote down**: the header row fits the viewport at every width, with the account control's name the only thing that yields; and user-supplied text never widens the page, in either of the two shapes it can fail in. Both came from defects — nine surfaces on unbreakable text (`§10.25`), every page at 320 px for a 21-character name (`§10.26`) — and both are now observed by the gate rather than asserted by this document alone. **The release's lesson is the gate's own**: three times it was green on a defect in front of it, because what it sees is exactly its fixture and its page list |
| 0.32.0 | **Front-end assets are served by the application, not by two CDNs** (`DEC-051`). Measured with both blocked, the product had been rendering **unstyled** — `.btn` 44 px → 17 px, Times New Roman, the account menu unable to collapse — while `NFR-CMP-002` claimed self-hostable. 451 KB of runtime JIT compiler became 14 KB of static CSS; DaisyUI is vendored as the prebuilt stylesheet it already is. **§17.7 closes with a named population**, not a named limit: `§5.7`'s rule now covers every `<a>`, `<button>` and `<summary>`, with one **declared** exception and three call sites excluded by design — the calendar's event chips, whose block height is proportional to an appointment's duration and would misrepresent the schedule at a 44 px floor, and the per-indicator *why* toggle. **§17.8 opens**: the rendered layout is observed by no test, the counterpart to §17.6 that nobody had written down |
| 0.29.0 | Fourth consecutive amendment at the release — `DEC-028`'s criterion has now been met every release since 0.26.0. **§17.2 closes, partially**, and the word carries weight: `NFR-A11Y-003` is met in full, `FR-HLT-007` on two limbs of three, with history deferred and one indicator excepted rather than served. §3.3 gains SCR-32 and §4.2 one route. SCR-08's basis obligation and SCR-20's tabular obligation are both rewritten from *not implemented* to what shipped. **The design recorded here is the second one**: the first — query filters over the issue list — could not express three of the six indicators, and was caught by the implementer reproducing this document's own table before building on it. Two acceptance-axis rows lose their `(gap §17.2)` marks |
| 0.28.0 | Third consecutive amendment at the release. The muted text floor moves `/60` → `/70` across every screen — 111 sites measured below AA, with the four-steps-to-two cost stated rather than implied. §15 gains an assertive live region beside the polite one, conflict announcements moving to it. The sprint burndown and the completed-work chart's median line are suppressed below two distinct contributors, with **no copy explaining the absence** — the explanation would disclose what the suppression withholds. States plainly that touch targets remain below this project's own 44 px target at 139 controls |
| 0.27.0 | Amended at the release for the second consecutive time. §15 gains all four destructive deletes — and the window it closes is one **this document created**, since §17.4's interstitial introduced a `GET`/pause/`POST` where a single `POST` had been. §4.2 records the narrowed request contract on the two delete routes, with all four request shapes and their responses. §10.3 gains the three delete-confirmation consequences, including the issue cascade the 0.25.0 screen omitted on a route that has cascaded since migration `0015`, and this project's first pluralised count in prose |
| 0.26.0 | Covers 0.25.0 and 0.26.0, amended **at the release rather than three releases later** — the first time `DEC-028`'s criterion has been met since it was written. §17.4 closes, and the shape that shipped is not the one this document proposed: four flows gain a server-rendered interstitial (SCR-29 to SCR-31), the other five keep their dialogs because each is reversible, and the `confirm()` dialog was **not** retained as an enhancement on the four. §3.3 gains three screens; §4.2 gains three `GET` halves and three status routes, and records that `change_status` now answers `200` with the new `updated_at` where `/status/board` answered `204`. §5.8 is rewritten per surface — its "no flow depends on scripting" line had been false for four releases, which is what §17.4 was. §17.6 opens: the enhanced path is executed by no test. §10.3 records the board's three sentences moving into the message table, and undo's two distinct failure messages. **0.24.0's `SCR-P-*` retirement is finished** — the provisional identifiers had survived in eight places, including two §6 headings still marked *(planned)* for screens that shipped at 0.22.0 and 0.23.0 |
| 0.25.0 | Covered by the 0.26.0 amendment above rather than separately: the two releases are one design change, shipped in the order `DEC-021` requires — the no-JavaScript path at 0.25.0, the enhancement over it at 0.26.0 |
| 0.24.0 | Covers 0.22.0, 0.23.0 and 0.24.0 in one amendment, the criterion having been missed at each. §3.3's three `SCR-P-*` provisional identifiers become `SCR-26/27/28` and the provisional forms are retired. Six reserved HTML routes move to §4.2 — RFC 0003's `/inbox/silence/resume` shipped as `/inbox/resume`. New §4.5 records the one removed route and why it gets no redirect. §10.3's calendar footnote moves from planned to shipped, with the note that RFC 0002 carried a conflicting normative version of it. §17.3 and §17.5 close; §17.5's privacy question was withdrawn rather than answered |
| 0.21.0 | §10.1 records that the vocabulary table is now enforced by test rather than by convention, and points at `requirements.md` §1.7.1/§1.7.2. §10.3's team footnote corrected **to the shipped copy**, with the reasoning; the concurrency-conflict text moves from planned to shipped and carries two mechanism defects. §17.1 closes. §17.4 (destructive confirmation without JavaScript) and §17.5 (assignment limited to the project owner) open |
| 0.19.1 | Consolidated external design at `0.19.1`: screen identifiers assigned; routes reconciled with shipped implementation; sub-issue, edit-split, status-segment, search, calendar, and sprint-plan surfaces added; 403-not-404 rule corrected; requirement traceability established; design gaps recorded |
