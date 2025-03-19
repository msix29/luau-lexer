pub type PositionComponent = u32;

/// A struct representing a specific point in a document. Lines and characters are
/// zero-based.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    /// The line in which this position points to, starting from 0.
    pub line: PositionComponent,

    /// The character in the [`line`](Position::line) that this position points to,
    /// starting from 0.
    pub character: PositionComponent,
}

impl Position {
    pub const MAX: Self = Self::new(PositionComponent::MAX, PositionComponent::MAX);
    pub const MIN: Self = Self::new(PositionComponent::MIN, PositionComponent::MIN);

    #[inline]
    pub const fn new(line: PositionComponent, character: PositionComponent) -> Self {
        Self { line, character }
    }

    /// Offsets the current position by lines and characters. If you're adding both lines
    /// and characters, making sure to set characters to `0` before calling this function
    /// to ensure correct results.
    #[inline]
    pub fn offset(&mut self, lines: i32, characters: i32) {
        self.line = self.line.saturating_add_signed(lines);
        self.character = self.character.saturating_add_signed(characters);
    }

    /// Sets line to a specific value.
    #[inline]
    pub fn set_line(&mut self, line: u32) {
        self.line = line;
    }

    /// Sets line to a specific value.
    #[inline]
    pub fn set_character(&mut self, character: u32) {
        self.character = character;
    }

    /// Checks whether or not this position is inside the passed range.
    #[inline]
    pub fn is_in_bounds(self, start: Self, end: Self) -> bool {
        self.is_after(start) & self.is_before(end)
    }

    /// Checks whether or not this position is after the passed position
    #[inline]
    pub fn is_after(self, position: Position) -> bool {
        self.line > position.line
            || position.line == self.line && self.character >= position.character
    }

    /// Checks whether or not this position is before the passed position
    #[inline]
    pub fn is_before(self, position: Position) -> bool {
        self.line < position.line
            || position.line == self.line && self.character <= position.character
    }
}
