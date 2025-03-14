use crate::{
    prelude::{Keyword, Lexable, Lexer, LexerError, Literal, PartialKeyword, TokenType},
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
