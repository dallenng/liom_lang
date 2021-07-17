#![feature(variant_count)]

use std::convert::TryFrom;
use std::{fmt, mem};

use crate::TokenKind::*;

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

    pub const KIND_MAX: u8 = Error as u8;

    pub const KIND_COUNT: usize = mem::variant_count::<Self>();

    pub const KINDS: [Self; Self::KIND_COUNT] = [
        Whitespace, Comment, LetKw, Ident, Number, Plus, Minus, Star, Slash, Equals, LParen,
        RParen, LBrace, RBrace, Error,
    ];

    pub const KINDS_WITH_REGEX: [(Self, &'static str); Self::KIND_COUNT - 1] = [
        (Whitespace, r"[ \n]+"),
        (Comment, r"//.*"),
        (LetKw, r"let"),
        (Ident, r"[a-zA-Z][a-zA-Z0-9_]*|_+[a-zA-Z0-9][a-zA-Z0-9_]*"),
        (Number, r"[0-9]+"),
        (Plus, r"\+"),
        (Minus, r"-"),
        (Star, r"\*"),
        (Slash, r"/"),
        (Equals, r"="),
        (LParen, r"\("),
        (RParen, r"\)"),
        (LBrace, r"\{"),
        (RBrace, r"\}"),
    ];

    pub const fn is_trivia(self) -> bool {
        matches!(self, Whitespace | Comment)
    }

    pub const fn is_keyword(self) -> bool {
        matches!(self, LetKw)
    }

    pub const fn is_ident(self) -> bool {
        matches!(self, Ident)
    }

    pub const fn is_literal(self) -> bool {
        matches!(self, Number)
    }

    pub const fn is_symbol(self) -> bool {
        matches!(
            self,
            Plus | Minus | Star | Slash | Equals | LParen | RParen | LBrace | RBrace
        )
    }

    pub const fn is_error(self) -> bool {
        matches!(self, Error)
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
            Self::KIND_MIN..=Self::KIND_MAX => Ok(unsafe { mem::transmute(value) }),
            _ => Err(()),
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Whitespace => "whitespace",
            Comment => "comment",
            LetKw => "'let'",
            Ident => "identifier",
            Number => "number",
            Plus => "'+'",
            Minus => "'-'",
            Star => "'*'",
            Slash => "'/'",
            Equals => "'='",
            LParen => "'('",
            RParen => "')'",
            LBrace => "'{'",
            RBrace => "'}'",
            Error => "an unrecognized token",
        })
    }
}
