//! Serves `static/`'s eight files from memory, embedded into the
//! binary at compile time (`FIV-005`, `§10.41`) rather than resolved
//! against the process's working directory at request time.
//!
//! `cargo install peisear` publishes no `static/` directory alongside
//! the installed binary (`static_js_scan.rs`'s own measurement,
//! `cargo package -p peisear --list`: seven files, none of them
//! under `static/`) -- so a filesystem [`tower_http::services::ServeDir`]
//! 404s on every path, on every installed copy. The content has to
//! already be inside the binary by the time a request for it arrives.

use axum::{
    body::Body,
    extract::Path,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use include_dir::{Dir, include_dir};

/// `static/`'s contents at compile time, rooted the same two
/// directories above this crate's manifest that
/// [`crate::static_js_scan`] already resolves at runtime for its own,
/// separate reasons -- one real path, read two different ways.
static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../static");

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
pub async fn serve(Path(path): Path<String>) -> Response {
    #[cfg(debug_assertions)]
    if let Some(bytes) = read_from_disk_in_debug(&path) {
        return (
            [(header::CONTENT_TYPE, content_type_for(&path))],
            Body::from(bytes),
        )
            .into_response();
    }

    let Some(file) = STATIC_DIR.get_file(&path) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    (
        [(header::CONTENT_TYPE, content_type_for(&path))],
        Body::from(file.contents()),
    )
        .into_response()
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
