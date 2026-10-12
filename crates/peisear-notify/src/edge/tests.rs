use super::*;

/// `NTF-001`: every kind an edge helper **emits** must be in
/// `kind::all_user_facing()`. That list is what the preferences page
/// renders **and what *Silence all* writes** -- a kind that fires but is
/// missing from it has no preference row for anyone, so the default
/// channels apply and a user who silenced everything still receives it.
/// The list deliberately omits `PROJECT_TREND_DECLINE` today because
/// nothing emits it (its exclusion, with the reason, is in
/// `peisear-core`'s `enumeration_guard`); this is the other half of that
/// bargain, and fails the day an emitter for it is written here.
///
/// **Scope, said plainly:** it reads *this file's own parent* --
/// `edge.rs` is where every `DispatchEvent` is built today -- so an
/// emitter added in another file is not seen.
#[test]
fn every_emitted_kind_is_user_facing() {
    let source = include_str!("../edge.rs");
    let production = &source[..source.find("#[cfg(test)]").expect("the test module marker")];

    let mut emitted: Vec<&str> = Vec::new();
    let mut rest = production;
    while let Some(at) = rest.find("kind_id::") {
        let name: &str = {
            let after = &rest[at + "kind_id::".len()..];
            let end = after
                .find(|c: char| !(c.is_ascii_uppercase() || c == '_'))
                .unwrap_or(after.len());
            &after[..end]
        };
        if !name.is_empty() && !emitted.contains(&name) {
            emitted.push(name);
        }
        rest = &rest[at + "kind_id::".len()..];
    }
    assert!(
        emitted.len() >= 2,
        "found {} emitted kinds in edge.rs; the scan's assumption about how \
         events name their kind may have changed: {emitted:?}",
        emitted.len()
    );

    let user_facing = kind_id::all_user_facing();
    for name in emitted {
        let value = match name {
            "BURNOUT_OVERLOAD" => kind_id::BURNOUT_OVERLOAD,
            "BURNOUT_STALLED" => kind_id::BURNOUT_STALLED,
            "PROJECT_TREND_DECLINE" => kind_id::PROJECT_TREND_DECLINE,
            other => panic!(
                "edge.rs emits kind::{other}, which this guard does not know: add it \
                 here, and make sure it is in kind::all_user_facing()"
            ),
        };
        assert!(
            user_facing.contains(&value),
            "edge.rs emits {name} ({value:?}) but kind::all_user_facing() does not \
             list it: the preferences page will not offer it and *Silence all* will \
             not reach it. Add it to all_user_facing() (and remove its exclusion in \
             peisear-core's enumeration_guard)"
        );
    }
}
