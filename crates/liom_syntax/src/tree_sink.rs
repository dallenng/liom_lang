use liom_lexer::Token;
use liom_parser::{NodeKind, ParseError};
use rowan::{GreenNodeBuilder, Language, TextLen, TextRange};

use crate::{LiomLanguage, Parse, SyntaxError};

#[derive(Debug)]
pub struct TreeSink<'t> {
    text: &'t str,
    tokens: &'t [Token<'t>],
    pos: usize,
    builder: GreenNodeBuilder<'static>,
    errors: Vec<SyntaxError>,
}

impl<'t> liom_parser::TreeSink for TreeSink<'t> {
    fn start_node(&mut self, kind: NodeKind) {
        self.builder.start_node(LiomLanguage::kind_to_raw(kind.into()));
        self.eat_trivia();
    }

    fn finish_node(&mut self) {
        self.builder.finish_node();
    }

    fn token(&mut self) {
        self.do_token();
        self.eat_trivia();
    }

    fn error(&mut self, error: ParseError) {
        let token = self.tokens.get(self.pos);
        let range = token.map_or(TextRange::empty(self.text.text_len()), |t| {
            TextRange::at(t.offset.try_into().unwrap(), t.text.text_len())
        });

        self.errors.push(SyntaxError::new(error, range));
    }
}

impl<'t> TreeSink<'t> {
    pub fn new(text: &'t str, tokens: &'t [Token<'t>]) -> Self {
        Self { text, tokens, pos: 0, builder: GreenNodeBuilder::new(), errors: Vec::new() }
    }

    pub fn finish(self) -> Parse {
        Parse { green_node: self.builder.finish(), errors: self.errors }
    }

    fn eat_trivia(&mut self) {
        while self.tokens.get(self.pos).map_or(false, |t| t.kind.is_trivia()) {
            self.do_token();
        }
    }

    fn do_token(&mut self) {
        let Token { kind, text, .. } = self.tokens[self.pos];
        let kind = LiomLanguage::kind_to_raw(kind.into());

        self.builder.token(kind, text);
        self.pos += 1;
    }
}
