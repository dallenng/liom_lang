use std::ops::Range;

use super::*;
use crate::parse;

fn check(text: &str, expected_errors: impl IntoIterator<Item = (ValidationErrorKind, Range<u32>)>) {
    let parse = parse(text);

    let expected_errors = expected_errors
        .into_iter()
        .map(|(kind, range)| {
            ValidationError::new(kind, TextRange::new(range.start.into(), range.end.into()))
        })
        .collect::<Vec<_>>();

    assert_eq!(validate(&parse.syntax()), expected_errors);
}

#[test]
fn validate_ok_literal() {
    check("123", []);
}

#[test]
fn validate_too_large_literal() {
    check("99999999999999999999", [(ValidationErrorKind::NumberLiteralTooLarge, 0..20)]);
}
