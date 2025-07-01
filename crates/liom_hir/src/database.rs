use arena::Arena;
use liom_syntax::ast::{self, AstToken};

use crate::{BinaryOp, Expr, Stmt, UnaryOp};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Database {
    exprs: Arena<Expr>,
}

impl Database {
    pub const fn new() -> Self {
        Self { exprs: Arena::new() }
    }

    pub fn lower(&mut self, root: &ast::Root) -> impl Iterator<Item = Stmt> {
        root.stmts().filter_map(move |stmt| self.lower_stmt(stmt))
    }

    fn lower_stmt(&mut self, stmt: ast::Stmt) -> Option<Stmt> {
        let stmt = match stmt {
            ast::Stmt::VariableDef(def) => Stmt::VariableDef {
                name: def.name()?.text().into(),
                value: self.lower_expr(def.value()),
            },
            ast::Stmt::Expr(expr) => Stmt::Expr(self.lower_expr(Some(expr))),
        };

        Some(stmt)
    }

    fn lower_expr(&mut self, expr: Option<ast::Expr>) -> Expr {
        expr.map_or(Expr::Missing, |expr| match expr {
            ast::Expr::Literal(literal) => Expr::Literal(literal.value().text().parse().ok()),
            ast::Expr::VariableRef(var) => Expr::VariableRef(var.name().text().into()),
            ast::Expr::Binary(expr) => self.lower_binary_expr(&expr),
            ast::Expr::Unary(expr) => self.lower_unary_expr(&expr),
            ast::Expr::Paren(expr) => self.lower_expr(expr.expr()),
        })
    }

    fn lower_binary_expr(&mut self, expr: &ast::BinaryExpr) -> Expr {
        let op = match expr.op() {
            ast::BinaryOp::Add(_) => BinaryOp::Add,
            ast::BinaryOp::Sub(_) => BinaryOp::Sub,
            ast::BinaryOp::Mul(_) => BinaryOp::Mul,
            ast::BinaryOp::Div(_) => BinaryOp::Div,
        };

        let lhs = self.lower_expr(expr.lhs());
        let rhs = self.lower_expr(expr.rhs());

        Expr::Binary { lhs: self.exprs.alloc(lhs), rhs: self.exprs.alloc(rhs), op }
    }

    fn lower_unary_expr(&mut self, expr: &ast::UnaryExpr) -> Expr {
        let op = match expr.op() {
            ast::UnaryOp::Neg(_) => UnaryOp::Neg,
        };

        let expr = self.lower_expr(expr.expr());

        Expr::Unary { expr: self.exprs.alloc(expr), op }
    }
}
