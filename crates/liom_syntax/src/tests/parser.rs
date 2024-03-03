use std::ops::Range;

use expect_test::{expect, Expect};

use super::*;

mod grammar;

fn check(text: &str, expected_tree: &Expect) {
    let parse = parse(text);
    expected_tree.assert_eq(&format!("{parse:#?}"));
}

fn check_error(
    expected: Vec<TokenKind>,
    found: Option<TokenKind>,
    range: Range<usize>,
    output: &str,
) {
    let range = TextRange::new(range.start.try_into().unwrap(), range.end.try_into().unwrap());
    let error = SyntaxError::new(ParseError::new(expected, found), range);

    assert_eq!(format!("{error}"), output);
}

#[test]
fn parse_nothing() {
    check("", &expect![["Root@0..0"]]);
}

#[test]
fn parse_whitespace() {
    check("   ", &expect![[r#"
Root@0..3
  Whitespace@0..3 "   ""#]]);
}

#[test]
fn parse_comment() {
    check("// hello!", &expect![[r#"
Root@0..9
  Comment@0..9 "// hello!""#]]);
}

#[test]
fn one_expected_did_find() {
    check_error(
        vec![TokenKind::Equals],
        Some(TokenKind::Ident),
        10..20,
        "error at 10..20: expected '=', but found identifier",
    );
}

#[test]
fn one_expected_did_not_find() {
    check_error(vec![TokenKind::RParen], None, 5..6, "error at 5..6: expected ')'");
}

#[test]
fn multiple_expected_did_find() {
    check_error(
        vec![TokenKind::Number, TokenKind::Ident, TokenKind::Minus, TokenKind::LParen],
        Some(TokenKind::LetKw),
        100..105,
        "error at 100..105: expected number, identifier, '-' or '(', but found 'let'",
    );
}

#[test]
fn two_expected_did_find() {
    check_error(
        vec![TokenKind::Plus, TokenKind::Minus],
        Some(TokenKind::Equals),
        0..1,
        "error at 0..1: expected '+' or '-', but found '='",
    );
}
