//! [`Comment`] struct

use smol_str::SmolStr;

use crate::prelude::{Lexable, Lexer, LuauString};

/// A comment.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Comment {
    ///```lua
    /// -- single line
    /// ```
    SingleLine(SmolStr),

    ///```lua
    /// --[[ multiline ]]
    /// --[[
    ///     multiline
    /// ]]
    /// --[==[
    ///     multiline
    /// ]==]
    /// ```
    MultiLine(SmolStr),
}

impl Comment {
    /// Parses a [`Comment::SingleLine`].
    fn parse_inner(lexer: &mut Lexer) -> SmolStr {
        let mut characters = vec!['-', '-'];

        while let Some(character) = lexer.current_char() {
            if character == '\n' || character == '\r'  {
                break;
            }

            characters.push(character);
            lexer.increment_position_by_char(character);
        }

        SmolStr::from_iter(characters)
    }
}

impl Lexable for Comment {
    fn try_lex(lexer: &mut Lexer) -> Option<Self> {
        if lexer.current_char() == Some('[') {
            Some(Self::MultiLine(
                format!("--{}", LuauString::parse_multi_line(lexer)).into(),
            ))
        } else {
            Some(Self::SingleLine(Self::parse_inner(lexer)))
        }
    }
}
