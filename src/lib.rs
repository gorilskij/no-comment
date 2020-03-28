pub use crate::without_comments::{WithoutComments, IntoWithoutComments};

mod without_comments;

#[cfg(test)]
mod tests {
    use super::IntoWithoutComments;
    
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
        
        for string in strings.iter() {
            let without_comments = string
                .chars()
                .without_comments()
                .collect::<String>();
            
            assert_eq!(&without_comments, string);
        }
    }
    
    #[test]
    fn test_line_comments() {
        let strings = [
            ("With a line comment // this is it", "With a line comment "), // note the trailing space
            ("3 /// should work", "3 "),
            ("4 //// //", "4 "),
            ("With close block// */ would panic if it were text", "With close block"),
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
                .without_comments()
                .collect::<String>();
            
            assert_eq!(&without_comments, check);
        }
    }
    
    #[test]
    fn test_block_comments() {
        let strings = [
            ("With a /* block */ comment", "With a  comment"), // note the double space
            ("With a /* nested /* block */ comment */", "With a "), // note the trailing space
            ("With a /**/ short one", "With a  short one"),
            ("Con/***/fusing", "Confusing"),
            ("Auto-close/* unclosed", "Auto-close"),
            ("With/*\n*/ a newline", "With a newline"),
            ("Nested /* \n /* \n */ \n */newlines", "Nested newlines"),
            ("Line comment/* // this one */ ignored", "Line comment ignored"),
            ("/**/", ""),
            ("/*~*/", ""),
            ("/*\n\t//\nstill a comment*/", ""),
            ("One /* one *//* two */ Two", "One  Two"),
            ("A/* /* one *//* two *//* three /**/*/ */B", "AB"),
        ];
        
        for (string, check) in strings.iter() {
            let without_comments = string
                .chars()
                .without_comments()
                .collect::<String>();
            
            assert_eq!(&without_comments, check);
        }
    }
    
    #[test]
    #[should_panic]
    fn test_block_comments_panic() {
        let _ = "*/".chars().without_comments().collect::<String>();
    }
    
    #[test]
    fn test_combined() {
        let strings = [
            ("Line// comment\nAnd block/* comment */", "Line\nAnd block"),
            ("Block/* comment */// and a line", "Block"),
            ("//\n/**////*", "\n"),
            ("/* //\n still a comment", ""),
            ("/*S*/he /*be*/lie/*ve*/d", "he lied"),
            ("S/*he */be/*lie*/ve//d", "Sbeve"),
        ];
        
        for (string, check) in strings.iter() {
            let without_comments = string
                .chars()
                .without_comments()
                .collect::<String>();
                
            assert_eq!(&without_comments, check);
        }
    }
}