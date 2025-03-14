use crate::{
    prelude::{
        CompoundOperator, Keyword, Lexable, Lexer, LexerError, Literal, Operator, PartialKeyword,
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
                let next_character = lexer.next_char();
                if matches!(next_character, Some('0'..='9')) {
                    if let Some(number) = Literal::parse_number(lexer) {
                        return Some(Self::Literal(number));
                    }
                } else if next_character == Some('.') {
                    lexer.consume('.');
                    lexer.consume('.');

                    if lexer.consume('.') {
                        return Some(Self::Symbol(Symbol::Ellipses));
                    } else {
                        let operator = Operator::Concatenation;

                        return CompoundOperator::try_from_operator(operator, lexer.next_char())
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
            '>' if lexer.next_char() == Some('=') => {
                lexer.increment_position(2);

                return Some(Self::CompoundOperator(
                    CompoundOperator::GreaterThanOrEqualTo,
                ));
            }
            '<' if lexer.next_char() == Some('=') => {
                lexer.increment_position(2);

                return Some(Self::CompoundOperator(CompoundOperator::LessThanOrEqualTo));
            }
            '-' if lexer.next_char() == Some('>') => {
                lexer.increment_position(2);

                return Some(Self::Symbol(Symbol::Arrow));
            }
            ':' if lexer.next_char() == Some(':') => {
                lexer.increment_position(2);

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
                    _ => return Some(Self::Identifier(word)),
                }
            }
            _ => {
                if let Some(symbol) = Symbol::try_from_char(character) {
                    return Some(Self::Symbol(symbol));
                }
                if let Some(operator) = Operator::try_from_chars(character, lexer.next_char()) {
                    return CompoundOperator::try_from_operator(operator, lexer.next_char())
                        .map(Self::CompoundOperator)
                        .or(Some(Self::Operator(operator)));
                }
            }
        }

        lexer.increment_position(1);

        Some(Self::Error(LexerError::new(
            start,
            format!("Unexpected character: {}", character),
            Some(lexer.lexer_position),
        )))
    }
}
