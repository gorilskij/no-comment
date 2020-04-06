use lazy_static::lazy_static;
use crate::IntoWithoutComments;
use crate::languages::c;

#[test]
fn test_no_comments() {
    let strings = [
        "This is a string",
        "This is one\nwith a newline",
        "One with a trailing newline\n",
        "\nA leading newline",
        "\nBoth\n",
        "\n",
        "\n\n",
        "\n \n",
        "Double  space",
        " Leading",
        "Trailing ",
        " Both again ",
        "          ",
        "",
        "A # python line comment",
        "A ''' python block comment '''",
        "Another \"\"\" one, this time unclosed",
        "A -- haskell line comment",
        "A {- haskell -} block {- comment",
    ];

    for string in strings.iter() {
        let without_comments = string
            .chars()
            .without_comments(c())
            .collect::<String>();

        assert_eq!(
            &without_comments, string,
            "\"{}\" should be \"{}\"",
            without_comments, string
        );
    }
}

#[test]
fn test_line_comments() {
    let strings = [
        ("With a line comment // this is it", "With a line comment "), // note the trailing space
        ("3 /// should work", "3 "),
        ("4 //// //", "4 "),
        (
            "With close block// */ would panic if it were text",
            "With close block",
        ),
        ("Even closer //*}", "Even closer "),
        ("// Just comment", ""),
        ("//", ""),
        ("///", ""),
        ("A//\nB", "A\nB"),
        ("//\n//\n", "\n\n"),
        ("Not a block //* */ still line comment", "Not a block "),
    ];

    for (string, check) in strings.iter() {
        let without_comments = string
            .chars()
            .without_comments(c())
            .collect::<String>();

        assert_eq!(&without_comments, check);
    }
}

#[test]
fn test_block_comments() {
    let strings = [
        ("With a /* block */ comment", "With a  comment"),
        ("Nested /* open pattern /* ignored */ text", "Nested  text"),
        ("With a /**/ short one", "With a  short one"),
        ("c()on/***/fusing", "c()onfusing"),
        ("Auto-close/* unclosed", "Auto-close"),
        ("With/*\n*/ a newline", "With a newline"),
        (
            "Nested /* \n /* <- useless \n */newlines",
            "Nested newlines",
        ),
        (
            "Line comment/* // this one */ ignored",
            "Line comment ignored",
        ),
        ("/**/", ""),
        ("/*~*/", ""),
        ("/*\n\t//\nstill /* /* a comment*/", ""),
        ("One /* one *//* two */ Two", "One  Two"),
        ("A/* not_open -> /*/B", "AB"),
    ];

    for (string, check) in strings.iter() {
        let without_comments = string
            .chars()
            .without_comments(c())
            .collect::<String>();

        assert_eq!(&without_comments, check);
    }
}

#[test]
#[should_panic]
fn test_block_comment_close_panic() {
    let _ = "*/"
        .chars()
        .without_comments(c())
        .collect::<String>();
}

#[test]
fn test_block_and_line_together() {
    let strings = [
        ("Line// comment\nAnd block/* comment */", "Line\nAnd block"),
        ("Block/* comment */// and a line", "Block"),
        ("//\n/**////*", "\n"),
        ("/* //\n still a comment", ""),
        ("Unclosed //", "Unclosed "),
        ("/*S*/he /*be*/lie/*ve*/d", "he lied"),
        ("S/*he */be/*lie*/ve//d", "Sbeve"),
    ];

    for (string, check) in strings.iter() {
        let without_comments = string
            .chars()
            .without_comments(c())
            .collect::<String>();

        assert_eq!(&without_comments, check);
    }
}