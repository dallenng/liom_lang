use std::iter::FusedIterator;

use liom_token::TokenKind;

use crate::matcher::Matcher;

mod matcher;
#[cfg(test)]
mod tests;

pub fn lex(text: &str) -> Lexer<'_> {
    Lexer::new(text)
}

#[derive(Debug, Clone)]
pub struct Lexer<'s> {
    text: &'s str,
    matcher: Matcher,
    pos: usize,
}

impl<'s> Lexer<'s> {
    fn new(text: &'s str) -> Self {
        Self { text, matcher: Matcher::new(), pos: 0 }
    }

    fn next_token(&mut self) -> Option<Token<'s>> {
        if self.pos == self.text.len() {
            return None;
        }

        let start_pos = self.pos;
        let kind = if let Some((kind, len)) = self.matcher.find(&self.text[start_pos..]) {
            self.pos += len;
            kind
        } else {
            self.pos += 1;
            while !self.text.is_char_boundary(self.pos) {
                self.pos += 1;
            }

            let text_len = self.text.len();
            while self.pos < text_len && !self.matcher.is_match(&self.text[self.pos..]) {
                self.pos += 1;
                while !self.text.is_char_boundary(self.pos) {
                    self.pos += 1;
                }
            }

            TokenKind::Error
        };

        Some(Token::new(kind, &self.text[start_pos..self.pos], start_pos))
    }
}

impl<'s> Iterator for Lexer<'s> {
    type Item = Token<'s>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

impl<'s> FusedIterator for Lexer<'s> {}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Token<'s> {
    pub kind: TokenKind,
    pub text: &'s str,
    pub offset: usize,
}

impl<'s> Token<'s> {
    const fn new(kind: TokenKind, text: &'s str, offset: usize) -> Self {
        Self { kind, text, offset }
    }
}
