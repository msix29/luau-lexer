use crate::lexer::Lexer;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
    Plus,
    Minus,
    Division,
    FloorDivision,
    Multiplication,
    Modulo,
    Exponentiation,

    Concatenation,

    NotEqual,

    And,
    Or,
    Not,

    Intersection,
    Union,

    Length,
}

impl Operator {
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
            _ => None,
        };
        if value.is_some() {
            lexer.consume(character);
        }

        value
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompoundOperator {
    PlusEqual,
    MinusEqual,
    FloorDivisionEqual,
    DivisionEqual,
    MultiplicationEqual,
    ModuloEqual,
    ExponentiationEqual,

    ConcatenationEqual,

    EqualEqual,
    LessThanOrEqualTo,
    GreaterThanOrEqualTo,
}

impl CompoundOperator {
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
