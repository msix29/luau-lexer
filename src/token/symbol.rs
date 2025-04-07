//! [`Symbol`] struct.

use std::fmt::Write;

/// Generates the [`Symbol`] enum.
macro_rules! generate_symbols {
    ($(#[$meta:meta])* $vis:vis enum $struct: ident {
        $( $(#[$name_meta:meta])* $char: literal => $name: ident ),* $(,)?
    }) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        $vis enum $struct {
            $( $(#[$name_meta])*  $name, )*
            /// `.`
            Dot,

            /// `...`
            Ellipses,

            /// `->`
            Arrow,

            /// `::`
            Typecast,
        }

        impl $struct {
            /// Try creating the current item from a character
            pub fn try_from_char(character: char, lexer: &mut crate::prelude::Lexer) -> Option<Self> {
                let value = match character {
                    $( $char => Some(Self::$name), )*
                    _ => None,
                };
                if value.is_some() {
                    lexer.consume(character);
                }

                value
            }
        }

        impl std::fmt::Display for $struct {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::Dot => f.write_str("."),
                    Self::Ellipses => f.write_str("..."),
                    Self::Arrow => f.write_str("->"),
                    Self::Typecast => f.write_str("::"),
                    $( Self::$name => f.write_char($char), )*
                }
            }
        }
    };
}

generate_symbols!(
    /// A luau symbol like `(` and `)`
    pub enum Symbol {
        /// `{`
        '{' => OpeningCurlyBrackets,

        /// `}`
        '}' => ClosingCurlyBrackets,

        /// `[`
        '[' => OpeningBrackets,

        /// `]`
        ']' => ClosingBrackets,

        /// `<`
        '<' => OpeningAngleBrackets,

        /// `>`
        '>' => ClosingAngleBrackets,

        /// `(`
        '(' => OpeningParenthesis,

        /// `)`
        ')' => ClosingParenthesis,

        /// `;`
        ';' => Semicolon,

        /// `:`
        ':' => Colon,

        /// `=`
        '=' => Equal,

        /// `,`
        ',' => Comma,
    }
);
