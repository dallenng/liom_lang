#![feature(variant_count)]

use std::{fmt, mem};

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TokenKind {
    Whitespace,
    Comment,
    LetKw,
    Ident,
    Number,
    Plus,
    Minus,
    Star,
    Slash,
    Equals,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Error,
}

impl TokenKind {
    pub const KIND_MIN: u8 = 0;

    pub const KIND_MAX: u8 = Self::Error as u8;

    pub const KIND_COUNT: usize = mem::variant_count::<Self>();

    pub const KINDS: [Self; Self::KIND_COUNT] = [
        Self::Whitespace,
        Self::Comment,
        Self::LetKw,
        Self::Ident,
        Self::Number,
        Self::Plus,
        Self::Minus,
        Self::Star,
        Self::Slash,
        Self::Equals,
        Self::LParen,
        Self::RParen,
        Self::LBrace,
        Self::RBrace,
        Self::Error,
    ];

    pub const KINDS_WITH_REGEX: [(Self, &'static str); Self::KIND_COUNT - 1] = [
        (Self::Whitespace, r"[ \n]+"),
        (Self::Comment, r"//.*"),
        (Self::LetKw, r"let"),
        (Self::Ident, r"[a-zA-Z][a-zA-Z0-9_]*|_+[a-zA-Z0-9][a-zA-Z0-9_]*"),
        (Self::Number, r"[0-9]+"),
        (Self::Plus, r"\+"),
        (Self::Minus, r"-"),
        (Self::Star, r"\*"),
        (Self::Slash, r"/"),
        (Self::Equals, r"="),
        (Self::LParen, r"\("),
        (Self::RParen, r"\)"),
        (Self::LBrace, r"\{"),
        (Self::RBrace, r"\}"),
    ];

    pub const fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    pub const fn is_keyword(self) -> bool {
        matches!(self, Self::LetKw)
    }

    pub const fn is_ident(self) -> bool {
        matches!(self, Self::Ident)
    }

    pub const fn is_literal(self) -> bool {
        matches!(self, Self::Number)
    }

    pub const fn is_symbol(self) -> bool {
        matches!(
            self,
            Self::Plus
                | Self::Minus
                | Self::Star
                | Self::Slash
                | Self::Equals
                | Self::LParen
                | Self::RParen
                | Self::LBrace
                | Self::RBrace
        )
    }

    pub const fn is_error(self) -> bool {
        matches!(self, Self::Error)
    }
}

impl From<TokenKind> for u8 {
    fn from(kind: TokenKind) -> Self {
        kind as u8
    }
}

impl TryFrom<u8> for TokenKind {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, <Self as TryFrom<u8>>::Error> {
        match value {
            // Safe because `value` is a valid representation of `TokenKind`
            Self::KIND_MIN..=Self::KIND_MAX => Ok(unsafe { mem::transmute::<u8, Self>(value) }),
            _ => Err(()),
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Whitespace => "whitespace",
            Self::Comment => "comment",
            Self::LetKw => "'let'",
            Self::Ident => "identifier",
            Self::Number => "number",
            Self::Plus => "'+'",
            Self::Minus => "'-'",
            Self::Star => "'*'",
            Self::Slash => "'/'",
            Self::Equals => "'='",
            Self::LParen => "'('",
            Self::RParen => "')'",
            Self::LBrace => "'{'",
            Self::RBrace => "'}'",
            Self::Error => "an unrecognized token",
        })
    }
}
