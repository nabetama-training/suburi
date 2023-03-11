//! Given a string s consisting of words and spaces,
//! return the length of the last word in the string.
//! A word is a maximal
//! substring
//!  consisting of non-space characters only.
pub fn length_of_last_word(s: String) -> i32 {
    s.split_whitespace().last().unwrap_or("").len() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_length_of_last_word() {
        struct TestCase {
            input: String,
            output: i32,
        }
        let test_cases = [
            TestCase {
                input: "Hello Wolrd".to_string(),
                output: 5,
            },
            TestCase {
                input: "   fly me   to   the moon  ".to_string(),
                output: 4,
            },
            TestCase {
                input: "luffy is still joyboy".to_string(),
                output: 6,
            },
        ];
        for tc in test_cases {
            assert_eq!(tc.output, length_of_last_word(tc.input));
        }
    }
}
