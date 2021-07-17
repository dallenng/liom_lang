use liom_node::NodeKind;
use liom_token::TokenKind;

use crate::parser::{CompletedMarker, Parser};
use crate::TokenSource;

impl<'t, T: TokenSource> Parser<'t, T> {
    pub(super) fn expr(&mut self) -> Option<CompletedMarker> {
        self.expr_binding_power(0)
    }

    fn expr_binding_power(&mut self, min_binding_power: u8) -> Option<CompletedMarker> {
        let mut lhs = self.lhs()?;

        loop {
            let op = if self.at(TokenKind::Plus) {
                BinaryOp::Add
            } else if self.at(TokenKind::Minus) {
                BinaryOp::Sub
            } else if self.at(TokenKind::Star) {
                BinaryOp::Mul
            } else if self.at(TokenKind::Slash) {
                BinaryOp::Div
            } else {
                break;
            };

            let (left_binding_power, right_binding_power) = op.binding_power();

            if left_binding_power < min_binding_power {
                break;
            }

            self.bump();

            let m = lhs.precede(self);
            let rhs = self.expr_binding_power(right_binding_power);
            lhs = m.complete(self, NodeKind::InfixExpr);

            if rhs.is_none() {
                break;
            }
        }

        Some(lhs)
    }

    fn lhs(&mut self) -> Option<CompletedMarker> {
        if self.at(TokenKind::Number) {
            Some(self.literal())
        } else if self.at(TokenKind::Ident) {
            Some(self.variable_ref())
        } else if self.at(TokenKind::Minus) {
            Some(self.prefix_expr())
        } else if self.at(TokenKind::LParen) {
            Some(self.paren_expr())
        } else {
            self.error();
            None
        }
    }

    fn literal(&mut self) -> CompletedMarker {
        assert!(self.at(TokenKind::Number));

        let m = self.start();
        self.bump();
        m.complete(self, NodeKind::Literal)
    }

    fn variable_ref(&mut self) -> CompletedMarker {
        assert!(self.at(TokenKind::Ident));

        let m = self.start();
        self.bump();
        m.complete(self, NodeKind::VariableRef)
    }

    fn prefix_expr(&mut self) -> CompletedMarker {
        assert!(self.at(TokenKind::Minus));

        let m = self.start();

        let op = UnaryOp::Neg;
        let (_, right_binding_power) = op.binding_power();

        self.bump();

        self.expr_binding_power(right_binding_power);

        m.complete(self, NodeKind::PrefixExpr)
    }

    fn paren_expr(&mut self) -> CompletedMarker {
        assert!(self.at(TokenKind::LParen));

        let m = self.start();

        self.bump();
        self.expr_binding_power(0);
        self.expect(TokenKind::RParen);

        m.complete(self, NodeKind::ParenExpr)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl BinaryOp {
    fn binding_power(self) -> (u8, u8) {
        match self {
            BinaryOp::Add | BinaryOp::Sub => (1, 2),
            BinaryOp::Mul | BinaryOp::Div => (3, 4),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum UnaryOp {
    Neg,
}

impl UnaryOp {
    fn binding_power(self) -> ((), u8) {
        match self {
            UnaryOp::Neg => ((), 5),
        }
    }
}
