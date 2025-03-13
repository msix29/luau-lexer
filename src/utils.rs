#[inline]
pub fn is_numeric(character: char) -> bool {
    matches!(character, '0'..='9' | '_')
}
