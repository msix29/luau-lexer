use crate::position::Position;

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LexerError {
    start: Position,
    message: String,
    end: Option<Position>,
}

impl LexerError {
    #[inline]
    pub fn new(start: Position, message: String, end: Option<Position>) -> Self {
        Self { start, message, end }
    }

    #[inline]
    pub fn start(&self) -> Position {
        self.start
    }

    #[inline]
    pub fn end(&self) -> Option<Position> {
        self.end
    }
}
