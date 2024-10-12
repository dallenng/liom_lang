use liom_token::TokenKind;
use regex_automata::{DFA, DenseDFA};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[derive(Debug, Clone)]
pub struct Matcher {
    rules: [Rule; GENERATED_DFA.len()],
}

impl Matcher {
    pub fn new() -> Self {
        Self {
            rules: GENERATED_DFA.map(|(kind, dfa_bytes)| {
                // Safe because we are using bytes generated by a `DenseDFA`.
                let dfa = unsafe { DenseDFA::from_bytes(dfa_bytes) };
                Rule::new(kind, dfa)
            }),
        }
    }

    pub fn is_match(&self, text: &str) -> bool {
        self.rules.iter().any(|r| r.is_match(text))
    }

    pub fn find(&self, text: &str) -> Option<(TokenKind, usize)> {
        // max returns the last max so we reverse iteration to get the first max.
        self.rules.iter().rev().filter_map(|r| r.find(text)).max_by(|a, b| a.1.cmp(&b.1))
    }
}

#[derive(Debug, Clone)]
struct Rule {
    kind: TokenKind,
    dfa: DenseDFA<&'static [StateID], StateID>,
}

impl Rule {
    const fn new(kind: TokenKind, dfa: DenseDFA<&'static [StateID], StateID>) -> Self {
        Self { kind, dfa }
    }

    fn is_match(&self, text: &str) -> bool {
        self.dfa.is_match(text.as_ref())
    }

    fn find(&self, text: &str) -> Option<(TokenKind, usize)> {
        self.dfa.find(text.as_ref()).map(|len| (self.kind, len))
    }
}
