//! [`Keyword`] and [`PartialKeyword`] enums.

/// A macro to generate an enum for keywords.
macro_rules! generate_keyword_enum {
    ($(#[$meta:meta])* $vis:vis enum $struct: ident {
        $( $(#[$name_meta:meta])* $str: literal => $name: ident ),* $(,)?
    }) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        $vis enum $struct {
            $( $(#[$name_meta])* $name, )*
        }

        impl $struct {
            /// Try creating this item from a string.
            pub fn try_from_str(value: &str) -> Option<Self> {
                match value {
                    $( $str => Some(Self::$name), )*
                    _ => None,
                }
            }
        }

        impl std::fmt::Display for $struct {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(match self {
                    $( Self::$name => $str, )*
                })
            }
        }
    };
}

generate_keyword_enum!(
    /// Words that can only be used as be keywords. Check [`PartialKeyword`].
    pub enum Keyword {
        /// The `local` keyword.
        "local" => Local,

        /// The `function` keyword.
        "function" => Function,

        /// The `if` keyword.
        "if" => If,

        /// The `elseif` keyword.
        "elseif" => Elseif,

        /// The `then` keyword.
        "then" => Then,

        /// The `else` keyword.
        "else" => Else,

        /// The `while` keyword.
        "while" => While,

        /// The `for` keyword.
        "for" => For,

        /// The `in` keyword.
        "in" => In,

        /// The `do` keyword.
        "do" => Do,

        /// The `break` keyword.
        "break" => Break,

        /// The `return` keyword.
        "return" => Return,

        /// The `end` keyword.
        "end" => End,

        /// The `repeat` keyword.
        "repeat" => Repeat,

        /// The `until` keyword.
        "until" => Until,

        /// The `nil` keyword.
        "nil" => Nil,
    }
);
generate_keyword_enum!(
    /// Words that can be keywords or identifiers, depending on the context.
    /// Check [`Keyword`].
    pub enum PartialKeyword {
        /// The `type` keyword
        "type" => Type,

        /// The `continue` keyword
        "continue" => Continue,

        /// The `export` keyword
        "export" => Export,

        /// The `typeof` keyword
        "typeof" => TypeOf,
    }
);
