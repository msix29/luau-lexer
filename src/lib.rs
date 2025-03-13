macro_rules! reexport {
    ($($name: ident),* $(,)?) => {
        $( pub mod $name; )*

        pub mod prelude {
            $( pub use crate::$name::*; )*
        }
    };
}

mod utils;
reexport!(lexer, state, position, error);
