# Handoffs — status drift

**Not RFC-governed.** A requirement's `*Status*` saying one thing while the
release says another — **six instances, all the architect's**, one of which
(`FR-SPR-004`) cost two handoffs: one shipped to `main` and half reverted, one
written and withdrawn unbuilt.

| ID | Link | What | Release |
|---|---|---|---|
| REQ-003 | [REQ-003](./REQ-003-is-a-stale-status-mechanically-detectable.md) | **An investigation, with a possible build at the end.** Is this catchable by a machine? Four candidate rules, each to be run over the six historical instances with its hit rate and its **false-positive count over all 162 entries** reported. **If no rule works, that is the deliverable** — and the remedy becomes procedural, for which there is evidence: the dev team has caught three of the six, every one while reading the specification to source changelog content. | 0.43.0 |
