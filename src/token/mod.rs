//! The [`Token`] struct.

mod r#impl;

use lsp_types::Position;
use smol_str::SmolStr;

use crate::prelude::{ParseError, PositionExt};

crate_reexport!(literal, keyword, symbol, operator, comment);

/// A single token. Every [`lexable`](crate::lexer::Lexable) item becomes
/// a token in [`Lexer::next_token()`](crate::lexer::Lexer::next_token).
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Token {
    /// The starting position of this token
    pub start: Position,

    /// The trivia before the token.
    pub leading_trivia: Vec<Trivia>,

    /// The actual info of the token.
    pub token_type: TokenType,

    /// The trivia after the token.
    pub trailing_trivia: Vec<Trivia>,

    /// The ending position of this token.
    pub end: Position,
}

/// Trivia that can be before and after a token.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Trivia {
    /// Spaces, be it whitespace, tabs, new lines, etc.
    Spaces(SmolStr),

    /// Comment, single or multi line.
    Comment(Comment),
}

impl Token {
    /// Creates an empty token with the specified type. This is only used when
    /// creating tokens that don't have actual positions.
    #[inline]
    pub const fn empty(token_type: TokenType) -> Self {
        Self {
            start: Position::MAX,
            leading_trivia: Vec::new(),
            token_type,
            trailing_trivia: Vec::new(),
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
        leading_trivia: Vec<Trivia>,
        trailing_trivia: Vec<Trivia>,
    ) -> Token {
        Token {
            start,
            leading_trivia,
            token_type: self,
            trailing_trivia,
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
