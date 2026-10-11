# Handoffs — documentation accuracy

**0.51.0.** The release `042` scheduled and `0.49.0` could not hold: the
documents that describe the code, the document that introduces the product,
the guards that check them, and the two things the screenshots exposed.

**Five rounds and a candidate.** Each has one subject, because the list is
long and a round that carries two is a round nobody can review cleanly.

| ID | Link | What | Blocked on |
|---|---|---|---|
| DOC-001 | [the crate set and the manifest](./DOC-001-the-crate-set-and-the-manifest.md) | **`DOCS-006`'s countable half.** Both architecture documents and `docs/README.md` omit `peisear-i18n` from the workspace they enumerate; `deployment.md`'s *What to ship* lists **two of `static/`'s eight** files. **And `docs_scan`'s rules 1 and 3 were written deferred waiting for exactly this** — `rule_1_deferred_names_the_architecture_docs_gap` and `rule_3_deferred_names_the_missing_manifest_entries` assert the gap *exists*. Close the documents, flip the rules, delete the deferral tests | — |
| DOC-002 | *(after `RFC 0019`)* | **Superseded by `RFC 0019`.** The citations are not a cleanup — they are 137 symptoms of published documents deferring to unpublished ones. The RFC proposes moving the key information into the repository, after which rule 5 stands as written with no carve-out | **Owner decision, `RFC 0019 §7`** |
| DOC-003 | *(after DOC-001)* | **The front door.** `RFC 0016` proposal 2 — `README.md`'s hero, overview, *Why peisear?* and features — plus `README.md:77`'s PostgreSQL claim, `scaling.md:91-94`'s version of the same overclaim and its wrong `ROADMAP` cross-reference, and `first-run.md`'s sample log target (`peisear_web` where the code's is `peisear`) | — |
| DOC-004 | *(after DOC-001)* | **The guards that read one file too few.** `CONTRIBUTING.md:21`'s *"Rust 1.85+"* against `Cargo.toml`'s 1.88, and **rule 2 extended to read it**; `updated_at_authority_scan`'s single spelling (`updated_at = CURRENT_TIMESTAMP`, where the requirement bans the write) **with `pool.rs`'s plant relocated out of `peisear-storage/src`**; **`NFR-MNT-005`'s 25 files moved to `src/<mod>/tests.rs`** — not a text amendment; `RFC 0019 §3` reverses that ruling, since `project-instructions-rust.md` already gives the shape; and the scratch-hygiene convention written into the handoff exit conditions and `CONTRIBUTING.md` | — |
| DOC-005 | *(last, and in this order)* | **What the pictures showed.** **First** the `grow()` alignment defect — `TOUCH_TARGET`'s `min-h-11` with nothing centring the content, so a grown text element sits above its shorter neighbour (two instances published in `board.png`/`list.png` and `issue_detail.png`). **Then** the sprint-plan fixture, so the fourth screenshot is captured after the fix, not before it — the same ordering `DOCS-008` used for the brand string | — |
| REL-0.51.0 | *(the candidate)* | The cut, `DEC-028`, and **`requirements.md §11`'s two migration rows** from `RFC 0017` — import from other trackers first, instance migration and upgrade second | — |

---

## The decision `DOC-002` waits on

**Superseded by `RFC 0019`, 2026-10-12.** The three options set out here —
narrow the rule, commit the reviews, withdraw it — were framed on the
assumption that handoffs and RFCs are internal. **They are published.** The
owner's correction and what re-reading `.git-exclude/rules/` turned up are
in `rfcs/proposed/019-what-is-in-the-repository.md`.

## Carried, not scheduled

- **Whether the threat model should name the authorised colleague** — the
  second owner decision recorded in `042`, still open. `SEC-001` added data
  at rest; this is the other half, and eight `NFR-PRIV-*` requirements
  already defend against a legitimate teammate the document never mentions.
