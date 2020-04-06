# No Comment

![Version](https://img.shields.io/badge/Version-0.0.1-red.svg)
![Minimum Rust version: 1.31](https://img.shields.io/badge/Minimum%20Rust%20Version-1.31-brightgreen.svg)

Remove rust-style line and block comments from a `char` iterator.

This crate provides the `WithoutComments` iterator and the `IntoWithoutComments` trait implemented for
all `Iterator<Item=char>` providing the `without_comments` method.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
without-comments = "0.0.1"
```

main.rs:
```rust
fn main() {
        use std::fs::read_to_string;

        let without_comments = read_to_string("tests.txt")
            .unwrap()
            .chars()
            .without_comments()
            .collect::<String>();

        println!("{}", without_comments);
}
```

test.txt:
```text
This is text // this is a line comment
This is more text /* this is a block comment
/* this one is nested */ */
This is text again
/* If a block comment is left open, it keeps
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

## Dependencies

This crate has no external dependencies