# Handoffs — documentation accuracy

**0.51.0.** The release `042` scheduled and `0.49.0` could not hold: the
documents that describe the code, the document that introduces the product,
the guards that check them, and the two things the screenshots exposed.

**Five rounds and a candidate.** Each has one subject, because the list is
long and a round that carries two is a round nobody can review cleanly.

| ID | Link | What | Blocked on |
|---|---|---|---|
| DOC-001 | [the crate set and the manifest](./DOC-001-the-crate-set-and-the-manifest.md) | **`DOCS-006`'s countable half.** Both architecture documents and `docs/README.md` omit `peisear-i18n` from the workspace they enumerate; `deployment.md`'s *What to ship* lists **two of `static/`'s eight** files. **And `docs_scan`'s rules 1 and 3 were written deferred waiting for exactly this** — `rule_1_deferred_names_the_architecture_docs_gap` and `rule_3_deferred_names_the_missing_manifest_entries` assert the gap *exists*. Close the documents, flip the rules, delete the deferral tests | — |
| DOC-002 | [the nineteen links](./DOC-002-the-nineteen-links.md) | **`RFC 0019`, accepted.** The nineteen citations inside documentation, classified one at a time — four are not defects — plus the false statement in `rfcs/done/011` (the CDP driver is committed, not untracked), and **rule 5 narrowed to documents, RFCs and published doc comments** with four named exclusions, then switched on | — |
| DOC-006 | *(sized by DOC-002's survey)* | **Decisions stated where they take effect.** `RFC 0019 §3`: a decision is stated **once**, in the document it governs, at a location the other mentions point to. `DEC-013` is named as a P0 requirement's source and says nothing anywhere; `DEC-049` is discussed four times in `requirements.md` and is still unreadable. **In situ is necessary and not sufficient** | — |
| DOC-003 | [the front door](./DOC-003-the-front-door.md) | **The front door.** `RFC 0016` proposal 2 — `README.md`'s hero, overview, *Why peisear?* and features — plus `README.md:77`'s PostgreSQL claim, `scaling.md:91-94`'s version of the same overclaim and its wrong `ROADMAP` cross-reference, and `first-run.md`'s sample log target (`peisear_web` where the code's is `peisear`) | — |
| DOC-004 | *(after DOC-001)* | **The guards that read one file too few.** **`DOC-001`'s review F-3 first**: rule 1 reads the **root** `README.md`, not `docs/README.md` — four documents claim a crate count, the rule covers three, and the uncovered one is the one that was wrong. Then `CONTRIBUTING.md:21`'s *"Rust 1.85+"* against `Cargo.toml`'s 1.88, and **rule 2 extended to read it**; `updated_at_authority_scan`'s single spelling (`updated_at = CURRENT_TIMESTAMP`, where the requirement bans the write) **with `pool.rs`'s plant relocated out of `peisear-storage/src`**; **`NFR-MNT-005`'s 25 files moved to `src/<mod>/tests.rs`** — not a text amendment; `RFC 0019 §3` reverses that ruling, since `project-instructions-rust.md` already gives the shape; and the scratch-hygiene convention written into the handoff exit conditions and `CONTRIBUTING.md` | — |
| DOC-005 | [what the pictures showed](./DOC-005-what-the-pictures-showed.md) | **What the pictures showed.** **First** the `grow()` alignment defect — **whose root is `DEC-049` itself**, not the helper: *"a `Grow` cluster inside a positive-`gap` container is **presumed** to satisfy clause (2) with no further verification"*. A presumption written into a rule, applied at **262 call sites across 18 files**, and nobody looked at the result until a screenshot was taken. **Revisit the decision, do not only centre the content** — `TOUCH_TARGET`'s `min-h-11` with nothing centring the content, so a grown text element sits above its shorter neighbour (two instances published in `board.png`/`list.png` and `issue_detail.png`). **Then** the sprint-plan fixture, so the fourth screenshot is captured after the fix, not before it — the same ordering `DOCS-008` used for the brand string | — |
| REL-0.51.0 | *(the candidate)* | The cut, `DEC-028`, and **`requirements.md §11`'s two migration rows** from `RFC 0017` — import from other trackers first, instance migration and upgrade second | — |

---

## Carried, not scheduled

- **Whether the threat model should name the authorised colleague** — the
  second owner decision recorded in `042`, still open. `SEC-001` added data
  at rest; this is the other half, and eight `NFR-PRIV-*` requirements
  already defend against a legitimate teammate the document never mentions.
