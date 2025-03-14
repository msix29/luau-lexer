use crate::{
    prelude::{Lexable, Lexer, LexerError, Literal, TokenType},
    utils::is_identifier_start,
};

impl Lexable for TokenType {
    fn try_lex(lexer: &mut Lexer) -> Option<Self> {
        let character = lexer.current_char()?;
        let start = lexer.lexer_position;

        match character {
            '0'..='9' => {
                if let Some(number) = Literal::parse_number(lexer) {
                    return Some(TokenType::Literal(number));
                }
            }
            '.' if matches!(lexer.next_char(), Some('0'..='9')) => {
                if let Some(number) = Literal::parse_number(lexer) {
                    return Some(TokenType::Literal(number));
                }
            }
            '\'' | '"' | '`' | '[' => {
                if let Some(string) = Literal::parse_string(lexer) {
                    return Some(TokenType::Literal(string));
                }
            }
            _ if is_identifier_start(character) => {
                let word = lexer.consume_identifier();

                return Some(Self::Identifier(word));
            }
            _ => (),
        }

        lexer.increment_position(1);

        Some(TokenType::Error(LexerError::new(
            start,
            format!("Unexpected character: {}", character),
            Some(lexer.lexer_position),
        )))
    }
}
