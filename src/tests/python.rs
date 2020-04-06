use lazy_static::lazy_static;
use crate::IntoWithoutComments;
use crate::languages::python;

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
        "A -- haskell line comment",
        "A {- haskell -} block {- comment",
    ];

    for string in strings.iter() {
        let without_comments = string
            .chars()
            .without_comments(python())
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
        ("With a line comment # this is it", "With a line comment "),
        ("2 ## should work", "2 "),
        ("4 #### ##", "4 "),
        (
            "With block# ''' <- would panic if it were text",
            "With block",
        ),
        ("# Just comment", ""),
        ("#", ""),
        ("##", ""),
        ("A#\nB", "A\nB"),
        ("#\n#\n", "\n\n"),
        (
            "text#comment#still comment'''same comment'''comment",
            "text",
        ),
    ];

    for (string, check) in strings.iter() {
        let without_comments = string
            .chars()
            .without_comments(python())
            .collect::<String>();

        assert_eq!(&without_comments, check);
    }
}

#[test]
fn test_block_comments() {
    let strings = [
        ("With a ''' block ''' comment", "With a  comment"),
        (
            "With a \"\"\" different block \"\"\" comment",
            "With a  comment",
        ),
        ("One ''' in \"\"\" the ''' other", "One  other"),
        ("And \"\"\" the ''' other \"\"\" way", "And  way"),
        // ("Nested /* open pattern /* ignored */ text", "Nested  text"),
        ("With a '''''' short one", "With a  short one"),
        ("And \"\"\"\"\"\" another", "And  another"),
        ("Con''' '' ' '' '''fusing", "Confusing"),
        ("Auto-close''' unclosed", "Auto-close"),
        ("And another\"\"\" unprinted", "And another"),
        ("With'''\n''' a newline", "With a newline"),
        ("With\"\"\"\n\n\"\"\" two newlines", "With two newlines"),
        (
            "Line comment''' # this one ''' ignored",
            "Line comment ignored",
        ),
        ("''''''", ""),
        ("\"\"\"~\"\"\"", ""),
        ("'''\n\t//\nstill'\"\"\"''a comment'''", ""),
        ("One ''' one '''''' two ''' Two", "One  Two"),
        ("A''''' <- first 3 open, next 2 ignored '''B", "AB"),
    ];

    for (string, check) in strings.iter() {
        let without_comments = string
            .chars()
            .without_comments(python())
            .collect::<String>();

        assert_eq!(&without_comments, check);
    }
}

#[test]
fn test_block_and_line_together() {
    let strings = [
        ("Line# comment\nAnd block''' comment '''", "Line\nAnd block"),
        ("Block\"\"\" comment \"\"\"# and a line", "Block"),
        ("#\n''''''#'''", "\n"),
        ("\"\"\" #\n still a comment", ""),
        ("Unclosed #", "Unclosed "),
        ("'''S'''he \"\"\"be\"\"\"lie'''ve'''d", "he lied"),
        ("S\"\"\"he \"\"\"be'''lie'''ve#d", "Sbeve"),
    ];

    for (string, check) in strings.iter() {
        let without_comments = string
            .chars()
            .without_comments(python())
            .collect::<String>();

        assert_eq!(&without_comments, check);
    }
}