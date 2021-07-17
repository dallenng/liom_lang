use super::*;

mod expr;
mod stmt;

#[test]
fn parse_multiple_statements() {
    check(
        "let a = 1
a",
        &expect![[r#"
Root@0..11
  VariableDef@0..10
    LetKw@0..3 "let"
    Whitespace@3..4 " "
    Ident@4..5 "a"
    Whitespace@5..6 " "
    Equals@6..7 "="
    Whitespace@7..8 " "
    Literal@8..10
      Number@8..9 "1"
      Whitespace@9..10 "\n"
  VariableRef@10..11
    Ident@10..11 "a""#]],
    );
}
