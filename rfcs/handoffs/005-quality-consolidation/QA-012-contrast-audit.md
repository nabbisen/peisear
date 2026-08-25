# QA-012 — The contrast audit, aimed where the failures are

**Issued by**: Architect
**Date**: 2026-08-25
**Priority**: P1 — `NFR-A11Y-005`, *Not verified* since 0.19.1
**Governing RFC**: [005](../../proposed/005-quality-consolidation.md) §4
**Depends on**: nothing. Independent of `QA-011` — that one is keyboard and
live regions, this one is colour. Either order.

---

## 1. Read RFC 005 §4 first — it was rewritten today

The original text said to run a checker over the documented colour pairs and,
where a Tailwind class fails AA, swap it for the next darker or lighter
variant.

**That would have found almost nothing**, and the reason matters more than the
correction: **we barely use named colour classes for body text.** The theme is
DaisyUI's `corporate` (`components/layout.rs:29`), and its tokens are not ours
to audit — if `base-content` on `base-100` failed AA that would be a theme
choice, and there is no "next darker variant" of a semantic token to swap to.

## 2. What is actually ours: the opacity modifier, 130 times

| Class | Uses in `src/components/` |
|---|---|
| `text-base-content/60` | 67 |
| `text-base-content/70` | 39 |
| `text-base-content/50` | 19 |
| `text-base-content/80` | 3 |
| `text-base-content/40` | 2 |
| `text-base-content/30` | 2 |
| `opacity-30` … `opacity-90` | 32 |

Every one **reduces** the contrast the theme provides, and every one is a
decision this project made rather than inherited. A token that passes AA at
full strength can fail at 60% and will almost certainly fail at 30%.

**Reproduce the counts first**, and report them — if they do not match, the
rest of this handoff is aimed wrong.

## 3. The measurement

For each **distinct pair** actually rendered — modified foreground over its
real background, not over an assumed white — compute the ratio and record it
against 4.5:1 (AA normal text) and 3:1 (AA large text, 18.66px+ or 14px bold).

Two things to get right, because both are easy to get wrong:

- **Resolve the token to a real colour.** `base-content` and `base-100` under
  `corporate` are concrete values; find them rather than assuming a generic
  grey-on-white. Say where you got them from.
- **The opacity composites against the background**, so the effective
  foreground is a blend, not the token at reduced alpha in isolation. Compute
  the composite, then the ratio.

**Small text at 50% or below is where I expect failures**, and 23 usages sit
there. `text-base-content/60` at 67 uses is the one that matters most by
volume — if it fails, it is the largest single fix in this audit and the
decision about it is not yours alone; report it before changing 67 call sites.

**Report the whole table**, passes included. An audit that lists only failures
cannot be told from one that stopped early.

## 4. What to do with a failure

**Do not start rewriting call sites.** Bring the table back first.

The likely fixes, in the order I would prefer them: raise the modifier (`/60` →
`/70`); use the theme's own muted token if one exists that passes; or leave it
and record the deviation with its reason. **Do not add a custom colour** — the
original §4 was right about that.

`NFR-A11Y-004` is adjacent and worth a sentence if you notice it while looking:
state must not be carried by colour alone. The badges pair an icon with each
colour (`✓`, `⚠`, `—`), which looks correct — say whether it held everywhere
you looked, without going hunting.

## 5. Where the results go — an open question, not a decision

The original §4 said `docs/src/accessibility.md`. **Do not create that file.**
`docs/src/` contains only `assets/`, there is no `book.toml`, and `DEC-020` —
where this project's documents live — is unresolved. Creating a file there
would silently answer a question the owner has not.

Put the table in the review-request package and **recommend** a home. That
recommendation is genuinely useful to me: you will have just been the first
person to want a place to put a document of this kind.

## 6. Is any of this guardable?

Ask the question; do not assume the answer is yes.

The family's usual move — a text scan asserting no `text-base-content/{30,40,50}`
appears in `src/` — is available and cheap. But contrast is a property of a
**pair**, and a blanket ban on a modifier would fail on a legitimate use over a
darker background while passing a bad pair at `/60`.

**Measure first, then say what a guard could honestly assert.** If the honest
answer is "the pairs are enumerable and here is the scan", propose it. If it is
"this needs rendering and cannot be a text scan", say that — `§10.15` already
records that this project does not execute its own front end, and a guard that
pretends otherwise is worse than none.

## 7. Escalate rather than deciding

- If §2's counts do not reproduce, stop.
- **If `text-base-content/60` fails, stop and report before touching any call
  site.** Sixty-seven changes is a visual change to most of the product and the
  owner should see the number first.
- If the theme's tokens cannot be resolved to concrete values without adding a
  tool or a dependency, report that rather than adding one.
- If a pair fails at 3:1 as well as 4.5:1 — failing even the large-text
  threshold — flag it separately. That is not a borderline call.

## 8. Acceptance

1. §2's counts reproduced and reported.
2. The full pair table, passes and failures, with the resolved token values and
   their source named.
3. Composite-with-background arithmetic, not token-in-isolation.
4. No call site changed before the table is reviewed.
5. §5's recommendation made, no file created under `docs/src/`.
6. §6 answered honestly either way.
7. fmt and clippy exit 0; `DEC-007` gate set green; three consecutive
   `cargo test --workspace` runs.

## 9. Required review-request format

Workflow §9.2. The table in full. §6's answer as prose.
