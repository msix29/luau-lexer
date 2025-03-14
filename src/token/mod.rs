mod r#impl;

use crate::prelude::{LexerError, Position};

crate_reexport!(literal, keyword);

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
    Identifier(String),
    Keyword(Keyword),
    PartialKeyword(PartialKeyword),
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

impl_from!(TokenType <= {
    Error(LexerError),
    Literal(Literal),
    Keyword(Keyword),
    PartialKeyword(PartialKeyword),
});
