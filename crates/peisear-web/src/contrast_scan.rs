//! `QA-013` §4 (RFC 005 §4) — with `/70` established as the muted-tier
//! floor (`QA-012`'s measurement, `QA-013`'s call-site sweep), a text
//! scan over `text-base-content/N` for `N < 70` becomes an honest
//! assertion rather than a guess: no legitimate use exists below `/70`
//! against any background this theme defines, so there is no passing
//! case for a scan to mis-flag.
//!
//! **Pinned measurement.** `daisyui@4.12.14`'s `corporate` theme,
//! resolved from its actual shipped CSS (`static/daisyui.min.css`,
//! vendored by `ASSET-001`/`DEC-051` — pinned by version and SHA-256,
//! see `style/tailwindcss/README.md`; the exact version
//! `components/layout.rs` loads, and now the file this crate ships
//! rather than one fetched from `cdn.jsdelivr.net` at runtime) —
//! `QA-012`'s own source, not assumed: `base-content` `#181A2A`, `base-100`
//! `#FFFFFF`, `base-200` `#E8E8E8`, `base-300` `#D1D1D1`. `/70` clears
//! 4.5:1 against all three (6.36:1 / 5.76:1 / 5.15:1); everything below
//! it either fails outright or passes by a margin small enough that
//! the next theme tweak erases it (`/60` on `base-100`: 4.54:1,
//! a 0.04 margin). A future DaisyUI upgrade or theme change
//! invalidates this measurement, and this guard's premise with it —
//! re-verify, don't just re-run, if either changes.
//!
//! **Scope: `text-base-content/N` classes only, not bare `opacity-*`.**
//! `opacity` is an element-level CSS property, not a text colour — it
//! dims whatever the element contains, which is sometimes text
//! (composites identically to a `text-base-content/N` at the same
//! level) and sometimes not (`calendar.rs`'s two empty `<td>` cells,
//! dimming a border, not text — `QA-013` §2 excluded them explicitly).
//! Telling the two apart needs rendering, which this project does not
//! execute (`§10.15`'s standing limit) — a blanket ban on
//! `opacity-10`..`opacity-60` would be wrong about those two cells,
//! and a scan trying to read "is this element's content text" from
//! `.rs` source would be guessing at exactly what `QA-013` §2 had to
//! check by hand, three times, before trusting a bare `opacity-*`
//! site's arithmetic (two sites turned out to compound against an
//! already-alpha'd ancestor; one turned out to sit on a
//! `bg-primary`-tinted background instead of a theme base colour —
//! see `QA-013`'s review request). A guard that covered
//! `text-base-content/N` and silently pretended to cover `opacity-*`
//! too would be this series' fifth instance of the shape it has spent
//! `QA-009` and `QA-010` closing: a guard whose reach is narrower than
//! its name suggests.
//!
//! ## `CLR-003`, `§10.61` — a second ban, the one token family short
//!
//! `NFR-A11Y-005`'s own acceptance read as "the theme's own tokens
//! were never the problem," generalised from `QA-012`/`QA-013`'s own
//! honest measurement of exactly one token family
//! (`text-base-content/N`). Series `064` measured the other four a
//! theme actually ships and found fifteen shipped sites below 4.5:1 —
//! every destructive action in the product among them (`CLR-001`,
//! `CLR-002`). This ban closes that gap at the only point this
//! project checks anything without rendering: a class token, before
//! it ships.
//!
//! **The measured ratios**, resolved the same way as above —
//! `base-100`/`base-200`/`base-300` columns, re-verify rather than
//! re-run if the theme or DaisyUI version changes:
//!
//! | foreground | hex | `base-100` | `base-200` | `base-300` |
//! |---|---|---|---|---|
//! | `success` | `#00a96e` | 3.03:1 | 2.47:1 | 1.98:1 |
//! | `error` | `#ff5861` | 3.08:1 | 2.50:1 | 2.01:1 |
//! | `primary` | `#4d6eff` | 4.21:1 | 3.42:1 | 2.75:1 |
//! | `base-content` | `#181a2a` | 17.21:1 | 14.01:1 | 11.25:1 |
//!
//! `warning`, `info`, `secondary` and `accent` render at **zero** text
//! or link sites in `components/` today (confirmed by grep, not
//! assumed from their absence being unremarkable) — banned
//! prophylactically, the same four families rather than three plus a
//! promise, and measured if a use is ever proposed rather than
//! guessed now.
//!
//! Two solid pairs already pass, which is why the repairs in
//! `CLR-001`/`CLR-002` reuse patterns the product already has rather
//! than inventing a colour: `error-content` on `error` **6.83:1**,
//! `success-content` on `success` **6.93:1**. `btn-primary`, the
//! product's main action at 37 sites, measures **4.83:1** — a pass by
//! 0.33, the figure most worth re-checking after any theme change.
//!
//! **The rule**: refuse, in `components/**`, any of
//! [`SEMANTIC_FAMILIES`] spelled `text-{family}` or `link-{family}`
//! (bare or with any `/N` opacity suffix — a prefix match catches
//! both, the same shape [`BANNED_MODIFIERS`] already uses), and the
//! pair `btn-outline` with `btn-{family}` **on one element** — the
//! spelling `CLR-001` found that a `text-*`/`link-*` grep alone
//! misses (`projects.rs`'s own delete link read its colour through
//! `btn-outline btn-error` instead). Checking "on one element" rather
//! than "anywhere in the file" needs the same per-element token list
//! `css_scan` already extracts correctly (continuations joined,
//! interpolation skipped), so this scan reuses
//! [`crate::css_scan::quoted_string_spans`],
//! [`crate::css_scan::is_token_source`] and
//! [`crate::css_scan::tokens_from_content`] rather than a second,
//! looser tokenizer — `GRD-001`'s own reasoning for sharing
//! `strip_line_comments` applies here too: these three carry state
//! and edge cases, and a second copy is a second chance to be wrong
//! differently.
//!
//! **Three sites measured as correctly using a semantic colour on
//! text, excluded by their own exact literal** — the
//! `alignment_scan::NAMED_ESCALATION_EXCLUSION_TAG` precedent: a
//! second, *different* element carrying the same class is still
//! caught, because the match is against the literal class value, not
//! a file or line reference. See [`NAMED_EXCLUSION_CLASS_VALUES`] for
//! the two literals (one covers two identical sites) and why each
//! passes.
//!
//! **Does not compute a per-site ratio.** Which background an element
//! actually renders on is this scan's one hard question, and
//! `CLR-001` demonstrated — twice, on sites this handoff's own
//! figures got wrong — that only the rendered tree answers it. This
//! stays a ban on tokens, the shape `BANNED_MODIFIERS` already uses,
//! not a computation this scan cannot do honestly.
//!
//! ## `PLC-001`–`PLC-003`, `§10.62` — a third mechanism: colours no
//! `components/**` class token ever carries
//!
//! The two mechanisms above both read a class token a component
//! writes. Series `065` found a third shape neither can see: a text
//! colour the *stylesheet itself* applies, by a bare element or
//! pseudo-element selector (`input::placeholder`), or baked into a
//! class a component writes for an unrelated reason (`.menu-title`,
//! `.table`'s own `thead`/`tfoot` descendant rule) — eighteen sites
//! read below 4.5:1 this way (`PLC-001`'s seventeen placeholders at
//! 2.54:1, `PLC-002`'s one account-menu line at 1.85:1, the fade
//! compounding with the span's own `opacity-70`) and eight more sat on
//! the exact `/60` tier this requirement already rejected as a floor
//! (4.53:1 — a pass by 0.03, failing the moment a table header ever
//! renders on `base-200`).
//!
//! **A scan that flags every below-floor rule in the served sheets
//! does not work, because this series' own repairs are rules too.**
//! `tailwind.css`'s `input::placeholder{color:#9ca3af}` still exists —
//! `app.css`'s own override beats it in the browser, which no class
//! token says. The guard has to tell *a rule that reaches a reader*
//! from *a rule something else beats*, which [`ACKNOWLEDGED_SHEET_RULES`]
//! does by naming, for each reachable below-floor rule, the exact
//! literal that proves what beats it — an `app.css` override
//! ([`AcknowledgedSheetRule::beaten_by_needle`] found in
//! `beaten_by_file`), or, for `.menu-title`'s one site, the child
//! `<span>`'s own `text-base-content/70` winning by ordinary
//! inheritance rather than a second stylesheet rule.
//!
//! **Reachable is load-bearing, not decorative.** `.stat-title`/
//! `.stat-desc` resolve to `/60` in the pinned sheet unconditionally,
//! but zero `components/**` sites write either class today — so they
//! need no acknowledgement, and [`selector_is_reachable`] (reusing the
//! same written-token population `css_scan` computes) is what lets
//! this scan tell that apart from a rule that *is* reached: a bare
//! element/pseudo-element selector is always reachable (no component
//! can avoid emitting an `<input>` or a `<thead>`); a class-based one
//! is reachable only if some component writes that literal class.
//! [`EXEMPT_SHEET_RULES`] (the disabled tiers, WCAG 1.4.3 — text that
//! is part of an inactive UI component) skip this gate entirely and
//! are acknowledged regardless, because their own first class token
//! (`.input`, `.menu`) is written everywhere independent of whether
//! the `:disabled`/`.disabled` half is ever triggered — a reachability
//! check would read these as live when they are not.
//! [`NON_TEXT_SHEET_SELECTORS`] (`.toggle`) are excluded for a
//! different reason: the `color` property there feeds `currentColor`
//! into a border and background, painting no text at all — WCAG
//! 1.4.11, out of this scan's own scope.
//!
//! **Two failures, planted separately** (`PLC-003`'s own review
//! request has both, quoted): a `components/**` site made to write
//! `stat-title` (a below-floor rule becoming newly reachable, caught
//! as unacknowledged); `app.css`'s own placeholder override removed in
//! a scratch worktree (an acknowledged rule's `beaten_by_needle` going
//! missing, caught by [`fn@sheet_leak_scan`]'s own liveness half,
//! separately from whether the underlying sheet rule itself is still
//! present).
//!
//! **Not built: a browser-rendered sweep.** The only check that
//! resolves a cascade *correctly* in every case renders the page and
//! samples a computed colour, the same method this series used by
//! hand throughout (`§10.52`). This guard is a static acknowledgement
//! list instead — this project's own established shape for exactly
//! this situation (`css_scan`'s allowlist, `contrast_scan`'s own named
//! exclusions above) — named as a `ROADMAP.md` candidate with its own
//! cost rather than built here.
//!
//! ## `EDG-001`–`EDG-003`, `§10.63` — a fourth mechanism: the two
//! colours above are not the only ones a sheet applies
//!
//! Every mechanism above reads `color:` — text a reader reads. Series
//! `066` measured a **non-text** channel this scan had explicitly
//! named out of its own scope ([`NON_TEXT_SHEET_SELECTORS`]'s own
//! doc comment: "WCAG 1.4.11 ... out of this scan's own scope"):
//! `border-color` and `outline-color`, the colours that say a control
//! exists and where the keyboard is. `daisyui.min.css` draws every
//! `.input`/`.select`/`.textarea`/`.checkbox` boundary and every one
//! of their own focus outlines at `base-content/20` — **1.52:1** on a
//! card, **1.50:1** on the page body, against WCAG 1.4.11's 3:1 — the
//! exact shape `§10.62` found for text, one property over: a colour
//! the *stylesheet* applies, reachable by a component's own written
//! class, invisible to a scan that reads only `color:`.
//!
//! **`EDG-001`/`EDG-002` repaired both**, on the product's own existing
//! tiers, no new colour: every focus ring to full `base-content`
//! (**17.21:1**, the tier `.checkbox:focus-visible` already used),
//! every resting boundary to `/70` (**6.42:1** on a card, **5.24:1** on
//! the page body) — **not** the `/50` this series' own handoff
//! recommended, which measured at **2.73:1** against the page body
//! (a semi-transparent border composites over the *control's own
//! white fill*, not the page behind it — `EDG-002`'s own correction,
//! reproduced independently by the architect's review). The focused
//! border is left at `/20` on purpose: `.input:focus` outranks the
//! bare class on specificity, so the ring alone carries the focus
//! state, checked by screenshot rather than assumed (`EDG-002 §4`).
//!
//! **The guard, [`boundary_leak_scan`], is `§10.62`'s own shape moved
//! one property over, not reinvented.** [`below_floor_boundary_declarations`]
//! enumerates every `border-color:`/`outline-color:` declaration in
//! the three served sheets resolving (by the same `/70` floor
//! already established, never a computed ratio — see below) below it,
//! restricted to a selector [`selector_is_reachable`] admits. Each
//! must be named in [`ACKNOWLEDGED_BOUNDARY_RULES`] (the exact literal
//! proving what beats it) or [`EXEMPT_BOUNDARY_RULES`] (a deliberate,
//! visually-checked choice to leave it, `EDG-002 §4`'s own three
//! sites) — unnamed and reachable fails here, same as `§10.62`.
//!
//! **Resolving the fraction is not computing the ratio.** A
//! `border-color`/`outline-color` declaration in the pinned sheet
//! names its alpha one of two ways: inline (`oklch(var(--bc)/.2)`,
//! `app.css`'s own repairs), or through a sibling custom property
//! re-declared by nearly every rule with a different value
//! (`.checkbox{border-color:…oklch(var(--bc)/var(--tw-border-opacity))…;
//! --tw-border-opacity:0.2}`) — [`resolve_boundary_fraction`] reads
//! either, within the one rule block the reference came from
//! ([`enclosing_block_span`]), and compares the result to this
//! project's own already-measured `/70` floor. **It never asks what
//! that fraction composites against, which side of the border the
//! adjacent colour sits on, or what the control's own fill paints** —
//! the three facts `EDG-002` needed and the architect's own `/50`
//! recommendation got wrong holding all three in his head. The figure
//! beside each selector below is a human's, measured by screenshot,
//! not this scan's: the test asserts the acknowledgement names a real
//! rule and still-present proof, never that a ratio is right.
//!
//! **Planted twice** (`EDG-003`'s own review request has both):
//! `.toggle:focus-visible` is `/.2` in the pinned sheet and zero
//! `components/**` sites write `class="toggle"` today — made reachable
//! in a scratch worktree, caught unacknowledged; one of `app.css`'s
//! own `EDG-001`/`EDG-002` overrides removed in another, caught as an
//! acknowledgement whose proof went missing.
//!
//! **Found by scoping a release, like `§10.60`–`§10.62`** — the fourth
//! `010`'s own first sentence had always named ("the boundaries of
//! interactive controls") and the fourth time this project's own
//! `/50` figure needed a measurement to be trusted.

use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::css_scan::{is_token_source, quoted_string_spans, tokens_from_content};
use crate::scan_support::strip_line_comments;

/// Every `text-base-content/N` modifier this project has measured as
/// having no passing case against any of `base-100`/`base-200`/
/// `base-300` — `/70` and above are not banned; they are the floor.
const BANNED_MODIFIERS: [&str; 6] = [
    "text-base-content/10",
    "text-base-content/20",
    "text-base-content/30",
    "text-base-content/40",
    "text-base-content/50",
    "text-base-content/60",
];

/// Every semantic DaisyUI colour family `§10.61` bans from a text or
/// link position. `success`/`error`/`primary` are measured (this
/// module's own doc comment); `warning`/`info`/`secondary`/`accent`
/// render at zero sites today and are banned on the same reasoning
/// rather than left to be found the hard way later.
pub(crate) const SEMANTIC_FAMILIES: [&str; 7] = [
    "success",
    "error",
    "warning",
    "info",
    "primary",
    "secondary",
    "accent",
];

/// Two sites' exact class-attribute literal, excluded from
/// `§10.61`'s ban because each was measured and found correct — a
/// third site carrying a *different* class value, even one that
/// still names the same family, is not exempted by this list.
///
/// `error_page.rs:18`: a 48px status number. Large text (≥24px, WCAG's
/// own large-text definition), so the 3:1 route applies, not 4.5:1 —
/// `error` on `base-100` measures 3.08:1, which clears it.
///
/// `auth.rs:15` and `layout.rs:211`: the decorative brand bullet, a
/// glyph standing in for a bullet point, not text carrying a reader's
/// own meaning — WCAG 1.4.11 (non-text contrast, 3:1) applies, not
/// 1.4.3. `primary` on `base-100` measures 4.21:1, which clears 3:1
/// with room. Both sites render the identical literal; one entry
/// covers both.
///
/// `CLR-003` round 2: a bare "matches something" check does not carry
/// the `alignment_scan` precedent's own weight for `"text-primary"` —
/// two words, not a long opening tag, and exactly the class value a
/// future ordinary-body-text site would write (4.21:1, the violation
/// this series exists to prevent) would have been excluded by name,
/// silently, with both this list's own guard tests still green. Each
/// entry below is now paired with the **exact** live count
/// ([`NAMED_EXCLUSION_COUNTS`]), the `css_scan` population shape, so a
/// third occurrence fails by name rather than being inherited.
/// One [`NAMED_EXCLUSIONS_PATH`] entry — the exact class-attribute
/// literal, the exact number of live `components/**` elements it must
/// match (re-derived by `grep`, not copied forward — a count that
/// grows past this is a new site claiming an old exemption, which
/// must be measured and justified on its own, not inherited silently),
/// and why the exclusion holds. `reason` is read by a human, not this
/// scan — kept so the file stays the one place either reader needs to
/// understand an exclusion, not just apply it.
#[derive(Deserialize)]
struct ExclusionEntry {
    #[serde(rename = "classValue")]
    class_value: String,
    #[serde(rename = "liveCount")]
    live_count: usize,
    #[allow(dead_code)]
    reason: String,
}

/// `SWP-001` (`068`): moved out of this module's own Rust constants and
/// into one file both readers read — `contrast_scan` here, and
/// `browser-checks/contrast-gate.mjs` in Node, which needs the same
/// exact class-value literal to exempt the identical two sites from a
/// *rendered* 1.4.3 check for the same reason. **`liveCount` is read
/// only on this side** — `contrast-gate.mjs` has no source population
/// to check a count against, so it reads `classValue` alone and
/// ignores the rest of each entry; that is the cost of one home
/// instead of two lists, stated rather than hidden. The alternative
/// (keep this a Rust-only const, let the gate hardcode its own copy of
/// the two literals) is the two-lists-plus-a-comparison shape the
/// owner's review already rejected for this exact file, one series
/// earlier.
const NAMED_EXCLUSIONS_PATH: &str = "../../browser-checks/contrast-exclusions.json";

fn named_exclusions() -> Vec<ExclusionEntry> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(NAMED_EXCLUSIONS_PATH);
    let raw = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    serde_json::from_str(&raw).unwrap_or_else(|e| panic!("parse {}: {e}", path.display()))
}

/// A `components/**` class token this scan bans outright: a semantic
/// family spelled as `text-{family}` or `link-{family}`, bare or with
/// any `/N` suffix (a prefix check catches both, same as
/// [`BANNED_MODIFIERS`] above).
fn is_banned_semantic_token(token: &str) -> bool {
    SEMANTIC_FAMILIES.iter().any(|family| {
        token.starts_with(&format!("text-{family}")) || token.starts_with(&format!("link-{family}"))
    })
}

/// True if `tokens` is the one shape `CLR-001` found that a bare
/// `text-*`/`link-*` scan misses: `btn-outline` carrying the colour
/// through a paired `btn-{family}` instead of a text class.
fn has_banned_outline_pairing(tokens: &[String]) -> bool {
    tokens.iter().any(|t| t == "btn-outline")
        && tokens.iter().any(|t| {
            SEMANTIC_FAMILIES
                .iter()
                .any(|family| t == &format!("btn-{family}"))
        })
}

/// One `components/**` file's own semantic-colour violations:
/// `(file, token-or-pairing description)`. Each `class=`/`grow(...)`/
/// `grow_text(...)` literal is read once as a whole token list, so the
/// outline pairing is checked against sibling tokens from the same
/// element, not merely the same file.
fn semantic_violations_in(path: &Path, exclusions: &[ExclusionEntry]) -> Vec<(String, String)> {
    let raw = fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let stripped = strip_line_comments(&raw);
    let display = path.display().to_string();
    let mut out = Vec::new();
    for (quote_pos, content_start, content_end) in quoted_string_spans(&stripped) {
        if !is_token_source(&stripped, quote_pos) {
            continue;
        }
        let content = &stripped[content_start..content_end];
        if exclusions.iter().any(|e| e.class_value == content) {
            continue;
        }
        let tokens = tokens_from_content(content);
        for token in &tokens {
            if is_banned_semantic_token(token) {
                out.push((display.clone(), token.clone()));
            }
        }
        if has_banned_outline_pairing(&tokens) {
            out.push((
                display.clone(),
                format!("btn-outline + btn-{{family}} in {content:?}"),
            ));
        }
    }
    out
}

/// Every `.rs` file under `dir`, recursively — the same walk
/// `prose_scan`'s `collect_rs_files` uses since `QA-010` widened it to
/// all of `src/`, duplicated here rather than shared: a straight-line
/// helper, the kind `requirements.md §10.38` says may stay duplicated
/// (unlike `strip_line_comments`, now `scan_support`'s one shared copy).
fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries {
        let entry = entry.expect("dir entry");
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

/// `§10.61`'s own scope: `components/**` only, the same bound
/// `css_scan` already draws and for the same reason — a class token
/// outside a component has no render site this scan can reason about.
fn components_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("components")
}

fn static_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("static")
}

/// The three stylesheets `layout.rs:68-70` serves, read separately —
/// `css_scan::stylesheet_text` concatenates them for a token-presence
/// check that does not care which file wins; this scan does, so each
/// stays addressable by name. Comments stripped: `app.css`'s own
/// prose (this series' repairs quote the exact rule they beat, e.g.
/// `color:oklch(var(--bc)/.6)`, for a reader) would otherwise read as
/// a second, spurious below-floor declaration.
fn served_sheets() -> [(&'static str, String); 3] {
    let dir = static_dir();
    let read = |name: &str| {
        let path = dir.join(name);
        let raw =
            fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        strip_css_comments(&raw)
    };
    [
        ("daisyui.min.css", read("daisyui.min.css")),
        ("tailwind.css", read("tailwind.css")),
        ("app.css", read("app.css")),
    ]
}

/// Strips every `/* ... */` block comment, replacing its content with
/// spaces rather than removing it — byte positions this module's own
/// `abs` offsets rely on stay valid, and `enclosing_selector`'s own
/// backward brace-walk never crosses into what was a comment.
fn strip_css_comments(source: &str) -> String {
    let bytes = source.as_bytes();
    let mut out: Vec<u8> = bytes.to_vec();
    let mut i = 0;
    while i + 1 < bytes.len() {
        if bytes[i] == b'/' && bytes[i + 1] == b'*' {
            let start = i;
            i += 2;
            while i + 1 < bytes.len() && !(bytes[i] == b'*' && bytes[i + 1] == b'/') {
                i += 1;
            }
            let end = (i + 2).min(bytes.len());
            for b in out.iter_mut().take(end).skip(start) {
                if *b != b'\n' {
                    *b = b' ';
                }
            }
            i = end;
        } else {
            i += 1;
        }
    }
    String::from_utf8(out).expect("stylesheet stays valid UTF-8 after comment-blanking")
}

/// Parses a CSS alpha fraction's textual form (`"1"`, `"0"`, `".4"`,
/// `".65"`) into hundredths (`100`, `0`, `40`, `65`) — the unit this
/// module's own `/70` floor is already stated in.
fn parse_fraction_hundredths(text: &str) -> Option<u32> {
    if let Some(decimal) = text.strip_prefix('.') {
        let value: u32 = decimal.parse().ok()?;
        match decimal.len() {
            1 => Some(value * 10),
            2 => Some(value),
            _ => None,
        }
    } else {
        let value: u32 = text.parse().ok()?;
        Some(value * 100)
    }
}

/// The nearest `:` before `pos`, stopping at a declaration boundary
/// (`;`/`{`) rather than crossing into an unrelated property —
/// **not** "the immediately preceding character," since DaisyUI wraps
/// every resolved colour as `color:var(--fallback-bc,oklch(var(--bc)/.4))`,
/// so several bytes of that wrapper sit between the property's own
/// `:` and the `oklch(` this module searches for.
fn colon_immediately_before(text: &str, pos: usize) -> Option<usize> {
    let bytes = text.as_bytes();
    let mut i = pos;
    while i > 0 {
        i -= 1;
        match bytes[i] {
            b':' => return Some(i),
            b';' | b'{' => return None,
            _ => continue,
        }
    }
    None
}

/// The declaration's own property name immediately before `colon_pos`
/// — back to the previous `;`/`{` (or file start), trimmed. `"color"`
/// matches `color:` only, never `background-color:`/`border-color:`
/// — the boundary character excludes them, the same shape
/// [`colon_immediately_before`] uses to exclude a wrong declaration.
fn property_name_before(text: &str, colon_pos: usize) -> &str {
    let before = &text[..colon_pos];
    let start = before.rfind([';', '{']).map(|i| i + 1).unwrap_or(0);
    before[start..].trim()
}

/// The selector immediately enclosing the declaration at `pos` —
/// walks backward tracking brace depth, the same shape
/// [`crate::css_scan::quoted_string_spans`]'s own sibling
/// `enclosing_call_text` already proved for parentheses, so this
/// finds the *innermost* rule even inside `@media`, never a media
/// query's own outer condition.
fn enclosing_selector(text: &str, pos: usize) -> String {
    let bytes = text.as_bytes();
    let mut i = pos;
    let mut depth: i32 = 0;
    let mut open = None;
    while i > 0 {
        i -= 1;
        match bytes[i] {
            b'}' => depth += 1,
            b'{' => {
                if depth == 0 {
                    open = Some(i);
                    break;
                }
                depth -= 1;
            }
            _ => {}
        }
    }
    let Some(open) = open else {
        return String::new();
    };
    let mut j = open;
    while j > 0 {
        j -= 1;
        if bytes[j] == b'}' {
            j += 1;
            break;
        }
    }
    normalize_selector(text[j..open].trim())
}

/// Collapses a selector's own insignificant whitespace (a line wrap,
/// the space `app.css`'s hand-formatted rules carry after a comma) to
/// the single-space form the vendored sheets' minified text already
/// uses, so one literal in [`ACKNOWLEDGED_SHEET_RULES`] matches either
/// source without a second, looser copy per file.
fn normalize_selector(raw: &str) -> String {
    raw.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .replace(", ", ",")
}

/// Every below-`/70` `color:` declaration's `(selector, value)` pair
/// in `sheet` — the `oklch(var(--bc)/N)` shape every in-scope DaisyUI
/// rule uses, plus the one literal hex `tailwind.css` hardcodes for
/// its placeholder rule instead of `var(--bc)`.
fn below_floor_declarations(sheet: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();

    let bc_needle = "oklch(var(--bc)/";
    let mut search_from = 0;
    while let Some(rel) = sheet[search_from..].find(bc_needle) {
        let abs = search_from + rel;
        let value_start = abs + bc_needle.len();
        search_from = value_start;
        let Some(close_rel) = sheet[value_start..].find(')') else {
            break;
        };
        let fraction_text = &sheet[value_start..value_start + close_rel];
        let Some(hundredths) = parse_fraction_hundredths(fraction_text) else {
            continue;
        };
        if hundredths >= 70 {
            continue;
        }
        let Some(colon_pos) = colon_immediately_before(sheet, abs) else {
            continue;
        };
        if property_name_before(sheet, colon_pos) == "color" {
            let selector = enclosing_selector(sheet, abs);
            out.push((selector, format!("oklch(var(--bc)/{fraction_text})")));
        }
    }

    let hex_needle = "#9ca3af";
    let mut search_from = 0;
    while let Some(rel) = sheet[search_from..].find(hex_needle) {
        let abs = search_from + rel;
        search_from = abs + hex_needle.len();
        let Some(colon_pos) = colon_immediately_before(sheet, abs) else {
            continue;
        };
        if property_name_before(sheet, colon_pos) == "color" {
            let selector = enclosing_selector(sheet, abs);
            out.push((selector, hex_needle.to_string()));
        }
    }

    out
}

/// Every class token written by some `components/**` element —
/// recomputed here rather than exposing `css_scan`'s own private
/// population, to decide whether a below-floor sheet rule is actually
/// reachable: `.stat-title`/`.stat-desc` are in the pinned sheet
/// unconditionally but reach no reader until a component writes that
/// class.
fn written_class_tokens() -> std::collections::BTreeSet<String> {
    let dir = components_dir();
    let mut files = Vec::new();
    collect_rs_files(&dir, &mut files);
    let mut tokens = std::collections::BTreeSet::new();
    for path in &files {
        let raw =
            fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        let stripped = strip_line_comments(&raw);
        for (quote_pos, content_start, content_end) in quoted_string_spans(&stripped) {
            if !is_token_source(&stripped, quote_pos) {
                continue;
            }
            for token in tokens_from_content(&stripped[content_start..content_end]) {
                tokens.insert(token);
            }
        }
    }
    tokens
}

/// True if any comma-separated alternative in `selector` is reachable:
/// a bare element/pseudo-element alternative (no leading class) always
/// is, since no component can avoid emitting an `<input>` or a
/// `<thead>`; a class-based alternative is reachable only if some
/// component writes that alternative's own first class token.
fn selector_is_reachable(selector: &str, written: &std::collections::BTreeSet<String>) -> bool {
    selector
        .split(',')
        .any(|alt| alternative_is_reachable(alt.trim(), written))
}

/// The first class token of one selector alternative, unescaping CSS's
/// own `\` selector-escape (the inverse of
/// [`crate::css_scan::escape_css_token`]) — `None` if the alternative
/// does not start with `.` at all (a bare element/pseudo-element,
/// always reachable).
fn alternative_is_reachable(alt: &str, written: &std::collections::BTreeSet<String>) -> bool {
    let bytes = alt.as_bytes();
    if bytes.first() != Some(&b'.') {
        return true;
    }
    let mut class = String::new();
    let mut i = 1;
    while i < bytes.len() {
        match bytes[i] {
            b'\\' if i + 1 < bytes.len() => {
                class.push(bytes[i + 1] as char);
                i += 2;
            }
            b':' | b'[' | b'>' | b'~' | b'+' | b'(' | b')' => break,
            b if (b as char).is_ascii_whitespace() => break,
            b => {
                class.push(b as char);
                i += 1;
            }
        }
    }
    written.contains(&class)
}

/// A below-floor sheet rule reachable by a component, with the exact
/// literal that must still be found before this rule is trusted to
/// never reach a reader.
struct AcknowledgedSheetRule {
    /// The selector exactly as [`below_floor_declarations`] extracts
    /// it (comma-joined, normalized whitespace — see
    /// [`normalize_selector`]).
    selector: &'static str,
    /// Path from `CARGO_MANIFEST_DIR`: either the `app.css` override
    /// rule, or the one component file whose own repair beats this
    /// rule by ordinary inheritance rather than a second stylesheet
    /// rule.
    beaten_by_file: &'static str,
    /// A literal substring of `beaten_by_file` proving the thing that
    /// beats this rule is still there.
    beaten_by_needle: &'static str,
}

/// Every reachable below-`/70` sheet rule this series found, and what
/// beats each one before it reaches a reader — `PLC-003`'s own
/// acceptance item 2: "paired with its exact count," here a literal
/// needle rather than a count, since each of these is beaten by one
/// named thing rather than claiming an exemption shared across sites.
///
/// `.menu-title` (`/.4`): `layout.rs`'s own one rendering site wraps
/// its text in a child `<span>` carrying an explicit
/// `text-base-content/70`, which wins for that text by ordinary
/// inheritance — not a second stylesheet rule, so `beaten_by_file`
/// names the component rather than `app.css`.
///
/// `.table :where(thead,tfoot)` (`/.6`) and
/// `input::placeholder,textarea::placeholder` /
/// `input::-moz-placeholder,textarea::-moz-placeholder` (`#9ca3af`):
/// each beaten by `app.css`'s own override rule, `PLC-001`/`PLC-002`/
/// `PLC-003` in turn — see each rule's own comment there for why the
/// cascade reason differs between the table rule (sheet order) and
/// the placeholder rules (specificity).
const ACKNOWLEDGED_SHEET_RULES: &[AcknowledgedSheetRule] = &[
    AcknowledgedSheetRule {
        selector: ".menu-title",
        beaten_by_file: "src/components/layout.rs",
        beaten_by_needle: "<span class=\"text-xs text-base-content/70 truncate\">",
    },
    AcknowledgedSheetRule {
        selector: ".table :where(thead,tfoot)",
        beaten_by_file: "static/app.css",
        beaten_by_needle: ".table :where(thead, tfoot) {",
    },
    AcknowledgedSheetRule {
        selector: "input::placeholder,textarea::placeholder",
        beaten_by_file: "static/app.css",
        beaten_by_needle: ".input::placeholder",
    },
    AcknowledgedSheetRule {
        selector: "input::-moz-placeholder,textarea::-moz-placeholder",
        beaten_by_file: "static/app.css",
        beaten_by_needle: ".input::-moz-placeholder",
    },
];

/// Exempt under WCAG 1.4.3 — text that is part of an inactive
/// user-interface component — acknowledged unconditionally, **not**
/// gated on [`selector_is_reachable`]: each compound selector's own
/// first class token (`.input`, `.select`, `.textarea`, `.menu`) is
/// written everywhere regardless of whether the `:disabled`/
/// `.disabled` half is ever triggered, so that check would read these
/// as live findings when, today, none of the four disabled states
/// renders anywhere in `components/**` (confirmed by grep: no call
/// site sets `disabled` on an input/select/textarea or a menu `<li>`).
/// If one ever does, it stays exempt on the same WCAG reasoning, not
/// a newly-required acknowledgement.
const EXEMPT_SHEET_RULES: &[&str] = &[
    ".menu li.disabled",
    ".input-disabled,.input:disabled,.input:has(> input[disabled]),.input[disabled]",
    ".select-disabled,.select:disabled,.select[disabled]",
    ".textarea-disabled,.textarea:disabled,.textarea[disabled]",
];

/// Not a text colour: `.toggle`'s own `color` property is read back
/// only via `currentColor`, by its own `border-color`/
/// `background-color` — no text node is ever painted by this rule.
/// WCAG 1.4.11 (non-text contrast), not 1.4.3, and out of this scan's
/// own scope (text a reader reads).
const NON_TEXT_SHEET_SELECTORS: &[&str] = &[".toggle"];

fn beaten_by_file_text(rel_path: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

/// One below-floor sheet rule found reachable by a component but named
/// in neither [`ACKNOWLEDGED_SHEET_RULES`] nor [`EXEMPT_SHEET_RULES`]
/// — the failure this guard exists to raise before a theme bump or a
/// new DaisyUI class ships a faded colour unnoticed.
#[derive(Debug)]
struct UnacknowledgedSheetRule {
    selector: String,
    value: String,
}

/// This scan's full result — see each field; a scan that finds
/// everything empty is a pass, same as the two scans above, but a
/// scan that silently examined zero sheets must not read as one
/// (`§10.36`), which is why `sheet_leak_scan` panics rather than
/// returning empty on a missing file.
struct SheetLeakResult {
    unacknowledged: Vec<UnacknowledgedSheetRule>,
    /// An [`ACKNOWLEDGED_SHEET_RULES`] selector no longer found as a
    /// below-floor rule in any served sheet — the vendored CSS moved
    /// or changed and this acknowledgement is now stale.
    stale_acknowledgements: Vec<&'static str>,
    /// An [`ACKNOWLEDGED_SHEET_RULES`] entry whose own
    /// `beaten_by_needle` no longer appears in `beaten_by_file` — the
    /// thing that was supposed to stop this rule reaching a reader is
    /// gone, but the acknowledgement does not know that.
    no_longer_beaten: Vec<&'static str>,
}

fn sheet_leak_scan() -> SheetLeakResult {
    let sheets = served_sheets();
    let written = written_class_tokens();

    let mut all_selectors: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    let mut unacknowledged = Vec::new();

    for (_name, text) in &sheets {
        for (selector, value) in below_floor_declarations(text) {
            all_selectors.insert(selector.clone());
            if NON_TEXT_SHEET_SELECTORS.contains(&selector.as_str())
                || EXEMPT_SHEET_RULES.contains(&selector.as_str())
                || ACKNOWLEDGED_SHEET_RULES
                    .iter()
                    .any(|r| r.selector == selector)
            {
                continue;
            }
            if selector_is_reachable(&selector, &written) {
                unacknowledged.push(UnacknowledgedSheetRule { selector, value });
            }
        }
    }

    let mut stale_acknowledgements = Vec::new();
    let mut no_longer_beaten = Vec::new();
    for rule in ACKNOWLEDGED_SHEET_RULES {
        if !all_selectors.contains(rule.selector) {
            stale_acknowledgements.push(rule.selector);
            continue;
        }
        if !beaten_by_file_text(rule.beaten_by_file).contains(rule.beaten_by_needle) {
            no_longer_beaten.push(rule.selector);
        }
    }

    SheetLeakResult {
        unacknowledged,
        stale_acknowledgements,
        no_longer_beaten,
    }
}

/// `border-color` and `outline-color` — the two non-text channels
/// `§10.63` watches. `color` is `§10.62`'s own, deliberately not
/// repeated here.
const BOUNDARY_PROPERTIES: [&str; 2] = ["border-color", "outline-color"];

/// Parses a CSS alpha fraction in *either* shape this project's
/// pinned sheet actually uses — `".2"`/`".65"` (bare, `app.css`'s own
/// inline literals before this round, and most of `daisyui.min.css`'s
/// inline ones) or `"0.2"`/`"0.7"` (`app.css`'s own `EDG-001`/`EDG-002`
/// repairs, and every `--tw-border-opacity`/`--tw-outline-opacity`
/// sibling declaration) — into hundredths. [`parse_fraction_hundredths`]
/// (`§10.62`'s own, used only by the `color:` scan above) does not
/// parse the leading-zero form, because no `color:` declaration in any
/// served sheet has ever used it; this round's own repairs do, so a
/// second parser is honest about covering a shape the first was never
/// asked to.
fn parse_boundary_fraction(text: &str) -> Option<u32> {
    let text = text.trim();
    if let Some(decimal) = text.strip_prefix("0.") {
        return match decimal.len() {
            1 => Some(decimal.parse::<u32>().ok()? * 10),
            2 => decimal.parse().ok(),
            _ => None,
        };
    }
    parse_fraction_hundredths(text)
}

/// The `base-content` fraction inside a `border-color`/`outline-color`
/// value's own inline `oklch(var(--bc) / N)` or `oklch(var(--bc)/N)` —
/// searched for *within* the value, not required as its prefix,
/// because the pinned sheet wraps every resolved colour as
/// `var(--fallback-bc,oklch(var(--bc)/N))`. `app.css`'s hand-formatted
/// spaces around `/` differ from `daisyui.min.css`'s minified none, so
/// both are accepted here rather than normalizing the sheet text
/// first. Returns `None` if `oklch(var(--bc)` is absent, **or** if
/// what follows its `/` is not a literal fraction at all — the
/// `--tw-border-opacity`/`--tw-outline-opacity` custom-property shape
/// (`.checkbox`'s own rule) reads `oklch(var(--bc)/var(--tw-border-
/// opacity))` here, which [`resolve_boundary_fraction`] must fall
/// through on rather than mistake for an unparsed literal.
fn inline_bc_fraction(value: &str) -> Option<&str> {
    let idx = value.find("oklch(var(--bc)")?;
    let rest = &value[idx + "oklch(var(--bc)".len()..];
    let rest = rest.trim_start().strip_prefix('/')?;
    let rest = rest.trim_start();
    let close = rest.find(')')?;
    Some(rest[..close].trim())
}

/// The `{`...`}` span of the rule enclosing the declaration at `pos`,
/// exclusive of the braces — the forward counterpart to
/// [`enclosing_selector`]'s backward walk, needed only here: a
/// `--tw-border-opacity`/`--tw-outline-opacity` custom property is
/// re-declared by nearly every rule in the pinned sheet with a
/// different value each time, so resolving one means staying inside
/// the single rule block the reference came from, never the sheet at
/// large.
fn enclosing_block_span(text: &str, pos: usize) -> (usize, usize) {
    let bytes = text.as_bytes();
    let mut i = pos;
    let mut depth: i32 = 0;
    let mut open = None;
    while i > 0 {
        i -= 1;
        match bytes[i] {
            b'}' => depth += 1,
            b'{' => {
                if depth == 0 {
                    open = Some(i);
                    break;
                }
                depth -= 1;
            }
            _ => {}
        }
    }
    let open = open.expect("a border-color/outline-color declaration has an enclosing rule");
    let mut j = open + 1;
    let mut close_depth: i32 = 1;
    while j < bytes.len() && close_depth > 0 {
        match bytes[j] {
            b'{' => close_depth += 1,
            b'}' => close_depth -= 1,
            _ => {}
        }
        j += 1;
    }
    (open + 1, j - 1)
}

/// The `base-content` fraction (in hundredths) a `border-color`/
/// `outline-color` declaration's own `value` resolves to, or `None`
/// if it is not a `base-content` fraction at all (`currentColor`,
/// `transparent`, a semantic-family token — out of this floor's own
/// concept, the same bound [`below_floor_declarations`] draws for
/// `color:`). **Never composites, never asks what is adjacent, never
/// asks what the control's own fill paints** — see this module's own
/// `§10.63` doc comment for why that line is not crossed.
fn resolve_boundary_fraction(sheet: &str, decl_pos: usize, value: &str) -> Option<u32> {
    if let Some(frac) = inline_bc_fraction(value)
        && let Some(hundredths) = parse_boundary_fraction(frac)
    {
        return Some(hundredths);
    }
    let var_name = if value.contains("var(--tw-border-opacity)") {
        "--tw-border-opacity"
    } else if value.contains("var(--tw-outline-opacity)") {
        "--tw-outline-opacity"
    } else {
        return None;
    };
    let (block_start, block_end) = enclosing_block_span(sheet, decl_pos);
    let block = &sheet[block_start..block_end];
    let needle = format!("{var_name}:");
    let pos = block.find(&needle).unwrap_or_else(|| {
        panic!(
            "{var_name} referenced in a border/outline-color declaration but not \
             set in the same rule -- this scan's assumption about how the vendored \
             sheet shapes this custom property no longer holds, re-derive by hand"
        )
    });
    let value_start = pos + needle.len();
    let value_end = block[value_start..]
        .find([';', '}'])
        .map(|i| value_start + i)
        .unwrap_or(block.len());
    parse_boundary_fraction(block[value_start..value_end].trim())
}

/// Every below-`/70` `border-color:`/`outline-color:` declaration's
/// `(selector, property, value)` in `sheet` — `§10.63`'s own
/// counterpart to [`below_floor_declarations`], one property name at
/// a time since the same selector can carry one acknowledged property
/// and one exempt one (`.input:focus,.input:focus-within`'s own
/// `outline-color`, acknowledged, and `border-color`, exempt).
fn below_floor_boundary_declarations(sheet: &str) -> Vec<(String, &'static str, String)> {
    let mut out = Vec::new();
    for property in BOUNDARY_PROPERTIES {
        let needle = format!("{property}:");
        let mut search_from = 0;
        while let Some(rel) = sheet[search_from..].find(needle.as_str()) {
            let abs = search_from + rel;
            let colon_pos = abs + property.len();
            let value_end = sheet[colon_pos + 1..]
                .find([';', '}'])
                .map(|i| colon_pos + 1 + i)
                .unwrap_or(sheet.len());
            let value = sheet[colon_pos + 1..value_end].trim();
            search_from = colon_pos + 1;
            let Some(hundredths) = resolve_boundary_fraction(sheet, abs, value) else {
                continue;
            };
            if hundredths >= 70 {
                continue;
            }
            let selector = enclosing_selector(sheet, abs);
            out.push((
                selector,
                property,
                format!("{value} ({hundredths} hundredths)"),
            ));
        }
    }
    out
}

/// A below-floor `border-color`/`outline-color` rule reachable by a
/// component, with the exact literal that must still be found before
/// it is trusted to never reach a reader — `§10.62`'s own
/// [`AcknowledgedSheetRule`], keyed by `(selector, property)` since
/// one selector can carry findings on both properties.
struct AcknowledgedBoundaryRule {
    selector: &'static str,
    property: &'static str,
    /// Path from `CARGO_MANIFEST_DIR` of the `app.css` rule that beats
    /// this one.
    beaten_by_file: &'static str,
    /// A literal substring of `beaten_by_file` proving the repair is
    /// still there.
    beaten_by_needle: &'static str,
}

/// Every reachable below-`/70` boundary/focus rule `EDG-001`/`EDG-002`
/// repaired, and what beats each — the selector exactly as the pinned
/// sheet itself writes it (`daisyui.min.css` gives `.input-bordered`,
/// `.select-bordered`, `.textarea-bordered` and `.checkbox` each their
/// own single-class rule, never one joined selector, even though
/// `app.css`'s own repair covers all four in one), normalized the same
/// way [`ACKNOWLEDGED_SHEET_RULES`] already is.
const ACKNOWLEDGED_BOUNDARY_RULES: &[AcknowledgedBoundaryRule] = &[
    AcknowledgedBoundaryRule {
        selector: ".input-bordered",
        property: "border-color",
        beaten_by_file: "static/app.css",
        beaten_by_needle: "border-color: oklch(var(--bc) / 0.7)",
    },
    AcknowledgedBoundaryRule {
        selector: ".select-bordered",
        property: "border-color",
        beaten_by_file: "static/app.css",
        beaten_by_needle: "border-color: oklch(var(--bc) / 0.7)",
    },
    AcknowledgedBoundaryRule {
        selector: ".textarea-bordered",
        property: "border-color",
        beaten_by_file: "static/app.css",
        beaten_by_needle: "border-color: oklch(var(--bc) / 0.7)",
    },
    AcknowledgedBoundaryRule {
        selector: ".checkbox",
        property: "border-color",
        beaten_by_file: "static/app.css",
        beaten_by_needle: "border-color: oklch(var(--bc) / 0.7)",
    },
    AcknowledgedBoundaryRule {
        selector: ".input:focus,.input:focus-within",
        property: "outline-color",
        beaten_by_file: "static/app.css",
        beaten_by_needle: "outline-color: oklch(var(--bc) / 1)",
    },
    AcknowledgedBoundaryRule {
        selector: ".select:focus",
        property: "outline-color",
        beaten_by_file: "static/app.css",
        beaten_by_needle: "outline-color: oklch(var(--bc) / 1)",
    },
    AcknowledgedBoundaryRule {
        selector: ".textarea:focus",
        property: "outline-color",
        beaten_by_file: "static/app.css",
        beaten_by_needle: "outline-color: oklch(var(--bc) / 1)",
    },
];

/// A below-floor `border-color`/`outline-color` rule left exactly as
/// the pinned sheet draws it, on purpose, checked rather than
/// assumed: `EDG-002 §4` screenshotted a focused field and found the
/// ring (acknowledged above, full `base-content`, 17.21:1) carries the
/// focus state alone, 2px outside a border specificity leaves faint —
/// raising it too would spend effort on a state the ring already
/// makes unambiguous. Exempt regardless of reachability (all three are
/// reachable; the reason is a visual decision, not an absence of a
/// reader), the same shape [`EXEMPT_SHEET_RULES`] uses for a different
/// reason.
struct ExemptBoundaryRule {
    selector: &'static str,
    property: &'static str,
}

const EXEMPT_BOUNDARY_RULES: &[ExemptBoundaryRule] = &[
    ExemptBoundaryRule {
        selector: ".input:focus,.input:focus-within",
        property: "border-color",
    },
    ExemptBoundaryRule {
        selector: ".select:focus",
        property: "border-color",
    },
    ExemptBoundaryRule {
        selector: ".textarea:focus",
        property: "border-color",
    },
];

/// One below-floor boundary/focus rule found reachable by a component
/// but named in neither [`ACKNOWLEDGED_BOUNDARY_RULES`] nor
/// [`EXEMPT_BOUNDARY_RULES`] — `§10.62`'s own
/// [`UnacknowledgedSheetRule`], one property over.
#[derive(Debug)]
struct UnacknowledgedBoundaryRule {
    selector: String,
    property: &'static str,
    value: String,
}

/// This scan's full result — `§10.62`'s own [`SheetLeakResult`], widened
/// with [`stale_exemptions`][Self::stale_exemptions] since
/// [`EXEMPT_BOUNDARY_RULES`] (unlike [`EXEMPT_SHEET_RULES`]) names a
/// specific rule rather than an unconditional carve-out, so it can go
/// stale the same way an acknowledgement can.
struct BoundaryLeakResult {
    unacknowledged: Vec<UnacknowledgedBoundaryRule>,
    stale_acknowledgements: Vec<(&'static str, &'static str)>,
    no_longer_beaten: Vec<(&'static str, &'static str)>,
    /// An [`EXEMPT_BOUNDARY_RULES`] entry no longer found as a
    /// below-floor rule in any served sheet.
    stale_exemptions: Vec<(&'static str, &'static str)>,
}

fn boundary_leak_scan() -> BoundaryLeakResult {
    let sheets = served_sheets();
    let written = written_class_tokens();

    let mut all_pairs: std::collections::BTreeSet<(String, &'static str)> =
        std::collections::BTreeSet::new();
    let mut unacknowledged = Vec::new();

    for (_name, text) in &sheets {
        for (selector, property, value) in below_floor_boundary_declarations(text) {
            all_pairs.insert((selector.clone(), property));
            if EXEMPT_BOUNDARY_RULES
                .iter()
                .any(|r| r.selector == selector && r.property == property)
                || ACKNOWLEDGED_BOUNDARY_RULES
                    .iter()
                    .any(|r| r.selector == selector && r.property == property)
            {
                continue;
            }
            if selector_is_reachable(&selector, &written) {
                unacknowledged.push(UnacknowledgedBoundaryRule {
                    selector,
                    property,
                    value,
                });
            }
        }
    }

    let mut stale_acknowledgements = Vec::new();
    let mut no_longer_beaten = Vec::new();
    for rule in ACKNOWLEDGED_BOUNDARY_RULES {
        if !all_pairs.contains(&(rule.selector.to_string(), rule.property)) {
            stale_acknowledgements.push((rule.selector, rule.property));
            continue;
        }
        if !beaten_by_file_text(rule.beaten_by_file).contains(rule.beaten_by_needle) {
            no_longer_beaten.push((rule.selector, rule.property));
        }
    }

    let mut stale_exemptions = Vec::new();
    for rule in EXEMPT_BOUNDARY_RULES {
        if !all_pairs.contains(&(rule.selector.to_string(), rule.property)) {
            stale_exemptions.push((rule.selector, rule.property));
        }
    }

    BoundaryLeakResult {
        unacknowledged,
        stale_acknowledgements,
        no_longer_beaten,
        stale_exemptions,
    }
}

#[cfg(test)]
mod tests;
