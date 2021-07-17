use super::*;

fn check(text: &str, kind: TokenKind) {
    assert_eq!(lex(text).next(), Some(Token::new(kind, text, 0)));
}

#[test]
fn lex_spaces_and_newlines() {
    check("  \n ", TokenKind::Whitespace);
}

#[test]
fn lex_comment() {
    check("// foo", TokenKind::Comment);
}

#[test]
fn lex_let_keyword() {
    check("let", TokenKind::LetKw);
}

// #[test]
// fn lex_fn_keyword() {
//     check("fn", TokenKind::FnKw);
// }

#[test]
fn lex_alphabetic_identifier() {
    check("ab_cd", TokenKind::Ident);
}

#[test]
fn lex_alphanumeric_identifier() {
    check("ab12_3cde_456", TokenKind::Ident);
}

#[test]
fn lex_mixed_case_identifier() {
    check("AB_Cdef", TokenKind::Ident);
}

#[test]
fn lex_single_char_identifier() {
    check("x", TokenKind::Ident);
}

#[test]
fn lex_underscore_identifier() {
    check("__1", TokenKind::Ident);
}

#[test]
fn lex_number() {
    check("123456", TokenKind::Number);
}

#[test]
fn lex_plus() {
    check("+", TokenKind::Plus);
}

#[test]
fn lex_minus() {
    check("-", TokenKind::Minus);
}

#[test]
fn lex_star() {
    check("*", TokenKind::Star);
}

#[test]
fn lex_slash() {
    check("/", TokenKind::Slash);
}

#[test]
fn lex_equals() {
    check("=", TokenKind::Equals);
}

#[test]
fn lex_left_parenthesis() {
    check("(", TokenKind::LParen);
}

#[test]
fn lex_right_parenthesis() {
    check(")", TokenKind::RParen);
}

#[test]
fn lex_left_brace() {
    check("{", TokenKind::LBrace);
}

#[test]
fn lex_right_brace() {
    check("}", TokenKind::RBrace);
}
