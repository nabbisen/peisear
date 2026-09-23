# PRIV-001 — seven assertions `NFR-PRIV-008` asks for and does not have

**Target release**: 0.37.0. **Governing RFC**: none — a P1 requirement's
coverage. **Source**: `.git-exclude/tasks/architect/019-nfr-priv-008-coverage-audit.md`.

## 1. What this is, and what it is not

`NFR-PRIV-008` (P1) asks that every endpoint in the personal-data inventory
have automated tests asserting **403 for another user, 403 for an
administrator, and 401 unauthenticated**. Its status reads *Partial*.

**I measured what is actually there. Seven assertions are missing across five
endpoints — and every behaviour they would assert is already correct.** I
probed each one over HTTP against a release build and each refused exactly as
it should. **These are regression tests for properties that hold. Nothing here
is a fix, and a failing test would be a surprise.** If one fails, that is an
escalation and a much bigger deal than this handoff.

## 2. What to add

### Class A — `/api/users/{user_id}/…`, where another user's id can be substituted

| endpoint | other user | administrator | unauthenticated |
|---|---|---|---|
| `burnout` | has it | has it | has it |
| **`capacity`** | has it | **add** | **add** |
| **`notifications`** | has it | **add** | **add** |

The administrator and unauthenticated cases exist once each, against `burnout`
only. Extend them to the other two — `team_admin_cannot_read_member_personal_data`
and `unauthed_api_users_returns_401_not_redirect` are the shapes to follow.

**Measured**: cross-user 403, anonymous 401, and a nonexistent user's id also
403 — identical to the cross-user code, which is `NFR-PRIV-006` holding. If you
add an assertion for that too, it belongs beside these.

### Class B2 — a **resource** id in the path, where another user's row can be named

| endpoint | cross-user attempt |
|---|---|
| `/inbox/{id}/read` | has it — 404 **and** the row untouched |
| **`POST /settings/capacity/{id}`** | **add** |
| **`POST /settings/capacity/{id}/close`** | **add** |
| **`POST /settings/capacity/{id}/delete`** | **add** |

**Follow `mark_read_does_not_affect_another_users_notification` exactly**: it
asserts the status *and* that the victim's row is unchanged afterwards. The
second half is the one that matters — a refusal that still mutated would pass a
status assertion.

**Measured**: all three refuse with **404**, and the row survives. Expect 404,
not 403 — §4 explains why that is right.

**These three requests carry `client_updated_at`.** A request without it fails
validation with 400 *before* reaching the ownership check, so a test that omits
it proves nothing. **Make the owner's identical request succeed first** in the
same test file, or you have not shown the refusal is about ownership.

## 3. The module doc, which is the reason this was missing

`auth_boundary.rs`'s own doc says settings mutations are *"not addressed by
`user_id` in the path, so 'cross-user POST' isn't expressible against them
today"*.

**Correct it.** It is true of `/settings/wip-limit`, which takes no path
parameter. It is **false** of `/settings/capacity/{id}`, which takes a row id —
naming another user's row is exactly the cross-user request, and I made it
three times. Say which routes the reasoning covers and which it does not.

Note while you are there that the distinction is already drawn twice in this
file: the `/inbox/{id}/read` test does precisely this attempt, and its
neighbour cites `QA-007-review.md` §2 — *"no `user_id` in the path" only rules
out impersonation*.

## 4. What must not change

- **No behaviour.** Every endpoint above already refuses correctly. If a test
  you write fails, **stop and report it** rather than changing the handler.
- **Do not change the 404 to a 403.** It is returned identically whether the
  row is absent or belongs to someone else, so it enumerates nothing — the
  access-safe denial the GUI specification asked for. The external design's
  *"403, uniformly"* is the thing that is wrong, and correcting it is mine.
- `auth_boundary`'s sixteen existing tests.

## 5. Verification

- The seven assertions added, in the two existing test files, and the module
  doc corrected.
- **Each new refusal test preceded by its owner-succeeds counterpart**, so a
  refusal is demonstrably about ownership and not about a malformed request.
- Report the new `DEC-007` count. No new test *file* should be needed; if one
  is, the command block changes and that is an escalation.
- `fmt`, `clippy`, three consecutive `cargo test --workspace`, `BROWSER-001`
  90/90.

## 6. Escalate rather than deciding

- **If any endpoint does not refuse.** That is a live privacy defect and it
  outranks everything else in this handoff.
- **If a fourth route with a resource id in its path turns up** in the
  personal-data inventory that §2 does not name.
- **If the owner-succeeds counterpart cannot be written** for one of the three
  capacity routes.

## 7. Exit condition

Seven assertions added and passing, each paired with a known positive; the
module doc saying which routes its reasoning covers; no behaviour changed; the
count reported with the block unchanged.

---

**Who holds what**: dev team — §2 and §3. Architect — the external design's
status table and `NFR-PRIV-008`'s own wording. **What's next**: review request.
