use super::*;

#[test]
fn every_grow_call_site_is_a_form_control_or_already_a_flex_container() {
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
        for (quote_pos, cs, ce) in quoted_string_spans(&stripped) {
            if !is_call_argument(&stripped, quote_pos, "grow") {
                continue;
            }
            let Some((tag, line)) = enclosing_tag(&stripped, quote_pos, "grow") else {
                // Unattributed: either a let-bound `grow(if cond {...}
                // else {...})` branch (the module doc's named limit) or
                // an identifier argument with no quoted literal at all
                // (none remain after DOC-005's fix -- MENU_FOCUS_RING's
                // four sites moved to grow_text).
                continue;
            };
            if FORM_CONTROL_TAGS.contains(&tag.as_str()) {
                continue;
            }
            let content = &stripped[cs..ce];
            if !is_flex_container(content) && !already_establishes_a_block_box(content) {
                offenders.push(format!(
                    "{}:{line}: <{tag}> class={content:?} composes grow(), but is \
                     neither a form control, already a flex container, nor already \
                     a block box (`block`/`card`) -- use grow_text() instead",
                    path.display()
                ));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "every components::grow(...) call site in src/components/ must already be \
         a flex container or a form control (NFR-A11Y-007, DEC-049 as amended by \
         DOC-005 §3.1) -- a plain text element reaching grow() directly, rather \
         than components::grow_text(), reproduces DOC-005 §1's alignment defect:\n{}",
        offenders
            .iter()
            .map(|o| format!("  {o}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn no_grow_text_call_site_is_a_form_control() {
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
        for (quote_pos, _cs, _ce) in quoted_string_spans(&stripped) {
            if !is_call_argument(&stripped, quote_pos, "grow_text") {
                continue;
            }
            if let Some((tag, line)) = enclosing_tag(&stripped, quote_pos, "grow_text")
                && FORM_CONTROL_TAGS.contains(&tag.as_str())
            {
                offenders.push(format!(
                    "{}:{line}: <{tag}> composes grow_text(), which adds \
                     inline-flex unconditionally -- a form control centres its \
                     own content natively and must use grow() instead",
                    path.display()
                ));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "no <input>/<select>/<textarea> in src/components/ may compose \
         components::grow_text(...) (DOC-005 §2/§3.1: inline-flex is meaningless \
         at best on a form control's native centring):\n{}",
        offenders
            .iter()
            .map(|o| format!("  {o}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
