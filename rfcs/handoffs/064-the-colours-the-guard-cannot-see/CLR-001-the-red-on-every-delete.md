# `CLR-001` — The red on every delete

**Round 1 of series `064`** (`0.66.0`, authorized 2026-11-21 — the row, not
the cut). Read `README.md` in this directory first: it carries the measured
tokens, the method, the three sites that pass, and the privacy floor.

**Scope: seven destructive affordances, all at 3.08:1 against a 4.5:1
requirement.** Nothing else.

---

## 1. The seven

| site | what it is | class today | size |
|---|---|---|---|
| `layout.rs:417` | **Sign out**, in the account menu | `btn btn-ghost btn-sm text-error w-full justify-start normal-case` | 14 px |
| `teams.rs:619` | **Remove** a member, in a table row, behind a `confirm()` dialog | `btn btn-ghost btn-xs text-error` | 12 px |
| `settings.rs:469` | **Remove** a capacity row, in a row, behind a `confirm()` | `btn btn-ghost btn-xs text-error` | 12 px |
| `issues.rs:2256` | delete an issue — a link to the confirmation page | `btn btn-ghost btn-sm text-error` | 14 px |
| `sprints.rs:719` | delete a sprint — same shape | `btn btn-ghost btn-sm text-error` | 14 px |
| `sprints.rs:747` | delete a sprint, the other code path | `btn btn-ghost btn-sm text-error` | 14 px |
| `projects.rs:295` | delete a project — a link to the confirmation page | `btn btn-error btn-outline btn-sm` | 14 px |

All seven render `#ff5861` text on `base-100`: **3.08:1**, against
`NFR-A11Y-005`'s **4.5:1**. None is large text, so none has the 3:1 route.

**`projects.rs:295` is the one a `text-error` grep misses** — the colour
reaches the text through `btn-outline` + `btn-error` instead. That is why
this round's census is by spelling and not by one pattern.

---

## 2. The structure this sits in, which is what decides the repair

**The warning is not carried by these buttons.** Four of the seven lead to
`confirmation.rs`'s own interstitial, which already carries it properly: a
card with `border-error/30`, a heading, a consequence line quoting what the
reader typed, Cancel as a ghost button, and **Delete as a solid
`btn-error` at 6.83:1** — an AA pass, and the pattern the product already
owns. Two more sit behind a native `confirm()` dialog, which is the warning
in those flows. **And one is Sign out, which is not destructive at all.**

So the colour on these seven is decoration on the *way in*, while the real
warning lives at the step that commits — and the decoration is what fails.

---

## 3. The architect's recommendation, and the alternative that must be
rejected on evidence rather than ignored

**Recommendation: drop the colour on all seven** — plain `btn btn-ghost
btn-sm`/`btn-xs`, which inherits `base-content` at **17.21:1** — and let the
confirmation step and the `confirm()` dialog carry the warning they already
carry. For `projects.rs:295`, that means `btn btn-outline btn-sm` (the
spelling three other outline buttons in the product already use).

**The alternative is solid `btn-error` on each**, which measures 6.83:1 and
would also satisfy AA. **Measure it before rejecting it**, and reject it on
what you see: six of the seven are quiet affordances, three of them inside
table rows where a solid red button dominates the row it belongs to, and the
product deliberately keeps its loud red for the step that actually deletes.
If the measurement says otherwise, that is the finding.

**`layout.rs:417` — Sign out — is a special case and must be reported as
one.** Removing the red there is both an AA repair and a **meaning**
change: signing out is reversible and is not a destructive action, so the
colour was wrong independent of contrast. Say that in the report rather than
folding it into the ratio, and if you think the owner may want the colour
kept as a visual anchor for the menu's last item, name that rather than
deciding it silently.

---

## 4. What to measure

- **Per site, before and after**: the rendered foreground, the background it
  sits on, and the ratio, in a browser rather than from the stylesheet.
  Expect `3.08:1 → 17.21:1` where the colour is dropped.
- **At 390 px and 1280 px**, whether the button's own box moves. It should
  not: `grow()` composes `TOUCH_TARGET` (`min-h-11` = 44 px) and
  `btn-ghost`'s own padding is unchanged by a colour, so a height change
  would mean something else moved.
- **The two `btn-xs` sites' size stays** (`teams.rs:619`,
  `settings.rs:469`). The `xs` tier is a recorded candidate and not this
  series'. Only the colour changes.
- **Focus and hover states**: `btn-ghost`'s hover background is
  `base-content/10`-ish; confirm the label still clears 4.5:1 **against the
  hovered background too**, reached by a real dispatched pointer move, not a
  forced pseudo-class. `§10.60`'s own contrast work measured hover states
  this way and found them lower than resting.
- **That the confirmation page is untouched**: `confirmation.rs:64`'s solid
  button stays exactly as it is, and `confirmation.rs:56`'s heading is
  `CLR-003`'s, not this round's.

---

## 5. Acceptance

1. **Seven sites accounted for**, each with before/after ratios and the
   background named; any site left unchanged carries a reason.
2. **Hover and focus states measured**, not assumed from the resting ratio.
3. **The three passing sites untouched** (`error_page.rs:18`, `auth.rs:15`,
   `layout.rs:211`) — confirmed by grep, stated in the report.
4. **`confirmation.rs` untouched.**
5. **`CHANGELOG.md`** under `[Unreleased] → Fixed`: what a reader could not
   read, and what changed. This is a defect, not an improvement.
6. **Gates in full**, per `README.md §5`, including `css_scan`'s population
   re-measured — dropping `text-error` at six sites may remove that token
   from `components/` entirely, and if it does, the number moves and the
   reason goes in the assertion's own comment.
7. **Privacy floor per artefact**; scratch under `.git-exclude/tmp/`.
8. **Review request** at
   `.git-exclude/review-request/CLR-001-the-red-on-every-delete/README.md`.

**If a measurement contradicts this file, the measurement wins.** In the
last series six recorded reasons were wrong and three of them were the
architect's; every one was caught by a round re-deriving rather than
trusting the handoff.
