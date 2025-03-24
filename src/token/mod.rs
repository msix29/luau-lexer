mod r#impl;

use crate::prelude::{ParseError, Position};

crate_reexport!(literal, keyword, symbol, operator, comment);

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    pub start: Position,
    pub spaces_before: String,
    pub token_type: TokenType,
    pub spaces_after: String,
    pub end: Position,
}

impl Token {
    pub const END_OF_FILE: Self = Self::empty(TokenType::EndOfFile);

    #[inline]
    pub const fn empty(token_type: TokenType) -> Self {
        Self {
            start: Position::MAX,
            spaces_before: String::new(),
            token_type,
            spaces_after: String::new(),
            end: Position::MAX,
        }
    }
}

impl PartialEq<TokenType> for Token {
    fn eq(&self, other: &TokenType) -> bool {
        &self.token_type == other
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenType {
    Error(ParseError),
    Literal(Literal),
    Identifier(String),
    Comment(Comment),
    Keyword(Keyword),
    PartialKeyword(PartialKeyword),
    Symbol(Symbol),
    Operator(Operator),
    CompoundOperator(CompoundOperator),
    EndOfFile,
}

impl TokenType {
    pub fn into_token(
        self,
        start: Position,
        end: Position,
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
    Error(ParseError),
    Literal(Literal),
    Keyword(Keyword),
    PartialKeyword(PartialKeyword),
    Symbol(Symbol),
    Operator(Operator),
    CompoundOperator(CompoundOperator),
});
