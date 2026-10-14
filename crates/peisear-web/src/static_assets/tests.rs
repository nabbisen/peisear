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
    let response = serve(Path("app.css".to_string())).await;
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
    let response = serve(Path("does-not-exist.css".to_string())).await;
    assert_eq!(response.status(), axum::http::StatusCode::NOT_FOUND);
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
