use crate::{error::LexerError, lexer::Lexer, prelude::Position};

crate_reexport!(literal);

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    pub start: Option<Position>,
    pub spaces_before: String,
    pub token_type: TokenType,
    pub spaces_after: String,
    pub end: Option<Position>,
}

impl Token {
    pub const END_OF_FILE: Self = Self {
        start: None,
        spaces_before: String::new(),
        token_type: TokenType::EndOfFile,
        spaces_after: String::new(),
        end: None,
    };
}

impl PartialEq<TokenType> for Token {
    fn eq(&self, other: &TokenType) -> bool {
        &self.token_type == other
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenType {
    Error(LexerError),
    Literal(Literal),
    EndOfFile,
}

impl TokenType {
    pub fn into_token(
        self,
        start: Option<Position>,
        end: Option<Position>,
        spaces_before: String,
        spaces_after: String,
    ) -> Token {
        Token {
            start,
            spaces_before,
            token_type: self,
            spaces_after,
            end,
        }
    }
}

impl TokenType {
    pub fn try_lex(lexer: &mut Lexer) -> Option<Self> {
        let character = lexer.current_char()?;
        let start = lexer.lexer_position;

        match character {
            '0'..='9' => {
                if let Some(number) = Literal::parse_from_number(lexer) {
                    return Some(TokenType::Literal(number));
                }
            }
            '\'' | '"' | '`' | '[' => {
                if let Some(string) = Literal::parse_from_string(lexer) {
                    return Some(TokenType::Literal(string));
                }
            }
            _ => (),
        }

        lexer.increment_position(1);

        Some(TokenType::Error(LexerError::new(
            start,
            format!("Unexpected character: {}", character),
            Some(lexer.lexer_position),
        )))
    }
}
