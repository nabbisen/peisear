//! Serves `static/`'s eight files from memory, embedded into the
//! binary at compile time (`FIV-005`, `§10.41`) rather than resolved
//! against the process's working directory at request time.
//!
//! `cargo install peisear` publishes no `static/` directory alongside
//! the installed binary (`static_js_scan.rs`'s own measurement,
//! `cargo package -p peisear --list`: seven files, none of them
//! under `static/`) -- so a filesystem `tower_http::services::ServeDir`
//! (this round's own predecessor here, dropped along with the
//! `fs` feature that provided it) 404s on every path, on every
//! installed copy. The content has to already be inside the binary by
//! the time a request for it arrives.

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::LazyLock;

use axum::{
    body::Body,
    extract::Path,
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response},
};
use include_dir::{Dir, DirEntry, File, include_dir};

/// `static/`'s contents at compile time, rooted the same two
/// directories above this crate's manifest that
/// [`crate::static_js_scan`] already resolves at runtime for its own,
/// separate reasons -- one real path, read two different ways.
static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../static");

/// Every file under `dir`, recursively (`FIV-005` round 3 `§12`):
/// [`Dir::files`] returns direct children only, while
/// [`Dir::get_file`] (via `get_entry`) recursively searches --
/// building [`ETAGS`] from the former while [`serve`] looks files up
/// with the latter is exactly the mismatch that let a nested
/// `static/vendor/foo.js` resolve in one and vanish in the other. One
/// walk, used everywhere a file list is needed, so there is only one
/// definition of "every file" to drift out of step with itself.
fn all_files<'a>(dir: &Dir<'a>, out: &mut Vec<&'a File<'a>>) {
    for entry in dir.entries() {
        match entry {
            DirEntry::File(f) => out.push(f),
            DirEntry::Dir(d) => all_files(d, out),
        }
    }
}

/// One strong `ETag` per embedded file, computed once at first use and
/// cached for the rest of the process's life (`FIV-005` round 2 `§9`:
/// hashing 2.93 MB on every request trades bandwidth for CPU, which is
/// not the point of caching at all). Keyed on the same relative path
/// [`serve`] receives, so the lookup is a hash-map hit, not a second
/// walk of the embedded tree -- and built by [`all_files`], the same
/// walk [`serve`]'s own `STATIC_DIR.get_file` can reach, so the two
/// can no longer disagree about what exists.
static ETAGS: LazyLock<HashMap<&'static str, String>> = LazyLock::new(|| {
    let mut files = Vec::new();
    all_files(&STATIC_DIR, &mut files);
    files
        .into_iter()
        .map(|f| (f.path().to_str().unwrap(), etag_for(f.contents())))
        .collect()
});

/// A strong `ETag` over `bytes`: quoted, and any byte difference
/// changes it (`DefaultHasher`/`SipHash`, not a cryptographic hash --
/// this tag only ever has to distinguish one build's embedded content
/// from another's within one running process, never resist a chosen-
/// input attacker, so std's own hasher is the whole dependency this
/// needs).
fn etag_for(bytes: &[u8]) -> String {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    bytes.hash(&mut hasher);
    format!("\"{:016x}\"", hasher.finish())
}

/// Does `headers`' `If-None-Match` (RFC 7232 §3.2) already name
/// `etag`? `*` matches any existing representation -- the caller only
/// reaches this once a file is confirmed to exist, so `*` is always a
/// match here. Otherwise a comma-separated list of quoted tags, any
/// one of which matching is enough.
fn if_none_match_matches(headers: &HeaderMap, etag: &str) -> bool {
    let Some(value) = headers
        .get(header::IF_NONE_MATCH)
        .and_then(|v| v.to_str().ok())
    else {
        return false;
    };
    value
        .split(',')
        .map(str::trim)
        .any(|candidate| candidate == "*" || candidate == etag)
}

/// `304 Not Modified`: the `ETag` repeated so the client knows which
/// representation was revalidated, and no body -- a body on a 304 is
/// the defect this round exists to close, with extra steps.
///
/// **`Cache-Control` repeated too** (`FIV-005` round 3 `§13`, RFC 9111
/// §4.3.4): a 304 is how a cache refreshes what it already stored, and
/// the directive governing that stored copy is part of what it needs
/// to update. Cost is near zero under `no-cache` -- there is no
/// freshness lifetime to recompute -- which is why this was a clause
/// to add rather than a design question to resolve.
fn not_modified(etag: &str) -> Response {
    (
        StatusCode::NOT_MODIFIED,
        [
            (header::ETAG, etag.to_string()),
            (header::CACHE_CONTROL, "no-cache".to_string()),
        ],
    )
        .into_response()
}

/// `200 OK` with the asset's content type, its `ETag`, and
/// `Cache-Control: no-cache` -- **store the body, but always
/// revalidate before using it** (`FIV-005` round 2 `§9`'s own
/// constraint): `/static/app.css` is the same URL forever, so a
/// `max-age` would serve a stale stylesheet against a newer page for
/// however long it lasted, and that failure mode -- a half-styled
/// page after an upgrade, cleared only by a hard refresh the user does
/// not know to perform -- is worse than the slow load this round
/// closes. `no-cache` keeps every navigation's body off the wire
/// (via a 304) without ever risking that window.
fn cached_response(content_type: &'static str, etag: &str, body: Body) -> Response {
    (
        [
            (header::CONTENT_TYPE, content_type.to_string()),
            (header::ETAG, etag.to_string()),
            (header::CACHE_CONTROL, "no-cache".to_string()),
        ],
        body,
    )
        .into_response()
}

/// `GET /static/{*path}`. The embedded set is exactly what `static/`
/// held at compile time, under the same relative paths -- so this
/// serves every file the old `ServeDir` did, not a curated subset.
///
/// **Content type by extension, not a MIME-sniffing crate**: `.css`
/// and `.js` are the only two extensions `static_js_scan`'s own
/// `SERVABLE_ASSET_TYPES` allows under this directory today, so the
/// match is exhaustive for what can actually be here, not a guess at
/// what might be.
///
/// **Debug builds read the file from disk first** (`FIV-005` §2's
/// development path): editing `static/app.css` and reloading a page
/// should not cost a relink. Measured before deciding -- a one-line
/// `static/app.css` edit forces a `cargo build -p peisear` relink of
/// `peisear-web` and `peisear` (the embedded bytes are part of their
/// compiled output), **18 seconds** on this machine, for a change a
/// contributor expects to see on the next page load. The embedded
/// copy is still the fallback in a debug build, not removed, so a
/// `cargo test`/`cargo build` running somewhere `static/` does not
/// exist on disk (a packaged source tarball, a stripped-down CI
/// checkout) keeps working exactly as release does. **Release builds
/// never touch the filesystem** -- the whole point of this module --
/// `#[cfg(debug_assertions)]` is compiled out of them entirely, not
/// merely branched around at runtime.
pub async fn serve(Path(path): Path<String>, headers: HeaderMap) -> Response {
    #[cfg(debug_assertions)]
    if let Some(bytes) = read_from_disk_in_debug(&path) {
        // Computed per request, deliberately, not cached alongside
        // `ETAGS`: a debug build's whole point is that disk content
        // can change between requests without a rebuild, so a cached
        // tag here would go stale the same way the old `ServeDir` made
        // live edits possible in the first place. Not the hot path
        // `§9`'s "compute once" is about -- that's the embedded,
        // production path below.
        let etag = etag_for(&bytes);
        if if_none_match_matches(&headers, &etag) {
            return not_modified(&etag);
        }
        return cached_response(content_type_for(&path), &etag, Body::from(bytes));
    }

    let Some(file) = STATIC_DIR.get_file(&path) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let etag = ETAGS.get(path.as_str()).expect(
        "ETAGS is built by all_files, the same recursive walk STATIC_DIR.get_file performs \
         via get_entry (FIV-005 round 3 §12) -- so anything get_file can resolve, ETAGS must \
         already have a tag for",
    );
    if if_none_match_matches(&headers, etag) {
        return not_modified(etag);
    }
    cached_response(content_type_for(&path), etag, Body::from(file.contents()))
}

/// `static/<path>` on disk, relative to this crate's manifest --
/// `static_js_scan.rs`'s own `CARGO_MANIFEST_DIR`-relative resolution,
/// not a new convention. `None` for anything outside `static/` itself
/// (`Path::new(path).components()` rejects `..` and absolute paths by
/// construction, since axum's own `{*path}` capture never includes a
/// leading `/`) or missing on disk -- either way, [`serve`] falls
/// through to the embedded copy.
#[cfg(debug_assertions)]
fn read_from_disk_in_debug(path: &str) -> Option<Vec<u8>> {
    use std::path::{Component, Path};
    let rel = Path::new(path);
    if rel.components().any(|c| !matches!(c, Component::Normal(_))) {
        return None;
    }
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let full = manifest_dir.join("../../static").join(rel);
    std::fs::read(full).ok()
}

/// Pulled out of [`serve`] so the match is testable without an async
/// runtime or a routed request.
fn content_type_for(path: &str) -> &'static str {
    match path.rsplit('.').next() {
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "text/javascript; charset=utf-8",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests;
