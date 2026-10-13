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

use std::fs;
use std::path::{Path, PathBuf};

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

#[cfg(test)]
mod tests;
