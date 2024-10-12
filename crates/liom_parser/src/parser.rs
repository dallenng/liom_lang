use std::mem;

use liom_node::NodeKind;
use liom_token::TokenKind;

use crate::event::Event;
use crate::{ParseError, TokenSource};

const RECOVERY_SET: &[TokenKind] = &[TokenKind::LetKw];

pub struct Parser<'t, T> {
    source: &'t mut T,
    events: Vec<Event>,
    expected_kinds: Vec<TokenKind>,
}

impl<'t, T> Parser<'t, T> {
    pub fn new(source: &'t mut T) -> Self {
        Self { source, events: Vec::new(), expected_kinds: Vec::new() }
    }

    pub fn start(&mut self) -> Marker {
        let pos = self.events.len();
        self.events.push(Event::Placeholder);

        Marker::new(pos)
    }
}

impl<T: TokenSource> Parser<'_, T> {
    pub fn parse(mut self) -> Vec<Event> {
        self.root();
        self.events
    }

    pub fn expect(&mut self, kind: TokenKind) {
        if self.at(kind) {
            self.bump();
        } else {
            self.error();
        }
    }

    pub fn error(&mut self) {
        let found = self.current();

        self.events.push(Event::Error(ParseError::new(mem::take(&mut self.expected_kinds), found)));

        if !self.at_set(RECOVERY_SET) && !self.at_end() {
            let m = self.start();
            self.bump();
            m.complete(self, NodeKind::Error);
        }
    }

    pub fn bump(&mut self) {
        self.expected_kinds.clear();
        self.source.bump();
        self.events.push(Event::Token);
    }

    pub fn at(&mut self, kind: TokenKind) -> bool {
        self.expected_kinds.push(kind);
        self.current() == Some(kind)
    }

    pub fn at_end(&self) -> bool {
        self.current().is_none()
    }

    fn at_set(&self, set: &[TokenKind]) -> bool {
        self.current().map_or(false, |k| set.contains(&k))
    }

    fn current(&self) -> Option<TokenKind> {
        self.source.current()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Marker {
    pos: usize,
    completed: bool,
}

impl Marker {
    const fn new(pos: usize) -> Self {
        Self { pos, completed: false }
    }

    pub fn complete<T>(mut self, parser: &mut Parser<'_, T>, kind: NodeKind) -> CompletedMarker {
        self.completed = true;

        let event = &mut parser.events[self.pos];
        assert_eq!(*event, Event::Placeholder);

        *event = Event::StartNode { kind, forward_parent: None };

        parser.events.push(Event::FinishNode);

        CompletedMarker { pos: self.pos }
    }
}

impl Drop for Marker {
    fn drop(&mut self) {
        assert!(self.completed || std::thread::panicking(), "Marker need to be completed");
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CompletedMarker {
    pos: usize,
}

impl CompletedMarker {
    pub fn precede<T>(self, parser: &mut Parser<'_, T>) -> Marker {
        let m = parser.start();

        match &mut parser.events[self.pos] {
            Event::StartNode { forward_parent, .. } => {
                *forward_parent = Some(m.pos - self.pos);
            }
            _ => {
                unreachable!();
            }
        }

        m
    }
}
