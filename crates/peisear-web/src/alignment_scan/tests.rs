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
                     neither a form control, already a flex container, nor already a \
                     block box (`block`/`card`) -- use grow_text() instead \
                     (grow_summary() if <{tag}> is summary -- MARK-002 deleted the \
                     third way grow() alone could be correct on one)",
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

/// `MARK-002` (`061`, `§10.58`) -- the guard this round exists to
/// ship, and the positive property `MARK-001`'s own scan could not
/// state: not "no `<summary>` composes `grow_text(`" (which passed
/// clean on a literal `inline-flex` baked into `grow(...)`'s own
/// argument instead), but **every real `<summary>` composes
/// `grow_summary(...)`, and its own argument carries no display class
/// either** -- `MARK-001`'s own lesson is that a literal `flex`/
/// `inline-flex`/`btn` can ride inside any function's argument
/// regardless of its name, so proving *which* function was called is
/// only half the property. Counts every literal `<summary` tag in the
/// stripped source directly (not only ones reachable backward from a
/// function call, since a bare-string `<summary>` has no call to
/// find), so a `<summary>` that lost its helper call entirely is
/// caught the same as one that kept the wrong one.
#[test]
fn every_summary_composes_grow_summary_with_no_display_class() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let components_dir = manifest_dir.join("src").join("components");
    let mut files = Vec::new();
    collect_rs_files(&components_dir, &mut files);
    assert!(
        !files.is_empty(),
        "found no .rs files under src/components/ -- the workspace layout \
         assumption this scan depends on may have changed"
    );

    let mut wrong_helper = Vec::new();
    let mut display_class = Vec::new();
    let mut total_real_summary_tags = 0usize;
    let mut correct_count = 0usize;
    let mut named_exclusion_count = 0usize;

    for path in &files {
        let source =
            fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        let stripped = strip_line_comments(&source);

        total_real_summary_tags += stripped.matches("<summary").count();
        named_exclusion_count += stripped.matches(NAMED_ESCALATION_EXCLUSION_TAG).count();

        for (quote_pos, cs, ce) in quoted_string_spans(&stripped) {
            for name in ["grow_summary", "grow", "grow_text"] {
                if !is_call_argument(&stripped, quote_pos, name) {
                    continue;
                }
                let Some((tag, line)) = enclosing_tag(&stripped, quote_pos, name) else {
                    continue;
                };
                if tag != "summary" {
                    continue;
                }
                if name != "grow_summary" {
                    wrong_helper.push(format!(
                        "{}:{line}: <summary> composes {name}(...), not \
                         grow_summary(...) -- a <summary> is never a flex \
                         container, and grow_summary() is the only helper that \
                         cannot set display on one (§10.58, MARK-002)",
                        path.display()
                    ));
                    continue;
                }
                correct_count += 1;
                let content = &stripped[cs..ce];
                if is_flex_container(content) {
                    display_class.push(format!(
                        "{}:{line}: grow_summary({content:?}) carries a display \
                         class (flex/inline-flex/btn-family) -- that is exactly \
                         the shape MARK-001's own scan could not see; this \
                         argument needs its layout moved to an inner element \
                         instead (§10.58, MARK-002)",
                        path.display()
                    ));
                }
            }
        }
    }

    assert_eq!(
        named_exclusion_count, 1,
        "expected touch_target_scan's own named escalation exclusion (issues.rs's \
         \"How this is calculated\" toggle) to appear exactly once as the literal \
         tag {NAMED_ESCALATION_EXCLUSION_TAG:?} -- found {named_exclusion_count}. \
         If it moved or its class changed, update NAMED_ESCALATION_EXCLUSION_TAG; \
         if a second bare <summary> appeared, that is a new exception for the \
         architect to decide, not one this constant should absorb silently."
    );

    let accounted_for = correct_count + wrong_helper.len() + named_exclusion_count;
    assert_eq!(
        total_real_summary_tags,
        accounted_for,
        "found {total_real_summary_tags} real <summary> tags in src/components/ \
         but could only account for {accounted_for} ({correct_count} correct, \
         {} wrong-helper, {named_exclusion_count} named exclusion) -- at least \
         one <summary> this scan cannot classify exists (perhaps composing a \
         dynamic class expression, not a quoted literal)",
        wrong_helper.len()
    );

    let offenders: Vec<_> = wrong_helper.into_iter().chain(display_class).collect();
    assert!(
        offenders.is_empty(),
        "every <summary> in src/components/ must compose grow_summary(...) with \
         no display class in its own argument (§10.58, MARK-002) -- it is never \
         a flex container, so nothing on it may set display:\n{}",
        offenders
            .iter()
            .map(|o| format!("  {o}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
