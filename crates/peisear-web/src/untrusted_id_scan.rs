//! `QA-021` §4 (RFC 005 §1, requirements baseline `§10.3`/`§11.5.4`) —
//! `§10.3` recorded a storage-layer gap that turned out not to exist:
//! every personal-data storage function already scopes on the
//! subject's identity (`user_capacities` 10/10, `notifications` 14/14,
//! `view_states` 3/3, `personal_metrics` 2/2, `user_burnout` 1/1,
//! `user_metrics_snapshots` 2/3 — the one exception is a job-side
//! aggregate with no per-subject caller at all), and no handler passes
//! a caller-supplied identity into any of them. `handlers/api_users.rs`'s
//! three personal-data endpoints (`burnout`, `capacity`,
//! `list_notifications`) all extract the path's `user_id`, validate it
//! against the session via `require_self`, and then use only
//! `user.id` (the session identity) for every storage call and every
//! response field. The `Path`-extracted value is checked and
//! discarded.
//!
//! **Two independent barriers already hold this invariant** —
//! `require_self` rejects a mismatch, and the path value is never used
//! even if it did not — so what's missing is not a third mechanism
//! (threading a requester parameter through thirty storage functions
//! to satisfy a `should`-level requirement, `§11.5.4`, that is already
//! achieved by a different route) but a guard on the invariant that
//! already holds: nothing today stops a *future* handler in this file
//! from passing the path-extracted `user_id` to a storage call or into
//! a response, instead of `user.id`.
//!
//! **What this asserts**: in `handlers/api_users.rs`, every bare use of
//! the identifier `user_id` (word-boundary matched, comments stripped,
//! struct-field labels like `pub user_id: String,` or `user_id:
//! user.id,` excluded since the colon marks a label rather than a
//! value) appears either as part of a `Path(user_id)` extraction site,
//! or on a line that also calls `require_self(`. A use anywhere else —
//! passed to a storage function, written into a response field —
//! fails the scan.
//!
//! **The name is pinned as well as its uses — `QA-021` round 2.** The
//! first version of this scan searched for the identifier `user_id`
//! by name, which the architect's own plant showed is not enough: a
//! handler that renames the extraction (`Path(uid): Path<String>`)
//! and routes `uid` straight into storage carries none of the
//! `user_id` needle at all, so the first assertion never sees it. A
//! second, independent assertion closes that: every `Path(` binding
//! in this file must destructure to the name `user_id`. Renaming the
//! binding now trips this assertion instead of silently escaping the
//! first one — the pair only works together, which is why they are
//! two separate tests rather than one merged check (same reasoning
//! `dec_007_scan`/`dec_007_ci_scan` split on: independent failure
//! attribution for two links in one chain).
//!
//! **Narrow by design, not by oversight.** `QA-021` §4 considered a
//! broader rule — "no `Path`-extracted identity reaches any storage
//! call", across every handler file — and rejected it as not cheaply
//! expressible: a text scan cannot distinguish an identity-shaped
//! `Path` parameter (`user_id`) from the many legitimate resource ids
//! handlers extract and pass to storage directly under a different
//! authorization model (`project_id`, `issue_id`, `slug`, `sprint_id`,
//! `team_id` — membership- or ownership-gated, not self-only). This
//! codebase has no type-level or naming convention that separates
//! "identity" parameters from "resource" parameters (all are plain
//! `String`), so a broader scan would have to guess which `Path`
//! fields are identity-shaped — exactly what `QA-021` §4 says not to
//! build. `handlers/teams.rs:278` and `:317` also take a caller-supplied
//! `target_user_id` (role change, member removal) — confirmed out of
//! scope: those are team-membership operations gated by
//! `can_manage_team()` (`QA-007`), not personal-data reads, and this
//! scan does not (and should not) reach them.

use std::fs;
use std::path::Path;

/// Every byte offset in `line` where the identifier `user_id` starts,
/// word-boundary matched (not preceded/followed by an identifier
/// character, so `session_user_id`/`path_user_id`/`target_user_id`
/// never match) and not immediately followed by a colon (allowing
/// intervening whitespace) — which excludes struct field labels and
/// declarations (`pub user_id: String,`, `user_id: user.id,`) from
/// being read as a use of the local path-extracted variable.
fn identifier_uses(line: &str) -> Vec<usize> {
    let bytes = line.as_bytes();
    let needle = "user_id";
    let is_ident_char = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
    let mut out = Vec::new();
    let mut start = 0;
    while let Some(rel) = line[start..].find(needle) {
        let idx = start + rel;
        let before_ok = idx == 0 || !is_ident_char(bytes[idx - 1]);
        let after_idx = idx + needle.len();
        let after_ok = after_idx >= bytes.len() || !is_ident_char(bytes[after_idx]);
        if before_ok && after_ok {
            let after_ws: String = line[after_idx..]
                .chars()
                .take_while(|c| c.is_whitespace())
                .collect();
            let followed_by_colon = line[after_idx + after_ws.len()..].starts_with(':');
            if !followed_by_colon {
                out.push(idx);
            }
        }
        start = idx + 1;
    }
    out
}

#[test]
fn path_extracted_user_id_only_reaches_require_self() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let file = manifest_dir.join("src/handlers/api_users.rs");
    let source =
        fs::read_to_string(&file).unwrap_or_else(|e| panic!("read {}: {e}", file.display()));

    let mut extraction_sites = 0usize;
    let mut violations = Vec::new();

    for (i, line) in source.lines().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("//") {
            continue; // comments, including doc comments naming `user_id`
        }
        if line.contains("Path(user_id)") {
            extraction_sites += 1;
        }
        for idx in identifier_uses(line) {
            // The extraction site itself (`Path(user_id): Path<String>`)
            // is a binding, not a use of the bound value -- skip the
            // occurrence that falls inside that exact substring.
            let in_extraction_site =
                line[..idx].ends_with("Path(") && line[idx..].starts_with("user_id)");
            if in_extraction_site {
                continue;
            }
            if !line.contains("require_self(") {
                violations.push(format!("  line {}: {}", i + 1, line.trim()));
            }
        }
    }

    assert!(
        extraction_sites > 0,
        "found no `Path(user_id)` extraction site in handlers/api_users.rs -- \
         this scan's target shape may have changed"
    );

    assert!(
        violations.is_empty(),
        "handlers/api_users.rs: the path-extracted `user_id` must only ever \
         reach a `require_self(...)` call -- every storage call and response \
         field must use the session identity (`user.id`) instead \
         (`QA-021`, requirements baseline `§10.3`/`§11.5.4`):\n{}",
        violations.join("\n")
    );
}

/// Every `Path(<name>)` extractor-destructuring binding found in
/// `line`, as `(name, line_offset_of_the_open_paren)`. Reads the
/// identifier between `Path(` and its closing `)` verbatim -- this
/// file's extractions are all single-value (`Path(user_id):
/// Path<String>`), not tuples, so no comma-splitting is needed here.
fn path_extraction_bindings(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut start = 0;
    while let Some(rel) = line[start..].find("Path(") {
        let open = start + rel + "Path(".len();
        if let Some(close_rel) = line[open..].find(')') {
            out.push(line[open..open + close_rel].trim().to_string());
            start = open + close_rel + 1;
        } else {
            break;
        }
    }
    out
}

/// `QA-021` round 2: the companion to the test above. Pins the
/// *name* every `Path(` extraction in this file must bind, so a
/// rename can't carry the untrusted id past the first assertion
/// under a different identifier.
#[test]
fn every_path_extraction_binds_the_name_user_id() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let file = manifest_dir.join("src/handlers/api_users.rs");
    let source =
        fs::read_to_string(&file).unwrap_or_else(|e| panic!("read {}: {e}", file.display()));

    let mut bindings_found = 0usize;
    let mut violations = Vec::new();

    for (i, line) in source.lines().enumerate() {
        if line.trim_start().starts_with("//") {
            continue;
        }
        for name in path_extraction_bindings(line) {
            bindings_found += 1;
            if name != "user_id" {
                violations.push(format!("  line {}: Path({name}) -- {}", i + 1, line.trim()));
            }
        }
    }

    assert!(
        bindings_found > 0,
        "found no `Path(...)` extractor binding in handlers/api_users.rs -- \
         this scan's target shape may have changed"
    );

    assert!(
        violations.is_empty(),
        "handlers/api_users.rs: every `Path(...)` extraction must bind the \
         name `user_id` -- a renamed binding would carry the untrusted path \
         id past the sibling test's name-based check undetected \
         (`QA-021` round 2):\n{}",
        violations.join("\n")
    );
}
