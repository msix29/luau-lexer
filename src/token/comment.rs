use smol_str::SmolStr;

use crate::{
    lexer::{Lexable, Lexer},
    token::LuauString,
};

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Comment {
    SingleLine(SmolStr),
    MultiLine(SmolStr),
}

impl Comment {
    fn parse_inner(lexer: &mut Lexer) -> SmolStr {
        let mut characters = vec!['-', '-'];

        while let Some(character) = lexer.current_char() {
            if character == '\n' {
                break;
            }

            characters.push(character);
            lexer.increment_position_by_char(character);
        }

        characters.iter().collect::<String>().into()
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
