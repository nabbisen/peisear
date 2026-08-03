//! The shipped locale set.
//!
//! **English only** (`NFR-LANG-005`, `DEC-022`) — do not add a second
//! variant here to ship a second language. The mechanism is proved
//! locale-agnostic by a fixture locale that exists only in this
//! crate's tests, never as a [`Locale`] variant (`I18N-001` §4.5):
//! adding a real second locale here without one actually being
//! scheduled is exactly the drift `RFC 006` open question 4 declines
//! to risk.

use crate::en;
use crate::message::MessageKey;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Locale {
    English,
}

impl Locale {
    /// Render `key` in this locale. Exhaustive over [`Locale`] the
    /// same way `en::render` is exhaustive over [`MessageKey`] — one
    /// arm per shipped locale, no wildcard.
    pub fn render(self, key: MessageKey) -> String {
        match self {
            Locale::English => en::render(key),
        }
    }
}
