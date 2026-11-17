# TYPE-001 round 2 — what the tree actually says

**Series:** [062 — the smallest text on the screen](./README.md)
**Release:** `0.64.0`, authorized 2026-11-17 — the row, not the cut
**Blocked on:** `TYPE-001`, closed 2026-11-17 and **accepted** — the
code is right and nothing in it is reopened here.
**Register entry:** `§10.60`, still **open**

Round 1's work was good, and its contrast measurement is the best thing
either of these two series has produced: a conservative sampling floor
with the reason it is a floor, hover reached by a real dispatched mouse
move, no npm dependency, the pre-change table taken before any edit,
and the one cell the data does not fully support disclosed rather than
smoothed over. **None of that is revisited.**

This round is **three measurements and one piece of reasoning.** It is
small on purpose.

---

## §1 Why there is a round 2 — and what the architect already fixed

`§10.60`, as round 1 wrote it, said *three of the four* sites moved to
the guarded spelling and that `calendar.rs:185` *"kept bare
`opacity-70`"*. **The tree says all four moved** —
`calendar.rs:201` is `text-[10px] text-base-content/70 block truncate`,
and that site's own in-code comment says the colour moved. The review
request carried the same contradiction against its own post-change
table, and argued at length for a decision the round did not take.

`§10.58`'s class, two days after it shipped in `0.63.0`: a document's
body asserting a state the code does not have, with nothing reading one
against the other.

**The architect has already corrected the fact in `§10.60`**, because
the entry is published, the error was a fact about the tree rather than
a judgement, and the correct text was in hand. **Do not re-correct it.**
The correction records that the code is right and the record was wrong.

**The ruling, so nothing here is an open question:** keeping all four on
`text-base-content/70` is right. The argument written for the other
decision — that the guarded spelling gives *"a false 'now guarded'
appearance"*, since `contrast_scan` bans `/10..60` and `/70` is simply
not banned, with pinned margins keyed to the three theme base colours
rather than to `bg-primary/N` — **is all true and is the wrong
comparison.** The question is not whether the scan verifies this site
today; it is what happens the day someone lowers it.
`text-base-content/50` fails `contrast_scan` by name. `opacity-50` is
invisible to everything this project owns. **The guarded spelling buys a
future failure, not a present assurance**, which is what a backstop is.
The honest-labelling worry is already discharged where it belongs:
`NFR-A11Y-005`'s own entry states both limits of its guard in terms, and
a spelling does not have to carry a disclaimer the requirement carries.

**What this round owes on it: the reasoning, in `§10.60`, in one or two
sentences.** In your own words if you agree with it. **If you do not
agree, file a review request and the architect will rule again** — do
not change the code back without one. Round 3 of `GRAM-002` is the
precedent, and it was right to stop.

## §2 The badge's cost was measured in the wrong dimension

`calendar.rs:110`, the month/week project badge — **raised to
`text-xs`, cost reported as zero at 390 and 1280.** The height claim is
correct and so is its mechanism: the badge is inline on a line whose
12px siblings already set the height.

**This badge does not live in the height dimension.** Its parent:

```
<a class="block text-xs px-1.5 py-0.5 rounded bg-primary/10 ... truncate">
    <span class="tabular-nums opacity-70 mr-1">{time}</span>
    <span>{title}</span>
    {badge}
</a>
```

`truncate` is `white-space: nowrap; overflow: hidden; text-overflow:
ellipsis`. Time, title and badge are one unbreakable line clipped at the
`<a>`'s width, and **the badge is the last child — the first thing the
clip eats.** Raising it 10px → 12px widens it by about a fifth.

**On a clipped line, the cost of larger text is characters lost, not
pixels gained.** This badge exists to say which project a block belongs
to — the deliberately textual answer to a question this project refuses
to answer with colour alone (`calendar.rs:83–91`, `RFC 002` must-have
6). *Fewer characters visible* is the cost that matters here, and it was
not measured.

**Measure, at 390 and 1280, in both month and week view:** how much of
the badge is actually painted, before and after the change, for a short
project name, a typical one, and one long enough to be clipped at both
sizes. Report painted width or characters — not the line's height.

**A finding of *free* is a real result and may well be the answer**: in
a seven-column month grid at 390 px a cell is narrow enough that the
badge may already be clipped away entirely before and after. **If so,
say that plainly** — it is a more interesting finding than *zero*,
because it says this badge does not survive a phone at all, and that is
a question about whether the information belongs on that surface.
**Record it; do not act on it.** Removing or moving the badge is a
design decision for `TYPE-003` with the other project-screen work, not
for this round.

## §3 One clearance that was not checked

`calendar.rs:216`, the hour label — **raised, and its vertical cost
measured properly** (line-height 15px → 16px against ~40px of per-hour
row, ~24px of headroom).

**The horizontal clearance was not checked.** The label is `absolute
left-0`; the day view's blocks are `absolute left-12 right-1`, so the
label has **48px** before it sits underneath the block column.

Arithmetic suggests `00:00` in tabular figures at 12px is around 30px,
so this very probably clears — **and that arithmetic is not offered as
evidence.** This series' predecessor closed six days ago on exactly
this: two things sharing a row, overlapping, with the gate that measures
*overflow* reporting a green cell because nothing asked about
*occlusion*. `overflow-gate`'s 120/120 cannot see this, and
`occlusion-gate` walks `header.navbar` only.

**Measure the rendered width of one hour label at 12px and state the
clearance against 48px.** One number. If it clears, that is the round's
answer and it closes the question.

## §4 The clipped third line, named rather than fixed

Round 1 found, by measuring across 15/20/30/45/60-minute events, that
the day-view badge's own line is clipped by the block's
`overflow-hidden` on anything under roughly an hour — **at 10px and at
12px alike**. That is why the site was kept, and keeping it on that
reason was the right call.

**That clipping is a defect in its own right.** The architect has
recorded it in `§10.60` as found-and-not-fixed. **Nothing is owed here
except not quietly fixing it** — it is not a type question, it belongs
to whichever round or release takes the calendar's block layout, and
four readings have now passed over it, which is precisely why it is
written down.

## §5 The census — nothing owed, one thing not to inherit

The architect's population figures were wrong, in the handoff, in
`ROADMAP.md` and in `§10.60`: a plain `grep -c` counts a comment that
*mentions* a class alongside a site that *renders* it. Corrected
everywhere, with the method named as the fault:

| | published | render sites |
|---|---|---|
| total `text-xs` | 126 | **120** |
| `/today`, `TYPE-002` | 23 | **21** |
| project screens, `TYPE-003` | 35 | **34**, now **36** |
| excluded remainder | 68 | **65** |
| `text-sm` | 157 | **151** |

```bash
grep -rn 'text-xs' crates/ --include='*.rs' | grep -vE ':[[:space:]]*//' | wc -l
```

**Round 1's own *"two hits"* for the arbitrary-value census was four**
for the same reason — two of them comments the round itself had just
written. The outcome (one render site left) was right; the evidence was
not. **Use the command above, and re-run it after writing comments, not
before.**

Writing a class name into a comment is not a fault and nothing should
change about it.

## §6 Acceptance

1. **The reasoning in `§10.60`** (§1) — or a review request disagreeing
   with the ruling.
2. **The badge's painted width, before and after, 390 and 1280, month
   and week, three name lengths** (§2).
3. **The hour label's rendered width against 48px** (§3).
4. **`CHANGELOG.md`**: the four colour spellings named as changed with
   no visible effect, or deliberately not named with the reason given in
   the review request. Silence in both is what is not acceptable.
5. **Tests per `.github/CONTRIBUTING.md` → *Before you open a pull
   request* → *Tests***, read there rather than copied from here.
   Baseline after round 1: `DEC-007` **687**, `cargo test -p peisear-web
   --lib` **246** — both confirmed independently by the architect.
   Re-derive rather than assume, and count the artefact with `wc -l`.
6. **`overflow-gate.mjs`** if any class in a rendered component changes.
   If this round changes no class — which is the likely outcome — say so
   and skip it, rather than running it to have something to report.
7. **Clippy both ways**, as CI runs it:
   ```bash
   cargo clippy --workspace --all-targets -- -D warnings
   cargo clippy --workspace --all-targets --no-default-features --features postgres -- -D warnings
   ```
8. **No screenshot or other non-text artefact committed without being
   checked for local information** — a path under `/home`, a real
   display name, a hostname. Named per artefact, because nothing scans
   them.

**`§10.60` stays open.** No type floor, no type requirement, no scan
over text sizes, no `text-xs` site touched, `NFR-A11Y-005` and
`contrast_scan` untouched.

## §7 Where to file a question

`.git-exclude/review-request/TYPE-001-round-2-…/README.md` — the
question, what was tried, and what you are declining to do without a
ruling. A blocking question goes in a file, never in chat.
