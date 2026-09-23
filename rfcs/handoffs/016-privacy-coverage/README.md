# Handoffs — privacy coverage

**Not RFC-governed.** `NFR-PRIV-008` (P1) has read *Partial* since it was
written, pointing at an audit as the thing that would complete it. The audit
completed at 0.27.0 and found no reachable boundary; what it did not leave
behind was tests for every endpoint it checked.
`.git-exclude/tasks/architect/019-nfr-priv-008-coverage-audit.md` measures the
shortfall: **seven assertions across five endpoints, every one of them a
regression test for a property that already holds.**

| ID | Link | What | Release |
|---|---|---|---|
| PRIV-001 | [PRIV-001](./PRIV-001-the-seven-missing-assertions.md) | ✅ Done. **Five, not the seven I counted** — two Class A cells were already covered and the implementer checked rather than inherited. Each new refusal paired with a known positive; the module doc corrected. Planting showed the three capacity tests load-bearing, and measured `§10.3`'s two independent barriers for the first time. | 0.37.0 |

## The classification is the useful part

A cross-user test is only writable where a cross-user request is
**constructible**. Three classes:

- **identity in the path** (`/api/users/{id}/…`) — substitute another id;
- **a resource id in the path** (`/settings/capacity/{id}`, `/inbox/{id}/read`)
  — name another user's row;
- **nothing in the path** — the session is the only identity, so "another
  user's" cannot be expressed at all, and the unauthenticated case is the only
  assertion the requirement can ask for.

The middle class is the one that was missed, because the file's own doc
described it as the third.
