//! Notification dispatch pipeline.
//!
//! ## Architecture
//!
//! Three stages connected by a tokio `mpsc::channel`:
//!
//! 1. **Detection** (callers, typically a snapshot/jobs loop in
//!    `peisear-web`). After observing a state change, the caller
//!    builds a [`DispatchEvent`] using helpers in [`edge`] and
//!    `tx.send`s it.
//! 2. **Filtering + delivery** ([`dispatch::dispatch_loop`]).
//!    Receives events, resolves the user's
//!    [`peisear_core::notifications::Preference`] (or default),
//!    applies severity and cooldown filters, persists an audit
//!    row, fans out to channels.
//! 3. **Channel send** (per-channel impls in [`channel`]).
//!    `in_app` is a no-op (the audit row is the artefact);
//!    `email` and `webhook` perform their own I/O.
//!
//! ## Cooperative shutdown
//!
//! The loop exits when its receiver returns `None`. The caller
//! holds the [`DispatchTx`] and can drop it to drain and stop
//! the loop, or send a oneshot for synchronous shutdown.
//!
//! ## Why a dedicated crate
//!
//! See `crates/peisear-notify/README.md`. Short answer: the
//! transport dependencies (SMTP today, more later) shouldn't
//! bleed into the web crate's compile graph, and a future
//! `peisear-ai` summariser will want to produce events without
//! depending on the web crate.

pub mod channel;
pub mod config;
pub mod dispatch;
pub mod edge;
pub mod email;

pub use dispatch::{DISPATCH_CHANNEL_BUFFER, DispatchEvent, DispatchRx, DispatchTx, dispatch_loop};
pub use edge::{detect_burnout_overload_edge, detect_burnout_stalled_edge};
