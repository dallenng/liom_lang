use liom_node::NodeKind;

use crate::parser::{CompletedMarker, Parser};
use crate::TokenSource;

mod expr;
mod stmt;

impl<'t, T: TokenSource> Parser<'t, T> {
    pub fn root(&mut self) -> CompletedMarker {
        let m = self.start();

        while !self.at_end() {
            self.stmt();
        }

        m.complete(self, NodeKind::Root)
    }
}
