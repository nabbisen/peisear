use super::*;

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
    for path in files.iter().filter(|p| {
        !p.components().any(|c| {
            c.as_os_str()
                .to_str()
                .is_some_and(|s| s.starts_with("touch_target_scan"))
        })
    }) {
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
            if is_hidden_input(tag) {
                continue;
            }
            let declares_target = tag.contains("class=grow(")
                || tag.contains("class=grow_text(")
                || tag.contains("data-inline-text-link")
                || class_value_identifier_binding_calls_grow(&stripped, start, tag)
                || is_named_escalation_exclusion(tag)
                || is_label_wrapped_with_grow(&stripped, start);
            if !declares_target {
                let line = stripped[..start].matches('\n').count() + 1;
                let snippet: String = tag.chars().take(80).collect();
                offenders.push(format!("{}:{line}: {snippet:?}...", path.display()));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "every interactive element (<a>, <button>, <summary>, <input> other \
         than type=\"hidden\", <select>, <textarea>) must declare a 44x44 \
         touch target via components::grow(...) or components::grow_text(...), \
         carry data-inline-text-link if it is a link inside a block of running \
         text, or sit inside a <label class=grow(...)> that provides the \
         hit area (NFR-A11Y-007, DEC-050) -- TT-004 and TT-007 brought every \
         site this scan found on the tree they shipped against into one of \
         those states, so a new offender means either a new control shipped \
         without a declaration or an existing one lost it:\n{}",
        offenders
            .iter()
            .map(|o| format!("  {o}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
