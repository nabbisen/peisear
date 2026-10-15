use super::*;

/// `static/`'s eight files, embedded at compile time -- mirrors
/// `static_js_scan`'s own filesystem enumeration, but against the
/// embedded set rather than the source tree, so a drift between "what
/// the binary carries" and "what `static/` holds on disk" shows up
/// here rather than only at the review's own manual walk.
const EXPECTED_FILES: &[&str] = &[
    "app.css",
    "board.js",
    "calendar.js",
    "daisyui.min.css",
    "dm.js",
    "plan.js",
    "search.js",
    "tailwind.css",
];

#[test]
fn the_embedded_set_holds_exactly_the_eight_files_static_js_scan_also_sees() {
    let mut embedded: Vec<&str> = STATIC_DIR
        .files()
        .map(|f| f.path().to_str().unwrap())
        .collect();
    embedded.sort();
    assert_eq!(
        embedded, EXPECTED_FILES,
        "the embedded static/ set no longer matches the eight files this project ships -- \
         a file was added to or removed from static/ without this constant moving with it"
    );
}

#[test]
fn content_type_is_exhaustive_for_the_two_extensions_static_actually_has() {
    assert_eq!(content_type_for("app.css"), "text/css; charset=utf-8");
    assert_eq!(
        content_type_for("daisyui.min.css"),
        "text/css; charset=utf-8"
    );
    assert_eq!(
        content_type_for("board.js"),
        "text/javascript; charset=utf-8"
    );
    assert_eq!(content_type_for("no-extension"), "application/octet-stream");
}

#[tokio::test]
async fn serve_returns_a_real_file_with_its_content_type() {
    let response = serve(Path("app.css".to_string()), HeaderMap::new()).await;
    assert_eq!(response.status(), axum::http::StatusCode::OK);
    assert_eq!(
        response.headers().get(header::CONTENT_TYPE).unwrap(),
        "text/css; charset=utf-8"
    );
}

#[tokio::test]
async fn serve_404s_for_a_path_not_in_the_embedded_set() {
    // `cargo test` runs in a debug profile, so this also exercises
    // the disk fallback's own miss case -- static/ on disk doesn't
    // have this file either, so both paths agree it is a 404.
    let response = serve(Path("does-not-exist.css".to_string()), HeaderMap::new()).await;
    assert_eq!(response.status(), axum::http::StatusCode::NOT_FOUND);
}

/// `FIV-005` round 2 `§10`: a test per header this round adds.
#[tokio::test]
async fn serve_carries_an_etag_and_a_no_cache_directive() {
    let response = serve(Path("app.css".to_string()), HeaderMap::new()).await;
    assert_eq!(response.status(), axum::http::StatusCode::OK);
    assert!(
        response.headers().get(header::ETAG).is_some(),
        "a 200 for a real asset must carry an ETag -- there is nothing for a client to \
         revalidate against otherwise"
    );
    assert_eq!(
        response.headers().get(header::CACHE_CONTROL).unwrap(),
        "no-cache",
        "store the body, but always revalidate before using it -- /static/app.css is the \
         same URL forever, so anything looser risks serving stale CSS against a newer page"
    );
}

/// `FIV-005` round 2 `§10`'s own words: *"that second one is the whole
/// round; the first without it is `§10.36` again."* A matching
/// `If-None-Match` must answer 304 with an empty body, not 200 with
/// the full content repeated.
#[tokio::test]
async fn a_matching_if_none_match_returns_304_with_no_body() {
    let first = serve(Path("app.css".to_string()), HeaderMap::new()).await;
    let etag = first
        .headers()
        .get(header::ETAG)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let mut conditional = HeaderMap::new();
    conditional.insert(header::IF_NONE_MATCH, etag.parse().unwrap());
    let second = serve(Path("app.css".to_string()), conditional).await;

    assert_eq!(second.status(), axum::http::StatusCode::NOT_MODIFIED);
    assert_eq!(
        second
            .headers()
            .get(header::ETAG)
            .unwrap()
            .to_str()
            .unwrap(),
        etag,
        "the client must be told which representation was revalidated"
    );
    let body = axum::body::to_bytes(second.into_body(), usize::MAX)
        .await
        .unwrap();
    assert!(
        body.is_empty(),
        "a body on a 304 is the defect this round exists to close, with extra steps"
    );
}

/// A stale or unrelated `If-None-Match` must not short-circuit a real
/// request -- the conditional-request handling is a real comparison,
/// not a rubber stamp.
#[tokio::test]
async fn a_non_matching_if_none_match_still_returns_200() {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::IF_NONE_MATCH,
        "\"not-the-real-tag\"".parse().unwrap(),
    );
    let response = serve(Path("app.css".to_string()), headers).await;
    assert_eq!(response.status(), axum::http::StatusCode::OK);
}

/// `If-None-Match: *` (RFC 7232 §3.2) matches any existing
/// representation -- by the time `serve` reaches this check, the file
/// is already confirmed to exist, so `*` always matches here.
#[tokio::test]
async fn a_wildcard_if_none_match_returns_304() {
    let mut headers = HeaderMap::new();
    headers.insert(header::IF_NONE_MATCH, "*".parse().unwrap());
    let response = serve(Path("app.css".to_string()), headers).await;
    assert_eq!(response.status(), axum::http::StatusCode::NOT_MODIFIED);
}

/// `FIV-005` round 2 `§10`: two different asset bodies must produce
/// two different tags, so the tag is over content and not a constant
/// every file happens to share.
#[tokio::test]
async fn different_assets_get_different_etags() {
    let css = serve(Path("app.css".to_string()), HeaderMap::new()).await;
    let js = serve(Path("search.js".to_string()), HeaderMap::new()).await;
    assert_ne!(
        css.headers().get(header::ETAG),
        js.headers().get(header::ETAG),
        "two different files must not share a tag"
    );
}

#[test]
fn etag_for_changes_with_the_bytes_it_covers() {
    let a = etag_for(b"one");
    let b = etag_for(b"two");
    assert_ne!(a, b);
    assert_eq!(
        a,
        etag_for(b"one"),
        "the same bytes must hash to the same tag every time"
    );
}

#[test]
fn if_none_match_matches_handles_lists_and_the_wildcard() {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::IF_NONE_MATCH,
        "\"other\", \"mine\"".parse().unwrap(),
    );
    assert!(if_none_match_matches(&headers, "\"mine\""));
    assert!(!if_none_match_matches(&headers, "\"not-present\""));

    let mut wildcard = HeaderMap::new();
    wildcard.insert(header::IF_NONE_MATCH, "*".parse().unwrap());
    assert!(if_none_match_matches(&wildcard, "\"anything\""));

    assert!(!if_none_match_matches(&HeaderMap::new(), "\"mine\""));
}

#[cfg(debug_assertions)]
#[test]
fn disk_fallback_reads_a_real_file_from_static() {
    let bytes = read_from_disk_in_debug("app.css").expect("static/app.css exists on disk");
    let embedded = STATIC_DIR.get_file("app.css").unwrap().contents();
    assert_eq!(
        bytes, embedded,
        "the disk copy and the embedded copy have diverged -- static/ changed after the \
         binary was built, which is exactly the case this fallback exists to show live"
    );
}

#[cfg(debug_assertions)]
#[test]
fn disk_fallback_refuses_a_path_that_climbs_out_of_static() {
    assert_eq!(read_from_disk_in_debug("../Cargo.toml"), None);
    assert_eq!(read_from_disk_in_debug("../../Cargo.toml"), None);
}

#[cfg(debug_assertions)]
#[test]
fn disk_fallback_returns_none_for_a_missing_file() {
    assert_eq!(read_from_disk_in_debug("does-not-exist.css"), None);
}
