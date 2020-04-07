# No Comment

![Version](https://img.shields.io/badge/Version-0.0.2-red.svg)
![Minimum Rust version: 1.36](https://img.shields.io/badge/Minimum%20Rust%20Version-1.36-brightgreen.svg)

Remove comments from a `char` iterator.

This crate provides the `WithoutComments` iterator and the `IntoWithoutComments` trait implemented for
all `Iterator<Item=char>` providing the `without_comments` method. Comment specifications are available for
rust-style, c-style, python-style, and haskell-style line and block comments, a way to specify custom
comment specifications is planned. This crate is intended to be used for removing comments from text,
not from code, for this reason, `"\*"` will still open a block comment in rust mode because string literals
have no semantic significance.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
without-comments = "0.0.2"
```

main.rs:
```rust
fn main() {
        use std::fs::read_to_string;
        use no_comment::{IntoWithoutComments as _, languages};

        let without_comments = read_to_string("tests.txt")
            .unwrap()
            .chars()
            .without_comments(languages::rust())
            .collect::<String>();

        println!("{}", without_comments);
}
```

test.txt:
```text
This is text // this is a (rust) line comment
This is more text /* this is a (rust) block comment
/* this one is nested */ */
This is text again
/* If a comment is left open, it keeps
going until the end.
```

output:
```text
This is text 
This is more text 
This is text again

```

Note that trailing spaces and newlines are preserved, showing whitespace
the output looks like this:

```text
This·is·text·¶
This·is·more·text·¶
This·is·text·again¶

```