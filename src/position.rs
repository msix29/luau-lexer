//! The [`PositionExt`]

use lsp_types::Position;

/// Extands the [`Position`] type by adding helper functions.
pub trait PositionExt {
    /// The largest value that can be represented by a [`Position`].
    const MAX: Self;
    /// The smallest value that can be represented by a [`Position`].
    const MIN: Self;

    /// Offsets the current position by lines and characters. If you're adding both
    /// lines and characters, making sure to set characters to `0` before calling
    /// this function to ensure correct results.
    fn offset(&mut self, lines: i32, characters: i32);

    /// Sets line to a specific value.
    fn set_line(&mut self, line: u32);

    /// Sets character to a specific value.
    fn set_character(&mut self, character: u32);

    /// Checks whether or not this position is between the 2 passed positions.
    fn is_in_bounds(&self, start: Self, end: Self) -> bool;

    /// Checks whether or not this position is after the passed position.
    fn is_after(&self, position: Self) -> bool;

    /// Checks whether or not this position is before the passed position.
    fn is_before(&self, position: Self) -> bool;
}

impl PositionExt for Position {
    const MAX: Self = Self {
        line: u32::MAX,
        character: u32::MAX,
    };

    const MIN: Self = Self {
        line: u32::MIN,
        character: u32::MIN,
    };

    #[inline]
    fn offset(&mut self, lines: i32, characters: i32) {
        self.line = self.line.saturating_add_signed(lines);
        self.character = self.character.saturating_add_signed(characters);
    }

    #[inline]
    fn set_line(&mut self, line: u32) {
        self.line = line;
    }

    #[inline]
    fn set_character(&mut self, character: u32) {
        self.character = character;
    }

    #[inline]
    fn is_in_bounds(&self, start: Self, end: Self) -> bool {
        self.is_after(start) & self.is_before(end)
    }

    #[inline]
    fn is_after(&self, position: Self) -> bool {
        self.line > position.line
            || position.line == self.line && self.character >= position.character
    }

    #[inline]
    fn is_before(&self, position: Self) -> bool {
        self.line < position.line
            || position.line == self.line && self.character <= position.character
    }
}
