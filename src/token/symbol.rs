macro_rules! generate_symbols {
    ($(#[$meta:meta])? $vis:vis enum $struct: ident {
        $( $char: literal => $name: ident ),* $(,)?
    }) => {
        $(#[$meta])?
        #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
        $vis enum $struct {
            $( $name, )*
            Dot,
            Ellipses,
        }

        impl $struct {
            pub fn try_from_char(value: char) -> Option<Self> {
                match value {
                    $( $char => Some(Self::$name), )*
                    _ => None,
                }
            }
        }
    };
}

generate_symbols!(
    pub enum Symbol {
        '{' => OpeningCurlyBrackets,
        '}' => ClosingCurlyBrackets,

        '[' => OpeningBrackets,
        ']' => ClosingBrackets,

        '<' => OpeningAngleBrackets,
        '>' => ClosingAngleBrackets,

        '(' => OpeningParenthesis,
        ')' => ClosingParenthesis,

        ';' => Semicolon,
        ':' => Colon,

        ',' => Comma,
        // These are handled manually in the lexer.
        // '.' => Dot,
        // '...' => Ellipses,
    }
);
