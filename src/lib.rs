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

mod utils;
reexport!(lexer, state, position, error, token);
