use std::ops::{Deref, DerefMut};

use smol_str::SmolStr;

use crate::{
    error::ParseError,
    state::State,
    token::{Token, TokenType}, utils::can_be_identifier,
};

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Lexer<'a> {
    pub(crate) input: &'a str,
    pub(crate) chars: Vec<char>,
    pub/* (crate) */ errors: Vec<ParseError>,

    pub(crate) state: State,
}

impl<'a> Lexer<'a> {
    #[inline]
    pub fn new(input: &'a str) -> Self {
        Self::default().with_input(input)
    }

    #[inline]
    pub fn with_input(mut self, input: &'a str) -> Self {
        self.set_input(input);
        self
    }

    #[inline]
    pub fn set_input(&mut self, input: &'a str) {
        self.input = input;
        self.chars = input.chars().collect();
        self.last_whitespace = self.skip_whitespace();
    }

    #[inline]
    pub fn save_state(&self) -> State {
        self.state.clone()
    }

    #[inline]
    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }

    pub fn next_token(&mut self) -> Token {
        if !self.errors.is_empty() {
            let error = self.errors.remove(0);
            let start = error.start();

            return TokenType::Error(error).into_token(
                start,
                self.lexer_position,
                "",
                "",
            );
        }

        let start = self.lexer_position;

        TokenType::try_lex(self)
            .map(|token_type| {
                let whitespaces = self.skip_whitespace();
                let spaces_before = self.last_whitespace.clone();
                let spaces_after = whitespaces.clone();

                self.last_whitespace = whitespaces;

                token_type.into_token(
                    start,
                    self.lexer_position,
                    spaces_before,
                    spaces_after,
                )
            })
            .unwrap_or_else(|| Token::END_OF_FILE)
    }

    #[inline]
    pub fn current_char(&self) -> Option<char> {
        self.chars.get(self.position).copied()
    }

    #[inline]
    pub fn next_char(&self) -> Option<char> {
        self.chars.get(self.position + 1).copied()
    }

    #[inline]
    pub fn consume(&mut self, character: char) -> bool {
        if self.current_char() == Some(character) {
            self.increment_position_by_char(character);

            true
        } else {
            false
        }
    }

    #[inline]
    pub fn consume_with_next(&mut self, character: char) -> bool {
        if self.next_char() == Some(character) {
            let current_char = self.current_char().unwrap();

            self.increment_position_by_char(current_char);
            self.increment_position_by_char(character);

            true
        } else {
            false
        }
    }

    pub fn check_keyword(&mut self, keyword: &str) -> bool {
        if self.input[self.position..].starts_with(keyword) {
            self.increment_position(keyword.len() as u32);

            return true;
        }

        false
    }

    pub fn consume_identifier(&mut self) -> SmolStr {
        let start = self.position;
        while let Some(character) = self.current_char() {
            if can_be_identifier(character) {
                self.increment_position_by_char(character);
            } else {
                break;
            }
        }

        self.input[start..self.position].into()
    }

    pub fn skip_whitespace(&mut self) -> SmolStr {
        let start = self.position;
        while let Some(character) = self.current_char() {
            if character.is_whitespace() {
                self.increment_position_by_char(character);
            } else {
                break;
            }
        }

        (start != self.position)
            .then(|| self.input[start..self.position].into())
            .unwrap_or_default()
    }
}

impl Deref for Lexer<'_> {
    type Target = State;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl DerefMut for Lexer<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

pub trait Lexable: Sized {
    fn try_lex(lexer: &mut Lexer) -> Option<Self>;
}
