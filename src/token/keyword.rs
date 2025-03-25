macro_rules! generate_keyword_enum {
    ($(#[$meta:meta])? $vis:vis enum $struct: ident {
        $( $str: literal => $name: ident ),* $(,)?
    }) => {
        $(#[$meta])?
        #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        $vis enum $struct {
            $( $name, )*
        }

        impl $struct {
            pub fn try_from_str(value: &str) -> Option<Self> {
                match value {
                    $( $str => Some(Self::$name), )*
                    _ => None,
                }
            }
        }
    };
}

generate_keyword_enum!(
    pub enum Keyword {
        "local" => Local,
        "function" => Function,
        "if" => If,
        "elseif" => Elseif,
        "then" => Then,
        "else" => Else,
        "while" => While,
        "for" => For,
        "in" => In,
        "do" => Do,
        "break" => Break,
        "return" => Return,
        "end" => End,
        "repeat" => Repeat,
        "until" => Until,
        "nil" => Nil,
    }
);
generate_keyword_enum!(
    pub enum PartialKeyword {
        "type" => Type,
        "continue" => Continue,
        "export" => Export,
        "typeof" => TypeOf,
    }
);
