use liom_lexer::Token;
use liom_parser::TokenKind;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TokenSource {
    tokens: Vec<TokenKind>,
    current: (Option<TokenKind>, usize),
}

impl liom_parser::TokenSource for TokenSource {
    fn current(&self) -> Option<TokenKind> {
        self.current.0
    }

    fn nth(&self, n: usize) -> Option<TokenKind> {
        token(&self.tokens, self.current.1 + n)
    }

    fn bump(&mut self) {
        if self.current.0.is_some() {
            let pos = self.current.1 + 1;
            self.current = (token(&self.tokens, pos), pos);
        }
    }
}

impl TokenSource {
    pub fn new(tokens: &[Token<'_>]) -> Self {
        let tokens = tokens
            .iter()
            .filter_map(|t| if t.kind.is_trivia() { None } else { Some(t.kind) })
            .collect::<Vec<_>>();

        let current = (token(&tokens, 0), 0);

        Self { tokens, current }
    }
}

fn token(tokens: &[TokenKind], pos: usize) -> Option<TokenKind> {
    tokens.get(pos).copied()
}
