#[macro_use]
extern crate derive_more;

pub use without_comments::{IntoWithoutComments, WithoutComments};

// TODO redocument, update readmes, mention that comments started in strings are still comments ("/*" starts a block comment)

pub mod languages;

mod without_comments;

#[cfg(test)]
mod tests {
    use super::IntoWithoutComments;
    use crate::languages;

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
        ];

        use languages::*;

        for language in &[rust, c, python, haskell] {
            for string in strings.iter() {
                let without_comments = string
                    .chars()
                    .without_comments(language())
                    .collect::<String>();

                assert_eq!(
                    &without_comments, string,
                    "\"{}\" should be \"{}\"",
                    without_comments, string
                );
            }
        }
    }

    #[test]
    fn test_line_comments_rust_and_c() {
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

        use languages::{c, rust};

        for language in &[rust, c] {
            for (string, check) in strings.iter() {
                let without_comments = string
                    .chars()
                    .without_comments(language())
                    .collect::<String>();

                assert_eq!(&without_comments, check);
            }
        }
    }

    #[test]
    fn test_line_comments_python() {
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

        use languages::{c, rust};

        for (string, check) in strings.iter() {
            let without_comments = string
                .chars()
                .without_comments(languages::python())
                .collect::<String>();

            assert_eq!(&without_comments, check);
        }
    }

    #[test]
    fn test_line_comments_haskell() {
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
        ];

        for (string, check) in strings.iter() {
            let without_comments = string
                .chars()
                .without_comments(languages::haskell())
                .collect::<String>();

            assert_eq!(&without_comments, check);
        }
    }

    #[test]
    fn test_block_comments_rust() {
        let strings = [
            ("With a /* block */ comment", "With a  comment"), // note the double space
            ("With a /* nested /* block */ comment */", "With a "), // note the trailing space
            ("With a /**/ short one", "With a  short one"),
            ("Con/***/fusing", "Confusing"),
            ("Auto-close/* unclosed", "Auto-close"),
            ("With/*\n*/ a newline", "With a newline"),
            ("Nested /* \n /* \n */ \n */newlines", "Nested newlines"),
            (
                "Line comment/* // this one */ ignored",
                "Line comment ignored",
            ),
            ("/**/", ""),
            ("/*~*/", ""),
            ("/*\n\t//\nstill a comment*/", ""),
            ("One /* one *//* two */ Two", "One  Two"),
            ("A/* /* one *//* two *//* three /**/*/ */B", "AB"),
        ];

        for (string, check) in strings.iter() {
            let without_comments = string
                .chars()
                .without_comments(languages::rust())
                .collect::<String>();

            assert_eq!(&without_comments, check);
        }
    }

    #[test]
    fn test_block_comments_c() {
        let strings = [
            ("With a /* block */ comment", "With a  comment"),
            ("Nested /* open pattern /* ignored */ text", "Nested  text"),
            ("With a /**/ short one", "With a  short one"),
            ("Con/***/fusing", "Confusing"),
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
                .without_comments(languages::c())
                .collect::<String>();

            assert_eq!(&without_comments, check);
        }
    }

    #[test]
    fn test_block_comments_python() {
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
                .without_comments(languages::python())
                .collect::<String>();

            assert_eq!(&without_comments, check);
        }
    }

    #[test]
    fn test_block_comments_haskell() {
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
        ];

        for (string, check) in strings.iter() {
            let without_comments = string
                .chars()
                .without_comments(languages::haskell())
                .collect::<String>();

            assert_eq!(&without_comments, check);
        }
    }

    #[test]
    #[should_panic]
    fn test_block_comments_panic_rust() {
        let _ = "*/"
            .chars()
            .without_comments(languages::rust())
            .collect::<String>();
    }

    #[test]
    #[should_panic]
    fn test_block_comments_panic_c() {
        let _ = "*/"
            .chars()
            .without_comments(languages::c())
            .collect::<String>();
    }

    #[test]
    #[should_panic]
    fn test_block_comments_panic_haskell() {
        let _ = "-}"
            .chars()
            .without_comments(languages::haskell())
            .collect::<String>();
    }

    // TODO add python and haskell to the mix
    #[test]
    fn test_combined() {
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
                .without_comments(languages::rust())
                .collect::<String>();

            assert_eq!(&without_comments, check);
        }
    }
}
