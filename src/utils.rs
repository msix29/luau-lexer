#[inline]
pub fn is_numeric(character: char) -> bool {
    matches!(character, '0'..='9' | '_')
}

#[inline]
pub fn is_identifier_start(character: char) -> bool {
    matches!(character, 'a'..='z' | 'A'..='Z' | '_')
}
#[inline]
pub fn can_be_identifier(character: char) -> bool {
    is_identifier_start(character) | is_numeric(character)
}
