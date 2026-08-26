# QA-007 — Authorization audit

**Issued by**: Architect
**Date**: 2026-08-25
**Priority**: P0 — 0.27.0
**Governing RFC**: [005](../../done/005-quality-consolidation.md) §1
**Depends on**: `QA-005` should land first, as for `QA-006`. Independent of
`QA-006` — that one audits locks, this one audits boundaries. Either order.

---

## 1. What this is

RFC 005 §1's table, filled in for every endpoint carrying personal data
(`§11.5.1`) or per-user mutation. Reconciled against the router on
2026-08-25: **seven rows were added** — two from 0.24.0, five from 0.25.0 —
and all seven are unaudited.

The row that said "(Phase D mutations — fill in as they ship)" was a promise
to a future that arrived twice while nobody's step was to fill it in.

## 2. The three `GET` delete rows are new in kind

Before 0.25.0, no `GET` in this application rendered a page whose only purpose
was to authorise a mutation. Three do now, and they are the priority:

| Route | Check in the handler |
|---|---|
| `GET /projects/{id}/delete` | `find_accessible` **plus an owner check** |
| `GET …/issues/{iid}/delete` | `find_accessible` |
| `GET …/sprints/{sid}/delete` | `can_manage_team` |

**`GET /projects/{id}/delete` is a narrower boundary than
`GET /projects/{id}`.** The interstitial refuses a non-owner with `NotFound`;
the project page it is reached from does not.

**Audit whether each `POST` half enforces exactly what its `GET` half does.**
A `GET` that refuses and a `POST` that does not is a boundary that exists only
in the user interface — the interstitial would look like a control while the
mutation stayed reachable by anyone who could construct the request. Prove it
by posting **without** the `GET`: a cross-user `POST` straight to each delete
route, asserting the same status the `GET` gives.

That test shape is the deliverable here, more than the table is.

## 3. The ignored test

RFC 005 §1 records `POST /settings/wip-limit` as `partial`, pending a
user-scoped POST surface, and refers to an ignored test. Requirements baseline
§9.3 says that test was **withdrawn** at 0.20.0, not ignored, with the cause
recorded: settings mutations are self-scoped by session rather than addressed
by `user_id` in the path, so the boundary it asserted cannot exist.

**Two of our documents disagree about the same test.** Establish which is true
by reading `tests/auth_boundary.rs`, and correct whichever is wrong — the
baseline is mine and the RFC is mine, so either correction is fine and leaving
both standing is not.

## 4. The rest of the table

For every row: name the auth check as it appears in the handler, name the
cross-user test that asserts it, and mark the status. Where a check is
`implicit (no user_id in the URL)`, say what makes it implicit — a session
extractor is not the same guarantee as a session extractor plus a scoped
query, and §10.3 (storage-layer authorisation absent) is open precisely
because the second layer does not exist.

**Do not add the storage layer in this handoff.** §10.3 is Phase E's own item
and is larger than an audit.

New tests land in `tests/auth_boundary.rs`.

**Report the rows that are already correct too.**

## 5. Escalate rather than deciding

- If any `POST` half accepts a request its `GET` half refuses, **stop and
  report before fixing.** That is a live authorisation defect and I want to
  see it before it moves.
- If a row's "implicit" check turns out to rest on a query that is not
  user-scoped, report it as a §10.3 instance rather than fixing it here.
- If §3's disagreement resolves in a third way — the test exists and is
  neither ignored nor withdrawn — say so.

## 6. Acceptance

1. RFC 005 §1's table complete — every row, every cell, correct rows included.
2. A cross-user `POST`-without-`GET` test for each of the three delete routes.
3. §3's contradiction resolved in whichever document is wrong.
4. Each new boundary test demonstrated failing with its check removed — one at
   a time.
5. fmt and clippy exit 0; `DEC-007` gate set green; three consecutive
   `cargo test --workspace` runs.

## 7. Required review-request format

Workflow §9.2. Include §2's `POST`-without-`GET` results per route, and state
plainly whether any boundary was found reachable.
