#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
    Plus,
    Minus,
    Division,
    FloorDivision,
    Multiplication,
    Modulo,
    Exponentiation,

    Equal,
    NotEqual,

    Length,
}

impl Operator {
    pub fn try_from_chars(value: char, next_char: Option<char>) -> Option<Self> {
        match value {
            '+' => Some(Self::Plus),
            '-' => Some(Self::Minus),
            '/' if next_char == Some('/') => Some(Self::FloorDivision),
            '/' => Some(Self::Division),
            '*' => Some(Self::Multiplication),
            '%' => Some(Self::Modulo),
            '^' => Some(Self::Exponentiation),
            '=' => Some(Self::Equal),
            '~' if next_char == Some('=') => Some(Self::NotEqual),
            '#' => Some(Self::Length),
            _ => None,
        }
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

    EqualEqual,
    LessThanOrEqualTo,
    GreaterThanOrEqualTo,
}

impl CompoundOperator {
    pub fn try_from_chars(operator: Operator, next_char: Option<char>) -> Option<Self> {
        if next_char != Some('=') {
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
            Operator::Equal => Some(Self::EqualEqual),
            _ => None,
        }
    }
}
