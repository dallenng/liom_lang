use liom_parser::{NodeKind, TokenKind};

use crate::{SyntaxElement, SyntaxExt, SyntaxNode, SyntaxToken};

macro_rules! cast {
    ($s:ident) => {
        if Self::can_cast($s.kind_ext()) {
            Some(Self($s))
        } else {
            None
        }
    };
}

macro_rules! node {
    ($k:ident) => {
        node!($k, $k);
    };
    ($k:ident, $t:ident) => {
        #[derive(Debug, Clone, Eq, PartialEq, Hash)]
        pub struct $t(SyntaxNode);

        impl AstNode for $t {
            fn can_cast(kind: NodeKind) -> bool {
                kind == NodeKind::$k
            }

            fn cast(syntax: SyntaxNode) -> Option<Self>
            where
                Self: Sized,
            {
                cast!(syntax)
            }

            fn syntax(&self) -> &SyntaxNode {
                &self.0
            }
        }
    };
}

macro_rules! token {
    ($k:ident) => {
        token!($k, $k);
    };
    ($k:ident, $t:ident) => {
        #[derive(Debug, Clone, Eq, PartialEq, Hash)]
        pub struct $t(SyntaxToken);

        impl AstToken for $t {
            fn can_cast(kind: TokenKind) -> bool {
                kind == TokenKind::$k
            }

            fn cast(syntax: SyntaxToken) -> Option<Self>
            where
                Self: Sized,
            {
                cast!(syntax)
            }

            fn syntax(&self) -> &SyntaxToken {
                &self.0
            }
        }
    };
}

pub trait AstNode {
    fn can_cast(kind: NodeKind) -> bool;

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> &SyntaxNode;

    fn kind(&self) -> NodeKind {
        self.syntax().kind_ext()
    }
}

pub trait AstToken {
    fn can_cast(kind: TokenKind) -> bool;

    fn cast(syntax: SyntaxToken) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> &SyntaxToken;

    fn kind(&self) -> TokenKind {
        self.syntax().kind_ext()
    }

    fn text(&self) -> &str {
        self.syntax().text()
    }
}

token! {Ident}
token! {Number}
token! {Plus}
token! {Minus}
token! {Star}
token! {Slash}
token! {LParen}
token! {RParen}

node! {Root}

impl Root {
    pub fn stmts(&self) -> impl Iterator<Item = Stmt> {
        self.syntax().children().filter_map(Stmt::cast)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Stmt {
    VariableDef(VariableDef),
    Expr(Expr),
}

impl AstNode for Stmt {
    fn can_cast(kind: NodeKind) -> bool {
        match kind {
            NodeKind::VariableDef => true,
            _ => Expr::can_cast(kind),
        }
    }

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        match syntax.kind_ext() {
            NodeKind::VariableDef => Some(Self::VariableDef(VariableDef(syntax))),
            _ => Some(Self::Expr(Expr::cast(syntax)?)),
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            Stmt::VariableDef(inner) => inner.syntax(),
            Stmt::Expr(inner) => inner.syntax(),
        }
    }
}

node! {VariableDef}

impl VariableDef {
    pub fn name(&self) -> Option<Ident> {
        self.syntax()
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find_map(Ident::cast)
    }

    pub fn value(&self) -> Option<Expr> {
        self.syntax().children().find_map(Expr::cast)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Expr {
    Literal(Literal),
    VariableRef(VariableRef),
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Paren(ParenExpr),
}

impl AstNode for Expr {
    fn can_cast(kind: NodeKind) -> bool {
        matches!(
            kind,
            NodeKind::Literal
                | NodeKind::VariableRef
                | NodeKind::InfixExpr
                | NodeKind::PrefixExpr
                | NodeKind::ParenExpr
        )
    }

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        match syntax.kind_ext() {
            NodeKind::Literal => Some(Self::Literal(Literal(syntax))),
            NodeKind::VariableRef => Some(Self::VariableRef(VariableRef(syntax))),
            NodeKind::InfixExpr => Some(Self::Binary(BinaryExpr(syntax))),
            NodeKind::PrefixExpr => Some(Self::Unary(UnaryExpr(syntax))),
            NodeKind::ParenExpr => Some(Self::Paren(ParenExpr(syntax))),
            _ => None,
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            Expr::Literal(inner) => inner.syntax(),
            Expr::VariableRef(inner) => inner.syntax(),
            Expr::Binary(inner) => inner.syntax(),
            Expr::Unary(inner) => inner.syntax(),
            Expr::Paren(inner) => inner.syntax(),
        }
    }
}

node! {Literal}

impl Literal {
    pub fn value(&self) -> Number {
        match self.syntax().first_token().and_then(Number::cast) {
            Some(number) => number,
            None => unreachable!(),
        }
    }
}

node! {VariableRef}

impl VariableRef {
    pub fn name(&self) -> Ident {
        match self.syntax().first_token().and_then(Ident::cast) {
            Some(ident) => ident,
            None => unreachable!(),
        }
    }
}

node! {InfixExpr, BinaryExpr}

impl BinaryExpr {
    pub fn lhs(&self) -> Option<Expr> {
        self.syntax().children().find_map(Expr::cast)
    }

    pub fn rhs(&self) -> Option<Expr> {
        self.syntax().children().filter_map(Expr::cast).nth(1)
    }

    pub fn op(&self) -> BinaryOp {
        match self
            .syntax()
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find_map(BinaryOp::cast)
        {
            Some(op) => op,
            None => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum BinaryOp {
    Add(Plus),
    Sub(Minus),
    Mul(Star),
    Div(Slash),
}

impl AstToken for BinaryOp {
    fn can_cast(kind: TokenKind) -> bool {
        matches!(
            kind,
            TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Slash
        )
    }

    fn cast(syntax: SyntaxToken) -> Option<Self>
    where
        Self: Sized,
    {
        match syntax.kind_ext() {
            TokenKind::Plus => Some(Self::Add(Plus(syntax))),
            TokenKind::Minus => Some(Self::Sub(Minus(syntax))),
            TokenKind::Star => Some(Self::Mul(Star(syntax))),
            TokenKind::Slash => Some(Self::Div(Slash(syntax))),
            _ => None,
        }
    }

    fn syntax(&self) -> &SyntaxToken {
        match self {
            BinaryOp::Add(inner) => inner.syntax(),
            BinaryOp::Sub(inner) => inner.syntax(),
            BinaryOp::Mul(inner) => inner.syntax(),
            BinaryOp::Div(inner) => inner.syntax(),
        }
    }
}

node! {PrefixExpr, UnaryExpr}

impl UnaryExpr {
    pub fn expr(&self) -> Option<Expr> {
        self.syntax().children().find_map(Expr::cast)
    }

    pub fn op(&self) -> UnaryOp {
        match self
            .syntax()
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find_map(UnaryOp::cast)
        {
            Some(op) => op,
            None => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum UnaryOp {
    Neg(Minus),
}

impl AstToken for UnaryOp {
    fn can_cast(kind: TokenKind) -> bool {
        matches!(kind, TokenKind::Minus)
    }

    fn cast(syntax: SyntaxToken) -> Option<Self>
    where
        Self: Sized,
    {
        match syntax.kind_ext() {
            TokenKind::Minus => Some(Self::Neg(Minus(syntax))),
            _ => None,
        }
    }

    fn syntax(&self) -> &SyntaxToken {
        match self {
            UnaryOp::Neg(inner) => inner.syntax(),
        }
    }
}

node! {ParenExpr}

impl ParenExpr {
    pub fn expr(&self) -> Option<Expr> {
        self.syntax().children().find_map(Expr::cast)
    }
}
