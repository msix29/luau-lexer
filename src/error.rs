use smol_str::SmolStr;

use crate::position::Position;

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ParseError {
    start: Position,
    message: SmolStr,
    end: Option<Position>,
}

impl ParseError {
    #[inline]
    pub fn new(start: Position, message: impl Into<SmolStr>, end: Option<Position>) -> Self {
        Self {
            start,
            message: message.into(),
            end,
        }
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
