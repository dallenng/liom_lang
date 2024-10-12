use liom_node::NodeKind;
use liom_token::TokenKind;

use crate::TokenSource;
use crate::parser::{CompletedMarker, Parser};

impl<T: TokenSource> Parser<'_, T> {
    pub(super) fn stmt(&mut self) -> Option<CompletedMarker> {
        if self.at(TokenKind::LetKw) { Some(self.variable_def()) } else { self.expr() }
    }

    fn variable_def(&mut self) -> CompletedMarker {
        assert!(self.at(TokenKind::LetKw));
        let m = self.start();
        self.bump();

        self.expect(TokenKind::Ident);
        self.expect(TokenKind::Equals);

        self.expr();

        m.complete(self, NodeKind::VariableDef)
    }
}
