use liom_syntax::ast::AstNode;

use super::*;

fn parse(text: &str) -> ast::Root {
    ast::Root::cast(liom_syntax::parse(text).syntax()).unwrap()
}

fn check_stmt(text: &str, expected_stmt: &Stmt) {
    let root = parse(text);
    let stmt = root.stmts().next().unwrap();
    let stmt = Database::default().lower_stmt(stmt).unwrap();

    assert_eq!(&stmt, expected_stmt);
}

fn check_expr(text: &str, expected_expr: &Expr, expected_database: &Database) {
    let root = parse(text);
    let stmt = root.stmts().next().unwrap();
    let expr = match stmt {
        ast::Stmt::Expr(expr) => expr,
        ast::Stmt::VariableDef(_) => panic!(),
    };
    let mut database = Database::default();
    let expr = database.lower_expr(Some(expr));

    assert_eq!(&expr, expected_expr);
    assert_eq!(&database, expected_database);
}

#[test]
fn lower_variable_def() {
    check_stmt("let foo = bar", &Stmt::VariableDef {
        name: "foo".into(),
        value: Expr::VariableRef("bar".into()),
    });
}

#[test]
fn lower_variable_def_without_name() {
    let root = parse("let = 10");
    let stmt = root.stmts().next().unwrap();
    assert!(Database::default().lower_stmt(stmt).is_none());
}

#[test]
fn lower_variable_def_without_value() {
    check_stmt("let a =", &Stmt::VariableDef { name: "a".into(), value: Expr::Missing });
}

#[test]
fn lower_expr_stmt() {
    check_stmt("123", &Stmt::Expr(Expr::Literal(Some(123))));
}

#[test]
fn lower_literal() {
    check_expr("999", &Expr::Literal(Some(999)), &Database::default());
}

#[test]
fn lower_variable_ref() {
    check_expr("foo", &Expr::VariableRef("foo".into()), &Database::default());
}

#[test]
fn lower_binary_expr() {
    let mut exprs = Arena::default();
    let lhs = exprs.alloc(Expr::Literal(Some(1)));
    let rhs = exprs.alloc(Expr::Literal(Some(2)));

    check_expr("1 + 2", &Expr::Binary { lhs, rhs, op: BinaryOp::Add }, &Database { exprs });
}

#[test]
fn lower_binary_expr_without_rhs() {
    let mut exprs = Arena::default();
    let lhs = exprs.alloc(Expr::Literal(Some(10)));
    let rhs = exprs.alloc(Expr::Missing);

    check_expr("10 -", &Expr::Binary { lhs, rhs, op: BinaryOp::Sub }, &Database { exprs });
}

#[test]
fn lower_unary_expr() {
    let mut exprs = Arena::default();
    let expr = exprs.alloc(Expr::Literal(Some(10)));

    check_expr("-10", &Expr::Unary { expr, op: UnaryOp::Neg }, &Database { exprs });
}

#[test]
fn lower_unary_expr_without_expr() {
    let mut exprs = Arena::default();
    let expr = exprs.alloc(Expr::Missing);

    check_expr("-", &Expr::Unary { expr, op: UnaryOp::Neg }, &Database { exprs });
}

#[test]
fn lower_paren_expr() {
    check_expr("(((((abc)))))", &Expr::VariableRef("abc".into()), &Database::default());
}
