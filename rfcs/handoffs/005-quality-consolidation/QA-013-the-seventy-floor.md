# QA-013 — Raise the muted tier to a `/70` floor

**Issued by**: Architect
**Date**: 2026-08-26
**Priority**: P1 — `NFR-A11Y-005`, measured and failing
**Governing RFC**: [005](../../proposed/005-quality-consolidation.md) §4
**Depends on**: `QA-012`, closed. Its table is the input; do not re-measure.

**Owner-approved**: the `/70` floor was put to the owner with a figure of
"roughly 88 call sites" and accepted. **The real figure is 111** — see §2. The
undercount is mine and is corrected below; the decision stands, and if the size
changes the owner's mind, that is the owner's to say.

---

## 1. What was decided and why

`QA-012` measured every opacity-modified foreground against every background
this theme defines. **`/70` is the lowest tier that passes AA with margin**:
6.36:1 on `base-100`, 5.76:1 on `base-200`, 5.15:1 on `base-300`.

Everything below it either fails or passes by a rounding error:

| Class | Sites | `base-100` | `base-200` |
|---|---|---|---|
| `/60` | 67 | 4.54:1 — passes by **0.04** | **4.23:1 fails** |
| `/50` | 19 | **3.32:1 fails** | **3.16:1 fails** |
| `/40` | 2 | **2.50:1 — fails 3:1 too** | **2.42:1 — fails 3:1 too** |
| `/30` | 2 | **1.93:1 — fails 3:1 too** | **1.89:1 — fails 3:1 too** |

Three `/60` uses render on `base-200`: `auth.rs`'s login and register subtitle.
**The first text a new user reads fails AA today.**

The 0.04 is the reason this is a floor rather than a repair. A muted tier that
passes by four hundredths is one theme adjustment away from failing with
nothing to report it.

**`/50` is held to 4.5:1, not 3:1.** It clears the large-text threshold, but
none of its uses are large text — `QA-012` checked for `font-bold`/`font-semibold`
alongside them and found none — and `NFR-A11Y-005` carves out nothing for
secondary metadata.

## 2. The work list — 111 sites, not 88

Reproduce these counts first. If they differ, stop and report.

| Class | Sites | Action |
|---|---|---|
| `text-base-content/60` | 67 | → `/70` |
| `text-base-content/50` | 19 | → `/70` |
| `text-base-content/40` | 2 | → `/70` |
| `text-base-content/30` | 2 | → `/70` |
| `opacity-60` | 20 | → `opacity-70` **where it affects text** |
| `opacity-50` | 1 | → `opacity-70` **where it affects text** |
| `opacity-30` | 2 | **unchanged — see below** |

**My arithmetic error, for the record**: I put "roughly 88" to the owner by
counting `/40` and `/30` as two sites together rather than two each, and by
leaving the 23 bare `opacity-*` uses out of a total I had already agreed
composite identically. 90 + 21 = 111.

**The two `opacity-30` sites stay** — `calendar.rs:219` and `:255`, empty
`<td>` cells dimming a non-current-month border. Not text, not a contrast
question. `QA-012` excluded them explicitly rather than counting them; keep it
that way and say so in the package.

**Check each bare `opacity-*` site before changing it.** `QA-012` established
that they inherit the ambient `text-base-content` and composite the same way,
but that was a sampling of rendering contexts, not all 23. An `opacity-60` on a
container also dims its children, and one sitting inside a `badge-*` or a
`text-primary` element composites against a different foreground entirely.
**If any does, stop and report it** — the arithmetic in §1 would not apply and
I would need to re-measure.

## 3. What this changes about the design, which is not only arithmetic

The muted tier currently has four steps — `/50`, `/60`, `/70`, `/80`. After
this it has two. **Visual hierarchy that was carried by four levels of grey
will be carried by two**, and the places that used `/50` for "least important"
will now look identical to the places that used `/70` for "somewhat less
important".

**Do not solve this by inventing a new tier.** Adding a custom colour is out
(RFC 005 §4's original text was right about that), and a `/65` would sit at
about 4.9:1 on `base-100` and fail on `base-200` — the same trap one notch up.

**Report where the flattening is worst**, with the file and what distinction is
being lost. Hierarchy can be carried by size, weight, or placement rather than
by contrast, and deciding that is a design change beyond this handoff. I want
the list, not a fix.

## 4. The scan

With `/70` as the floor, a text scan becomes honest for the whole banned range:
**no `text-base-content/N` below `/70` has a passing case against any
background this theme defines.** There is no legitimate use to mis-flag.

Assert that `text-base-content/10` through `/60` do not appear anywhere under
`crates/peisear-web/src/`. Same family as the four existing guards, and
`prose_scan`'s widened collector already walks that tree.

**Bare `opacity-*` stays out of the scan**, and the reason belongs in the
guard's doc comment rather than only here: `opacity` is not a text property.
`calendar.rs`'s two empty cells are a legitimate non-text use, so a blanket ban
would be false, and distinguishing text from non-text needs rendering —
`§10.15`'s standing limit. **A guard that covered one and pretended to cover
the other would be this series' fifth instance of the thing it has been
closing.**

**Pin what makes the ban true.** The scan rests on `daisyui@4.12.14`'s
`corporate` theme: `base-content` `#181A2A`, backgrounds `#FFFFFF`, `#E8E8E8`,
`#D1D1D1`. Put those values and that version in the doc comment, so a future
reader upgrading DaisyUI knows this measurement is what they are invalidating.

## 5. Escalate rather than deciding

- **If §2's counts do not reproduce, stop.**
- If any bare `opacity-*` site turns out to composite against something other
  than `base-content`, stop and report — §2.
- **If raising a specific site to `/70` makes it illegible for a different
  reason** — over a coloured background, inside a badge — stop on that site and
  report it rather than forcing the rule through.
- If §3's flattening turns out to erase a distinction that is doing real work
  somewhere, name it and leave it; do not invent a replacement.

## 6. Acceptance

1. §2's counts reproduced and reported before any change.
2. 111 sites changed; the two `opacity-30` calendar cells untouched, with that
   stated.
3. Every bare `opacity-*` site confirmed text-bearing and
   `base-content`-inheriting before it moved.
4. The scan present, running in CI, demonstrated against a planted
   `text-base-content/60`; its doc comment pinning the DaisyUI version and the
   four resolved values.
5. §3's flattening list reported — files and lost distinctions, no fixes.
6. fmt and clippy exit 0; `DEC-007` gate set green; three consecutive
   `cargo test --workspace` runs.

## 7. Required review-request format

Workflow §9.2. §3's list as prose. The plant transcript. State plainly whether
any site in §5's third case was found.
