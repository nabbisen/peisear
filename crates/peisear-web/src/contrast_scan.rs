//! `QA-013` §4 (RFC 005 §4) — with `/70` established as the muted-tier
//! floor (`QA-012`'s measurement, `QA-013`'s call-site sweep), a text
//! scan over `text-base-content/N` for `N < 70` becomes an honest
//! assertion rather than a guess: no legitimate use exists below `/70`
//! against any background this theme defines, so there is no passing
//! case for a scan to mis-flag.
//!
//! **Pinned measurement.** `daisyui@4.12.14`'s `corporate` theme,
//! resolved from its actual shipped CSS
//! (`https://cdn.jsdelivr.net/npm/daisyui@4.12.14/dist/full.min.css`,
//! the exact version `components/layout.rs` loads) — `QA-012`'s own
//! source, not assumed: `base-content` `#181A2A`, `base-100`
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
/// all of `src/`, duplicated here rather than shared for a fifteen-line
/// helper, matching this codebase's existing convention for
/// `strip_line_comments`.
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

#[test]
fn no_muted_text_below_the_seventy_floor() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let src_dir = manifest_dir.join("src");
    let mut files = Vec::new();
    collect_rs_files(&src_dir, &mut files);
    assert!(
        !files.is_empty(),
        "found no .rs files under src/ -- the workspace layout assumption \
         this scan depends on may have changed"
    );

    // This module's own file is excluded: `BANNED_MODIFIERS` has to
    // name the literal strings it bans, so a naive scan of every `.rs`
    // file would flag this file for containing its own ban list —
    // the same self-mention shape `prose_scan`'s doc comment and
    // `dec_007_scan`'s history paragraph both had to be excluded from
    // their own scans for, just via a literal array instead of prose.
    let mut offenders = Vec::new();
    for path in files
        .iter()
        .filter(|p| p.file_name().and_then(|n| n.to_str()) != Some("contrast_scan.rs"))
    {
        let source =
            fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        for needle in BANNED_MODIFIERS {
            if source.contains(needle) {
                offenders.push(format!("{}: {needle}", path.display()));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "these files use a text-base-content opacity modifier below the /70 floor \
         -- QA-012 and QA-013 measured every one of /10 through /60 as failing AA \
         against at least one real background this theme defines (see this module's \
         doc comment for the resolved values and DaisyUI version):\n{}",
        offenders
            .iter()
            .map(|o| format!("  {o}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
