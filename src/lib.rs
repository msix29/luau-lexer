//! # Luau Lexer
//!
//! A lossless lexer for the luau programming language. Lexes source code into
//! tokens while preserving all spaces and comments.
//!
//! # Usage:
//!
//! ```rust
//! use luau_lexer::prelude::Lexer;
//!
//! let code = r#"local foo = "Hello, World!""#;
//! let mut lexer = Lexer::new(code);
//!
//! let mut token = lexer.next_token();
//! while token != TokenType::EndOfFile {
//!     println!("{token:?}");
//!
//!     token = lexer.next_token();
//! }
//! ```
//!
//! # Note
//!
//! * This lexer does not stop parsing when it finds an error
//! * This lexer only lexes each token when asked to do so.

#![deny(unsafe_code)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(missing_docs)]
#![allow(unused)]
#![warn(clippy::absolute_paths)]

/// A simple macro to reexport modules and include them in [`prelude`].
macro_rules! reexport {
    ($($name: ident),* $(,)?) => {
        $( pub mod $name; )*

        /// Loads all needed items for outside crates to use.
        pub mod prelude {
            $( pub use crate::$name::*; )*
        }
    };
}

/// A simple macro to reexport modules without including them in [`prelude`].
macro_rules! crate_reexport {
    ($($name: ident),* $(,)?) => {
        $( pub mod $name; )*

        $( pub use $name::*; )*
    };
}

/// Implements the [`From`] trait for the passed structs. This is meant for
/// enums only.
///
/// # Usage
///
/// ```ignore
/// impl_from!(name <= {
///     enum1::struct1,
/// }) // This will implement `From<enum1::struct1>` for `name`
///
/// ```
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
