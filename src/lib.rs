#![deny(unsafe_code)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(missing_docs)]
#![allow(unused)]
#![warn(clippy::absolute_paths)]

macro_rules! reexport {
    ($($name: ident),* $(,)?) => {
        $( pub mod $name; )*

        pub mod prelude {
            $( pub use crate::$name::*; )*
        }
    };
}
macro_rules! crate_reexport {
    ($($name: ident),* $(,)?) => {
        $( pub mod $name; )*

        $( pub use $name::*; )*
    };
}
macro_rules! impl_from {
    ($struct: ident <= { $($enum: ident ($type: ty)),* $(,)? }) => {
        $(
            impl From<$type> for $struct {
                #[inline]
                fn from(value: $type) -> Self {
                    Self::$enum(value)
                }
            }
        )*
    };
}

mod utils;
reexport!(lexer, state, position, error, token);
