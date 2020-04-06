use crate::languages::haskell;
use crate::IntoWithoutComments as _;

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
        "A // rust line comment",
        "A /* rust block comment */",
        "A /*/* nested */*/ one",
        "Another /* one, this time unclosed",
        "A # python line comment",
        "A ''' python ''' block \"\"\" comment",
    ];

    for string in strings.iter() {
        let without_comments = string
            .chars()
            .without_comments(haskell())
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
        ("With a line comment -- this is it", "With a line comment "),
        ("3 --- should work", "3 "),
        ("4 ---- --", "4 "),
        (
            "With close block-- -} would panic if it were text",
            "With close block",
        ),
        ("Even closer ---}", "Even closer "),
        ("-- Just comment", ""),
        ("--", ""),
        ("---", ""),
        ("A--\nB", "A\nB"),
        ("--\n--\n", "\n\n"),
        (
            "Not a block --{--} still {- -} line comment",
            "Not a block ",
        ),
        (
            "String literals \"--\" are ignored, this is a comment",
            "String literals \"",
        ),
    ];

    for (string, check) in strings.iter() {
        let without_comments = string
            .chars()
            .without_comments(haskell())
            .collect::<String>();

        assert_eq!(&without_comments, check);
    }
}

#[test]
fn test_block_comments() {
    let strings = [
        ("With a {- block -} comment", "With a  comment"), // note the double space
        ("With a {- nested {- block -} comment -}", "With a "), // note the trailing space
        ("With a {--} short one", "With a  short one"),
        ("Con{---}fusing", "Confusing"),
        ("Auto-close{- unclosed", "Auto-close"),
        ("With{-\n-} a newline", "With a newline"),
        ("Nested {- \n {- \n -} \n -}newlines", "Nested newlines"),
        (
            "Line comment{- -- this one -} ignored",
            "Line comment ignored",
        ),
        ("{--}", ""),
        ("{-~-}", ""),
        ("{-\n\t//\nstill a comment-}", ""),
        ("One {- one -}{- two -} Two", "One  Two"),
        ("A{- {- one -}{- two -}{- three {--}-} -}B", "AB"),
        (
            "String \" literals {- comment \" -}are ignored",
            "String \" literals are ignored",
        ),
    ];

    for (string, check) in strings.iter() {
        let without_comments = string
            .chars()
            .without_comments(haskell())
            .collect::<String>();

        assert_eq!(&without_comments, check);
    }
}

#[test]
#[should_panic]
fn test_block_comment_close_panic() {
    let _ = "-}".chars().without_comments(haskell()).collect::<String>();
}

#[test]
fn test_block_and_line_together() {
    let strings = [
        ("Line-- comment\nAnd block{- comment -}", "Line\nAnd block"),
        ("Block{- comment -}-- and a line", "Block"),
        ("--\n{--}--{-", "\n"),
        ("{- --\n still a comment", ""),
        ("Unclosed --", "Unclosed "),
        ("{-S-}he {-be-}lie{-ve-}d", "he lied"),
        ("S{-he -}be{-lie-}ve--d", "Sbeve"),
    ];

    for (string, check) in strings.iter() {
        let without_comments = string
            .chars()
            .without_comments(haskell())
            .collect::<String>();

        assert_eq!(&without_comments, check);
    }
}
