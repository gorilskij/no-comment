pub use without_comments::{WithoutComments, IntoWithoutComments};

mod without_comments;

/*
    For now the functionality is very basic, if a '* /' (close block comment) is encountered,
    it panics, if a block comment is not closed, it's assumed to be closed at the end of the string.
    Nested block comments are treated correctly.
    Will still return a '\n' for each line, even if the line is entirely a line comment.
    i.e. "//...\n//...\n//...\n" will return "\n\n\n"
*/