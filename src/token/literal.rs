use crate::{
    prelude::{Lexable, Lexer, ParseError},
    utils::is_numeric,
};

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
                lexer.errors.push(ParseError::new(
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
            lexer.errors.push(ParseError::new(
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
            lexer.errors.push(ParseError::new(
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
            lexer.errors.push(ParseError::new(
                start,
                "Malformed multi-line string.".to_string(),
                Some(lexer.lexer_position),
            ));
        }

        Some(characters.iter().collect::<String>())
    }
}

impl Lexable for LuauString {
    fn try_lex(lexer: &mut Lexer) -> Option<Self> {
        match lexer.current_char()? {
            '"' => Self::try_parse_inner(lexer, '"').map(Self::DoubleQuotes),
            '\'' => Self::try_parse_inner(lexer, '\'').map(Self::SingleQuotes),
            '`' => Self::try_parse_inner(lexer, '`').map(Self::Bacticks),
            '[' => Self::try_parse_multi_line(lexer).map(Self::MultiLine),
            _ => unreachable!("Invalid quote type."),
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LuauNumber {
    Plain(String),
    Binary(String),
    Hex(String),
}

impl LuauNumber {
    fn parse_number_inner(lexer: &mut Lexer) -> Option<Self> {
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
                    break lexer.errors.push(ParseError::new(
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

        Some(Self::Plain(lexer.input[start..lexer.position].to_string()))
    }

    fn parse_hex_number(lexer: &mut Lexer) -> Option<Self> {
        let start = lexer.position;
        let mut found_digit = false;
        let mut is_faulty = false;

        lexer.consume('0');
        lexer.consume('x');

        loop {
            let Some(current_char) = lexer.current_char() else {
                break;
            };

            if current_char.is_ascii_hexdigit() {
                lexer.increment_position_by_char(current_char);
                found_digit = true;
            } else {
                break is_faulty = !current_char.is_whitespace();
            }
        }

        // ? Do we exit or return the faulty number?
        if !found_digit {
            lexer.errors.push(ParseError::new(
                lexer.lexer_position,
                "Hexadecimal numbers must have at least one digit after '0x'.".to_string(),
                None,
            ));
        }
        if found_digit && is_faulty {
            lexer.errors.push(ParseError::new(
                lexer.lexer_position,
                "Hexadecimal numbers must only contain hexadecimal digits.".to_string(),
                None,
            ));
        }

        Some(Self::Hex(lexer.input[start..lexer.position].to_string()))
    }

    fn parse_binary_number(lexer: &mut Lexer) -> Option<Self> {
        let start = lexer.position;
        let mut found_digit = false;
        let mut is_faulty = false;

        lexer.consume('0');
        lexer.consume('b');

        loop {
            let Some(current_char) = lexer.current_char() else {
                break;
            };

            if current_char == '0' || current_char == '1' {
                lexer.increment_position_by_char(current_char);
                found_digit = true;
            } else {
                break is_faulty = !current_char.is_whitespace();
            }
        }

        // ? Do we exit or return the faulty number?
        if !found_digit {
            lexer.errors.push(ParseError::new(
                lexer.lexer_position,
                "Binary number must have at least one digit after '0b'.".to_string(),
                None,
            ));
        }
        if found_digit && is_faulty {
            lexer.errors.push(ParseError::new(
                lexer.lexer_position,
                "Binary number must only have 1s and 0s.".to_string(),
                None,
            ));
        }

        Some(Self::Binary(lexer.input[start..lexer.position].to_string()))
    }
}

impl Lexable for LuauNumber {
    fn try_lex(lexer: &mut Lexer) -> Option<Self> {
        match (lexer.current_char()?, lexer.next_char()) {
            ('0', Some('b')) => Self::parse_binary_number(lexer),
            ('0', Some('x')) => Self::parse_hex_number(lexer),
            _ => Self::parse_number_inner(lexer),
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Literal {
    Number(LuauNumber),
    String(LuauString),
    Boolean(bool),
}

impl Literal {
    #[inline]
    pub fn parse_number(lexer: &mut Lexer) -> Option<Self> {
        LuauNumber::try_lex(lexer).map(Self::Number)
    }

    #[inline]
    pub fn parse_string(lexer: &mut Lexer) -> Option<Self> {
        LuauString::try_lex(lexer).map(Self::String)
    }
}

impl Lexable for Literal {
    /// This just marks literals as lexable, refrain from using it. Use
    /// [`Literal::parse_number`], or [`Literal::parse_string`], or the more
    /// specific [`LuauString::try_lex`], and [`LuauNumber::try_lex`] instead.
    fn try_lex(_: &mut Lexer) -> Option<Self> {
        panic!(
            "\
            `Literal::try_lex()` should never be used. \
            Please read the documentation for this function."
        )
    }
}

impl_from!(Literal <= {
    Number(LuauNumber),
    String(LuauString),
    Boolean(bool),
});
