# I18N-005c — Team and sprint surfaces

**Issued by**: Architect
**Date**: 2026-08-10
**Priority**: P1
**Depends on**: I18N-005a (pattern settled)
**Parallel with**: 005b, 005d, 005e

Pattern rules are in the queue README; only what is specific to this group
appears here.

---

## 1. Scope

`components/sprints.rs` (~31 literals) and `components/teams.rs` (~20), plus
user-visible strings in the matching handlers.

## 2. The normative footnote — the one thing in this group that must not move

`components/teams.rs` renders the team-detail privacy footnote required by
`FR-TEAM-005`:

> Project trends and workload distribution are visible to all members.
> Personal sustainability data (burnout panel, /today) is visible only to the
> individual concerned. Admin is a management role, not an oversight role.

External design §10.3 states this wording is **normative and MUST NOT be
paraphrased**. It is the sentence by which the product explains its own privacy
boundary to a team, and `FR-TEAM-005`'s acceptance criterion is that the string
"management role, not an oversight role" appears on that screen.

Convert it **byte-exactly**. Not reflowed, not re-punctuated, not split into
sentence-per-key unless the rendered result is byte-identical.

If the guard objects to any part of it, **stop and escalate**. A guard rejection
here means either the guard's term list is wrong or a normative requirement is —
and neither is resolvable by editing the sentence.

Note it references `/today`. That is already correct; do not "fix" it.

## 3. Sprint vocabulary is requirement-bound

`FR-SPR-003`: work not completed in a sprint is described as **"carried over"** —
never as failure or shortfall. §1.7 prohibits the alternatives.

Sprint lifecycle words (`planned`, `active`, `completed`) are a closed set
appearing in more than one place: parameterise per rule 3.

The burndown chart's caption states what the chart does *and does not* show
(external design §6 SCR-20). That is copy, and it is load-bearing — RFC 004's
chart rules forbid an ideal line or a prediction line, and the caption is what
tells a reader the absence is deliberate.

## 4. Watch for

- **Team member tables** mix our column headings with user display names. Rule
  1 applies directly.
- **Role words** (admin, member, viewer) are a closed set — parameterise.
- **Last-admin guard** and **non-member concealment** messages
  (`FR-TEAM-003`, `FR-TEAM-004`): the second must not reveal whether a team
  exists. If converting it makes the two refusal paths render differently, that
  is a disclosure finding, not a copy detail — escalate.

## 5. Tests

Guard covers new entries; exhaustiveness holds; rendered output semantically
identical. **Assert the `FR-TEAM-005` footnote renders byte-identically** —
add that assertion if none exists, because "semantically identical" is too
loose a bar for a string a requirement quotes verbatim.

## 6. Acceptance

1. No user-visible literal left in either component or their handlers.
2. The privacy footnote renders byte-identically, asserted by test.
3. Sprint and role closed sets parameterised.
4. Guard passes; rendered output semantically identical elsewhere.
5. fmt and clippy exit 0; suite counts unchanged.
6. Survey reported per 005a §4.1.

## 7. Prohibited

Do not paraphrase, reflow, or re-punctuate the footnote. Do not reword sprint
carry-over language. Do not make the two refusal paths in §4 distinguishable.

## 8. Review focus to request

1. The footnote assertion — that it genuinely compares byte-for-byte.
2. Whether any closed set you parameterised is actually reused across templates,
   or whether flat variants were right (rule 3).
3. Anything in the refusal paths that changed shape.
