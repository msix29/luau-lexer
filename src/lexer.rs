//! The actual lexer.

use smol_str::SmolStr;
use std::ops::{Deref, DerefMut};

use crate::{
    error::ParseError,
    state::State,
    token::{Comment, Token, TokenType, Trivia},
    utils::can_be_identifier,
};

/// The main component of this crate, the lexer.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Lexer<'a> {
    /// The input text
    pub(crate) input: &'a str,

    /// The characters in the input
    pub(crate) chars: Vec<char>,

    /// The errors met during lexing. They are added when [`Lexer::next_token`] is
    /// called and gets emptied before any new tokens are lexed.
    pub(crate) errors: Vec<ParseError>,

    /// The current state of the lexer.
    pub(crate) state: State,
}

impl<'a> Lexer<'a> {
    /// Create a new [`Lexer`].
    #[inline]
    pub fn new(input: &'a str) -> Self {
        Self::default().with_input(input)
    }

    /// Set the lexer's input. Meant to be chained.
    #[inline]
    pub fn with_input(mut self, input: &'a str) -> Self {
        self.set_input(input);
        self
    }

    /// Set the lexer's input.
    #[inline]
    pub fn set_input(&mut self, input: &'a str) {
        self.input = input;
        self.chars = input.chars().collect();
        self.last_trivia = self.skip_trivia();
    }

    /// Save the current [`State`]. To be used with [`Lexer::set_state`].
    #[inline]
    pub fn save_state(&self) -> State {
        self.state.clone()
    }

    /// Set the current [`State`]. To be paired with [`Lexer::save_state`].
    #[inline]
    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }

    /// Lex the next token. This will return any errors met while parsing the
    /// *previous* token before lexing a new one.
    pub fn next_token(&mut self) -> Token {
        if !self.errors.is_empty() {
            let error = self.errors.remove(0);
            let start = error.start();

            return TokenType::Error(error).into_token(
                start,
                self.lexer_position,
                Vec::new(),
                Vec::new(),
            );
        }

        let start = self.lexer_position;

        let token_type = TokenType::try_lex(self).unwrap_or(TokenType::EndOfFile);

        let trivia = self.skip_trivia();
        let leading_trivia = self.last_trivia.clone();
        let trailing_trivia = trivia.clone();

        self.last_trivia = trivia;

        token_type.into_token(start, self.lexer_position, leading_trivia, trailing_trivia)
    }

    /// Get the current character.
    #[inline]
    pub fn current_char(&self) -> Option<char> {
        self.chars.get(self.position).copied()
    }

    /// Get the next character.
    #[inline]
    pub fn next_char(&self) -> Option<char> {
        self.chars.get(self.position + 1).copied()
    }

    /// Move the lexer after the current character if it matches the passed one,
    /// and return if it did so.
    #[inline]
    pub fn consume(&mut self, character: char) -> bool {
        if self.current_char() == Some(character) {
            self.increment_position_by_char(character);

            true
        } else {
            false
        }
    }

    /// Like [`Lexer::consume`] but checks for the next character instead. Moves
    /// the lexer after both the current and next character.
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

    /// Consume the next identifier and return it. This assumes there's at least
    /// one character to form a valid identifier at the current position,
    pub fn consume_identifier(&mut self) -> SmolStr {
        let start = self.byte_position;
        while let Some(character) = self.current_char() {
            if can_be_identifier(character) {
                self.increment_position_by_char(character);
            } else {
                break;
            }
        }

        self.input[start..self.byte_position].into()
    }

    /// Get the trivia after the current position and move the lexer to after them.
    pub fn skip_trivia(&mut self) -> Vec<Trivia> {
        let mut trivia = Vec::new();

        loop {
            let spaces = self.skip_whitespace();

            if !spaces.is_empty() {
                trivia.push(Trivia::Spaces(spaces));
            } else if self.current_char() == Some('-') && self.consume_with_next('-') {
                trivia.push(Trivia::Comment(Comment::try_lex(self).unwrap()));
            } else {
                break;
            }
        }

        trivia
    }

    /// Get the whitespaces after the current positive and move the lexer to after
    /// them.
    pub fn skip_whitespace(&mut self) -> SmolStr {
        let start = self.byte_position;
        while let Some(character) = self.current_char() {
            if character.is_whitespace() {
                self.increment_position_by_char(character);
            } else {
                break;
            }
        }

        (start != self.byte_position)
            .then(|| self.input[start..self.byte_position].into())
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

/// A trait which means this item can be lexed.
pub trait Lexable: Sized {
    /// Try lexing the item.
    fn try_lex(lexer: &mut Lexer) -> Option<Self>;
}
