use liom_node::NodeKind;

use crate::TokenSource;
use crate::parser::{CompletedMarker, Parser};

mod expr;
mod stmt;

impl<T: TokenSource> Parser<'_, T> {
    pub fn root(&mut self) -> CompletedMarker {
        let m = self.start();

        while !self.at_end() {
            self.stmt();
        }

        m.complete(self, NodeKind::Root)
    }
}
