# Handoffs — a notification that cannot fire

**Not RFC-governed.** `project_trend_decline` has a constant, a label, message
strings and a component branch — and **no emitter**. `ROADMAP.md` lists it
under deferred Phase 2 candidates.

| ID | Link | What | Release |
|---|---|---|---|
| NTF-001 | [NTF-001](./NTF-001-a-preference-row-for-a-notification-that-cannot-fire.md) | Because the kind is in `all_user_facing()`, `/settings/notifications` offers In-app, Email, Webhook and severity controls for a notification that can never arrive — **and `all_kinds_silenced` requires it to be silenced before `FR-NTF-006`'s banner will say everything is.** A user asking *have I turned everything off?* gets the wrong answer. One function changes; the plumbing stays for whoever builds it. | 0.41.0 |
