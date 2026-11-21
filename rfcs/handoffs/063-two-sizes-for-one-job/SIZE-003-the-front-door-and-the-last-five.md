# `SIZE-003` — The front door, and the last five

**`SIZE-002` is accepted** (four raised, five left with neighbours named, the
spelling converted at `teams.rs:577`, and the `overflow-gate.mjs` coupling
**deleted** via a named hook whose allowlist guard was widened rather than
exempted). Read `README.md` in this directory first.

**This is the series' last round.** It carries five sites, one re-read, and
the series' own closing.

---

## 1. The five sites

| site | what it is | the thing to read |
|---|---|---|
| `auth.rs:24` | the **flash alert** on the public sign-in and registration pages — `alert alert-warning text-xs py-2` | the front door's own warning, at the smallest size in the product. Who sees it: someone whose sign-in just failed |
| `auth.rs:53` | the centred line under the login form | its neighbours above and below, and the form's own sizes |
| `auth.rs:124` | the same shape under the registration form | whether the two pages agree with each other after the change |
| `settings.rs:81` | the guidance paragraph inside the capacity **conflict box** (`aria-live="polite"`), beneath a `font-medium` label and the message itself — `text-xs mt-1 opacity-80` | the box's own ambient size, and that this is live-region text a reader is being asked to act on |
| `settings.rs:187` | a `mt-2` paragraph, `text-base-content/70` | its neighbours |

**Read each by its neighbours** — name the element above, the element below,
and their measured sizes — then decide. Measure every change at **390 px and
1280 px**. Removal is on the table; so is leaving a site, with a reason that
survives those neighbours.

**`settings.rs:81` carries a bare `opacity-80`.** If you raise it, the
spelling question comes with it (`text-base-content/80` is the guarded
spelling and already in use at `me.rs:569`), under `§10.60`'s condition:
single level, no faded ancestor. Leaving the spelling is an outcome if you
say why.

**Two sites are a pair.** `auth.rs:53` and `:124` are the same shape on the
two public pages. Whatever is decided, decide it for both and say so — a
split there is the series' own defect in miniature.

---

## 2. One re-read carried from `SIZE-002`

`teams.rs:311` — the *by email* qualifier inside the invite disclosure's own
`<summary>` — was left at 12 px because *"no second instance of this shape
exists elsewhere to converge toward."*

**There is one, and it disagrees.** `settings.rs:359–361` is an inline
primary-plus-secondary pair inside a `grow_summary()` summary
(`<span class="font-medium">{row.points}</span>` then its unit suffix), and
**both halves render at 14 px** — that summary sits in a `table table-sm`
whose rows are `.875rem`. The `teams.rs` pair is 16 px beside **12 px**, a
wider gap than any site `SIZE-002` raised.

**Re-decide it with the comparison named.** Either outcome is acceptable:
raise the qualifier, or keep it and say what distinguishes a *qualifier* from
a *unit suffix* — the second is a real argument, it simply was not the one
made. Measure the summary's own row at both widths if it moves; it is a
touch target, so report its height too (the 44 px floor is held by
`grow_summary()`'s `min-h-11` and must not be re-mechanised).

**The rule this adds to the series' method**, now in `README.md §4`:
*"nothing else does this"* is a **universal claim**, and it needs the search
that justifies it, named — or it must be written as *"I found no other
instance after looking at X"*, which is bounded and checkable. Three
population claims in this series have been asserted rather than enumerated,
and **one of the three was the architect's.**

---

## 3. The series' closing, which this round also owes

After the five sites are decided:

1. **The final census**, re-derived with `README.md §2`'s command, per file,
   stated as the number the series ends on. It stands at **44** now.
2. **A reconciled rounds table** in this directory's `README.md`: every
   round, its outcome, and its site count — `LBL-001` (15, closed over two
   rounds), `SIZE-001` (18, closed over two), `SIZE-002` (9, closed), and
   this one.
3. **What the series leaves behind, named rather than implied**: the **21**
   out-of-scope sites (`calendar.rs` 11, `notification_preferences.rs` 4,
   `notifications.rs` 3, `search.rs` 2, `layout.rs` 1), the **thirteen
   `sprints.rs` captions** decided as a set, and the sites left standing in
   `teams.rs` and `sprint_plan.rs` with their reasons. A later release should
   be able to read this one file and know what was decided and what was not.
4. **No register entry, and no rule.** `§10.60` stays closed; the series
   writes no floor, scale or type requirement. If this round believes one is
   now warranted, it says so as a finding and stops — that decision is the
   architect's and then the owner's.

**What this round must not do**: touch the 21, reopen the decided caption
set, raise `btn-xs`/`select-xs`, sweep the remaining bare-`opacity` sites
beyond its own two, or cut a release. `DEC-056`'s legs are the architect's,
on the owner's approval at exit criterion 7.

---

## 4. Acceptance

1. **Five sites accounted for**, neighbours named and measured, with
   `auth.rs:53`/`:124` decided as one.
2. **`settings.rs:81`'s spelling** decided explicitly if it is raised.
3. **`teams.rs:311` re-decided** with the `settings.rs:359` comparison named.
4. **Census re-derived** — **44** now, less whatever is raised — and stated
   per file.
5. **`(289, 3)`** confirmed, or updated **with the reason** if a token
   leaves or joins `components/`.
6. **The gate line in full**: the `DEC-007` block's count with real exit
   codes (**687**), `fmt`, `clippy` both variants, `contrast_scan`,
   `touch_target_scan`, `alignment_scan`, `overflow-gate.mjs`, and
   `disclosure-marker-gate.mjs` **only if** a `<summary>` or its own
   `<details>` ancestor moves (`teams.rs:311` would make it so).
7. **`CHANGELOG.md`** under `[Unreleased] → Changed`, as the earlier rounds
   did: what moved, and what was left and why.
8. **The series' closing**, per §3.
9. **Privacy floor per artefact**, per `README.md §8.7`.
10. **Review request**:
    `.git-exclude/review-request/SIZE-003-the-front-door-and-the-last-five/README.md`.

**If a measurement contradicts this file, the measurement wins.** That has
happened in every round of this series, twice on the architect's own
sentences, and each time it improved the result.
