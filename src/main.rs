use crate::without_comments::IntoWithoutComments;

mod without_comments;

/*
    For now the functionality is very basic, if a '* /' (close block comment) is encountered,
    it panics, if a block comment is not closed, it's assumed to be closed at the end of the string.
    Nested block comments are treated correctly.
*/

fn main() {
    let text = "S/*he */be/*lie*/ve//d";
    let truth = text.chars().without_comments().collect::<String>();
    println!("{}", truth);
}