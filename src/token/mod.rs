//! The [`Token`] struct.

mod r#impl;

use smol_str::SmolStr;

use crate::prelude::{ParseError, Position};

crate_reexport!(literal, keyword, symbol, operator, comment);

/// A single token. Every [`lexable`](crate::lexer::Lexable) item becomes
/// a token in [`Lexer::next_token()`](crate::lexer::Lexer::next_token).
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Token {
    /// The starting position of this token
    pub start: Position,

    /// The spaces before the token.
    pub spaces_before: SmolStr,

    /// The actual info of the token.
    pub token_type: TokenType,

    /// The spaces after the token.
    pub spaces_after: SmolStr,

    /// The ending position of this token.
    pub end: Position,
}

impl Token {
    /// The end of file constant token.
    pub const END_OF_FILE: Self = Self::empty(TokenType::EndOfFile);

    /// Creates an empty token with the specified type. This is only used when
    /// creating tokens that don't have actual positions.
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

/// All token types.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum TokenType {
    /// An unknown type.
    Error(ParseError),

    /// A luau literal
    Literal(Literal),

    /// An identifier, like a variable name.
    Identifier(SmolStr),

    /// A comment
    Comment(Comment),

    /// A luau-reserved-keyword
    Keyword(Keyword),

    /// A word that can both be a keyword and an identifier.
    PartialKeyword(PartialKeyword),

    /// Symbols like `(` and `)`.
    Symbol(Symbol),

    /// Operators like `+` and `and`
    Operator(Operator),

    /// Compound operators like `+=` and `//=`
    CompoundOperator(CompoundOperator),

    /// The end of file token.
    EndOfFile,
}

impl TokenType {
    /// Turn this token type into a [`Token`] with the passed properties.
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
    /// Try converting this token type into a string.
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
                    | LuauString::Backticks(smol_str)
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
