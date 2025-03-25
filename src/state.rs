use smol_str::SmolStr;

use crate::position::{Position, PositionComponent};

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct State {
    pub(crate) position: usize,
    pub(crate) lexer_position: Position,
    pub(crate) last_whitespace: SmolStr,
}

impl State {
    pub fn increment_position_by_char(&mut self, character: char) {
        self.position += 1;

        match character {
            '\n' => {
                self.lexer_position.character = 0;
                self.lexer_position.line += 1;
            }
            _ => self.lexer_position.character += 1,
        }
    }

    pub fn increment_position(&mut self, amount: PositionComponent) {
        self.position += amount as usize;
        self.lexer_position.character += amount;
    }

    #[inline]
    pub fn lexer_position(&self) -> Position {
        self.lexer_position
    }
}
