use std::convert::{TryFrom, TryInto};
use std::fmt;

use liom_parser::{NodeKind, ParseError, TokenKind};
use rowan::{GreenNode, TextRange};

use crate::syntax_kind::SyntaxKind;
use crate::token_source::TokenSource;
use crate::tree_sink::TreeSink;

pub mod ast;
pub mod syntax_kind;
#[cfg(test)]
mod tests;
mod token_source;
mod tree_sink;
pub mod validation;

pub type SyntaxNode = rowan::SyntaxNode<LiomLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<LiomLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<LiomLanguage>;

pub trait SyntaxExt {
    type Kind;

    fn kind_ext(&self) -> Self::Kind;
}

impl SyntaxExt for SyntaxNode {
    type Kind = NodeKind;

    fn kind_ext(&self) -> Self::Kind {
        self.kind().try_into().unwrap()
    }
}

impl SyntaxExt for SyntaxToken {
    type Kind = TokenKind;

    fn kind_ext(&self) -> Self::Kind {
        self.kind().try_into().unwrap()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct LiomLanguage;

impl rowan::Language for LiomLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        Self::Kind::try_from(raw.0).expect("unexpected raw SyntaxKind")
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.into())
    }
}

pub fn parse(text: &str) -> Parse {
    let tokens = liom_lexer::lex(text).collect::<Vec<_>>();

    let mut token_source = TokenSource::new(&tokens);
    let mut tree_sink = TreeSink::new(text, &tokens);
    liom_parser::parse(&mut token_source, &mut tree_sink);

    tree_sink.finish()
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Parse {
    green_node: GreenNode,
    errors: Vec<SyntaxError>,
}

impl Parse {
    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }
}

impl fmt::Debug for Parse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let syntax_node = self.syntax();

        if f.alternate() {
            if let Some((last, errors)) = self.errors.split_last() {
                write!(f, "{syntax_node:#?}")?;
                for err in errors {
                    writeln!(f, "{err}")?;
                }
                write!(f, "{last}")
            } else {
                let formatted = format!("{syntax_node:#?}");
                write!(f, "{}", &formatted[..formatted.len() - 1])
            }
        } else {
            write!(f, "{syntax_node:?}")
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct SyntaxError {
    parse_error: ParseError,
    range: TextRange,
}

impl SyntaxError {
    const fn new(parse_error: ParseError, range: TextRange) -> Self {
        SyntaxError { parse_error, range }
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error at {:?}: {}", self.range, self.parse_error)
    }
}
