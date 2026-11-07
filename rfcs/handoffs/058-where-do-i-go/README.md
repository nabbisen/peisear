# Handoffs — where do I go

**No release number yet.** The owner authorized this sequence 2026-11-04,
after asking whether layout, typography and colour could be kinder to
**elder people** and then asking for the three to be reconsidered against
this project's fundamental philosophy — *"one by one with time we need:
App base layout(s), user workflows, and each view."*

**This series is unit one: the app base layout.** One round, and the next
unit is not written until this one closes. That pacing is the owner's
instruction, not a shortage of material.

**No RFC.** The RFC numbering is at `022`; `058` is a handoff series. The
rule this round applies is already written: `FR-NAV-001`, a **P1**, says
the primary navigation **MUST expose exactly five entry points — Today,
Inbox, Projects, Teams, Search**. What was missing is not a rule but a
reading of the word *expose*, and that reading is the architect's design
decision, recorded in the round.

---

## Rounds

| ID | Handoff | Subject | Blocked on | State |
|---|---|---|---|---|
| NAV-001 | [the five that are not there](./NAV-001-the-five-that-are-not-there.md) | `FR-NAV-001` is `Implemented` with an acceptance clause — *"navigation renders five entries"* — that this document already records as **a restatement nobody can check**. Measured against the source: of the five required entries, **none is visibly named at every width**; `Inbox` is an unlabelled glyph, `Projects` hides behind the brand link, `Today` and `Teams` are in a dropdown behind the reader's username, `Search` vanishes below `sm`, and `Settings` is exposed although it is not one of the five | — first in the series | **Closed 2026-11-04; one finding, which earns round 2.** All six decisions verified in source: five named entries in `<nav aria-label="Primary">` (`hidden md:flex`), the bell carrying `{t(NavLinkInbox)}` with its badge intact, `Projects` with its own entry, `Search` given a visible word rather than a `placeholder` that vanishes on typing, `Settings` **absent from the primary row** (grep count 0) and present in the account dropdown, and the collapse at `md:hidden` labelled `NavMenuLabel` — a word, never the username. **The architect re-planted independently and the scoping is load-bearing**: `Projects` removed from the primary row *only*, with the `Menu` duplicate left in place, and the test failed naming `"Projects"` — an unscoped `body.contains` would have passed. The round had anticipated exactly that and scoped to the primary block on `breadcrumb.rs`'s own `TT-003 §5` precedent. `browser-checks` 120 cells / 0 failing at the chosen `md` threshold, **with no shrinking and no muting** — the two things `§3` forbade; all 34 integration binaries re-run; `touch_target_scan` and `contrast_scan` re-run clean here too; `DEC-007` **637 → 639** (+2, integration), re-counted exact; rustdoc 6; `CI`/`Docs`/`Env Example Walk` all `success` at `96bd102`. **And it corrected the “33 integration binaries” figure in five live places while leaving every dated snapshot dated** — the architect re-checked both survivors and both are inside dated paragraphs, correctly left. `FR-NAV-001` amended by the architect from the round's proposed wording: **Met**, with a *locatable* acceptance naming both tests, replacing a clause `§10.35` had already recorded as uncheckable *“including its author”*. **The finding**: giving the bell its word made its visible label and accessible name disagree — visible **“Inbox”**, `aria-label` **“Notifications”**, which is **WCAG 2.5.3 Label in Name (Level A)**. It could not arise while the bell was glyph-only; adding the word, which was right, exposed it. **And it is a leftover rather than a design choice**: `FR-NAV-002`'s own acceptance records the rename `/notifications` → `/inbox`, and the accessible name is the last place the old name survives *as the destination's name* |
| NAV-001 r2 | [the name the reader hears](./NAV-001-round-2-the-name-the-reader-hears.md) | `NavBellLabelNone`/`NavBellLabelUnread` → **“Inbox”** / **“Inbox ({count} unread)”**, at both call sites. The other five `Notification*` strings stay: the page is Inbox, the items in it are notifications, and collapsing that would be a worse defect than the one being fixed. Carries `peisear-i18n`'s own suite and its fixture locale, and asserts the **containment** rather than just changing the strings | NAV-001 | **Closed 2026-11-07, no findings against the round — and its own reported aside turns out to be twelve times larger than it claimed.** `NavBellLabelNone` → **“Inbox”**, `NavBellLabelUnread` → **“Inbox ({count} unread)”**; the other five `Notification*` strings untouched (grep: 5 remain), correctly — they name the items, not the destination. Two new tests in `navbar.rs`, **scoped to the primary `<nav>` the same way round 1 was**, asserting containment for both reachable shapes, with the unread one seeding a real notification row so the count is still named for a reader who cannot see the badge. **Architect's plant**: `NavBellLabelNone` reverted to `“Notifications”` → `the_bells_accessible_name_contains_its_visible_label_with_no_unread` **FAILED** with *the bell's accessible name “Notifications” must contain its visible label “Inbox”*, while the `with_unread` test correctly stayed green, since it reads the other key — the two are independent and both load-bearing. `DEC-007` **639 → 641** re-counted exact; `peisear-i18n`'s own four binaries re-run and nothing pinned the old string; `fmt`/`clippy` clean; `CI`/`Docs` `success` at `2e836a2`. **It also read the fixture locale correctly rather than “fixing” it**: its bell strings were always deliberately distinct, because its job is proving the renderer is not English-shaped, not satisfying 2.5.3. **And `§3` stayed inside its bounds** — it checked the one other pair that was already open in front of it (the search form's, which agrees) and audited nothing |
| NAV-001 r3 | [the badge that cannot tell](./NAV-001-round-3-the-badge-that-cannot-tell.md) | Round 2 reported, without generalising, that `/today` never computes the unread count. **The architect measured it: of 26 `<AppShell>` render sites, 14 thread the real count and 12 do not** — `issues.rs` ×5, `projects.rs` ×3, `me.rs` ×2, `search.rs`, `settings.rs`. The mechanism is `#[prop(default = 0)]`: **a missing count becomes a plausible zero**, and `AppShell`'s own rustdoc claims the opposite in so many words — *“Threaded through every handler that returns the shell”*. Round 2's own fix widened the reach, since the count is now in the **accessible name** too. **Decision: make the prop required**, so the compiler names every omission and the doc claim becomes structurally true | NAV-001 r2 | **Closed 2026-11-07, no findings against the work.** `unread_count` is **required** on `AppShell` — no default, no `Option` — and **all 26 render sites now thread it**, re-measured by the architect with the corrected method: **26 of 26, zero omissions.** The compiler enumerated the twelve rather than anyone re-deriving them, which the round says in terms was to avoid repeating the architect's own two miscounts. Eight handlers gained a `Requester`, and **that changes reachability for nobody**: `AppShell` requires `user: CurrentUser`, so every one of these handlers already held an authenticated identity, and `Requester` rejects by redirect exactly as `AuthUser` does — checked, because it was the one real risk in the round. **Architect's plant, aimed one level deeper than the round's**: theirs removed an argument from a `render_*` wrapper and got an arity error; mine removed the prop from an `<AppShell>` tag itself — `cargo build` **exit 101**, `argument #1 of type AppShellPropsBuilder_Error_Missing_required_field_unread_count is missing`. **The doc comment's claim is enforced by the type's own name**, which is the strongest form this evidence could take. `DEC-007` **641 → 642** — the round stated it could not verify the “from”, having not re-run the prior block, and said so rather than inventing one; **the architect had measured 641 in round 2's review**, so the delta is exactly the one new test. All gates re-run against real exit codes (`--lib` 0, `fmt` 0, `mdbook` 0); `CI`/`Docs` `success` at `d2ccaad`; the query is an indexed `COUNT(*)` served by `idx_notifications_user_created` from `0.13.0`, whose own comment already names this use. **One reporting gap, not a defect**: three `#[allow(clippy::too_many_arguments)]` were added (`issues.rs`, `search.rs`, **`settings.rs`'s `render_settings`**), and the report lists two — each legitimate, the new parameter crossing the threshold, but an `allow` suppresses what `-D warnings` otherwise enforces, so the inventory should be complete |

## The decision this round implements, and why it is not the dev team's

**"Expose" means visible and named, without an interaction.** The five
become a labelled row; the bell keeps its badge and gains its word;
Projects stops depending on a logo; Settings leaves the primary row for
the user dropdown; below the collapse threshold the five sit behind one
control labelled with a **word**, never the username.

**The reasoning is the reader the owner named.** A menu costs a two-step
interaction *plus* remembering that the menu is behind one's own name. For
someone whose acuity and short-term recall are both declining, that is the
expensive part. **Hiding is not simplifying** — *less is more* means less
to **decode**, not fewer things on screen. The base layout had already
applied "less" by hiding, and that is the half being reconsidered.

## Closed 2026-11-07 — unit one complete

**Three rounds, and the series' own subject widened twice without ever
leaving the base layout.** It began as *the five entry points are not
visible*. Making them visible gave the bell a word, which exposed a
**WCAG 2.5.3** mismatch between its visible label and its accessible
name. Fixing that required a test for the unread shape, which could not
be written against `/today` — because **twelve of twenty-six render sites
never computed the count at all**, and `#[prop(default = 0)]` turned every
one of them into a plausible zero.

**Each round found the next, and none of them was visible from the
first.** That is the argument for reading a layout rather than auditing it
against a checklist: `FR-NAV-001` was `Implemented` for twenty-two
releases with an acceptance clause `§10.35` had already recorded as
uncheckable, and the defect it was hiding was three defects deep.

**What the reader gets.** Five named destinations instead of four behind
their own username; a notification control with a word rather than a
glyph; an accessible name that matches what is on screen; and a badge that
is now correct on every authenticated page instead of fourteen of
twenty-six.

**What the type system now guarantees**, which is the part that cannot
drift: `unread_count` is a required prop, so the missing-count defect
cannot recur without a compile error whose own type is named
`AppShellPropsBuilder_Error_Missing_required_field_unread_count`.

**Unit two — user workflows — is not written yet, deliberately.** It needs
a designed read of rendered screens, which is work rather than an opinion,
and `browser-checks`'s 19-page fixture is the instrument. The owner's
pacing is one unit at a time with the time each needs.

## What this series is not

**Not a redesign of the five pages.** Their contents are later units, one
at a time, in the order the evidence gives — `me.rs` carries 20 of the 90
`text-xs`-with-muted-tier sites, `sprints.rs` and `issues.rs` 18 each.

**Not the contrast rule.** The architect proposed forbidding the muted
tier below 14 px and **withdrew it**: applied to those ninety sites it
would have *enlarged* ninety pieces of quiet secondary information, twenty
of them on the first screen a user sees, making the busiest screens
denser. It survives only as a **floor adopted after** the views are
decided — rule after design, not instead of it.

**Not a change to any route.** `FR-NAV-002`'s 308 redirects stay exactly
as they are.

## The trap this series will walk into, named once here

**A row of five labelled links is wider than a logo and a glyph**, and the
cheap way to make it fit is to shrink the type or mute it. **That is
forbidden**: it would reintroduce, in the primary navigation, the
small-and-faint combination this entire line of work began from. The lever
is the collapse threshold, not the type.

## The standing exception applies

A defect found in the shipped product while this series is open earns a
round of its own rather than being folded into the round that noticed it.
