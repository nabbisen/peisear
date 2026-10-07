# Handoffs — status drift

**Not RFC-governed.** A requirement's `*Status*` saying one thing while the
release says another — **six instances, all the architect's**, one of which
(`FR-SPR-004`) cost two handoffs: one shipped to `main` and half reverted, one
written and withdrawn unbuilt.

| ID | Link | What | Release |
|---|---|---|---|
| REQ-003 | [REQ-003](./REQ-003-is-a-stale-status-mechanically-detectable.md) | **An investigation, with a possible build at the end.** Is this catchable by a machine? Four candidate rules, each to be run over the six historical instances with its hit rate and its **false-positive count over all 162 entries** reported. **If no rule works, that is the deliverable** — and the remedy becomes procedural, for which there is evidence: the dev team has caught three of the six, every one while reading the specification to source changelog content. | 0.43.0 |
| DOCS-002 | [DOCS-002](./DOCS-002-nine-broken-links-in-the-published-api-docs.md) | `cargo doc --workspace` emits **23 warnings, nine of them unresolved intra-doc links** — and **docs.rs builds these crates**, so each one is live in the API docs of every released version. **Nothing has ever looked**: `clippy --all-targets` does not check doc links and `cargo doc` is in no gate. Count by class before fixing; the gate must be seen to fail. | 0.44.0 |

