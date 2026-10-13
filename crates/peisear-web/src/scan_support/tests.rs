use super::*;

#[test]
fn a_url_string_literal_survives_whole() {
    let source = r#"const NS: &str = "http://www.w3.org/2000/svg";"#;
    let stripped = strip_line_comments(source);
    assert!(
        stripped.contains(r#""http://www.w3.org/2000/svg""#),
        "a // inside a real string literal must not be read as a comment start: {stripped:?}"
    );
}

#[test]
fn a_real_trailing_comment_is_cut() {
    let source = "let x = 1; // a real comment\n";
    let stripped = strip_line_comments(source);
    assert!(
        !stripped.contains("a real comment"),
        "a genuine // line comment must still be stripped: {stripped:?}"
    );
    assert!(stripped.contains("let x = 1;"));
}

#[test]
fn a_double_slash_after_an_escaped_quote_inside_a_string_is_not_cut() {
    let source = r#"let s = "a\" // c";"#;
    let stripped = strip_line_comments(source);
    assert!(
        stripped.contains(r#""a\" // c""#),
        "a // following an escaped quote is still inside the string, not a comment: {stripped:?}"
    );
}

#[test]
fn an_unterminated_quote_does_not_panic_or_swallow_the_next_line() {
    let source = "let s = \"unterminated\nlet y = 2; // real comment\n";
    let stripped = strip_line_comments(source);
    assert!(
        stripped.contains("let y = 2;"),
        "an unterminated quote on one line must not swallow a later line's real \
         content: {stripped:?}"
    );
    assert!(!stripped.contains("real comment"));
}
