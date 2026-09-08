//! `QA-015` §5 (`NFR-A11Y-007`, RFC 005 §6) — `checkbox-xs` resolves to
//! `16×16px` in `daisyui@4.12.14`'s pinned CSS (`static/daisyui.min.css`,
//! vendored by `ASSET-001`/`DEC-051` — pinned by version and SHA-256,
//! see `style/tailwindcss/README.md`; the version `components/layout.rs`
//! loads, and now the file this crate ships rather than one fetched
//! from `cdn.jsdelivr.net` at runtime), below WCAG 2.2's own AA
//! floor (2.5.8: 24×24) — the only class `QA-014`'s survey found doing
//! that, everything else in `src/components/` is 24px or larger.
//! `QA-015` removed it from the three real checkbox controls that
//! carried it (`notification_preferences.rs`), reaching the bare
//! `.checkbox` class's own `24px` (`height:1.5rem;width:1.5rem`). This
//! keeps it from coming back.
//!
//! **Sibling to `contrast_scan`, not folded into it.** Same reasoning
//! `dec_007_scan`/`dec_007_ci_scan` already established: contrast
//! (`WCAG` 1.4.3) and touch-target size (`WCAG` 2.5.8) are different
//! properties measured against a different pinned CSS fact, and a
//! regression in one should not read as a failure of the other's own
//! name. `contrast_scan`'s own doc comment names the same DaisyUI
//! version and a disjoint set of resolved values; this module names its
//! own.
//!
//! **One needle, not a class of needles — until `TT-002` made it
//! safe to be more.** `QA-014`'s survey found six other classes below
//! 44px (`btn-sm`/`-xs`, `input-sm`/`-xs`, `select-sm`/`-xs`) still in
//! deliberate use pending `0.30.0`'s touch-target design pass —
//! banning any of those then would have failed on the current,
//! correct tree and gotten weakened until it passed, worse than no
//! guard (`QA-013`'s own reasoning for why `contrast_scan` stops at
//! `/60` and not higher). `checkbox-xs` was different from the start:
//! `QA-015` removed every use, and the class serves no purpose this
//! codebase still needs — `checkbox-sm` (`20px`, still below AA) and
//! the bare `checkbox` (`24px`, the class this project now uses) are
//! the only smaller-than-default sizes with any legitimate reason to
//! exist here, and neither is `checkbox-xs`.
//!
//! **`TT-002` (RFC 012 step 3) made the other six safe to enforce
//! too** — every sizing-class site in `src/components/` now composes
//! `components::TOUCH_TARGET` via `components::grow`, so
//! [`every_sizing_class_site_composes_the_touch_target`] makes
//! `NFR-A11Y-007`'s whole size clause unconstructible, not just one
//! class of it. Reads `components::TOUCH_TARGET`/`grow`'s own
//! behaviour rather than a hardcoded copy — the guard lives in this
//! crate and has no excuse for one (`TT-002-round2-review.md` §4: the
//! 44px value already has one home in production and seven in test
//! code; this guard must not be an eighth).
//!
//! **No exception list.** `TT-002` round 2 converted the last three
//! hardcoded `min-h-11 min-w-11` literals specifically so this guard
//! would need none (`TT-002-round2-review.md` §2). If a future change
//! needs one, that is a finding to report, not a line to add — this
//! module's own `checkbox-xs` history already states why: a rule that
//! fails on a correct tree gets weakened until it passes, and an
//! exception list is how that weakening looks in practice.
//!
//! **Scans string-literal contents, not lines and not raw file
//! text.** `QA-004` found a guard satisfied by a doc-comment mention;
//! `QA-005` found one satisfied by a commented-out line. The inverse
//! matters here too: a doc comment reading *"`btn-sm` is 32px"* must
//! not **fail** this guard either. [`quoted_string_spans`] walks the
//! source char-by-char and only yields the span between an actual
//! `"..."` Rust string literal's quotes.
//!
//! **`TT-003-review.md` §3 — that claim held in only one direction.**
//! A doc comment reading *"`btn-sm` is 32px"* (backticked, no quotes)
//! was already invisible, correctly. But `quoted_string_spans` has no
//! notion of comments at all, so a sizing class **inside quotes
//! inside a comment** — `// quoted: "input-xs" is 24px.` — read as a
//! real class literal and **failed the guard on a correct tree**,
//! exactly the failure mode this module's own `checkbox-xs` history
//! warns about. [`strip_line_comments`] runs first now, so both
//! directions hold: prose can neither satisfy the guard nor break it,
//! whether or not it happens to sit inside quotes.
//!
//! **That holds for `//` line comments only.** `strip_line_comments`
//! does not strip block comments, so a sizing class quoted inside a
//! `/* ... */` block still reads as a class literal and would fail
//! this scan on a correct tree. No block comment exists anywhere in
//! this crate, so the gap is latent rather than live — but it is the
//! one direction of the "both directions hold" claim above that is
//! not actually true, and a guard that fails on correct code is the
//! failure mode this module's own `checkbox-xs` history warns about.
//! Named here (architect, `TT-003` round-2 review) rather than fixed,
//! because stripping block comments correctly is the parser boundary
//! the paragraph below declines to cross.
//!
//! **A named limit, not a parser** — the same boundary `JS-002` hit
//! and the same answer: [`quoted_string_spans`] handles the one string
//! form every `class=` site in `src/components/` actually uses today
//! (a plain `"..."` literal, `\"` escapes recognised so an escaped
//! quote can't prematurely close a span) and does not attempt raw
//! strings (`r#"..."#`), byte strings, or multi-line literals. None
//! appear in a `class=` attribute in this tree; if one ever does, this
//! scan misses it rather than mis-parsing it — the same trade
//! `dm_fallback_boundary_scan`'s own doc comment makes for its class
//! of gap, named here rather than reached for a parser dependency.
//!
//! **Scope, stated because a green result reads as more than it
//! is** (`TT-001` §5, now a named limit on `NFR-A11Y-007` itself):
//! this guard keys off sizing classes, so it covers class-carrying
//! controls only. A plain `<a>` link, a breadcrumb, a whole-card link
//! — none carry `btn-*`/`input-*`/`select-*`, so none are counted,
//! checked, or claimed compliant by this scan. They sit **outside**
//! `TT-002`'s 139-control counting method entirely, not inside it and
//! passing. A green result here is a claim about class-carrying
//! controls, not about every interactive element in the tree.
//!
//! **`TT-003-review.md` §2 — bare checkboxes were the same kind of
//! gap, at the other end of `TT-002`'s own scope.** `class="checkbox"`
//! carries no `btn-*`/`input-*`/`select-*` sizing class, so
//! [`every_sizing_class_site_composes_the_touch_target`] never sees
//! it — and `TT-002` §8 already named the check this handoff needed
//! (*"assert each of the three checkbox source sites is wrapped in a
//! `<label>` carrying `grow(...)`"*), dropped when this module's own
//! §2 was first written. [`every_bare_checkbox_is_label_wrapped_with_grow`]
//! closes it: every `class="checkbox"` site must be enclosed by a
//! `<label class=grow(...)>` — the sanctioned `Expand` wrap
//! (`DEC-049` as amended) that keeps the box itself at 24px while the
//! label reaches 44px. A checkbox with no enclosing `<label>` at all,
//! or one whose `<label>` doesn't call `grow`, fails. `checkbox-xs`
//! and `checkbox-sm` stay out of scope here — `checkbox_xs_appears_nowhere`
//! already bans the one that mattered, and `checkbox-sm` (`20px`) has
//! no legitimate use in this tree today per this module's own opening
//! note.

use std::fs;
use std::path::{Path, PathBuf};

/// Every `.rs` file under `dir`, recursively — the same walk
/// `prose_scan`/`contrast_scan` use, duplicated rather than shared for
/// a fifteen-line helper.
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
fn checkbox_xs_appears_nowhere() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let src_dir = manifest_dir.join("src");
    let mut files = Vec::new();
    collect_rs_files(&src_dir, &mut files);
    assert!(
        !files.is_empty(),
        "found no .rs files under src/ -- the workspace layout assumption \
         this scan depends on may have changed"
    );

    let mut offenders = Vec::new();
    for path in files
        .iter()
        .filter(|p| p.file_name().and_then(|n| n.to_str()) != Some("touch_target_scan.rs"))
    {
        let source =
            fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        if source.contains("checkbox-xs") {
            offenders.push(path.display().to_string());
        }
    }

    assert!(
        offenders.is_empty(),
        "checkbox-xs resolves to 16x16px in the pinned DaisyUI CSS, below WCAG \
         2.2's AA touch-target floor (2.5.8: 24x24) -- QA-015 removed every use; \
         use the bare `checkbox` class (24px) instead:\n{}",
        offenders
            .iter()
            .map(|o| format!("  {o}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// Strips `//`-style line comments (plain, `///` doc comments, and
/// `//!` inner doc comments alike — all start with `//`) before
/// `quoted_string_spans` ever sees the text, so a sizing class or
/// `"checkbox"` mentioned inside a comment can neither satisfy nor
/// break either scan below (`TT-003-review.md` §3). Line-based,
/// duplicated from `prose_scan.rs`/`test_harness_scan.rs`'s own
/// identical function rather than shared — small enough that a
/// shared module would add more indirection than it saves. Does not
/// account for `//` appearing inside a real string literal (a URL,
/// say); no `class=` attribute in this tree carries one today, the
/// same documented limitation the two sibling copies already carry.
fn strip_line_comments(source: &str) -> String {
    source
        .lines()
        .map(|line| line.split("//").next().unwrap_or(""))
        .collect::<Vec<_>>()
        .join("\n")
}

/// The six sizing classes `TT-001`'s survey found below 44px and
/// `TT-002` made every site of compose `TOUCH_TARGET`.
const SIZING_CLASSES: [&str; 6] = [
    "btn-sm",
    "btn-xs",
    "input-sm",
    "input-xs",
    "select-sm",
    "select-xs",
];

/// Every `"..."` string-literal's content span in `source` —
/// `(quote_pos, content_start, content_end)`, where `quote_pos` is the
/// byte offset of the opening `"` and `content_start`/`content_end`
/// bound the text between the quotes. Handles `\"` escapes so an
/// escaped quote can't prematurely close a span; does not handle raw
/// strings, byte strings, or any other Rust string form (module doc's
/// named limit). All three offsets land on `"` or `\` byte positions,
/// both single-byte ASCII, so slicing `source` at them is always a
/// valid UTF-8 boundary regardless of non-ASCII content inside a
/// literal.
fn quoted_string_spans(source: &str) -> Vec<(usize, usize, usize)> {
    let bytes = source.as_bytes();
    let mut spans = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'"' {
            let quote_pos = i;
            let content_start = i + 1;
            i = content_start;
            while i < bytes.len() && bytes[i] != b'"' {
                if bytes[i] == b'\\' && i + 1 < bytes.len() {
                    i += 2;
                } else {
                    i += 1;
                }
            }
            spans.push((quote_pos, content_start, i.min(bytes.len())));
        }
        i += 1;
    }
    spans
}

/// True if the string literal whose opening quote sits at `quote_pos`
/// in `source` is the sole argument of a `grow(...)` call — i.e. the
/// text immediately before the quote, whitespace trimmed, ends with
/// `grow(`. This is `components::grow`'s exact call shape
/// (`class=grow("...")`); `TT-002` never composes a sizing class any
/// other way.
///
/// **`TT-004` round 3** widened this from "immediately preceded by
/// `grow(`" to also accept one branch of an `if`/`else` whose whole
/// value is `grow(...)`'s sole argument --
/// `grow(if cond { "..." } else { "..." })` -- the shape round 3
/// moved every affected `let` binding to, so this guard and
/// [`class_value_identifier_binding_calls_grow`] agree on what
/// "declares a target" means (round 2's review §3: change the code
/// so `grow(` is the outermost call, not the guard's cleverness).
/// Walks backward from `quote_pos` tracking bracket depth (`)`/`}`/`]`
/// open a skip region; `{`/`[` closes one, and is transparent at
/// depth 0 -- an `if`/`else` block boundary says nothing about what
/// encloses it, so the scan continues past it) until it finds the
/// nearest `(` at depth 0, then checks that paren is immediately
/// preceded by `grow`.
fn is_grow_call_argument(source: &str, quote_pos: usize) -> bool {
    let bytes = source.as_bytes();
    let mut depth: i32 = 0;
    let mut i = quote_pos;
    while i > 0 {
        i -= 1;
        match bytes[i] {
            b')' | b'}' | b']' => depth += 1,
            b'(' => {
                if depth == 0 {
                    return source[..i].trim_end().ends_with("grow");
                }
                depth -= 1;
            }
            b'{' | b'[' if depth > 0 => depth -= 1,
            _ => {}
        }
    }
    false
}

/// True if `content` (a string literal's contents) carries one of
/// [`SIZING_CLASSES`] as a whole space-separated token — an HTML
/// `class` attribute's own delimiter, so this needs no word-boundary
/// regex the way free text would: `"btn-small"` splits to one token,
/// `"btn-small"`, which is never equal to `"btn-sm"`.
fn carries_a_sizing_class(content: &str) -> bool {
    content
        .split_whitespace()
        .any(|token| SIZING_CLASSES.contains(&token))
}

#[test]
fn every_sizing_class_site_composes_the_touch_target() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let components_dir = manifest_dir.join("src").join("components");
    let mut files = Vec::new();
    collect_rs_files(&components_dir, &mut files);
    assert!(
        !files.is_empty(),
        "found no .rs files under src/components/ -- the workspace layout \
         assumption this scan depends on may have changed"
    );

    let mut offenders = Vec::new();
    for path in &files {
        let source =
            fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        let stripped = strip_line_comments(&source);
        for (quote_pos, content_start, content_end) in quoted_string_spans(&stripped) {
            let content = &stripped[content_start..content_end];
            if carries_a_sizing_class(content) && !is_grow_call_argument(&stripped, quote_pos) {
                let line = stripped[..quote_pos].matches('\n').count() + 1;
                offenders.push(format!(
                    "{}:{line}: {content:?} carries a sizing class but is not the \
                     argument of a components::grow(...) call",
                    path.display()
                ));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "every sizing-class site in src/components/ must reach a 44px target via \
         components::grow(...) (NFR-A11Y-007, DEC-049 as amended) -- TT-002 already \
         converted every site this scan found on the tree it shipped against, so a \
         new offender means either a new control shipped without grow() or an \
         existing one lost it:\n{}",
        offenders
            .iter()
            .map(|o| format!("  {o}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// True if `content` (a string literal's contents) carries the bare
/// `checkbox` class as a whole space-separated token — the same
/// delimiter reasoning as [`carries_a_sizing_class`]. Does not match
/// `checkbox-xs`/`checkbox-sm` (different tokens entirely); those are
/// out of this scan's scope, see the module doc.
fn carries_the_bare_checkbox_class(content: &str) -> bool {
    content.split_whitespace().any(|token| token == "checkbox")
}

/// True if the string literal whose opening quote sits at `quote_pos`
/// is a `class=` attribute's value — i.e. the text immediately before
/// the quote, whitespace trimmed, ends with `class=`. Needed because
/// `<input type="checkbox" class="checkbox">` carries the literal
/// text `"checkbox"` **twice**: once as `type`'s value, once as
/// `class`'s. [`carries_the_bare_checkbox_class`] matches both
/// (`type="checkbox"`'s content is the single token `"checkbox"` too)
/// -- caught planting a real bare checkbox during round 2, where the
/// unscoped version reported the one offending site twice. Only the
/// `class=` occurrence is the fact this scan cares about.
fn is_class_attribute_value(source: &str, quote_pos: usize) -> bool {
    source[..quote_pos].trim_end().ends_with("class=")
}

/// True if the bare `class="checkbox"` literal whose opening quote
/// sits at `quote_pos` in `source` is enclosed by a
/// `<label class=grow(...)>` wrapper — the sanctioned `Expand`
/// pattern (`DEC-049` as amended, `TT-002` §4): the box stays at its
/// native 24px, the label reaches 44px and participates in layout.
///
/// Finds the nearest `<label` before `quote_pos`, then requires two
/// things: that `<label` must not already be closed by a `</label>`
/// before reaching the checkbox (otherwise it is some earlier,
/// unrelated label and the checkbox is not actually inside it), and
/// that label's own opening tag must contain `class=grow(`.
fn is_label_wrapped_with_grow(source: &str, quote_pos: usize) -> bool {
    let before = &source[..quote_pos];
    let Some(label_start) = before.rfind("<label") else {
        return false;
    };
    if before[label_start..].contains("</label>") {
        return false;
    }
    let after_label = &source[label_start..];
    let Some(tag_end) = after_label.find('>') else {
        return false;
    };
    after_label[..tag_end].contains("class=grow(")
}

#[test]
fn every_bare_checkbox_is_label_wrapped_with_grow() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let components_dir = manifest_dir.join("src").join("components");
    let mut files = Vec::new();
    collect_rs_files(&components_dir, &mut files);
    assert!(
        !files.is_empty(),
        "found no .rs files under src/components/ -- the workspace layout \
         assumption this scan depends on may have changed"
    );

    let mut offenders = Vec::new();
    for path in &files {
        let source =
            fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        let stripped = strip_line_comments(&source);
        for (quote_pos, content_start, content_end) in quoted_string_spans(&stripped) {
            let content = &stripped[content_start..content_end];
            if carries_the_bare_checkbox_class(content)
                && is_class_attribute_value(&stripped, quote_pos)
                && !is_label_wrapped_with_grow(&stripped, quote_pos)
            {
                let line = stripped[..quote_pos].matches('\n').count() + 1;
                offenders.push(format!(
                    "{}:{line}: {content:?} must be enclosed by a \
                     <label class=grow(...)> wrapper -- the box itself stays \
                     24px, only the label reaches 44px",
                    path.display()
                ));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "every bare checkbox in src/components/ must be wrapped in a 44px \
         label (NFR-A11Y-007, DEC-049 as amended) -- TT-002 wrapped the three \
         it found, so a new offender means either a new checkbox shipped \
         unwrapped or an existing one lost its wrap:\n{}",
        offenders
            .iter()
            .map(|o| format!("  {o}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

// ─────────────────────────────────────────────────────────────────
// `TT-004` (`DEC-050`) -- from class-carrying controls to every
// interactive element.
// ─────────────────────────────────────────────────────────────────
//
// `DEC-050` replaces `DEC-049`'s named limit ("plain links are
// unassessed, not passing") with one rule and one declared
// exception: every interactive element must present a 44x44 target,
// and the sole exception is a link inside a block of running text,
// which must say so in the markup. This section is the guard for
// that rule -- everything above this line still guards the
// class-carrying subset `TT-002` covered; this covers the rest.
//
// **The declaration, not the guard, does the padding.** As `DEC-050`
// itself states: this guard proves a declaration (`grow(...)`, or
// the exemption marker) is present. It cannot prove the declaration
// *works* -- `min-h-11` on an element that turns out to render
// `display: inline` would still pass this guard while the rendered
// target stayed under 44px. `TT-004`'s own package measured every
// site this guard covers against a running instance before this
// module shipped; a future site needs the same measurement, not
// just a passing guard.
//
// **The declared exception.** `data-inline-text-link` is the marker
// `TT-004` chose -- present on an interactive element, it means "this
// is the WCAG 2.5.5/2.5.8 inline-link exception, verified by eye
// against the surrounding markup, not by this guard." The guard only
// checks that the marker is *present*, the same standard it holds
// `grow(...)` to.
//
// **`class=IDENT` is followed to its binding, not accepted on the
// name.** `TT-004` round 1 shipped a four-name allowlist here
// (`board_classes`/`list_classes`/`cls`/`class`) that checked only
// which identifier a tag's `class=` used, never what it was bound to
// -- `TT-004-review.md` §2 planted `let cls = "link link-hover";` on
// an otherwise-guarded site and the guard stayed green while the
// rendered target shrank from 44px to 20px. `cls` and `class` are
// among the most ordinary names in this codebase; granting either a
// pass on sight is exactly the shape this module's own `checkbox-xs`
// history warns about -- an allowlist that looks like an audit and
// behaves like an exemption. [`class_value_identifier_binding_calls_grow`]
// replaces it: for `class=IDENT`, the nearest `let IDENT = ...;` in
// the same file must itself call `grow(` somewhere in its
// initialiser. No allowlist to extend on faith -- a fifth
// `class=IDENT` site needs no decision, it only has to route through
// `grow(`.

/// **Two named exclusions, pending `TT-004` §5's escalation, not a
/// hole.** Both are sites this handoff's own package measured,
/// judged unsuitable for a uniform 44px pad, and escalated rather
/// than decided:
///
/// - **Calendar event-chip links** (`calendar.rs::render_block`,
///   `render_day_view`) -- keyed on `bg-primary/1`, a class fragment
///   unique to these two links (`bg-primary/10`, `bg-primary/15`) and
///   not shared by any other control in this crate, including
///   `calendar.rs`'s own already-declared prev/next and
///   Day/Week/Month links. The day view's own block height is
///   duration-proportional (a 15-minute appointment is visibly
///   shorter than a 2-hour one); a 44px floor would misrepresent
///   that, and the month/week view packs several chips into one day
///   cell, where a 44px row would dominate the cell.
/// - **The per-indicator "why" toggle** (`issues.rs::indicator_row`)
///   -- keyed on its exact, unwrapped class literal (the *only*
///   remaining site using this literal without `grow(...)`; its
///   twin at `HealthStrip`'s own "Indicators" toggle already carries
///   one). Up to seven of these chips render side by side in one
///   wrapped row (`HealthStrip`); padding each toggle to 44px would
///   make the disclosure controls, not the health data, the row's
///   dominant visual element.
///
/// Closing either is `TT-004` §5's decision, not this guard's to
/// make -- when it is, delete the matching arm here and the site
/// will need `grow(...)` (or a rework, if the decision goes the
/// other way) to keep passing.
fn is_named_escalation_exclusion(tag: &str) -> bool {
    tag.contains("bg-primary/1")
        || tag.contains("cursor-pointer text-base-content/70 hover:text-base-content")
}

/// The bare identifier in `tag`'s `class=` value, if it has one --
/// `None` if the tag carries no `class=` attribute, or if the value is
/// a quoted literal (`class="..."`) or a direct `class=grow(...)` call
/// rather than a plain variable reference. `class=cls` yields
/// `Some("cls")`; `class="cls"` and `class=grow("cls")` both yield
/// `None`, since neither is a bare identifier this function needs to
/// chase to a binding.
fn bare_class_identifier(tag: &str) -> Option<&str> {
    let pos = tag.find("class=")?;
    let after = &tag[pos + "class=".len()..];
    if after.starts_with('"') || after.starts_with("grow(") {
        return None;
    }
    let end = after
        .find(|c: char| c.is_whitespace() || c == '>')
        .unwrap_or(after.len());
    let ident = &after[..end];
    (!ident.is_empty()).then_some(ident)
}

/// True if `tag`'s `class=` value is a bare identifier (see
/// [`bare_class_identifier`]) and the *nearest* `let` binding of that
/// identifier **before** `tag_start` in `source` has `grow(` as the
/// **outermost call of its initialiser** -- `TT-004-review.md` round 2
/// §2/§3: requiring `grow(` merely to *appear* in the initialiser
/// passed `let cls = if cond { grow("...") } else { "...".to_string() };`,
/// where only one branch actually grows and the other reaches this
/// guard as a plain string. That is not a contrived shape -- it is
/// the one every affected site in this codebase used, since an
/// `if`/`else` choosing between two `grow(...)` calls looks identical
/// to source scanning as one choosing between a `grow(...)` call and
/// a plain literal.
///
/// **The fix changes the code, not the guard's cleverness** (round
/// 2's own review, §3, citing this project's own pattern -- `RFC
/// 006`, `QA-019`, `HLT-001`, `JS-003` -- move the fact to where it
/// can be checked): every affected `let` was restructured so `grow(`
/// wraps the whole conditional rather than sitting inside each
/// branch --
/// `let cls = grow(if is_current { "..." } else { "..." });` -- and
/// this function's rule is now simply **the initialiser, trimmed of
/// leading whitespace, starts with `grow(`**. No bracket-depth
/// tracking, no branch analysis: a `grow(` anywhere else in the
/// initialiser (buried in one arm, or inside a string literal) no
/// longer counts, because it isn't at the start.
///
/// Finds `let {ident}` as a whole word (the character right after the
/// identifier must not itself be an identifier character, so `let
/// cls` doesn't match inside `let clsx`) via `rfind` on the text
/// before `tag_start` -- the nearest preceding binding, not any
/// same-named one anywhere in the file (round 2's own self-caught
/// bug), the same reasoning [`is_label_wrapped_with_grow`] already
/// uses for `<label>` wraps.
fn class_value_identifier_binding_calls_grow(source: &str, tag_start: usize, tag: &str) -> bool {
    let Some(ident) = bare_class_identifier(tag) else {
        return false;
    };
    let needle = format!("let {ident}");
    let before = &source[..tag_start];
    let mut search_end = before.len();
    loop {
        let Some(pos) = before[..search_end].rfind(needle.as_str()) else {
            return false;
        };
        let after_ident = pos + needle.len();
        let is_word_boundary = source[after_ident..]
            .chars()
            .next()
            .is_none_or(|c| !c.is_alphanumeric() && c != '_');
        if is_word_boundary && let Some(eq_offset) = source[after_ident..].find('=') {
            let body_start = after_ident + eq_offset + 1;
            return source[body_start..].trim_start().starts_with("grow(");
        }
        search_end = pos;
    }
}

/// Every `<a`, `<button`, or `<summary` tag's span in `source` --
/// `(tag_start, span_end)`, `span_end` being the position of the
/// first `>` that is not inside a quoted string, scanned forward
/// from the tag's own start. Requires the character right after the
/// tag name to be whitespace or `>`, so `<a` doesn't also match some
/// future `<article`.
///
/// **A named limit, not a parser** -- the same boundary
/// [`quoted_string_spans`] states for itself: a bare (unquoted) `>`
/// inside a `{ }` Rust expression between a tag's own start and its
/// closing `>` (a comparison operator, say) would end the span
/// early. Verified empirically against this crate's actual tree
/// (`TT-004`): no interactive tag's opening carries one today.
fn interactive_tag_spans(source: &str) -> Vec<(usize, usize)> {
    const TAG_NAMES: [&str; 3] = ["a", "button", "summary"];
    let bytes = source.as_bytes();
    let mut spans = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'<' {
            let after_bracket = &source[i + 1..];
            let matched = TAG_NAMES.iter().find(|name| {
                after_bracket.starts_with(**name)
                    && after_bracket[name.len()..]
                        .chars()
                        .next()
                        .is_none_or(|c| c.is_whitespace() || c == '>')
            });
            if let Some(name) = matched {
                let mut j = i + 1 + name.len();
                let mut in_quote = false;
                while j < bytes.len() {
                    match bytes[j] {
                        b'"' => in_quote = !in_quote,
                        b'>' if !in_quote => break,
                        _ => {}
                    }
                    j += 1;
                }
                spans.push((i, j.min(bytes.len())));
                i = j;
                continue;
            }
        }
        i += 1;
    }
    spans
}

#[test]
fn every_interactive_element_declares_a_touch_target() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let components_dir = manifest_dir.join("src").join("components");
    let mut files = Vec::new();
    collect_rs_files(&components_dir, &mut files);
    assert!(
        !files.is_empty(),
        "found no .rs files under src/components/ -- the workspace layout \
         assumption this scan depends on may have changed"
    );

    let mut offenders = Vec::new();
    for path in &files {
        let source =
            fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        let stripped = strip_line_comments(&source);
        for (start, end) in interactive_tag_spans(&stripped) {
            let tag = &stripped[start..end];
            let declares_target = tag.contains("class=grow(")
                || tag.contains("data-inline-text-link")
                || class_value_identifier_binding_calls_grow(&stripped, start, tag)
                || is_named_escalation_exclusion(tag);
            if !declares_target {
                let line = stripped[..start].matches('\n').count() + 1;
                let snippet: String = tag.chars().take(80).collect();
                offenders.push(format!("{}:{line}: {snippet:?}...", path.display()));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "every interactive element (<a>, <button>, <summary>) must declare a \
         44x44 touch target via components::grow(...), or carry \
         data-inline-text-link if it is a link inside a block of running \
         text (NFR-A11Y-007, DEC-050) -- TT-004 brought every site this scan \
         found on the tree it shipped against into one of those two states, \
         so a new offender means either a new control shipped without a \
         declaration or an existing one lost it:\n{}",
        offenders
            .iter()
            .map(|o| format!("  {o}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
