//! Luau literals

use smol_str::SmolStr;

use crate::{
    prelude::{Lexable, Lexer, ParseError},
    utils::is_numeric,
};

/// A Luau string. The stored string will include the quotes/double quotes/backticks.
/// The only reason the different types actually exist is to allow the user to
/// easily know which one is used without needing to check the actual string.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum LuauString {
    ///```lua
    /// 'single quotes'
    /// ```
    SingleQuotes(SmolStr),

    ///```lua
    /// "double quotes"
    /// ```
    DoubleQuotes(SmolStr),

    ///```lua
    /// `backticks`
    /// ```
    Backticks(SmolStr),

    ///```lua
    /// [[ multi line ]]
    /// [[
    ///     multi line
    /// ]]
    /// [==[
    ///     multi line
    /// ]==]
    /// ```
    MultiLine(SmolStr),
}

impl LuauString {
    /// Counts the number of backslashes at the end of the passed characters array.
    fn count_back_slashes(characters: &[char]) -> usize {
        if characters.is_empty() {
            return 0;
        }

        let mut count = 0;
        let mut i = characters.len() - 1;

        while characters[i] == '\\' {
            count += 1;

            match i.checked_sub(1) {
                Some(new_i) => i = new_i,
                None => break,
            }
        }

        count
    }

    /// Whether or not the last character is escaped.
    fn is_escaped(characters: &[char]) -> bool {
        if characters.len() < 2 {
            false
        } else {
            Self::count_back_slashes(&characters[..characters.len() - 1]) % 2 != 0
        }
    }

    /// Whether or not the array ends with `\z`.
    #[inline]
    fn is_multi_line_escaped(characters: &[char]) -> bool {
        Self::is_escaped(characters) && characters[characters.len() - 1] == 'z'
    }

    /// Parses one of the single line variants:
    ///
    /// * [`LuauString::SingleQuotes`]
    /// * [`LuauString::DoubleQuotes`]
    /// * [`LuauString::Backticks`]
    fn parse_inner(lexer: &mut Lexer, quote_character: char) -> SmolStr {
        let mut characters = vec![quote_character];
        let start = lexer.lexer_position;
        let mut is_done = false;

        lexer.increment_position_by_char(quote_character);

        while let Some(character) = lexer.current_char() {
            if character == '\n' || character == '\r' && !Self::is_multi_line_escaped(&characters) {
                lexer.errors.push(ParseError::new(
                    start,
                    format!(
                        "SmolStr must be single line, use `\\z` here or add a {}.",
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

        characters.iter().collect::<String>().into()
    }

    /// Parses [`LuauString::MultiLine`].
    pub(crate) fn parse_multi_line(lexer: &mut Lexer) -> SmolStr {
        let mut characters = vec!['['];
        let start = lexer.lexer_position;
        let mut equals_count = 0;
        let mut is_done = false;

        lexer.consume('[');
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

        characters.iter().collect::<String>().into()
    }
}

impl Lexable for LuauString {
    fn try_lex(lexer: &mut Lexer) -> Option<Self> {
        match lexer.current_char()? {
            '"' => Some(Self::DoubleQuotes(Self::parse_inner(lexer, '"'))),
            '\'' => Some(Self::SingleQuotes(Self::parse_inner(lexer, '\''))),
            '`' => Some(Self::Backticks(Self::parse_inner(lexer, '`'))),
            '[' => Some(Self::MultiLine(Self::parse_multi_line(lexer))),
            _ => unreachable!("Invalid quote type."),
        }
    }
}

/// A luau number. The stored string will include the `0b`, or `0x`. The only
/// reason the different types actually exist is to allow the user to easily
/// know which one is used without needing to check the actual string.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum LuauNumber {
    ///```luau
    /// 1
    /// 1.1
    /// .1
    /// ```
    Plain(SmolStr),

    ///```luau
    /// 0b111001101
    /// ```
    Binary(SmolStr),

    ///```luau
    /// 0xAB02C
    /// ```
    Hex(SmolStr),
}

impl LuauNumber {
    /// Parses a [`LuauNumber::Plain`].
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

        Some(Self::Plain(lexer.input[start..lexer.position].into()))
    }

    /// Parses a [`LuauNumber::Hex`].
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

        Some(Self::Hex(lexer.input[start..lexer.position].into()))
    }

    /// Parses a [`LuauNumber::Binary`].
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

        Some(Self::Binary(lexer.input[start..lexer.position].into()))
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

/// A Luau literal value
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Literal {
    /// A numeric value
    Number(LuauNumber),

    /// A string
    String(LuauString),

    /// A boolean
    Boolean(bool),
}

impl Literal {
    /// Parses a [`Literal::Number`].
    #[inline]
    pub fn parse_number(lexer: &mut Lexer) -> Option<Self> {
        LuauNumber::try_lex(lexer).map(Self::Number)
    }

    /// Parses a [`Literal::String`].
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
