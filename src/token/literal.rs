use crate::{error::LexerError, lexer::Lexer, utils::is_numeric};

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LuauString {
    // the stored string will include the quotes/double quotes/backticks. The only
    // reason the different types actually exist is to allow the user to easily know
    // which one is used without needing to check the actual string.
    SingleQuotes(String),
    DoubleQuotes(String),
    Bacticks(String),
    MultiLine(String),
}

impl LuauString {
    pub fn try_parse(lexer: &mut Lexer) -> Option<Self> {
        match lexer.current_char()? {
            '"' => Self::try_parse_inner(lexer, '"').map(Self::DoubleQuotes),
            '\'' => Self::try_parse_inner(lexer, '\'').map(Self::SingleQuotes),
            '`' => Self::try_parse_inner(lexer, '`').map(Self::Bacticks),
            '[' => Self::try_parse_multi_line(lexer).map(Self::MultiLine),
            _ => unreachable!("Invalid quote type."),
        }
    }

    fn is_escaped(characters: &[char]) -> bool {
        let length = characters.len();
        if length == 0 {
            return false;
        }

        let has_backslash = characters[length - 2] == '\\';
        if length == 1 {
            has_backslash
        } else {
            has_backslash && characters[length - 3] != '\\'
        }
    }

    fn is_multi_line_escaped(characters: &[char]) -> bool {
        let mut iter = characters.iter().rev().skip_while(|c| c.is_whitespace()); // Skip trailing spaces

        let Some(&last) = iter.next() else {
            return false;
        };
        let Some(&second_last) = iter.next() else {
            return false;
        };

        second_last == '\\' && last == 'z'
    }

    fn try_parse_inner(lexer: &mut Lexer, quote_character: char) -> Option<String> {
        let mut characters = vec![quote_character];
        let start = lexer.lexer_position;
        let mut is_done = false;

        lexer.increment_position_by_char(quote_character);

        while let Some(character) = lexer.current_char() {
            if character == '\n' && !Self::is_multi_line_escaped(&characters) {
                lexer.errors.push(LexerError::new(
                    start,
                    format!(
                        "String must be single line, use `\\z` here or add a {}.",
                        quote_character
                    ),
                    Some(lexer.lexer_position),
                ));

                break;
            }

            characters.push(character);
            lexer.increment_position_by_char(character);

            if character == quote_character && !Self::is_escaped(&characters) {
                is_done = true;

                break;
            }
        }

        if !is_done {
            lexer.errors.push(LexerError::new(
                start,
                format!("Missing {} to close string.", quote_character),
                Some(lexer.lexer_position),
            ));
        }

        Some(characters.iter().collect::<String>())
    }

    fn try_parse_multi_line(lexer: &mut Lexer) -> Option<String> {
        let mut characters = vec!['['];
        let start = lexer.lexer_position;
        let mut equals_count = 0;
        let mut is_done = false;

        lexer.increment_position_by_char('[');
        while lexer.consume('=') {
            equals_count += 1;
            characters.push('=');
        }

        if lexer.consume('[') {
            characters.push('[');
        } else {
            lexer.errors.push(LexerError::new(
                start,
                "Missing `[`.".to_string(),
                Some(lexer.lexer_position),
            ));
        }

        while let Some(character) = lexer.current_char() {
            characters.push(character);
            lexer.increment_position_by_char(character);

            if character == ']' && !Self::is_escaped(&characters) {
                let mut matched_equals = true;

                for _ in 0..equals_count {
                    if let Some(character) = lexer.current_char() {
                        characters.push(character);
                        lexer.increment_position_by_char(character);

                        matched_equals = character == '=';
                    }
                }

                if matched_equals && lexer.consume(']') {
                    characters.push(']');
                    is_done = true;

                    break;
                }
            }
        }

        if !is_done {
            lexer.errors.push(LexerError::new(
                start,
                "Malformed multi-line string.".to_string(),
                Some(lexer.lexer_position),
            ));
        }

        Some(characters.iter().collect::<String>())
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Literal {
    Number(String),
    String(LuauString),
    Boolean(bool),
}

impl Literal {
    pub fn parse_from_number(lexer: &mut Lexer) -> Option<Self> {
        let start = lexer.position;
        let mut found_decimal = false;

        loop {
            let Some(current_char) = lexer.current_char() else {
                break;
            };

            if is_numeric(current_char) {
                lexer.increment_position_by_char(current_char);
            } else if current_char == '.' {
                if found_decimal {
                    break lexer.errors.push(LexerError::new(
                        lexer.lexer_position,
                        "Numbers can only have one decimal point.".to_string(),
                        None,
                    ));
                }

                lexer.increment_position_by_char(current_char);
                found_decimal = true;
            } else {
                break;
            }
        }

        Some(Self::Number(lexer.input[start..lexer.position].to_string()))
    }

    #[inline]
    pub fn parse_from_string(lexer: &mut Lexer) -> Option<Self> {
        LuauString::try_parse(lexer).map(Self::String)
    }
}
