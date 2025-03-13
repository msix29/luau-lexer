use luau_lexer::prelude::{Lexer, Literal, LuauString, TokenType};

macro_rules! generate_string_tests {
    ($( $(#[$meta: meta])? $fn_name: ident => $enum: ident ($str: literal) ),* $(,)?) => {
        $(
            $(#[$meta])?
            #[test]
            fn $fn_name() {
                let mut lexer = Lexer::new($str);

                assert_eq!(
                    lexer.next_token().token_type,
                    TokenType::Literal(Literal::String(LuauString::$enum($str.to_string())))
                );
                assert_eq!(
                    lexer.next_token().token_type,
                    TokenType::EndOfFile
                );
                // This line should never error as the lexer will return errors
                // instead of the correct token types, and thus will error above
                // but it's here just in case.
                assert!(lexer.errors.is_empty());
            }
        )*
    };
}

generate_string_tests!(
    single_quotes_empty => SingleQuotes("''"),
    single_quotes => SingleQuotes("'single quotes test'"),
    single_quotes_multi_line => SingleQuotes(r#"'single quotes\z\ntest'"#),

    double_quotes_empty => DoubleQuotes(r#""""#),
    double_quotes => DoubleQuotes(r#""double quotes test""#),
    double_quotes_multi_line => DoubleQuotes(r#""double quotes\z\ntest""#),

    backticks_empty => Bacticks("``"),
    backticks => Bacticks("`backticks test`"),
    backticks_multi_line => Bacticks(r#"`backticks\z\ntest`"#),

    multi_line_empty_1 => MultiLine("[[]]"),
    multi_line_empty_2 => MultiLine("[==[]==]"),
    multi_line_empty_3 => MultiLine("[==[]==== ]==]"),

    multi_line_1 => MultiLine("[[multi-line test]]"),
    multi_line_2 => MultiLine("[==[multi-line test]==]"),
    multi_line_3 => MultiLine("[==[multi-line]==== test]==]"),

    multi_line_4 => MultiLine("[[\nmulti-line test\n]]"),
    multi_line_5 => MultiLine("[==[\n\n\nmulti-line test]==]"),
    multi_line_6 => MultiLine("[==[multi-\nline]====\n test \n]==]"),
);

generate_string_tests!(
    #[should_panic] erroneous_single_quotes_empty => SingleQuotes("'"),
    #[should_panic] erroneous_single_quotes => SingleQuotes("'single\nquotes test"),

    #[should_panic] erroneous_double_quotes_empty => DoubleQuotes(r#"""#),
    #[should_panic] erroneous_double_quotes => DoubleQuotes(r#""\nouble quotes test"#),

    #[should_panic] erroneous_backticks_empty => Bacticks("`"),
    #[should_panic] erroneous_backticks => Bacticks("`backticks\ntest"),

    #[should_panic] erroneous_multi_line_empty_1 => MultiLine("[]]"),
    #[should_panic] erroneous_multi_line_empty_2 => MultiLine("[==[]=]"),
    #[should_panic] erroneous_multi_line_empty_3 => MultiLine("[=[]==== ]==]"),

    #[should_panic] erroneous_multi_line_1 => MultiLine("[[multi-line test]=]"),
    #[should_panic] erroneous_multi_line_2 => MultiLine("[==[multi-line test]]"),
    #[should_panic] erroneous_multi_line_3 => MultiLine("[====[multi-line]==== test]==]"),

    #[should_panic] erroneous_multi_line_4 => MultiLine("[[\nmulti-line test\n]=]"),
    #[should_panic] erroneous_multi_line_5 => MultiLine("[==[\n\n\nmulti-line test]=]"),
    #[should_panic] erroneous_multi_line_6 => MultiLine("[==[multi-\nline]====\n test \n]=]"),
);
