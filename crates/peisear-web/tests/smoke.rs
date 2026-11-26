//! Smoke tests — verify the app boots and the most common
//! happy paths produce the expected status codes / redirects.
//!
//! These intentionally don't assert HTML content. Phase A's
//! information architecture changes (e.g. `/me` → `/today`)
//! will exercise these via redirect status. UI rendering is
//! validated by Phase B via separate component-focused tests.

mod common;

use axum::http::StatusCode;
use common::auth::{TestUser, register_and_login};
use common::server::TestApp;

#[tokio::test]
async fn health_endpoint_returns_200() {
    let app = TestApp::spawn().await;
    let resp = app.server.get("/health").await;
    resp.assert_status_ok();
}

#[tokio::test]
async fn login_page_renders() {
    let app = TestApp::spawn().await;
    let resp = app.server.get("/login").await;
    resp.assert_status_ok();
}

#[tokio::test]
async fn register_page_renders() {
    let app = TestApp::spawn().await;
    let resp = app.server.get("/register").await;
    resp.assert_status_ok();
}

#[tokio::test]
async fn register_then_redirected_to_projects() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("alice");
    let _user_id = register_and_login(&app, &user).await;
    // After register, the production handler 303s to /projects.
    // Following that, /projects should now render successfully
    // (we have a session cookie in the saved jar).
    let resp = app.server.get("/projects").await;
    resp.assert_status_ok();
}

#[tokio::test]
async fn unauthenticated_projects_redirects_to_login() {
    let app = TestApp::spawn().await;
    let resp = app.server.get("/projects").await;
    // Per existing behaviour: protected routes redirect
    // unauthenticated users to /login.
    assert_eq!(resp.status_code(), StatusCode::SEE_OTHER);
    let location = resp
        .headers()
        .get(axum::http::header::LOCATION)
        .expect("redirect Location header")
        .to_str()
        .expect("ASCII Location header");
    assert!(
        location.starts_with("/login"),
        "expected redirect to /login, got: {location}"
    );
}

#[tokio::test]
async fn today_dashboard_loads_for_authenticated_user() {
    // /today is the canonical path as of v0.17.0 (renamed from /me).
    let app = TestApp::spawn().await;
    let user = TestUser::new("bob");
    register_and_login(&app, &user).await;

    let resp = app.server.get("/today").await;
    resp.assert_status_ok();
}

#[tokio::test]
async fn me_redirects_to_today_with_308() {
    // Legacy /me must 308-redirect to /today, preserving the
    // request method. The redirect itself does not require auth
    // — auth is enforced at /today after the redirect.
    let app = TestApp::spawn().await;
    let resp = app.server.get("/me").await;

    assert_eq!(
        resp.status_code(),
        StatusCode::PERMANENT_REDIRECT,
        "expected 308 from /me, got {}",
        resp.status_code()
    );
    let location = resp
        .headers()
        .get(axum::http::header::LOCATION)
        .expect("Location header present")
        .to_str()
        .expect("Location is ASCII");
    assert_eq!(location, "/today");
}

#[tokio::test]
async fn notifications_redirects_to_inbox_with_308() {
    let app = TestApp::spawn().await;
    let resp = app.server.get("/notifications").await;

    assert_eq!(resp.status_code(), StatusCode::PERMANENT_REDIRECT);
    let location = resp
        .headers()
        .get(axum::http::header::LOCATION)
        .expect("Location header present")
        .to_str()
        .expect("Location is ASCII");
    assert_eq!(location, "/inbox");
}

#[tokio::test]
async fn inbox_loads_for_authenticated_user() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("frank");
    register_and_login(&app, &user).await;

    let resp = app.server.get("/inbox").await;
    resp.assert_status_ok();
}

#[tokio::test]
async fn legacy_notifications_post_endpoints_redirect_with_308() {
    // The 308 redirect must preserve POST method on its way to
    // the new /inbox/* endpoints. The 308 status code is what
    // makes the difference vs. 301: a 301 would (at least
    // historically) let the client downgrade to GET.
    let app = TestApp::spawn().await;

    // mark-all-read
    let resp = app.server.post("/notifications/mark-all-read").await;
    assert_eq!(resp.status_code(), StatusCode::PERMANENT_REDIRECT);
    let location = resp
        .headers()
        .get(axum::http::header::LOCATION)
        .expect("Location present")
        .to_str()
        .unwrap();
    assert_eq!(location, "/inbox/mark-all-read");

    // mark one as read — preserves the {id} path parameter
    let resp = app.server.post("/notifications/some-notif-id/read").await;
    assert_eq!(resp.status_code(), StatusCode::PERMANENT_REDIRECT);
    let location = resp
        .headers()
        .get(axum::http::header::LOCATION)
        .expect("Location present")
        .to_str()
        .unwrap();
    assert_eq!(location, "/inbox/some-notif-id/read");
}

/// `SES-001` (`RFC 024`, `FR-AUTH-003`). Replaces
/// `logout_clears_session_and_redirects`, which drove a cookie-jar
/// client that logged out, then asked for `/projects` **with no
/// cookie**, and asserted a redirect — it verified that the client
/// forgot the credential, never that the credential itself stopped
/// working (`§10.36` instance 10). This test captures the token
/// *before* logging out and replays that exact value afterwards, on
/// both an HTML route (303 to `/login`) and an `/api/*` route (401
/// JSON) — the two rejection shapes `verified_requester_id` produces,
/// both exercised here rather than assumed symmetric.
///
/// **Plant**: run against the pre-`SES-001` tree (no `session_generation`
/// check in `verified_requester_id`), this test fails — the replayed
/// cookie still reaches `/projects` with a `200` and `/api/search`
/// with a `200`, not the `303`/`401` asserted below. Confirmed red
/// before this round's own fix landed, not assumed.
#[tokio::test]
async fn a_credential_captured_before_logout_is_refused_after_it_html_and_api() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("carol");

    // Capture the live session cookie as `register` issues it — a
    // subsequent request's own response carries no `Set-Cookie` (the
    // server only sets one at register/login time), so it has to be
    // read from here, not re-derived from a later page load.
    let register_resp = app
        .server
        .post("/register")
        .form(&[
            ("email", user.email.as_str()),
            ("password", user.password.as_str()),
            ("display_name", user.display_name.as_str()),
        ])
        .await;
    register_resp.assert_status(StatusCode::SEE_OTHER);
    let captured = register_resp
        .cookies()
        .iter()
        .next()
        .expect("a session cookie")
        .clone()
        .into_owned();

    // Verify authenticated first (the server's own jar now carries
    // the same cookie, set by the `register` call above).
    let resp = app.server.get("/projects").await;
    resp.assert_status_ok();

    // Log out (through the server's own jar, which is a separate
    // copy from `captured` above).
    let resp = app.server.post("/logout").await;
    resp.assert_status(StatusCode::SEE_OTHER);

    // The captured credential, replayed on an HTML route: refused,
    // not merely "the jar is empty" — `clear_cookies()` first proves
    // this request carries *only* the stale token, nothing else.
    let resp = app
        .server
        .get("/projects")
        .clear_cookies()
        .add_cookie(captured.clone())
        .await;
    assert_eq!(resp.status_code(), StatusCode::SEE_OTHER);

    // Same stale credential, an `/api/*` route: 401 JSON, the other
    // rejection shape `ApiRequester`/`ApiAuthUser` produce.
    let resp = app
        .server
        .get("/api/search?q=foo")
        .clear_cookies()
        .add_cookie(captured)
        .await;
    assert_eq!(resp.status_code(), StatusCode::UNAUTHORIZED);
}

/// `RFC 024` §4.1: the case a timestamp mechanism fails and a counter
/// does not. Immediately after logging out, in the same test, with no
/// sleep: logging back in must produce a session that works. A clock-
/// based revocation compared at whole-second resolution would admit a
/// same-second replay (wrong direction); compared finer, it would
/// refuse this exact re-login for the rest of that second (the
/// opposite failure, and the one a sleep in this test would have
/// papered over rather than proven absent).
#[tokio::test]
async fn logging_back_in_immediately_after_logout_establishes_a_working_session() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("dana");
    register_and_login(&app, &user).await;

    let resp = app.server.post("/logout").await;
    resp.assert_status(StatusCode::SEE_OTHER);

    // No sleep between logout and login — the whole point.
    let resp = app
        .server
        .post("/login")
        .form(&[
            ("email", user.email.as_str()),
            ("password", user.password.as_str()),
        ])
        .await;
    resp.assert_status(StatusCode::SEE_OTHER);

    let resp = app.server.get("/projects").await;
    resp.assert_status_ok();
}

/// `RFC 024` §4.3: a token minted before `session_generation` existed
/// (any `0.70.0` token) must keep working after the upgrade — the
/// `#[serde(default)]` property on `Claims.ses`. Minted by hand here,
/// since `jwt::issue` always stamps a generation now and `Claims`
/// stays private to `peisear_auth::jwt` on purpose; this is the exact
/// historical shape a real `0.70.0` cookie has, re-signed with the
/// same test secret the app under test verifies against.
#[tokio::test]
async fn a_pre_upgrade_token_with_no_generation_claim_still_verifies() {
    #[derive(serde::Serialize)]
    struct PreUpgradeClaims {
        sub: String,
        email: String,
        exp: i64,
        iat: i64,
        // Deliberately no `ses` field — this is the shape every
        // token minted before SES-001 actually has.
    }

    let app = TestApp::spawn().await;
    let user = TestUser::new("erin");
    let user_id = register_and_login(&app, &user).await;

    let now = chrono::Utc::now().timestamp();
    let claims = PreUpgradeClaims {
        sub: user_id,
        email: user.email.clone(),
        iat: now,
        exp: now + 60 * 60 * 24 * 7,
    };
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(common::server::JWT_SECRET.as_bytes()),
    )
    .expect("encode a pre-upgrade-shaped token");

    // A raw `Cookie:` header rather than `.add_cookie()` -- the typed
    // builder needs the `cookie` crate named directly, which this
    // integration-test binary has no reason to depend on for one
    // header value.
    let resp = app
        .server
        .get("/projects")
        .clear_cookies()
        .add_header(axum::http::header::COOKIE, format!("it_session={token}"))
        .await;
    assert_eq!(
        resp.status_code(),
        StatusCode::OK,
        "a pre-SES-001 token (no `ses` claim) must still verify -- the row's own \
         session_generation default (0) must match the claim's default (also 0)"
    );
}

/// `RFC 024` §3/§5, the one-sentence statement of the whole round:
/// ending a session ends it *everywhere*, not just on the device that
/// pressed Log out. Two independent sessions for the same account
/// (simulating two devices), captured separately; logging out through
/// one refuses the other's own, never-replayed-until-now credential.
#[tokio::test]
async fn logging_out_on_one_device_refuses_the_other_devices_session() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("frank");
    register_and_login(&app, &user).await;

    let cookie_of = |resp: axum_test::TestResponse| {
        resp.cookies()
            .iter()
            .next()
            .expect("a session cookie")
            .clone()
            .into_owned()
    };

    // "Device B": a second, independent login for the same account.
    let device_b_login = app
        .server
        .post("/login")
        .form(&[
            ("email", user.email.as_str()),
            ("password", user.password.as_str()),
        ])
        .await;
    device_b_login.assert_status(StatusCode::SEE_OTHER);
    let device_b_cookie = cookie_of(device_b_login);

    // Confirm device B's own session works before any logout.
    let resp = app
        .server
        .get("/projects")
        .clear_cookies()
        .add_cookie(device_b_cookie.clone())
        .await;
    resp.assert_status_ok();

    // Log out through the server's own jar ("device A").
    let resp = app.server.post("/logout").await;
    resp.assert_status(StatusCode::SEE_OTHER);

    // Device B's own credential, captured before any logout and never
    // itself used to log out, is refused too.
    let resp = app
        .server
        .get("/projects")
        .clear_cookies()
        .add_cookie(device_b_cookie)
        .await;
    assert_eq!(resp.status_code(), StatusCode::SEE_OTHER);
}

/// `QA-003`: `search.js`'s tag (`components/layout.rs:72`, inside
/// `AppShell`) had no test at all -- unlike `dm.js`/`board.js`, its
/// blast radius is every authenticated page, not one surface, since
/// every such page renders through `AppShell`. `smoke` is the right
/// home for this claim: it's about the shell, not about search.
///
/// `GET /static/search.js` itself is `every_static_asset_is_actually_
/// servable`, below.
#[tokio::test]
async fn search_js_is_referenced_in_the_app_shell() {
    let app = TestApp::spawn().await;
    let user = TestUser::new("dana");
    register_and_login(&app, &user).await;

    let resp = app.server.get("/today").await;
    let body = resp.text();
    assert!(
        body.contains(r#"<script src="/static/search.js" defer"#),
        "an authenticated page must reference search.js with defer: {body}"
    );
}

/// `FIV-005`: `static/` is embedded into the binary at compile time
/// (`static_assets.rs`) rather than resolved against the process's
/// working directory at request time, which closes a gap the three
/// reference tests above and in `status_control.rs` used to name in
/// their own doc comments -- `ServeDir::new("static")` resolved
/// against `cargo test`'s crate-root working directory, not the
/// workspace root `static/` lives under, so an HTTP-level "is it
/// actually served" assertion would have 404'd regardless of whether
/// the file was really reachable from a real deployment. The embedded
/// set has no working-directory dependency, so this now tests the
/// real thing.
#[tokio::test]
async fn every_static_asset_is_actually_servable() {
    let app = TestApp::spawn().await;
    for name in [
        "app.css",
        "tailwind.css",
        "daisyui.min.css",
        "search.js",
        "board.js",
        "calendar.js",
        "plan.js",
        "dm.js",
    ] {
        let resp = app.server.get(&format!("/static/{name}")).await;
        assert_eq!(
            resp.status_code(),
            StatusCode::OK,
            "GET /static/{name} must be 200, found {}",
            resp.status_code()
        );
    }

    let resp = app.server.get("/static/does-not-exist.js").await;
    assert_eq!(resp.status_code(), StatusCode::NOT_FOUND);
}

/// `FIV-005` round 2 `§10`: the installed-binary walk re-run, at the
/// router level this time -- round 1 proved the page renders once;
/// this asserts a second navigation does not pay for the body again.
/// A real `GET` returns an `ETag`; echoing it back as `If-None-Match`
/// must answer `304` with no body, the way a browser's own second
/// navigation would.
#[tokio::test]
async fn a_second_navigation_does_not_refetch_the_asset_body() {
    let app = TestApp::spawn().await;

    let first = app.server.get("/static/daisyui.min.css").await;
    assert_eq!(first.status_code(), StatusCode::OK);
    let etag = first
        .header(axum::http::header::ETAG)
        .to_str()
        .unwrap()
        .to_string();
    assert!(
        !first.as_bytes().is_empty(),
        "the first navigation must still carry the real body"
    );

    let second = app
        .server
        .get("/static/daisyui.min.css")
        .add_header(
            axum::http::header::IF_NONE_MATCH,
            etag.parse::<axum::http::HeaderValue>().unwrap(),
        )
        .await;
    assert_eq!(
        second.status_code(),
        StatusCode::NOT_MODIFIED,
        "a second navigation with the first response's own ETag must be 304, not a second \
         2.9 MB body"
    );
    assert!(
        second.as_bytes().is_empty(),
        "a 304 must carry no body -- wire bytes on the second navigation: {}",
        second.as_bytes().len()
    );
}

/// `FR-AUTH-001`'s second clause -- *"no plaintext password is
/// persisted or logged"* -- had zero matches for `plaintext` anywhere
/// in `tests/` or `peisear-auth` before `SES-001`/`RFC 024` §2: true,
/// and evidenced by nobody. This is the real test. A distinctive
/// marker password that cannot appear by coincidence, checked absent
/// from the stored row **and** from tracing output captured at
/// `debug` for the one request that carries it — the level the
/// architect's own manual walk used (`RFC 024` §2), not the default a
/// production deployment runs at.
///
/// **Plant**: in `register_submit`, replace `password::hash(&form.password)?`
/// with `form.password.clone()` in a scratch worktree and this test's
/// own storage assertion fails, naming the marker in `password_hash`
/// directly -- confirmed, not assumed (see the round's own report).
/// No equivalent plant for the log half: nothing in this crate's
/// request path ever formats the word `password`, and
/// `TraceLayer::new_for_http()` is unconfigured (its defaults log
/// method/path/status/latency, never a body), so there is no call
/// site to break in a way this test would need to catch — the
/// structural absence *is* the property, not a gap this plant leaves
/// unchecked.
#[tokio::test]
async fn a_distinctive_password_never_reaches_storage_or_logs() {
    #[derive(Clone, Default)]
    struct SharedBuf(std::sync::Arc<std::sync::Mutex<Vec<u8>>>);

    impl std::io::Write for SharedBuf {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0
                .lock()
                .expect("log buffer lock")
                .extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for SharedBuf {
        type Writer = SharedBuf;
        fn make_writer(&'a self) -> Self::Writer {
            self.clone()
        }
    }

    let app = TestApp::spawn().await;
    let marker_password = "Mrk3r-9f3a7c21-PlaintextCanary!";
    let email = "plaintext-marker@example.org";

    let buf = SharedBuf::default();
    let subscriber = tracing_subscriber::fmt()
        .with_writer(buf.clone())
        .with_env_filter("debug")
        .finish();

    // Thread-local, not process-global (`tracing::subscriber::set_default`,
    // not `set_global_default`) -- `TestApp::spawn`'s own postgres path
    // already installs a global one, and a second `try_init` there is a
    // no-op; this reaches the request this test cares about either way,
    // since `#[tokio::test]` defaults to a current-thread runtime and the
    // guard stays live across the `.await` below without crossing threads.
    let _log_guard = tracing::subscriber::set_default(subscriber);
    let resp = app
        .server
        .post("/register")
        .form(&[
            ("email", email),
            ("password", marker_password),
            ("display_name", "Plaintext Marker Test"),
        ])
        .await;
    resp.assert_status(StatusCode::SEE_OTHER);
    drop(_log_guard);

    let logged = String::from_utf8_lossy(&buf.0.lock().expect("log buffer lock")).into_owned();
    assert!(
        !logged.contains(marker_password),
        "the marker password leaked into tracing output captured at debug:\n{logged}"
    );

    let stored = peisear_storage::users::find_by_email(&app.db, email)
        .await
        .expect("query the just-registered row")
        .expect("the row exists");
    assert!(
        !stored.password_hash.contains(marker_password),
        "the marker password leaked into password_hash: {:?}",
        stored.password_hash
    );
    assert!(
        stored.password_hash.starts_with("$argon2"),
        "password_hash does not look like an Argon2 hash: {:?}",
        stored.password_hash
    );
}
