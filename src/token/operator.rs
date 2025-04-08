//! [`Operator`] and [`CompoundOperator`] structs.

use std::fmt::{Display, Formatter, Result};

use crate::prelude::Lexer;

/// A luau operator like `+` and `-`
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Operator {
    /// `+`
    Plus,

    /// `-`
    Minus,

    /// `/`
    Division,

    /// `//`
    FloorDivision,

    /// `*`
    Multiplication,

    /// `%`
    Modulo,

    /// `^`
    Exponentiation,

    /// `..`
    Concatenation,

    /// `~=`
    NotEqual,

    /// `and`
    And,

    /// `or`
    Or,

    /// `not`
    Not,

    /// `&`
    Intersection,

    /// `|`
    Union,

    /// `?`
    Optional,

    /// `#`
    Length,
}

impl Operator {
    /// Try converting the character (and maybe the next) into an operator.
    pub fn try_from_chars(character: char, lexer: &mut Lexer) -> Option<Self> {
        let value = match character {
            '+' => Some(Self::Plus),
            '-' => Some(Self::Minus),
            '/' if lexer.consume_with_next('/') => Some(Self::FloorDivision),
            '/' => Some(Self::Division),
            '*' => Some(Self::Multiplication),
            '%' => Some(Self::Modulo),
            '^' => Some(Self::Exponentiation),
            '~' if lexer.consume_with_next('=') => Some(Self::NotEqual),
            '#' => Some(Self::Length),
            '&' => Some(Self::Intersection),
            '|' => Some(Self::Union),
            '?' => Some(Self::Optional),
            _ => None,
        };
        if value.is_some() {
            lexer.consume(character);
        }

        value
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(match self {
            Self::Plus => "+",
            Self::Minus => "-",
            Self::FloorDivision => "//",
            Self::Division => "/",
            Self::Multiplication => "*",
            Self::Modulo => "%",
            Self::Exponentiation => "^",
            Self::NotEqual => "~=",
            Self::Length => "#",
            Self::Intersection => "&",
            Self::Union => "|",
            Self::Optional => "?",
            Self::Concatenation => "..",
            Self::And => "and",
            Self::Or => "or",
            Self::Not => "not",
        })
    }
}

/// A luau compound operator like `+=` and `//=`
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum CompoundOperator {
    /// `+=`
    PlusEqual,

    /// `-=`
    MinusEqual,

    /// `/=`
    DivisionEqual,

    /// `//=`
    FloorDivisionEqual,

    /// `*=`
    MultiplicationEqual,

    /// `%=`
    ModuloEqual,

    /// `^=`
    ExponentiationEqual,

    /// `..=`
    ConcatenationEqual,

    /// `==`
    EqualEqual,

    /// `<=`
    LessThanOrEqualTo,

    /// `>=`
    GreaterThanOrEqualTo,
}

impl CompoundOperator {
    /// Try creating a compound operator from the passed operator, depending on the
    /// next character.
    pub fn try_from_operator(operator: Operator, lexer: &mut Lexer) -> Option<Self> {
        if !lexer.consume('=') {
            return None;
        }

        match operator {
            Operator::Plus => Some(Self::PlusEqual),
            Operator::Minus => Some(Self::MinusEqual),
            Operator::FloorDivision => Some(Self::FloorDivisionEqual),
            Operator::Division => Some(Self::DivisionEqual),
            Operator::Multiplication => Some(Self::MultiplicationEqual),
            Operator::Modulo => Some(Self::ModuloEqual),
            Operator::Exponentiation => Some(Self::ExponentiationEqual),
            Operator::Concatenation => Some(Self::ConcatenationEqual),
            _ => None,
        }
    }
}

impl Display for CompoundOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(match self {
            Self::PlusEqual => "+=",
            Self::MinusEqual => "-=",
            Self::FloorDivisionEqual => "//=",
            Self::DivisionEqual => "/=",
            Self::MultiplicationEqual => "*=",
            Self::ModuloEqual => "%=",
            Self::ExponentiationEqual => "^=",
            Self::ConcatenationEqual => "..=",
            Self::EqualEqual => "==",
            Self::LessThanOrEqualTo => "<=",
            Self::GreaterThanOrEqualTo => ">=",
        })
    }
}
