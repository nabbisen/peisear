use super::*;

#[test]
fn escape_handles_meta_characters() {
    assert_eq!(escape_like_meta("100%"), "100\\%");
    assert_eq!(escape_like_meta("under_score"), "under\\_score");
    assert_eq!(escape_like_meta("a\\b"), "a\\\\b");
    // Backslash must come first; otherwise we'd
    // double-escape the introducer.
    assert_eq!(escape_like_meta("a%b"), "a\\%b");
}

#[test]
fn escape_passes_normal_text_through() {
    assert_eq!(escape_like_meta("login error"), "login error");
    assert_eq!(escape_like_meta("ログイン"), "ログイン");
}
