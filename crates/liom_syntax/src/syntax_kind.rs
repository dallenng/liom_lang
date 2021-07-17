use std::convert::TryFrom;
use std::fmt;

use liom_parser::{NodeKind, TokenKind};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SyntaxKind {
    Token(TokenKind),
    Node(NodeKind),
}

impl fmt::Debug for SyntaxKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SyntaxKind::Token(kind) => kind.fmt(f),
            SyntaxKind::Node(kind) => kind.fmt(f),
        }
    }
}

impl From<TokenKind> for SyntaxKind {
    fn from(kind: TokenKind) -> Self {
        Self::Token(kind)
    }
}

impl TryFrom<SyntaxKind> for TokenKind {
    type Error = ();

    fn try_from(value: SyntaxKind) -> Result<Self, <Self as TryFrom<SyntaxKind>>::Error> {
        match value {
            SyntaxKind::Token(kind) => Ok(kind),
            SyntaxKind::Node(_) => Err(()),
        }
    }
}

impl From<NodeKind> for SyntaxKind {
    fn from(kind: NodeKind) -> Self {
        Self::Node(kind)
    }
}

impl TryFrom<SyntaxKind> for NodeKind {
    type Error = ();

    fn try_from(value: SyntaxKind) -> Result<Self, <Self as TryFrom<SyntaxKind>>::Error> {
        match value {
            SyntaxKind::Token(_) => Err(()),
            SyntaxKind::Node(kind) => Ok(kind),
        }
    }
}

impl From<SyntaxKind> for u16 {
    fn from(kind: SyntaxKind) -> Self {
        match kind {
            SyntaxKind::Token(kind) => u16::from(u8::from(kind)),
            SyntaxKind::Node(kind) => u16::from(u8::from(kind)) + u16::from(u8::MAX) + 1,
        }
    }
}

impl TryFrom<u16> for SyntaxKind {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match u8::try_from(value) {
            Ok(value) => TokenKind::try_from(value).map(Self::Token),
            Err(_) => u8::try_from(value - u16::from(u8::MAX) - 1)
                .map_err(|_| ())
                .and_then(NodeKind::try_from)
                .map(Self::Node),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn syntax_kind_to_u16() {
        let mut kinds = HashMap::new();

        for k in TokenKind::KINDS.iter().map(|k| SyntaxKind::from(*k)) {
            assert!(kinds.insert(u16::from(k), k).is_none());
        }

        for k in NodeKind::KINDS.iter().map(|k| SyntaxKind::from(*k)) {
            assert!(kinds.insert(u16::from(k), k).is_none());
        }

        for i in 0..=u16::MAX {
            let kind = SyntaxKind::try_from(i);

            match kinds.get(&i) {
                None => assert!(kind.is_err()),
                Some(k) => assert_eq!(kind, Ok(*k)),
            }
        }
    }
}
