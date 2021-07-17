use super::*;

#[test]
fn parse_variable_definition() {
    check("let foo = bar", &expect![[r#"
Root@0..13
  VariableDef@0..13
    LetKw@0..3 "let"
    Whitespace@3..4 " "
    Ident@4..7 "foo"
    Whitespace@7..8 " "
    Equals@8..9 "="
    Whitespace@9..10 " "
    VariableRef@10..13
      Ident@10..13 "bar""#]]);
}

#[test]
fn parse_recover_on_let_token() {
    check(
        "let a =
let b = a",
        &expect![[r#"
Root@0..17
  VariableDef@0..8
    LetKw@0..3 "let"
    Whitespace@3..4 " "
    Ident@4..5 "a"
    Whitespace@5..6 " "
    Equals@6..7 "="
    Whitespace@7..8 "\n"
  VariableDef@8..17
    LetKw@8..11 "let"
    Whitespace@11..12 " "
    Ident@12..13 "b"
    Whitespace@13..14 " "
    Equals@14..15 "="
    Whitespace@15..16 " "
    VariableRef@16..17
      Ident@16..17 "a"
error at 8..11: expected number, identifier, '-' or '(', but found 'let'"#]],
    );
}
