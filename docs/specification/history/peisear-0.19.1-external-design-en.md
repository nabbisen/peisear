# peisear — External Design Specification

**Document type**: External design (basic design)
**Document status**: Baseline
**Covers release**: `0.19.1` (implementation through `0.19.0`)
**Language**: English (normative)
**Prepared**: 2026-07-27
**Intended repository location**: `docs/src/external-design.md`

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
| Header | `<header>` | Persistent. Carries search and unread count. |
| Search box | `<form role="search">` | MUST function without scripting (submits to `/search`). |
| Unread count | text | MUST be exposed as text, never colour alone → `NFR-A11Y-004`. |
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
| SCR-P-01 | Sprint plan | `/teams/{slug}/sprints/{sprint_id}/plan` | Planned (RFC 0001) | `FR-PLAN-001..005` |
| SCR-P-02 | Personal calendar | `/today/calendar` | Planned (RFC 0002) | `FR-CAL-001` |
| SCR-P-03 | Project calendar | `/projects/{id}/calendar` | Planned (RFC 0002) | `FR-CAL-002` |

Capacity and WIP-limit management are sections within SCR-22 rather
than separate screens; their mutation endpoints are listed in §8.

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
                                                            │ SCR-P-01 │
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
| GET | `/projects` | SCR-06 | User |
| GET, POST | `/projects/new` | SCR-07 | User |
| GET | `/projects/{id}` | SCR-08 | Project access |
| GET, POST | `/projects/{id}/edit` | SCR-09 | Owner / admin |
| POST | `/projects/{id}/delete` | Delete project | Owner / admin |
| GET, POST | `/projects/{id}/issues/new` | SCR-10 | Write access |
| GET, POST | `/projects/{id}/issues/{issue_id}` | SCR-11 (GET), update (POST) | Read / write |
| GET | `/projects/{id}/issues/{issue_id}/edit` | SCR-12 | Write access |
| POST | `/projects/{id}/issues/{issue_id}/status` | Status transition | Write access |
| POST | `/projects/{id}/issues/{issue_id}/delete` | Delete issue | Write access |
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
| POST | `/teams/{slug}/sprints/{sprint_id}/delete` | Delete sprint | Admin |
| GET | `/settings` | SCR-22 | Self |
| POST | `/settings/wip-limit` | Set WIP limit | Self |
| POST | `/settings/capacity` | Create capacity period | Self |
| POST | `/settings/capacity/{id}` | Update capacity period | Self |
| POST | `/settings/capacity/{id}/close` | Close open-ended period | Self |
| POST | `/settings/capacity/{id}/delete` | Delete capacity period | Self |
| GET, POST | `/settings/notifications` | SCR-23 | Self |
| POST | `/settings/notifications/silence-all` | Silence all | Self |
| POST | `/settings/notifications/ack-global` | Acknowledge global note | Self |
| GET | `/search` | SCR-24 | User |
| GET | `/static/*` | Static assets | Any |

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
| `/teams/{slug}/sprints/{sprint_id}/plan` | SCR-P-01 | RFC 0001 |
| `/teams/{slug}/sprints/{sprint_id}/plan/add` | SCR-P-01 action | RFC 0001 |
| `/teams/{slug}/sprints/{sprint_id}/plan/remove` | SCR-P-01 action | RFC 0001 |
| `/today/calendar` | SCR-P-02 | RFC 0002 |
| `/projects/{id}/calendar` | SCR-P-03 | RFC 0002 |
| `/inbox/silence/resume` | Inbox action | RFC 0003 |
| `/inbox/email-opt-in` | Inbox action | RFC 0003 |
| `/api/users/{user_id}/wip-limit` | API | `SPEC E.1` |
| `/api/users/{user_id}/calendar` | API | `SPEC E.1` |

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
target area → `NFR-A11Y-007`. This applies especially to status badges
in lists, disclosure toggles, and inbox rows.

### 5.8 Behaviour without JavaScript

| Feature | With script | Without script |
|---|---|---|
| Search | Type-ahead dropdown | Form submit to `/search` |
| Kanban board | Enhanced interactions | Static columns |
| All forms | Identical | Identical |
| All navigation | Identical | Identical |

No flow depends on scripting.

---

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
| Basis link | Route from each indicator to underlying issues, calculation, history → `FR-HLT-007` *(not implemented; §17.2)* |

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
| Tabular equivalent | → `NFR-A11Y-003` *(not implemented; §17.2)* |

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

### SCR-P-01 · Sprint plan (planned)

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

Obligations: button-driven moves, not drag → `FR-PLAN-002`; filters in
the URL → `FR-PLAN-003`; capacity hint advisory only → `FR-PLAN-004`;
team-scoped backlog → `FR-PLAN-005`; top-level issues only
→ `FR-SUB-006`; completed sprints render read-only → `FR-SPR-004`.

---

### SCR-P-02 / SCR-P-03 · Calendar (planned)

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
| Cross-user personal data | 403 | JSON / page | `FR-API-003` |
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

> Project trends and workload distribution are visible to all members.
> Personal sustainability data (burnout panel, /today) is visible only
> to the individual concerned. Admin is a management role, not an
> oversight role.

**Project calendar footnote** (planned):

> Calendar note: this view shows planned issue work for this project.
> Personal schedules are not aggregated here. Each member's individual
> calendar is private to that person.

**Concurrency conflict** (planned, Phase D): states that another member
changed the item first and that the latest state is now shown. Neutral
tone, neutral colour, no failure vocabulary → `FR-DM-005`.

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
| SCR-08 Project | Required | Disclosure | Indicator state as text; chart alternative *(gap §17.2)* | Badge + glyph | Required | n/a |
| SCR-11 Issue | Required | Action order | Segment `aria-pressed`; group named | Badge + text | **Required** | n/a |
| SCR-12 Issue edit | Required | First error | Field labels; error association | n/a | Required | n/a |
| SCR-16 Team | Required | Table order | Footnote read in order | Badge + text | Required | n/a |
| SCR-20 Sprint | Required | Chart alternative | Caption + tabular data *(gap §17.2)* | Pattern + colour | Required | n/a |
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
| SCR-P-01 Sprint plan | Columns stack; move actions remain, drag does not |
| SCR-P-02/03 Calendar | Day view is the mobile default; month becomes a chronological list |

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
update and lifecycle transitions, capacity period mutations.

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
| `FR-PLAN-*` | §6 SCR-P-01 |
| `FR-HLT-*` | §6 SCR-08 |
| `FR-PER-*` | §6 SCR-04 |
| `FR-NTF-*` | §6 SCR-05, §11 |
| `FR-SCH-*` | §3.2, §6 SCR-24 |
| `FR-CAL-*` | §6 SCR-P-02/03 |
| `FR-API-*` | §8 |
| `NFR-PRIV-*` | §2.2, §2.3, §6 SCR-04, §9 |
| `NFR-CONC-*` | §15 |
| `NFR-A11Y-*` | §5.5–5.7, §13 |
| `NFR-LANG-*` | §5.2, §10 |

---

## 17. Known design gaps

Carried forward from `requirements.md` §10, restated as external-design
consequences.

### 17.1 Health presentation exceeds the severity ceiling

The project detail screen currently renders a headline score
("Score N / 100"), exposes a severity above `Watch`, and uses danger
colouring for it. All three contradict `SPEC §28.2`, `§28.4`, `§28.5`
and `GUI §9`, and are recorded as violations of `FR-HLT-008` and
`NFR-LANG-002`.

**External design position.** SCR-08 as specified in §6 above is the
target: composite shown at equal weight with the other indicators, no
0–100 figure, no severity label above `Watch`, no danger colour. The
implementation should be brought to this specification rather than the
specification relaxed — the ceiling is a product-defining commitment,
not a stylistic preference.

### 17.2 Explainability affordances incomplete

Two obligations in this document are unimplemented:

- **Indicator basis route** (SCR-08): no path from an indicator to its
  underlying issues, calculation, and history → `FR-HLT-007`.
- **Chart alternatives** (SCR-20): no tabular equivalent or textual
  summary → `NFR-A11Y-003`.

Both are named in the Definition of Done, so the "Explainable"
condition cannot be considered met until they land.

### 17.3 Inbox affordances pending

Mark-all-read, the silence-resume banner, and the deferred email opt-in
are specified in §6 SCR-05 and RFC 0003 but not built.

---

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
| Cross-user personal data | "Return 404 or access-safe denial" (`GUI §3.4`) | **403, uniformly** | `SPEC Appendix E.2`; 404 would allow account enumeration by status code |

Additionally, the GUI specification predates and therefore does not
cover: the sub-issue hierarchy (SCR-13, §12), the read/edit URL split
(SCR-11/12), the status segment (SCR-11), the Today panel structure
(SCR-04), search screens (SCR-24), and the calendar and sprint-plan
surfaces (SCR-P-01..03).

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
| SCR-P-01 Sprint plan | `FR-PLAN-001..005` |
| SCR-P-02/03 Calendar | `FR-CAL-001..008` |

## Appendix B — Route-to-screen index

Implemented routes are listed in §4.2; reserved routes in §4.4. Every
route in §4.2 maps to exactly one screen in §3.3 or to a mutation
action documented in §7.

## Appendix C — Change history

| Version | Change |
|---|---|
| GUI v0.1 | Original GUI external specification, based on `0.16.0` |
| **This baseline** | Consolidated external design at `0.19.1`: screen identifiers assigned; routes reconciled with shipped implementation; sub-issue, edit-split, status-segment, search, calendar, and sprint-plan surfaces added; 403-not-404 rule corrected; requirement traceability established; design gaps recorded |
