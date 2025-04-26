# Luau Lexer

A lossless lexer for the luau programming language. Lexes source code into
tokens while preserving all spaces and comments.

## Usage

```rust
use luau_lexer::prelude::{Lexer, TokenType};
let code = r#"local foo = "Hello, World!""#;
let mut lexer = Lexer::new(code);
let mut token = lexer.next_token();

loop {
    println!("{token:?}");
    if token == TokenType::EndOfFile {
        break;
    }

    token = lexer.next_token();
}
```

## Note

* This lexer does not stop parsing when it finds an error
* This lexer only lexes each token when asked to do so.
