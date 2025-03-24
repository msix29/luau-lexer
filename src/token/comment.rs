use crate::{
    lexer::{Lexable, Lexer},
    token::LuauString,
};

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Comment {
    SingleLine(String),
    MultiLine(String),
}

impl Comment {
    fn parse_inner(lexer: &mut Lexer) -> Option<String> {
        let mut characters = Vec::new();

        while let Some(character) = lexer.current_char() {
            if character == '\n' {
                break;
            }

            characters.push(character);
            lexer.increment_position_by_char(character);
        }

        Some(characters.iter().collect::<String>())
    }
}

impl Lexable for Comment {
    fn try_lex(lexer: &mut Lexer) -> Option<Self> {
        if lexer.current_char() == Some('[') {
            LuauString::try_parse_multi_line(lexer)
                .map(|str| Self::MultiLine(format!("--{}", str)));
        }

        Self::parse_inner(lexer).map(Self::SingleLine)
    }
}
