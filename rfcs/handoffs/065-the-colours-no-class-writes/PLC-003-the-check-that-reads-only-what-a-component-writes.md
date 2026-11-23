# `PLC-003` — The check that reads only what a component writes

**Round 3 of series `065`, and the last.** `PLC-001` and `PLC-002` are
**accepted**: the seventeen placeholders and the nine remaining sites are
repaired, both cascade directions planted and watched winning, and the
alternative at the account menu measured rather than dismissed.

**Two things the rounds corrected, which this round inherits rather than
repeats:**

- **A placeholder has no page background.** `.input`/`.textarea` paint their
  own surface from `--b1` unconditionally, proved by sampling pure white
  inside a box sitting on the grey page body. Every placeholder reads the
  same ratio, and after the repair that ratio is **6.42:1** — not the
  6.34:1 the series predicted. Three methods differ in the last decimal
  (composite 6.34, the requirement's own record 6.36, rendered pixels 6.42)
  and **the rendered pixels are the authority**.
- **The eight table headers were never failing.** They read **4.53:1** — a
  pass by 0.03 — because all eight sit inside a `card bg-base-100`,
  including the two the series itself offered as *page body* candidates.
  They are repaired because **4.53:1 is the `/60` tier `NFR-A11Y-005`
  already rejected as a floor**, not because a reader could not read them.
  **The series' own row and index said otherwise; both are corrected, and
  the overstatement was the architect's.**

**So the honest count this round must carry: eighteen sites were failing**
(17 placeholders at 2.54:1, one account-menu line at 1.85:1) **and eight
more sat on a rejected tier.**

---

## 1. The guard, and the hard part that decides its shape

`contrast_scan` reads class tokens a component writes. It cannot see a
colour the stylesheet applies by element selector, nor one baked into a
component class — which is how all twenty-six sites got here.

**The naive guard does not work, and the reason is this series' own
repairs.** A scan that flags every below-floor text-colour rule in the
served sheets would flag `tailwind.css`'s `input::placeholder{#9ca3af}`
**forever**, because that rule still exists — the project now overrides it.
The guard has to distinguish *a rule that reaches a reader* from *a rule
something else beats*, and nothing short of a browser resolves the cascade
properly.

**Recommended shape — a static acknowledgement list, this project's own
pattern** (`contrast_scan`'s named exclusions, `css_scan`'s allowlist):

- Scan the three served sheets for text-colour declarations resolving below
  `base-content/70`, restricted to selectors a component can reach: an
  element or pseudo-element selector, or a class token some component
  writes (the intersection `css_scan` already computes for its population).
- Every such rule must appear in an **acknowledgement list**, each entry
  naming: the selector, its resolved ratio, **and what beats it** — either
  the `app.css` rule that overrides it, or the reason it is exempt.
- **A new below-floor rule that nothing acknowledges fails the gate**, which
  is the property this series exists to create: the next time a theme bump
  or a new daisyUI class brings a faded colour into reach, it is refused
  rather than shipped.
- **Pair each entry with its exact count**, as `CLR-003` round 2 had to
  learn: an acknowledgement that silently covers a second site is the hole
  that round closed in the previous series.
- **Exempt, by name and with the reason**: the disabled tiers
  (`.input:disabled` and friends at `/40`, `.menu li.disabled` at `/30`) —
  **WCAG 1.4.3 exempts text that is part of an inactive user-interface
  component.** Say it in the code, not just here.
- **`.stat-title`/`.stat-desc`** (`/60`) are reachable only if a component
  writes them, and **zero do today** — so they need no acknowledgement and
  the guard should fail on the day one appears. Confirm that is what
  happens, by plant.

**Plant it twice**, because it has two failure modes: once by adding a
`text-base-content/50` class to a component (a token in reach), and once by
removing an acknowledgement entry whose rule still reaches a reader.
Quote both failures.

**Name, do not build**: the only check that truly resolves this is one that
samples computed colours in a browser across the swept pages. Record it as a
candidate with its cost, and say why the static list is what ships now.

---

## 2. `NFR-A11Y-005`, amended a second time in two releases

- Its *Acceptance* names the new scan alongside the `/70` floor and
  `§10.61`'s semantic-colour ban — **three mechanisms, one requirement**.
- Its own history should now say plainly that **the floor was never the
  whole story**: `/70` governs what components write, `§10.61` governs which
  colour families they may write, and this round governs colours **no
  component writes at all**.
- Status stays **Met**, and says what makes it Met.

---

## 3. `§10.62`, opened and closed in this round

Record, because these are the parts a later reader needs:

- The class: **a check that reads what components write cannot see what the
  stylesheet applies.** Second instance in two releases of a guard's scope
  being narrower than its requirement's words — `§10.61` was the first.
- **The honest split**: eighteen failing, eight on a rejected tier; and the
  worst site was **1.85:1**, the reader's own e-mail address, where two
  fades multiplied (`.menu-title`'s `/40` × the span's own `opacity-70`).
- **The two cascade directions**, because they are the thing a future reader
  will get wrong: `tailwind.css` is served **last** and had to be
  out-specified; `daisyui.min.css` is served **first** and was beaten on
  order. Both were planted.
- **What the architect got wrong**: the row's claim that all three families
  failed, and a predicted 6.34:1 that measurement put at 6.42:1.
- That the class was found by **scoping a release**, not by any check —
  the same way `§10.60` and `§10.61` were.

---

## 4. Also in this round

**`app.css`'s own header comment** still says *"Tailwind's play CDN covers
the bulk of styling"*. The product stopped loading CSS from a CDN at
`0.32.0` (`§10.23`), and that file is now the project's **only** own
stylesheet and carries two colour rules this series added. Rewrite the
comment to say what the file is for and what the two new rules are doing
there — it is the first thing the next person reads.

---

## 5. Acceptance

1. The guard added, **planted twice** (a reachable below-floor token; a
   removed acknowledgement), both failures quoted, plants removed.
2. Acknowledgement entries **paired with exact counts**, each naming what
   beats the rule or why it is exempt; the disabled tiers exempt **in the
   code** with WCAG 1.4.3 named.
3. `NFR-A11Y-005` amended; **`§10.62` opened and closed** with §3's
   contents, including the architect's own two errors.
4. `app.css`'s header comment rewritten.
5. The failing population re-derived to **zero**, with the command, and the
   acknowledged-but-below-floor set stated as its own number.
6. `css_scan` `(287, 3)` — verify.
7. The full gate line per `README.md §6` with real exit codes, the
   `DEC-007` count (**689** as it stands), and **`contrast_scan` run after
   the new rule is added, not before**.
8. `CHANGELOG.md` under `[Unreleased] → Fixed`, completing the entry family,
   **plus one line for the guard** — and the entry should say *eighteen*,
   not twenty-six, since that is what was failing.
9. Privacy floor per artefact; scratch under `.git-exclude/tmp/`.
10. **Review request** at
    `.git-exclude/review-request/PLC-003-the-check-that-reads-only-what-a-component-writes/README.md`.

**After this round, series `065` is closed** and `0.67.0` goes to a release
candidate against exit criteria 1–6. No round cuts a release.
