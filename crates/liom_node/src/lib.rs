#![feature(variant_count)]

use std::mem;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum NodeKind {
    Root,
    Literal,
    VariableRef,
    VariableDef,
    InfixExpr,
    PrefixExpr,
    ParenExpr,
    Error,
}

impl NodeKind {
    pub const KIND_MIN: u8 = 0;

    pub const KIND_MAX: u8 = Self::Error as u8;

    pub const KIND_COUNT: usize = mem::variant_count::<Self>();

    pub const KINDS: [Self; Self::KIND_COUNT] = [
        Self::Root,
        Self::Literal,
        Self::VariableRef,
        Self::VariableDef,
        Self::InfixExpr,
        Self::PrefixExpr,
        Self::ParenExpr,
        Self::Error,
    ];
}

impl From<NodeKind> for u8 {
    fn from(kind: NodeKind) -> Self {
        kind as u8
    }
}

impl TryFrom<u8> for NodeKind {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, <Self as TryFrom<u8>>::Error> {
        match value {
            // Safe because `value` is a valid representation of `NodeKind`
            Self::KIND_MIN..=Self::KIND_MAX => Ok(unsafe { mem::transmute::<u8, Self>(value) }),
            _ => Err(()),
        }
    }
}
