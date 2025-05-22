//! Helper functions for the lexer.

/// Whether or not this character is valid to be in a number.
#[inline]
pub const fn is_numeric(character: char) -> bool {
    matches!(character, '0'..='9' | '_')
}

/// Whether or not this character is a valid identifier start.
#[inline]
pub const fn is_identifier_start(character: char) -> bool {
    matches!(character, 'a'..='z' | 'A'..='Z' | '_')
}

/// Whether or not this character is valid to be in an identifier.
#[inline]
pub const fn can_be_identifier(character: char) -> bool {
    is_identifier_start(character) | is_numeric(character)
}
