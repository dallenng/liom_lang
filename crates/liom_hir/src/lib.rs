use arena::Idx;
use smol_str::SmolStr;

pub mod database;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Stmt {
    VariableDef { name: SmolStr, value: Expr },
    Expr(Expr),
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Expr {
    Literal(Option<u64>),
    VariableRef(SmolStr),
    Binary { lhs: Idx<Self>, rhs: Idx<Self>, op: BinaryOp },
    Unary { expr: Idx<Self>, op: UnaryOp },
    Missing,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum UnaryOp {
    Neg,
}
