//! `FLOW-002` (unit two, `059`) — two rules, both about a reader who
//! has only the word on the control, or only the title bar, to go on.
//!
//! **Rule 1: every page title names the product.** Applied once, in
//! [`crate::components::layout::with_product_suffix`], which
//! [`crate::components::layout::Base`] calls on whatever title it is
//! given — never stored per-string in `peisear-i18n`. Five
//! `MessageKey` strings (`ErrorPageTitle`, `ProjectsListPageTitle`,
//! `ProjectNewPageTitle`, `ProjectEditPageTitle`,
//! `ProjectDetailPageTitle`) and two more (`LoginPageTitle`,
//! `RegisterPageTitle`) had already drifted into carrying the suffix
//! by hand — all seven are now bare, so the central helper doesn't
//! double it.
//!
//! **Why a helper and not "every page-title string ends with the
//! suffix," the shape this handoff's own acceptance names first**: a
//! real population of title call sites includes values that are not
//! `MessageKey`s at all — `team_name.clone()`, `sprint_name.clone()`,
//! a confirmation page's own `heading.clone()` — so no finite
//! `MessageKey` census could be the actual population without quietly
//! mis-stating it as complete. A central helper makes the rule true by
//! construction for every caller, `MessageKey` or not, which is the
//! stronger guarantee the handoff's own "built it once" instruction
//! was asking for.
//!
//! [`the_helper_always_appends_the_suffix`] pins the mechanism directly
//! over representative inputs. [`no_message_key_already_carries_the_suffix`]
//! is the regression guard for the other direction — the double-suffix
//! this round's own refactor could reintroduce — walked over
//! `MessageKey::all()`'s entire population (asserted, not implied:
//! `§10.36`).
//!
//! **Rule 2: the word on the door is the name inside.** For four of
//! `FR-NAV-001`'s five entries, the primary nav's own word and the
//! destination the reader actually lands on must answer to the same
//! name. **That property is checked against the real rendered page,
//! in `tests/nav_destination_naming.rs`, not here**: an early version
//! of this module compared two `MessageKey` constants I chose myself
//! for each pair, which is exactly "two literals that happen to match
//! today" one level removed — reverting `notifications.rs`'s own fix
//! during this round's own plant left that version passing, because
//! it never read `notifications.rs` at all. The live `<title>` is the
//! oracle now, not a pairing table I maintain by hand.
//!
//! `Today` was the one pair excluded from that integration test and
//! pinned here instead, as a pure `MessageKey` comparison asserting
//! the two render *differently* — `components::me::PersonalDashboardTitle`
//! rendered no issue at all, so the content decision the rename waited
//! on hadn't landed. **`NAME-001` (`060`) closes that gap, the other
//! way than the pending test expected**: not by giving `/today` new
//! content, but by renaming the nav word itself to `Me`, which
//! `PersonalDashboardTitle` already answers to. The pending test is
//! deleted in this same change, not left to fail — `nav_destination_naming`
//! now covers `/today` as its fifth live-rendered case instead.

use peisear_i18n::{Locale, MessageKey};

use crate::components::layout::with_product_suffix;

#[cfg(test)]
mod tests {
    use super::*;

    fn brand_suffix() -> String {
        format!(" — {}", Locale::English.render(MessageKey::AppBrandName))
    }

    #[test]
    fn the_helper_always_appends_the_suffix() {
        let suffix = brand_suffix();
        for input in [
            "",
            "Projects",
            "Edit Customer Portal",
            "a title — with its own dash already",
        ] {
            let out = with_product_suffix(input);
            assert!(
                out.ends_with(&suffix),
                "with_product_suffix({input:?}) = {out:?} does not end with {suffix:?}"
            );
        }
    }

    /// `§10.36`: the population is `MessageKey::all()` in full — every
    /// variant this crate defines, with representative payloads — not
    /// a hand-picked subset of "the title-shaped ones," which could
    /// silently miss one.
    #[test]
    fn no_message_key_already_carries_the_suffix() {
        let suffix = brand_suffix();
        let keys = MessageKey::all();
        assert_eq!(
            keys.len(),
            715,
            "MessageKey::all()'s own population moved -- re-measure before trusting this guard \
             examined anything"
        );
        let offenders: Vec<String> = keys
            .into_iter()
            .map(|k| Locale::English.render(k))
            .filter(|text| text.ends_with(&suffix))
            .collect();
        assert!(
            offenders.is_empty(),
            "these rendered strings already carry the product suffix -- Base applies it once \
             centrally, so a MessageKey string carrying it too would double it: {offenders:?}"
        );
    }
}
