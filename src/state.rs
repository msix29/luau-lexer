//! The [`State`] struct.

use lsp_types::Position;

use crate::token::Trivia;

/// A struct representing the state of a lexer at a specific time.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct State {
    /// The current character position.
    pub(crate) position: usize,

    /// The current [`position`](Position) in the file.
    pub(crate) lexer_position: Position,

    /// The spaces after the last parsed token.
    pub(crate) last_trivia: Vec<Trivia>,
}

impl State {
    /// Move the state by the passed character.
    pub const fn increment_position_by_char(&mut self, character: char) {
        self.position += 1;

        match character {
            '\n' => {
                self.lexer_position.character = 0;
                self.lexer_position.line += 1;
            }
            _ => self.lexer_position.character += 1,
        }
    }

    /// Move th state ahead by the passed amount of characters.
    pub const fn increment_position(&mut self, amount: u32) {
        self.position += amount as usize;
        self.lexer_position.character += amount;
    }

    /// Get the current file [`position`](Position).
    #[inline]
    pub const fn lexer_position(&self) -> Position {
        self.lexer_position
    }
}
