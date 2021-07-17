use std::fmt;

use rowan::TextRange;

use crate::ast::{AstNode, AstToken, Literal};
use crate::SyntaxNode;

#[cfg(test)]
mod tests;

pub fn validate(syntax: &SyntaxNode) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    for node in syntax.descendants() {
        if let Some(literal) = Literal::cast(node) {
            literal.validate(&mut errors);
        }
    }

    errors
}

impl Literal {
    fn validate(&self, errors: &mut Vec<ValidationError>) {
        if self.value().text().parse::<u64>().is_err() {
            errors.push(ValidationError::new(
                ValidationErrorKind::NumberLiteralTooLarge,
                self.value().syntax().text_range(),
            ));
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ValidationError {
    kind: ValidationErrorKind,
    range: TextRange,
}

impl ValidationError {
    const fn new(kind: ValidationErrorKind, range: TextRange) -> Self {
        ValidationError { kind, range }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error at {:?}: {}", self.range, self.kind)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum ValidationErrorKind {
    NumberLiteralTooLarge,
}

impl fmt::Display for ValidationErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationErrorKind::NumberLiteralTooLarge => write!(
                f,
                "number literal is larger than an integer's maximum value, {}",
                u64::MAX
            ),
        }
    }
}
