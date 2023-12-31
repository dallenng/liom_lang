use std::fmt;

pub use liom_node::NodeKind;
pub use liom_token::TokenKind;

use crate::parser::Parser;

mod event;
mod grammar;
mod parser;

pub fn parse(token_source: &mut impl TokenSource, tree_sink: &mut impl TreeSink) {
    let parser = Parser::new(token_source);
    let events = parser.parse();

    event::process(events, tree_sink);
}

pub trait TokenSource {
    fn current(&self) -> Option<TokenKind>;

    fn nth(&self, n: usize) -> Option<TokenKind>;

    fn bump(&mut self);
}

pub trait TreeSink {
    fn start_node(&mut self, kind: NodeKind);

    fn finish_node(&mut self);

    fn token(&mut self);

    fn error(&mut self, error: ParseError);
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ParseError {
    expected: Vec<TokenKind>,
    found: Option<TokenKind>,
}

impl ParseError {
    pub const fn new(expected: Vec<TokenKind>, found: Option<TokenKind>) -> Self {
        ParseError { expected, found }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some((first, expected)) = self.expected.split_first() {
            write!(f, "expected {first}")?;
            if let Some((last, expected)) = expected.split_last() {
                for kind in expected {
                    write!(f, ", {kind}")?;
                }
                write!(f, " or {last}")?;
            }
        }

        if let Some(found) = self.found {
            if self.expected.is_empty() {
                write!(f, "found {found}")?;
            } else {
                write!(f, ", but found {found}")?;
            }
        }

        Ok(())
    }
}
