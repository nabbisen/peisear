# peisear-notify

Notification dispatch pipeline for [peisear](../../README.md).

## Responsibility

This crate owns:

- **Edge detection**: deciding when a tracked signal warrants a
  fresh notification (e.g. a user crossing the burnout overload
  threshold for the first time, not every snapshot tick that
  they remain over).
- **Dispatch loop**: a tokio task that drains
  [`DispatchEvent`]s from an mpsc channel, applies user
  preferences and cooldown, and fans out to channels.
- **Channels**: one impl per delivery medium —
  - `in_app`: the audit row in the `notifications` table is
    itself the in-app artefact (the inbox reads from it).
  - `email`: SMTP via `wasm-smtp`. Configurable transport,
    Cloudflare Workers compatible (via `wasm-smtp-cloudflare`).
  - `webhook`: HTTP POST a JSON envelope to a per-user URL.

## Non-responsibilities

The crate **does not** own:

- HTTP routes for the inbox or preferences pages — those live
  in `peisear-web::handlers::notifications` and
  `peisear-web::handlers::notification_preferences`.
- UI rendering — that's `peisear-web::components::notifications`
  and `peisear-web::components::notification_preferences`.
- Domain types (`Notification`, `Preference`, `Severity`,
  `kind` and `channel` constants) — those live in
  `peisear-core::notifications` so any crate, including future
  ones like `peisear-ai`, can produce notification events
  without taking a heavy dependency on the dispatch pipeline.
- Storage CRUD — that's `peisear-storage::notifications`.

## Why a dedicated crate

The pipeline has its own dependency surface (SMTP, future
webhook HTTP client, possibly future AI-summarised digests).
Isolating it means the `peisear-web` crate stays lean: a
contributor working on the inbox UI doesn't pay the
compile-time cost of the SMTP transport, and a future
edge-targeted deployment can swap transports without changes
to the web crate.

## Why a verb-form name

Verb form follows the responsibility shape: this crate
*notifies*. Compare with `peisear-storage` (storage as a
concept), `peisear-auth` (auth as a system); `peisear-notify`
(notify as an action). The naming preserves room to grow
into adjacent verbs the project might want later — `peisear-
broadcast`, `peisear-summarise`, etc. — without a
"notifications" plural that would already feel claimed.
