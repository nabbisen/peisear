# TT-004 — one rule, one declared exception

**Governing RFC**: [012](../../done/012-touch-target-conformance.md),
**`DEC-050`** (2026-09-08)
**Target release**: 0.32.0
**Depends on**: `TT-003` — closed. **Read `DEC-050` first**; it amends
`DEC-049` and removes the named limit `TT-001`–`TT-003` were built around.

## 1. What changed

`DEC-049` was guarded by a scan keying off DaisyUI sizing classes. Everything
else was a **named limit** — "unassessed, not passing".

**The limit was measured. It is not narrow**: an account menu on every page
(four links at 34 px, sign-out at 19 px), `<summary>` disclosure toggles at
**16 px**, breadcrumb and back links at 20 px, the navbar brand at 28 px, and
`FR-HLT-007`'s indicator basis links at 17 px. Across ten pages **exactly one**
sub-44 control was a link inside running text.

`DEC-050`:

> **Every interactive element MUST present a 44 × 44 CSS px target.** The target
> need not equal the visible bounds. **The sole exception is a link inside a
> block of running text, which MUST be declared as such in the markup.** Targets
> of distinct elements MUST NOT overlap.

## 2. The trap, and it is why this is not just "add the class everywhere"

**`min-h-11` on an inline element does nothing.** An inline box ignores
`min-height`. Most of the controls in scope here are `<a>` and `<summary>`,
which are **not** `inline-flex` the way `.btn` is.

So if you add `grow()` to a breadcrumb link and stop there, **the source scan
will pass and the rendered target will still be 20 px.** A guard that reads
class names cannot tell the difference. That is the single most likely way for
this handoff to produce a false green.

**Every control changed here must be measured**, not assumed. §4.

## 3. Scope

**Part A — reach 44 px.** The measured population, on ten pages:

| Control | Now | Where |
|---|---|---|
| Account-menu links (Today/Teams/Inbox/Settings) | 165 × **34** | `layout.rs`, every page |
| Account-menu "Sign out" | 52 × **19** | `layout.rs`, every page |
| Navbar brand link | 128 × **28** | `layout.rs`, every page |
| `<summary>` disclosure toggles | × **16–20** | today, project detail, board, settings |
| Breadcrumb and back links | × **20** | most pages |
| Indicator basis links | 120 × **17** | project detail |

**This list came from ten pages and is not guaranteed complete.** Re-derive it;
if you find controls it misses, that is a finding.

**Padding the row, not enlarging the type.** `DEC-050` turns on target ≠ visible
size, and the whole reason the rule stayed at one number is that familiar type
sizes are preserved. **If a change makes text bigger, it is the wrong change.**

**Part B — declare the exception.** A link inside a block of running text is
exempt and **must say so in the markup** — one marker, greppable and countable.
Choose the marker; state it in `DEC-050`'s terms in the guard's doc comment.

**Expected population: about one.** If your marker lands on more than a handful,
stop and report — the exception silently absorbing the problem is the failure
mode this design is guarding against.

**Part C — extend `touch_target_scan`.** From class-carrying controls to **every
interactive element**, with the declared exception as the only escape.

**Part C lands last**, after A and B make the tree pass it. A guard that fails on
a correct tree gets weakened until it passes — this module's own doc comment
already says so.

## 4. Verification — the guard is not sufficient and the package must show both

**Source**: `touch_target_scan` passes, and fails when a control loses its
declaration and when the exception marker is removed from an exempt link. Plant
each separately.

**Rendered**: `.git-exclude/tools/cdp.mjs` — the harness `LAYOUT-001` and
`LAYOUT-002` used. **Measure every control in Part A and report its rendered
box.** The claim is a 44 px *target*; only measurement supports it.

- Every interactive element on the ten pages of §3 reaches 44 × 44, **or**
  carries the declared exception.
- **No overlaps introduced.** Padding rows makes elements larger, which is
  exactly how targets start overlapping. `DEC-049` clause 3 still applies, and
  `§10.19` records that `join` already overlaps by 1 px — do not make it worse.
- **No horizontal overflow introduced** at 390, 768, 1280, 1920. `LAYOUT-001`
  and `LAYOUT-002` both fixed overflow; a padding pass is a plausible way to
  reintroduce it.

**Do not add a browser step to CI.** RFC 011 step 4 is still open and this does
not pre-empt it.

## 5. Escalate rather than deciding

- **If a surface cannot take a 44 px row without becoming unfamiliar.** That is
  the owner's stated first principle and it outranks the rule — report the
  surface and what it would cost, rather than shipping something odd.
- **If the exception's population exceeds a handful.**
- **If extending the guard to all interactive elements needs a heuristic.** It
  must not. `DEC-050` chose a declared marker precisely so the guard stays
  deterministic; if it cannot, say so before writing one.
- **If §3's list is materially incomplete.**

## 6. Exit condition

Part A measured, Part B counted, Part C guarding with plants, no overlap and no
overflow introduced, `DEC-007` clean, three consecutive `cargo test --workspace`
runs.

**And a statement I need in order to settle Definition of Done item 5**: whether
`NFR-A11Y-007` is now met across every interactive element, or met with a
population you can name and count. **Item 5 is held pending exactly that
sentence**, and it will be moved once rather than moved twice.

---

**Who holds what**: dev team — A, B, C. **What's blocked**: `NFR-A11Y-007`'s
status and Definition of Done item 5. **What's next**: review request.
