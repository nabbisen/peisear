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

use std::fs;
use std::path::{Path, PathBuf};

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
const NAMED_EXCLUSION_CLASS_VALUES: [&str; 2] = ["text-5xl font-bold text-error", "text-primary"];

/// The exact number of live `components/**` elements each
/// [`NAMED_EXCLUSION_CLASS_VALUES`] entry must match, same index —
/// re-derived by `grep`, not copied forward: **1** for the status
/// number (`error_page.rs:18` alone), **2** for the brand bullet
/// (`auth.rs:15` and `layout.rs:211`, and no third site). A count that
/// grows past this is a new site claiming an old exemption, which
/// must be measured and justified on its own, not inherited silently.
const NAMED_EXCLUSION_COUNTS: [usize; 2] = [1, 2];

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
fn semantic_violations_in(path: &Path) -> Vec<(String, String)> {
    let raw = fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let stripped = strip_line_comments(&raw);
    let display = path.display().to_string();
    let mut out = Vec::new();
    for (quote_pos, content_start, content_end) in quoted_string_spans(&stripped) {
        if !is_token_source(&stripped, quote_pos) {
            continue;
        }
        let content = &stripped[content_start..content_end];
        if NAMED_EXCLUSION_CLASS_VALUES.contains(&content) {
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

#[cfg(test)]
mod tests;
