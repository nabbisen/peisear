//! The vocabulary guard: word-boundary-aware, case-insensitive
//! matching of rendered copy against the requirements baseline's
//! §1.7 prohibited-vocabulary list
//! (`.git-exclude/specs/peisear-0.20.0-requirements-en.md`).
//!
//! Deliberately dependency-free — no `regex`. Matching is a manual
//! substring scan with a boundary check on both sides of each hit,
//! which is enough for the closed, ASCII, system-authored copy this
//! crate ever renders (message templates and their closed-set
//! parameter labels — never raw user data, which is out of the
//! guard's scope by design; see the module doc on [`crate`]).
//!
//! ## Transcription of §1.7, and where it isn't a literal 1:1 copy
//!
//! §1.7 lists eight prohibited items. Six transcribe directly as
//! literal phrases. Two needed a judgment call, recorded here rather
//! than silently decided, per the handoff's "report rather than drop"
//! instruction:
//!
//! - **"performance is increasing / decreasing"** is one bullet using
//!   a slash for either direction. Encoded as two literal phrases:
//!   `"performance is increasing"` and `"performance is decreasing"`.
//! - **"you should X", "you must X"** — `X` is a placeholder, not part
//!   of the prohibited text. Encoded as the invariant directive
//!   prefixes `"you should"` and `"you must"`, which is the part
//!   doing the work regardless of what follows.
//! - **"emphasis on completion rate or achievement rate"** is not a
//!   phrase at all — "emphasis" is a framing/tone judgement, not
//!   vocabulary, and the guard "catches vocabulary, not tone" by the
//!   RFC's own stated limit. The two noun phrases it names,
//!   `"completion rate"` and `"achievement rate"`, are encoded
//!   literally, but that does **not** fully capture the bullet: copy
//!   could emphasise a completion percentage without using either
//!   phrase (e.g. a bare `"87%"` presented as an achievement). That
//!   residue needs human review (`FR-HLT-006`), same as tone always
//!   has. Reported, not silently dropped.
//! - **"Failed to update", "Error: outdated version"** (`SPEC
//!   §21.4.6`) are given as the two on-record instances of a broader
//!   failure-framing pattern — 0.20.0's own defect was exactly this
//!   shape (`"Failed to update status. Please refresh."`). Encoded
//!   broader than the two literal examples, as the invariant prefixes
//!   `"failed to"` and `"error:"`, since limiting the guard to the two
//!   exact historical strings would not have caught 0.20.0's own
//!   defect if it had been phrased even slightly differently. This is
//!   a generalisation beyond the literal text — flagged for review
//!   rather than assumed correct.
//!
//! Everything else — `"good progress"`, `"bad pace"`, `"concerning
//! trend"`, `"underperforming"`, `"failing to meet"`, `"velocity"`,
//! `"ranking"`, `"top performer"` — transcribes as a direct literal
//! phrase.
//!
//! ## A known gap: inflection
//!
//! Matching is exact-phrase, word-boundary-aware — `"top performer"`
//! does **not** match `"top performers"` (the trailing `s` fails the
//! boundary check on the far side of the match), and `"ranking"` does
//! not match `"rankings"` or `"ranked"`. §1.7's own text gives the
//! singular/base forms only; this guard transcribes exactly that
//! rather than guessing at which inflections should also be covered.
//! Reported as a limitation, not silently widened.

/// One prohibited term, alongside a short note on how it was
/// transcribed from §1.7 — literal, or a documented judgment call.
pub struct ProhibitedTerm {
    pub phrase: &'static str,
    pub note: &'static str,
}

/// §1.7, transcribed in full. See the module doc above for the two
/// entries that needed a judgment call rather than a literal copy.
pub const PROHIBITED_TERMS: &[ProhibitedTerm] = &[
    ProhibitedTerm {
        phrase: "performance is increasing",
        note: "literal — one half of the increasing/decreasing bullet",
    },
    ProhibitedTerm {
        phrase: "performance is decreasing",
        note: "literal — the other half",
    },
    ProhibitedTerm {
        phrase: "good progress",
        note: "literal",
    },
    ProhibitedTerm {
        phrase: "bad pace",
        note: "literal",
    },
    ProhibitedTerm {
        phrase: "you should",
        note: "directive prefix — the invariant part of \"you should X\"",
    },
    ProhibitedTerm {
        phrase: "you must",
        note: "directive prefix — the invariant part of \"you must X\"",
    },
    ProhibitedTerm {
        phrase: "concerning trend",
        note: "literal",
    },
    ProhibitedTerm {
        phrase: "underperforming",
        note: "literal",
    },
    ProhibitedTerm {
        phrase: "failing to meet",
        note: "literal",
    },
    ProhibitedTerm {
        phrase: "velocity",
        note: "literal — industry term carrying evaluative connotation",
    },
    ProhibitedTerm {
        phrase: "completion rate",
        note: "partial — the noun phrase from \"emphasis on completion rate\"; \
               the \"emphasis\" framing itself is tone, not vocabulary, and is \
               not fully captured by this literal match",
    },
    ProhibitedTerm {
        phrase: "achievement rate",
        note: "partial — same caveat as \"completion rate\"",
    },
    ProhibitedTerm {
        phrase: "ranking",
        note: "literal",
    },
    ProhibitedTerm {
        phrase: "top performer",
        note: "literal",
    },
    ProhibitedTerm {
        phrase: "failed to",
        note: "generalised from the literal example \"Failed to update\" to the \
               invariant failure-framing prefix — see module doc",
    },
    ProhibitedTerm {
        phrase: "error:",
        note: "generalised from the literal example \"Error: outdated version\" \
               to the invariant failure-framing prefix — see module doc",
    },
];

fn is_word_boundary(c: Option<char>) -> bool {
    match c {
        None => true,
        Some(c) => !(c.is_alphanumeric() || c == '_'),
    }
}

/// Returns every prohibited term that appears in `text` as a whole
/// word or phrase — case-insensitive, word-boundary aware. Empty
/// means clean.
pub fn find_violations(text: &str) -> Vec<&'static ProhibitedTerm> {
    let haystack = text.to_lowercase();
    let mut hits = Vec::new();
    for term in PROHIBITED_TERMS {
        let needle = term.phrase; // table entries are already lowercase
        let mut search_from = 0;
        while let Some(rel_idx) = haystack[search_from..].find(needle) {
            let start = search_from + rel_idx;
            let end = start + needle.len();
            let before = haystack[..start].chars().next_back();
            let after = haystack[end..].chars().next();
            if is_word_boundary(before) && is_word_boundary(after) {
                hits.push(term);
                break;
            }
            // Boundary check failed (e.g. matched inside a longer
            // word) — keep scanning past this position for a later,
            // valid occurrence of the same term.
            search_from = start + 1;
        }
    }
    hits
}
