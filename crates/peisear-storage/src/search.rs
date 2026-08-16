//! Global search across the user's accessible projects and
//! open issues. Phase A Step 4 (peisear-feature-spec-v2.1 §4.5).
//!
//! ## Scope
//!
//! - **Projects** the user can access (personal projects they
//!   own + team projects of teams they belong to). Match on
//!   `name`.
//! - **Open issues** in those projects. "Open" here means
//!   `status != 'done'` — i.e. `open` or `in_progress`. Match on
//!   `title`. Description is intentionally not included; the
//!   long-tail false positives ("the user typed 'login' and got
//!   30 issues with 'login' incidentally in their description")
//!   are more confusing than the rare hit they enable.
//!
//! Note that **completed issues are deliberately excluded** —
//! search is for "what's currently in flight". Completed work
//! has its own retrieval surfaces (sprint summaries, project
//! detail filters). This is the v2.1 spec §4.5 / decision
//! A-6 = B scope.
//!
//! ## Why LIKE rather than FTS5
//!
//! peisear targets small-to-medium teams (5–30 people). Full
//! scans of `issues` and `projects` at this scale finish in a
//! few milliseconds — well below typeahead UX latency. FTS5
//! would add (1) virtual-table schema, (2) INSERT/UPDATE/DELETE
//! sync triggers, (3) D1 portability concerns, and (4) tokenizer
//! choice for Japanese / English. The trade-off is unfavourable
//! at this scale. Future regression at higher data volumes is
//! tracked in ROADMAP under "List filter/sort future
//! enhancements" sibling section.
//!
//! ## LIKE meta-character escaping
//!
//! User input may contain `%` / `_` / `\` — the LIKE pattern
//! meta-characters. Without escaping, a search for "100%"
//! becomes a wildcard pattern matching "100" + anything, not
//! the literal string. We use `ESCAPE '\'` and prefix each
//! meta-character with `\` in [`escape_like_meta`].
//!
//! sqlx's `bind()` already prevents SQL injection — this is a
//! **search-correctness** concern, not a security one.

use crate::{Pool, StorageResult};

/// Maximum number of items per category in a single search
/// response. Caller may pass smaller `limit` for typeahead;
/// this is the upper bound.
pub const MAX_LIMIT: i64 = 200;

/// One search hit — either a project or an issue. Caller picks
/// up `kind` to dispatch to the right rendering path.
#[derive(Debug, Clone)]
pub enum SearchHit {
    Project {
        id: String,
        name: String,
    },
    Issue {
        id: String,
        project_id: String,
        project_name: String,
        title: String,
        /// The parent issue's title, when this hit is a
        /// sub-issue (`INBOX-001`, RFC 003 D3). `None` for a
        /// top-level issue. Carried in the same query via a
        /// `LEFT JOIN` — a second query per page is exactly
        /// what RFC 003's original test plan proposed and the
        /// handoff overrode.
        parent_title: Option<String>,
    },
}

/// Search projects the user can access whose name matches `q`.
///
/// `q` is the user's literal search string. We escape LIKE
/// meta-characters before binding so a query like "100%"
/// matches the literal `%` rather than acting as a wildcard.
/// Caller should pass `q.trim()` so leading/trailing whitespace
/// doesn't shift matches.
pub async fn projects_by_name(
    pool: &Pool,
    user_id: &str,
    q: &str,
    limit: i64,
) -> StorageResult<Vec<SearchHit>> {
    let pattern = format!("%{}%", escape_like_meta(q));
    let limit = limit.min(MAX_LIMIT);

    let rows: Vec<(String, String)> = sqlx::query_as(
        r#"
        SELECT p.id, p.name
        FROM projects p
        WHERE p.name LIKE ?1 ESCAPE '\'
          AND (
            (p.team_id IS NULL AND p.owner_id = ?2)
            OR EXISTS (
              SELECT 1 FROM team_memberships m
              WHERE m.team_id = p.team_id AND m.user_id = ?2
            )
          )
        ORDER BY p.updated_at DESC
        LIMIT ?3
        "#,
    )
    .bind(&pattern)
    .bind(user_id)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|(id, name)| SearchHit::Project { id, name })
        .collect())
}

/// Search open issues in the user's accessible projects whose
/// title matches `q`.
///
/// "Open" = `status != 'done'`, per v2.1 §4.5 decision A-6 = B.
/// The join to `projects` is needed for two purposes:
///
/// 1. enforce the same access predicate as
///    [`projects_by_name`] (a user shouldn't see issues in a
///    project they can't access); and
/// 2. carry the project name into the hit, so the search-results
///    UI can show "Login error · Customer Portal" without a
///    second round trip per result.
///
/// A second `LEFT JOIN` (`INBOX-001`, RFC 003 D3) carries the
/// parent issue's title into the same result set when this hit
/// is a sub-issue — one query, not a second batched fetch of
/// parent ids. A `LEFT JOIN` cannot drop a row: a top-level issue
/// (`parent_issue_id IS NULL`) still matches, with `parent_title`
/// simply `NULL`.
pub async fn open_issues_by_title(
    pool: &Pool,
    user_id: &str,
    q: &str,
    limit: i64,
) -> StorageResult<Vec<SearchHit>> {
    let pattern = format!("%{}%", escape_like_meta(q));
    let limit = limit.min(MAX_LIMIT);

    let rows: Vec<(String, String, String, String, Option<String>)> = sqlx::query_as(
        r#"
        SELECT i.id, i.project_id, p.name AS project_name, i.title,
               parent.title AS parent_title
        FROM issues i
        JOIN projects p ON p.id = i.project_id
        LEFT JOIN issues parent ON parent.id = i.parent_issue_id
        WHERE i.title LIKE ?1 ESCAPE '\'
          AND i.status != 'done'
          AND (
            (p.team_id IS NULL AND p.owner_id = ?2)
            OR EXISTS (
              SELECT 1 FROM team_memberships m
              WHERE m.team_id = p.team_id AND m.user_id = ?2
            )
          )
        ORDER BY i.updated_at DESC
        LIMIT ?3
        "#,
    )
    .bind(&pattern)
    .bind(user_id)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(
            |(id, project_id, project_name, title, parent_title)| SearchHit::Issue {
                id,
                project_id,
                project_name,
                title,
                parent_title,
            },
        )
        .collect())
}

/// Escape LIKE meta-characters so they match literally rather
/// than as wildcards. Used together with `ESCAPE '\'` in the
/// query.
///
/// Backslash itself is escaped first so we don't double-escape
/// the introducers in `\%` and `\_`.
fn escape_like_meta(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_handles_meta_characters() {
        assert_eq!(escape_like_meta("100%"), "100\\%");
        assert_eq!(escape_like_meta("under_score"), "under\\_score");
        assert_eq!(escape_like_meta("a\\b"), "a\\\\b");
        // Backslash must come first; otherwise we'd
        // double-escape the introducer.
        assert_eq!(escape_like_meta("a%b"), "a\\%b");
    }

    #[test]
    fn escape_passes_normal_text_through() {
        assert_eq!(escape_like_meta("login error"), "login error");
        assert_eq!(escape_like_meta("ログイン"), "ログイン");
    }
}
