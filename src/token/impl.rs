use crate::{
    prelude::{
        CompoundOperator, Keyword, Lexable, Lexer, Literal, Operator, ParseError, PartialKeyword,
        Symbol, TokenType,
    },
    utils::is_identifier_start,
};

impl Lexable for TokenType {
    fn try_lex(lexer: &mut Lexer) -> Option<Self> {
        let character = lexer.current_char()?;
        let start = lexer.lexer_position;

        match character {
            '0'..='9' => {
                if let Some(number) = Literal::parse_number(lexer) {
                    return Some(Self::Literal(number));
                }
            }
            '.' => {
                lexer.consume('.');

                let next_character = lexer.next_char();
                if matches!(next_character, Some('0'..='9')) {
                    if let Some(number) = Literal::parse_number(lexer) {
                        return Some(Self::Literal(number));
                    }
                } else if next_character == Some('.') {
                    lexer.consume('.');

                    if lexer.consume('.') {
                        return Some(Self::Symbol(Symbol::Ellipses));
                    } else {
                        let operator = Operator::Concatenation;

                        return CompoundOperator::try_from_operator(operator, lexer)
                            .map(Self::CompoundOperator)
                            .or(Some(Self::Operator(operator)));
                    }
                } else {
                    return Some(Self::Symbol(Symbol::Dot));
                }
            }
            '\'' | '"' | '`' | '[' => {
                if let Some(string) = Literal::parse_string(lexer) {
                    return Some(Self::Literal(string));
                }
            }
            '>' if lexer.consume_with_next('=') => {
                return Some(Self::CompoundOperator(
                    CompoundOperator::GreaterThanOrEqualTo,
                ));
            }
            '<' if lexer.consume_with_next('=') => {
                return Some(Self::CompoundOperator(CompoundOperator::LessThanOrEqualTo));
            }
            '-' if lexer.consume_with_next('>') => {
                return Some(Self::Symbol(Symbol::Arrow));
            }
            '=' => {
                lexer.consume('=');

                if lexer.consume('=') {
                    return Some(Self::CompoundOperator(CompoundOperator::EqualEqual));
                } else {
                    return Some(Self::Symbol(Symbol::Equal));
                }
            }
            ':' if lexer.consume_with_next(':') => {
                return Some(Self::Symbol(Symbol::Typecast));
            }
            _ if is_identifier_start(character) => {
                let word = lexer.consume_identifier();

                if let Some(keyword) = Keyword::try_from_str(&word) {
                    return Some(Self::Keyword(keyword));
                } else if let Some(partial_keyword) = PartialKeyword::try_from_str(&word) {
                    return Some(Self::PartialKeyword(partial_keyword));
                }

                match word.as_str() {
                    "true" => return Some(Self::Literal(Literal::Boolean(true))),
                    "false" => return Some(Self::Literal(Literal::Boolean(false))),
                    "and" => return Some(Self::Operator(Operator::And)),
                    "or" => return Some(Self::Operator(Operator::Or)),
                    "not" => return Some(Self::Operator(Operator::Not)),
                    _ => return Some(Self::Identifier(word)),
                }
            }
            _ => {
                if let Some(symbol) = Symbol::try_from_char(character, lexer) {
                    return Some(Self::Symbol(symbol));
                }
                if let Some(operator) = Operator::try_from_chars(character, lexer) {
                    return CompoundOperator::try_from_operator(operator, lexer)
                        .map(Self::CompoundOperator)
                        .or(Some(Self::Operator(operator)));
                }
            }
        }

        lexer.increment_position(1);

        Some(Self::Error(ParseError::new(
            start,
            format!("Unexpected character: {}", character),
            Some(lexer.lexer_position),
        )))
    }
}
