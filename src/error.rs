//! The [`ParseError`] struct.

use smol_str::SmolStr;
use lsp_types::Position;

/// An error that can be met during parsing.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ParseError {
    /// The starting location of the error.
    start: Position,

    /// The error message.
    message: SmolStr,

    /// The ending location of the error.
    end: Option<Position>,
}

impl ParseError {
    /// Create a new [`ParseError`].
    #[inline]
    pub fn new(start: Position, message: impl Into<SmolStr>, end: Option<Position>) -> Self {
        Self {
            start,
            message: message.into(),
            end,
        }
    }

    /// Get the start of the error.
    #[inline]
    pub fn start(&self) -> Position {
        self.start
    }

    /// Get the error message.
    #[inline]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Get the end of the error.
    #[inline]
    pub fn end(&self) -> Option<Position> {
        self.end
    }
}
