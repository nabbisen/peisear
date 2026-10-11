# Handoffs — documentation accuracy

**0.51.0.** The release `042` scheduled and `0.49.0` could not hold: the
documents that describe the code, the document that introduces the product,
the guards that check them, and the two things the screenshots exposed.

**Five rounds and a candidate.** Each has one subject, because the list is
long and a round that carries two is a round nobody can review cleanly.

| ID | Link | What | Blocked on |
|---|---|---|---|
| DOC-001 | [the crate set and the manifest](./DOC-001-the-crate-set-and-the-manifest.md) | **`DOCS-006`'s countable half.** Both architecture documents and `docs/README.md` omit `peisear-i18n` from the workspace they enumerate; `deployment.md`'s *What to ship* lists **two of `static/`'s eight** files. **And `docs_scan`'s rules 1 and 3 were written deferred waiting for exactly this** — `rule_1_deferred_names_the_architecture_docs_gap` and `rule_3_deferred_names_the_missing_manifest_entries` assert the gap *exists*. Close the documents, flip the rules, delete the deferral tests | — |
| DOC-002 | *(after the owner's decision below)* | **The citations.** **137** committed occurrences of `.git-exclude` outside `docs_scan.rs` itself, and **83 of the 104 files holding them are under `rfcs/handoffs`** — handoffs naming where their own reviews live. This is not a cleanup; it is a rule nobody has written. Flipping rule 5's `#[ignore]` depends on it | **Owner decision** |
| DOC-003 | *(after DOC-001)* | **The front door.** `RFC 0016` proposal 2 — `README.md`'s hero, overview, *Why peisear?* and features — plus `README.md:77`'s PostgreSQL claim, `scaling.md:91-94`'s version of the same overclaim and its wrong `ROADMAP` cross-reference, and `first-run.md`'s sample log target (`peisear_web` where the code's is `peisear`) | — |
| DOC-004 | *(after DOC-001)* | **The guards that read one file too few.** `CONTRIBUTING.md:21`'s *"Rust 1.85+"* against `Cargo.toml`'s 1.88, and **rule 2 extended to read it**; `updated_at_authority_scan`'s single spelling (`updated_at = CURRENT_TIMESTAMP`, where the requirement bans the write) **with `pool.rs`'s plant relocated out of `peisear-storage/src`**; `NFR-MNT-005`'s rule reconciled (`§10.37`); and the scratch-hygiene convention written into the handoff exit conditions and `CONTRIBUTING.md` | — |
| DOC-005 | *(last, and in this order)* | **What the pictures showed.** **First** the `grow()` alignment defect — `TOUCH_TARGET`'s `min-h-11` with nothing centring the content, so a grown text element sits above its shorter neighbour (two instances published in `board.png`/`list.png` and `issue_detail.png`). **Then** the sprint-plan fixture, so the fourth screenshot is captured after the fix, not before it — the same ordering `DOCS-008` used for the brand string | — |
| REL-0.51.0 | *(the candidate)* | The cut, `DEC-028`, and **`requirements.md §11`'s two migration rows** from `RFC 0017` — import from other trackers first, instance migration and upgrade second | — |

---

## The decision `DOC-002` waits on

**Recorded in `042`'s README since 2026-10-11 and now blocking a round.**

Rule 5 bans committed citations of `.git-exclude/`. Measured today: **137
occurrences outside the scan's own file**, and **83 of the 104 files that
hold them are handoffs in `rfcs/handoffs/`** — documents whose whole job is
to record why a decision was made, naming the review that made it.

**So the rule as written asks the project to stop referring to its own
reasoning.** Three ways out, and the choice is the owner's because it
decides what the repository is:

1. **Reviews become committed artefacts.** The citations become ordinary
   links, rule 5 narrows to "no citation of a path that is still
   gitignored", and the richest design record this project has stops being
   invisible. **Cost**: repository size, and a public audit trail of every
   review.
2. **The rule narrows to where it matters.** Handoffs and RFCs may cite the
   path; **published surfaces may not** — `README.md`, `docs/`, and any
   `//!` comment that reaches docs.rs. **Today exactly four files would be
   in scope**: three under `docs/specification`, one under
   `docs/development`, plus whatever `crates/peisear-web`'s three turn out
   to be. **Cheapest, and it keeps the defect the rule was written for** —
   `jwt.rs:13`'s doc comment citing a path no reader of docs.rs can open.
3. **The rule is withdrawn**, and the 137 stand. Honest, and it gives up a
   real check.

**My recommendation is 2.** The defect that opened this was a *published*
document pointing at something a reader cannot open; a handoff pointing at a
review the dev team and architect both have is not that defect. **1 is a
bigger question than a scan rule and should not be decided by one.**

## Carried, not scheduled

- **Whether the threat model should name the authorised colleague** — the
  second owner decision recorded in `042`, still open. `SEC-001` added data
  at rest; this is the other half, and eight `NFR-PRIV-*` requirements
  already defend against a legitimate teammate the document never mentions.
