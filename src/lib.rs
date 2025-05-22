#![doc = include_str!("../README.md")]
#![deny(unsafe_code)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(missing_docs)]
#![warn(clippy::absolute_paths)]
#![warn(clippy::missing_const_for_fn)]
#![deny(unused_must_use)]
#![deny(dead_code)]
#![deny(unused_assignments)]
#![warn(clippy::missing_errors_doc)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::doc_markdown)]
#![warn(clippy::module_name_repetitions)]
#![warn(clippy::wildcard_imports)]
#![warn(clippy::too_many_arguments)]
#![warn(clippy::large_types_passed_by_value)]
#![warn(clippy::needless_pass_by_value)]
#![warn(clippy::inefficient_to_string)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![warn(clippy::nursery)]

/// A simple macro to reexport modules and include them in [`prelude`].
macro_rules! reexport {
    ($($name: ident $({$($inner:ident as $inner_name:ident),+ $(,)?})?),* $(,)?) => {
        $( pub mod $name; )*

        /// Loads all needed items for outside crates to use.
        pub mod prelude {
            $(
                pub use crate::$name::*;
                $($( pub use crate::$name::$inner as $inner_name; )+)?
            )*
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

reexport!(
    lexer,
    state,
    position { Ext as PositionExt },
    error,
    token
);
