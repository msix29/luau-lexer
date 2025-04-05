mod r#impl;

use smol_str::SmolStr;

use crate::prelude::{ParseError, Position};

crate_reexport!(literal, keyword, symbol, operator, comment);

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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

impl TokenType {
    pub fn try_as_string(&self) -> Option<String> {
        match self {
            TokenType::Literal(literal) => match literal {
                Literal::Number(luau_number) => match luau_number {
                    LuauNumber::Plain(smol_str)
                    | LuauNumber::Binary(smol_str)
                    | LuauNumber::Hex(smol_str) => Some(smol_str.to_string()),
                },
                Literal::String(luau_string) => match luau_string {
                    LuauString::SingleQuotes(smol_str)
                    | LuauString::DoubleQuotes(smol_str)
                    | LuauString::Bacticks(smol_str)
                    | LuauString::MultiLine(smol_str) => Some(smol_str.to_string()),
                },
                Literal::Boolean(true) => Some("true".to_string()),
                Literal::Boolean(false) => Some("false".to_string()),
            },
            TokenType::Identifier(smol_str) => Some(smol_str.to_string()),
            TokenType::Comment(comment) => match comment {
                Comment::MultiLine(smol_str) | Comment::SingleLine(smol_str) => {
                    Some(smol_str.to_string())
                }
            },
            TokenType::Keyword(keyword) => Some(keyword.to_string()),
            TokenType::PartialKeyword(partial_keyword) => Some(partial_keyword.to_string()),
            TokenType::Symbol(symbol) => Some(symbol.to_string()),
            TokenType::Operator(operator) => Some(operator.to_string()),
            TokenType::CompoundOperator(compound_operator) => Some(compound_operator.to_string()),
            _ => None,
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
