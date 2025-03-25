mod r#impl;

use smol_str::SmolStr;

use crate::prelude::{ParseError, Position};

crate_reexport!(literal, keyword, symbol, operator, comment);

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    pub start: Position,
    pub spaces_before: SmolStr,
    pub token_type: TokenType,
    pub spaces_after: SmolStr,
    pub end: Position,
}

impl Token {
    pub const END_OF_FILE: Self = Self::empty(TokenType::EndOfFile);

    #[inline]
    pub const fn empty(token_type: TokenType) -> Self {
        Self {
            start: Position::MAX,
            spaces_before: SmolStr::new_inline(""),
            token_type,
            spaces_after: SmolStr::new_inline(""),
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
    Identifier(SmolStr),
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
        spaces_before: impl Into<SmolStr>,
        spaces_after: impl Into<SmolStr>,
    ) -> Token {
        Token {
            start,
            spaces_before: spaces_before.into(),
            token_type: self,
            spaces_after: spaces_after.into(),
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
